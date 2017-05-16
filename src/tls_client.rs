use errors::*;
use openssl::ssl::{SslConnectorBuilder, SslMethod, SslStream, SSL_VERIFY_NONE, SSL_VERIFY_PEER};
use openssl::x509::X509_FILETYPE_PEM;
use std::{io, fmt};

use std::net::TcpStream;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct TlsSessionConfig {
    pub domain: String,
    pub ca_file: Option<PathBuf>,
    pub private_key_file: Option<PathBuf>,
    pub certs_file: Option<PathBuf>,
    pub no_verify: bool,
}

#[derive(Debug)]
pub struct TlsClientDisconnected {
    session_config: TlsSessionConfig,
}

pub struct TlsClientConfigured {
    session_config: TlsSessionConfig,
    connector: SslConnectorBuilder,
}

impl fmt::Debug for TlsClientConfigured {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "TlsClientConfigured {{ session_config: {:?} }}",
               self.session_config)
    }
}

#[derive(Debug)]
pub struct TlsClientConnected {
    session_config: TlsSessionConfig,
    tls_session: SslStream<TcpStream>,
}

#[derive(Debug)]
pub struct TlsClient<C> {
    connection: C,
}

impl<C> TlsClient<C> {
    pub fn new() -> TlsClient<TlsClientDisconnected> {
        TlsClient::<TlsClientDisconnected> {
            connection: TlsClientDisconnected { session_config: TlsSessionConfig::default() },
        }
    }
}

impl TlsClient<TlsClientDisconnected> {
    pub fn configure(self,
                     session_config: &TlsSessionConfig)
                     -> Result<TlsClient<TlsClientConfigured>> {
        let session_config = session_config.clone();
        let mut connector = SslConnectorBuilder::new(SslMethod::tls())?;
        {
            let mut ctx = connector.builder_mut();

            // Set CA-file, or don't verify peer
            if let Some(ca_file) = session_config.ca_file.clone() {
                ctx.set_ca_file(ca_file.as_path())?;
            }

            // NO_VERIFY
            if session_config.no_verify {
                ctx.set_verify(SSL_VERIFY_NONE);
                ctx.set_verify_callback(SSL_VERIFY_PEER, |p, _| p);
            }

            // Set client certs file
            if let Some(certs_file) = session_config.certs_file.clone() {
                ctx.set_certificate_file(certs_file.as_path(), X509_FILETYPE_PEM)?;
            }

            // Set client private key file
            if let Some(private_key_file) = session_config.private_key_file.clone() {
                ctx.set_private_key_file(private_key_file.as_path(), X509_FILETYPE_PEM)?;
            }
        }

        Ok(TlsClient::<TlsClientConfigured> {
               connection: TlsClientConfigured {
                   session_config: session_config,
                   connector: connector,
               },
           })
    }
}

impl TlsClient<TlsClientConfigured> {
    pub fn connect(self, sock: TcpStream) -> Result<TlsClient<TlsClientConnected>> {
        let tls_session = self.connection
            .connector
            .build()
            .connect(self.connection.session_config.domain.as_ref(), sock)?;

        Ok(TlsClient::<TlsClientConnected> {
               connection: TlsClientConnected {
                   session_config: self.connection.session_config,
                   tls_session: tls_session,
               },
           })
    }
}

impl TlsClient<TlsClientConnected> {
    pub fn disconnect(&mut self) -> Result<TlsClient<TlsClientDisconnected>> {
        self.connection.tls_session.shutdown()?;
        Ok(TlsClient::<TlsClientDisconnected> {
               connection: TlsClientDisconnected {
                   session_config: self.connection.session_config.clone(),
               },
           })
    }
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
