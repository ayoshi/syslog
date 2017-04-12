use std::collections;
use std::fs;
use std::io;
use std::fmt;
use std::io::{Read, Write, BufReader};

use std::net::{SocketAddr, TcpStream};
use std::process;
use std::str;
use std::sync::Arc;

use rustls;
use rustls::{Session, ClientSession};
use webpki_roots;

/// This encapsulates the TCP-level connection, some connection
/// state, and the underlying TLS-level session.
pub struct TlsClient {
    socket: TcpStream,
    closing: bool,
    clean_closure: bool,
    tls_session: rustls::ClientSession,
}

// TODO: FIx
impl fmt::Debug for TlsClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "TLSCLIENT: {:?}", self.socket)
    }

}

/// We implement `io::Write` and pass through to the TLS session
impl io::Write for TlsClient {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.tls_session.write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.tls_session.flush()
    }
}

impl io::Read for TlsClient {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.tls_session.read(bytes)
    }
}

impl TlsClient {
    pub fn new(sock: TcpStream, hostname: &str, cfg: Arc<rustls::ClientConfig>) -> TlsClient {
        TlsClient {
            socket: sock,
            closing: false,
            clean_closure: false,
            tls_session: rustls::ClientSession::new(&cfg, hostname),
        }
    }

    fn read_source_to_end(&mut self, rd: &mut io::Read) -> io::Result<usize> {
        let mut buf = Vec::new();
        let len = try!(rd.read_to_end(&mut buf));
        self.tls_session.write_all(&buf).unwrap();
        Ok(len)
    }

    /// We're ready to do a read.
    fn do_read(&mut self) {
        // Read TLS data.  This fails if the underlying TCP connection
        // is broken.
        let rc = self.tls_session.read_tls(&mut self.socket);
        if rc.is_err() {
            println!("TLS read error: {:?}", rc);
            self.closing = true;
            return;
        }

        // If we're ready but there's no data: EOF.
        if rc.unwrap() == 0 {
            println!("EOF");
            self.closing = true;
            self.clean_closure = true;
            return;
        }

        // Reading some TLS data might have yielded new TLS
        // messages to process.  Errors from this indicate
        // TLS protocol problems and are fatal.
        let processed = self.tls_session.process_new_packets();
        if processed.is_err() {
            println!("TLS error: {:?}", processed.unwrap_err());
            self.closing = true;
            return;
        }

        // Having read some TLS data, and processed any new messages,
        // we might have new plaintext as a result.
        //
        // Read it and then write it to stdout.
        let mut plaintext = Vec::new();
        let rc = self.tls_session.read_to_end(&mut plaintext);
        if !plaintext.is_empty() {
            io::stdout().write_all(&plaintext).unwrap();
        }

        // If that fails, the peer might have started a clean TLS-level
        // session closure.
        if rc.is_err() {
            let err = rc.unwrap_err();
            println!("Plaintext read error: {:?}", err);
            self.clean_closure = err.kind() == io::ErrorKind::ConnectionAborted;
            self.closing = true;
            return;
        }
    }

    fn do_write(&mut self) {
        self.tls_session.write_tls(&mut self.socket).unwrap();
    }

    fn is_closed(&self) -> bool {
        self.closing
    }
}

#[derive(Debug)]
struct Args {
    flag_port: Option<u16>,
    flag_http: bool,
    flag_verbose: bool,
    flag_suite: Vec<String>,
    flag_proto: Vec<String>,
    flag_mtu: Option<usize>,
    flag_cafile: Option<String>,
    flag_no_tickets: bool,
    flag_auth_key: Option<String>,
    flag_auth_certs: Option<String>,
    arg_hostname: String,
}

// TODO: um, well, it turns out that openssl s_client/s_server
// that we use for testing doesn't do ipv6.  So we can't actually
// test ipv6 and hence kill this.
fn lookup_ipv4(host: &str, port: u16) -> SocketAddr {
    use std::net::ToSocketAddrs;

    let addrs = (host, port).to_socket_addrs().unwrap();
    for addr in addrs {
        if let SocketAddr::V4(_) = addr {
            return addr;
        }
    }

    unreachable!("Cannot lookup address");
}

