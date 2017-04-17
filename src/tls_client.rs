

use openssl::ssl::{SslConnectorBuilder, SslMethod, SslContextBuilder, SslStream, SSL_VERIFY_NONE,
                   SSL_VERIFY_PEER};
use openssl::x509::{X509FileType, X509_FILETYPE_PEM};
use std::fmt;
use std::fs;
use std::io;
use openssl;
use std::io::BufReader;

use std::net::TcpStream;
use std::path::PathBuf;
use std::str;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct TlsSessionConfig {
    pub domain: String,
    pub ca_file: Option<String>,
    pub private_key_file: Option<PathBuf>,
    pub certs_file: Option<PathBuf>,
    pub no_verify: bool,
}

#[derive(Debug)]
pub struct TlsClientDisconnected {}

pub struct TlsClientConfigured {
    connector: SslConnectorBuilder,
}

#[derive(Debug)]
pub struct TlsClientConnected {
    tls_session: SslStream<TcpStream>,
}

#[derive(Debug)]
pub struct TlsClient<C> {
    session_config: TlsSessionConfig,
    connection: C,
}

impl<C> TlsClient<C> {
    pub fn new() -> TlsClient<TlsClientDisconnected> {
        TlsClient::<TlsClientDisconnected> {
            session_config: TlsSessionConfig::default(),
            connection: TlsClientDisconnected {}
        }
    }
}

impl TlsClient<TlsClientDisconnected> {
    pub fn configure(self, session_config: TlsSessionConfig) -> TlsClient<TlsClientConfigured> {
        let mut connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap();
        {
            let mut ctx = connector.builder_mut();

            // Set CA-file, or don't verify peer
            if let Some(ca_file) = session_config.ca_file {
                ctx.set_ca_file("/syslog-ng/cacert.pem");
            }

            // NO_VERIFY
            if session_config.no_verify {
                ctx.set_verify(SSL_VERIFY_NONE);
                ctx.set_verify_callback(SSL_VERIFY_PEER, |p, _| p);
            }

            // Set client certs file
            if let Some(certs_file) = session_config.certs_file {
                ctx.set_certificate_file(certs_file.as_path(), X509_FILETYPE_PEM)
                    .unwrap();
            }

            // Set client private key file
            if let Some(certs_file) = session_config.certs_file {
                ctx.set_private_key_file(certs_file.as_path(), X509_FILETYPE_PEM)
                    .unwrap();
            }
        }

        TlsClient::<TlsClientConfigured> {
            session_config: session_config,
            connection: TlsClientConfigured {
                connector: connector,
            }
        }
    }
}

impl TlsClient<TlsClientConfigured> {
    fn connect(self, sock: TcpStream) -> Result<TlsClient<TlsClientConnected>, ()> {
        // TODO convert errors
        let tls_session = self.connection.connector
            .build()
            .connect(self.session_config.domain.as_ref(), sock).map_err(|e| ())?;

        Ok(TlsClient::<TlsClientConnected> {
            session_config: self.session_config,
            connection: TlsClientConnected {
                tls_session: tls_session,
            }
        }
           )
    }
    // TODO Implement disconnect
}

impl io::Write for TlsClient<TlsClientConnected> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.connection.tls_session.write(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.connection.tls_session.flush()
    }
}

impl io::Read for TlsClient<TlsClientConnected> {
    fn read(&mut self, bytes: &mut [u8]) -> io::Result<usize> {
        self.connection.tls_session.read(bytes)
    }
}
