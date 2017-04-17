use std::fs;
use std::io;
use std::fmt;
use std::io::BufReader;

use std::net::TcpStream;
use std::str;
use std::sync::Arc;

use native_tls;
// use webpki_roots;

/// This encapsulates the TCP-level connection, some connection
/// state, and the underlying TLS-level session.
#[derive(Debug)]
pub struct TlsClient {
    tls_session: native_tls::TlsStream<TcpStream>,
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
    pub fn new(sock: TcpStream, hostname: &str, cfg: TLSSessionConfig) -> TlsClient {
        let connector = native_tls::TlsConnector::builder().unwrap().build().unwrap();
        let tls_session = connector.connect(hostname, sock).unwrap();

        TlsClient {
            tls_session: tls_session
        }
    }
}

#[derive(Debug, Default)]
pub struct TLSSessionConfig {
    pub identity_file: Option<String>,
}


// fn load_identity(filename: &str, password: &str) -> native_tls::Pkcs12 {
//     let identity_file = fs::File::open(filename).expect("cannot open certificate file");
//     let mut reader = BufReader::new(identity_file);
//     native_tls::Pkcs12::from_der(&mut reader, password).unwrap()
// }


// Build a `ClientConfig` from our arguments
// pub fn make_config(args: &TLSSessionConfig) -> Arc<rustls::ClientConfig> {
//     let mut config = rustls::ClientConfig::new();

//     if !args.suite.is_empty() {
//         config.ciphersuites = lookup_suites(&args.suite);
//     }

//     if args.cafile.is_some() {
//         let cafile = args.cafile.as_ref().unwrap();

//         let certfile = fs::File::open(&cafile).expect("Cannot open CA file");
//         let mut reader = BufReader::new(certfile);
//         config.root_store
//             .add_pem_file(&mut reader)
//             .unwrap();
//     } else {
//         config.root_store.add_trust_anchors(&webpki_roots::ROOTS);
//     }

//     if args.no_tickets {
//         config.enable_tickets = false;
//     }

//     config.set_protocols(&args.proto);
//     config.set_mtu(&args.mtu);

//     if args.auth_key.is_some() || args.auth_certs.is_some() {
//         load_key_and_cert(&mut config,
//                           args.auth_key
//                               .as_ref()
//                               .expect("must provide auth-key with auth-certs"),
//                           args.auth_certs
//                               .as_ref()
//                               .expect("must provide auth-certs with auth-key"));
//     }

//     Arc::new(config)
// }