/// Find a ciphersuite with the given name
fn find_suite(name: &str) -> Option<&'static rustls::SupportedCipherSuite> {
    for suite in &rustls::ALL_CIPHERSUITES {
        let sname = format!("{:?}", suite.suite).to_lowercase();

        if sname == name.to_string().to_lowercase() {
            return Some(suite);
        }
    }

    None
}

/// Make a vector of ciphersuites named in `suites`
fn lookup_suites(suites: &[String]) -> Vec<&'static rustls::SupportedCipherSuite> {
    let mut out = Vec::new();

    for csname in suites {
        let scs = find_suite(csname);
        match scs {
            Some(s) => out.push(s),
            None => panic!("cannot look up ciphersuite '{}'", csname),
        }
    }

    out
}

fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
    let certfile = fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls::internal::pemfile::certs(&mut reader).unwrap()
}

fn load_private_key(filename: &str) -> rustls::PrivateKey {
    let keyfile = fs::File::open(filename).expect("cannot open private key file");
    let mut reader = BufReader::new(keyfile);
    let keys = rustls::internal::pemfile::rsa_private_keys(&mut reader).unwrap();
    assert!(keys.len() == 1);
    keys[0].clone()
}

fn load_key_and_cert(config: &mut rustls::ClientConfig, keyfile: &str, certsfile: &str) {
    let certs = load_certs(certsfile);
    let privkey = load_private_key(keyfile);

    config.set_single_client_cert(certs, privkey);
}

/// Build a `ClientConfig` from our arguments
pub fn make_config(args: &Args) -> Arc<rustls::ClientConfig> {
    let mut config = rustls::ClientConfig::new();

    if !args.flag_suite.is_empty() {
        config.ciphersuites = lookup_suites(&args.flag_suite);
    }

    if args.flag_cafile.is_some() {
        let cafile = args.flag_cafile.as_ref().unwrap();

        let certfile = fs::File::open(&cafile).expect("Cannot open CA file");
        let mut reader = BufReader::new(certfile);
        config.root_store
            .add_pem_file(&mut reader)
            .unwrap();
    } else {
        config.root_store.add_trust_anchors(&webpki_roots::ROOTS);
    }

    if args.flag_no_tickets {
        config.enable_tickets = false;
    }

    config.set_protocols(&args.flag_proto);
    config.set_mtu(&args.flag_mtu);

    if args.flag_auth_key.is_some() || args.flag_auth_certs.is_some() {
        load_key_and_cert(&mut config,
                          args.flag_auth_key
                              .as_ref()
                              .expect("must provide --auth-key with --auth-certs"),
                          args.flag_auth_certs
                              .as_ref()
                              .expect("must provide --auth-certs with --auth-key"));
    }

    Arc::new(config)
}

// Parse some arguments, then make a TLS client connection
// somewhere.
// fn main() {
//     let version = env!("CARGO_PKG_NAME").to_string() + ", version: " + env!("CARGO_PKG_VERSION");

//     // let port = args.flag_port.unwrap_or(443);
//     // let addr = lookup_ipv4(args.arg_hostname.as_str(), port);

//     // let config = make_config(&args);

//     let sock = TcpStream::connect(&addr).unwrap();
//     // let mut tlsclient = TlsClient::new(sock, &args.arg_hostname, config);

//     // if args.flag_http {
//     //     let httpreq = format!("GET / HTTP/1.1\r\nHost: {}\r\nConnection: \
//     //                            close\r\nAccept-Encoding: identity\r\n\r\n",
//     //                           args.arg_hostname);
//     //     tlsclient.write_all(httpreq.as_bytes()).unwrap();
//     // } else {
//     //     let mut stdin = io::stdin();
//     //     tlsclient.read_source_to_end(&mut stdin).unwrap();
//     // }

// }
