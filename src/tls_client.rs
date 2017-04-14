use std::fs;
use std::io;
use std::fmt;
use std::io::BufReader;

use std::net::TcpStream;
use std::str;
use std::sync::Arc;

use rustls;
use webpki_roots;

/// This encapsulates the TCP-level connection, some connection
/// state, and the underlying TLS-level session.
pub struct TlsClient {
    socket: TcpStream,
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
            tls_session: rustls::ClientSession::new(&cfg, hostname),
        }
    }
}

#[derive(Debug, Default)]
pub struct TLSSessionConfig {
    suite: Vec<String>,
    proto: Vec<String>,
    mtu: Option<usize>,
    cafile: Option<String>,
    no_tickets: bool,
    auth_key: Option<String>,
    auth_certs: Option<String>,
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
pub fn lookup_suites(suites: &[String]) -> Vec<&'static rustls::SupportedCipherSuite> {
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
    println!("{:?}", keys);
    assert!(keys.len() == 1);
    keys[0].clone()
}

fn load_key_and_cert(config: &mut rustls::ClientConfig, keyfile: &str, certsfile: &str) {
    let certs = load_certs(certsfile);
    let privkey = load_private_key(keyfile);

    config.set_single_client_cert(certs, privkey);
}

/// Build a `ClientConfig` from our arguments
pub fn make_config(args: &TLSSessionConfig) -> Arc<rustls::ClientConfig> {
    let mut config = rustls::ClientConfig::new();

    if !args.suite.is_empty() {
        config.ciphersuites = lookup_suites(&args.suite);
    }

    if args.cafile.is_some() {
        let cafile = args.cafile.as_ref().unwrap();

        let certfile = fs::File::open(&cafile).expect("Cannot open CA file");
        let mut reader = BufReader::new(certfile);
        config.root_store
            .add_pem_file(&mut reader)
            .unwrap();
    } else {
        config.root_store.add_trust_anchors(&webpki_roots::ROOTS);
    }

    if args.no_tickets {
        config.enable_tickets = false;
    }

    config.set_protocols(&args.proto);
    config.set_mtu(&args.mtu);

    if args.auth_key.is_some() || args.auth_certs.is_some() {
        load_key_and_cert(&mut config,
                          args.auth_key
                              .as_ref()
                              .expect("must provide auth-key with auth-certs"),
                          args.auth_certs
                              .as_ref()
                              .expect("must provide auth-certs with auth-key"));
    }

    Arc::new(config)
}
