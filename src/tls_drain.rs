use slog::{Drain, OwnedKeyValueList, Record};
use slog_stream::Format as StreamFormat;
use std::io;
use std::io::{Write, Cursor};
use std::marker::PhantomData;
use std::net::{TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
// use native_tls::{TlsConnector, TlsStream};
use tls_client::{TlsClient, lookup_suites, load_key_and_cert};
use rustls;
use webpki_roots;

/// Delimited messages
pub struct DelimitedMessages;

/// Framed messages
pub struct FramedMessages;

/// State: `TLSDisconnected`` for the TLS drain
#[derive(Debug)]
pub struct TLSDisconnected {
    addr: SocketAddr,
}

/// State: `TLSConnected` for the TLS drain
#[derive(Debug)]
pub struct TLSConnected {
    stream: Arc<Mutex<TlsClient>>,
    addr: SocketAddr,
}

/// TLS drain
#[derive(Debug)]
pub struct TLSDrain<T, C, F>
    where F: StreamFormat
{
    formatter: F,
    connection: C,
    _message_type: PhantomData<T>,
}

impl<T, F> TLSDrain<T, TLSDisconnected, F>
    where F: StreamFormat
{
    /// TLSDrain constructor
    pub fn new(addr: SocketAddr, formatter: F) -> TLSDrain<T, TLSDisconnected, F> {
        TLSDrain::<T, TLSDisconnected, F> {
            formatter: formatter,
            connection: TLSDisconnected { addr: addr },
            _message_type: PhantomData,
        }
    }

    /// Connect TLS stream
    pub fn connect(self) -> io::Result<TLSDrain<T, TLSConnected, F>> {
        // TODO Fix all unwraps
        // TODO Fix domain name validation
        // let mut file = File::open("/syslog-ng/pfx").unwrap();
        // let mut pkcs12 = vec![];
        // file.read_to_end(&mut pkcs12).unwrap();
        // let pkcs12 = Pkcs12::from_der(&pkcs12, "hunter2").unwrap();

        let mut config = rustls::ClientConfig::new();
        config.root_store.add_trust_anchors(&webpki_roots::ROOTS);
        load_key_and_cert(&mut config, "/syslog-ng/privkey.pem", "/syslog-ng/cacert.pem");

        let stream = TcpStream::connect(self.connection.addr)?;
        let mut stream = TlsClient::new(stream, "syslog-ng", Arc::new(config));

        // let connector = TlsConnector::builder().expect("Builder 1").build().expect("Builder 2");
        // let stream = TcpStream::connect(self.connection.addr)?;
        // let mut stream = connector.connect("google.com", stream).unwrap();
        // let stream = connector.danger_connect_without_providing_domain_for_certificate_verification_and_server_name_indication(
        //     stream).unwrap();
        Ok(TLSDrain::<T, TLSConnected, F> {
               formatter: self.formatter,
               connection: TLSConnected {
                   stream: Arc::new(Mutex::new(stream)),
                   addr: self.connection.addr,
               },
               _message_type: PhantomData,
           })
    }
}

impl<T, F> TLSDrain<T, TLSConnected, F>
    where F: StreamFormat
{
    /// Disconnect TLS stream, completing all operations
    pub fn disconnect(self) -> io::Result<TLSDrain<T, TLSDisconnected, F>> {
        //TODO: Fix
        // self.connection
        //     .stream
        //     .lock()
        //     .map_err(|_| io::Error::new(io::ErrorKind::Other, "Couldn't acquire lock"))
        //     .and_then(|mut s| s.shutdown())?;
        Ok(TLSDrain::<T, TLSDisconnected, F> {
               formatter: self.formatter,
               connection: TLSDisconnected { addr: self.connection.addr },
               _message_type: PhantomData,
           })
    }
}

// RFC3164 messages over TLS don't require framed headers
impl<F> Drain for TLSDrain<DelimitedMessages, TLSConnected, F>
    where F: StreamFormat
{
    type Error = io::Error;

    #[allow(dead_code)]
    fn log(&self, info: &Record, logger_values: &OwnedKeyValueList) -> io::Result<()> {

        // Should be thread safe - redo the buffering
        let mut buf = Vec::<u8>::with_capacity(4096);

        self.formatter.format(&mut buf, info, logger_values)?;
        self.connection
            .stream
            .lock()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Couldn't acquire lock"))
            .and_then(|mut s| s.write(buf.as_slice()))?;

        Ok(())
    }
}

// RFC5424 messages require framed delimition, first we need to send
// the length of the message in octets
impl<F> Drain for TLSDrain<FramedMessages, TLSConnected, F>
    where F: StreamFormat
{
    type Error = io::Error;

    fn log(&self, info: &Record, logger_values: &OwnedKeyValueList) -> io::Result<()> {

        // Should be thread safe - redo the buffering
        let mut buf = Cursor::new(Vec::<u8>::with_capacity(10));

        self.formatter.format(&mut buf, info, logger_values)?;
        let length = buf.position();

        self.connection
            .stream
            .lock()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Couldn't acquire lock"))
            .and_then(|mut s| {
                          // Space spearated frame length
                          s.write_fmt(format_args!("{} ", length))?;
                          s.write(buf.into_inner().as_slice())
                      })?;

        Ok(())
    }
}

/// `TLSDrain` sending delimited messages
/// RFC3164 over TLS is generally used this way
/// but some servers accepting RFC5424 work with it too
pub type TLSDrainDelimited<C, F> = TLSDrain<DelimitedMessages, C, F>;

/// `TLSDrain` sending framed messages
/// Mostly for sending RFC5424 messages - rsyslog, syslog-ng will use this format
pub type TLSDrainFramed<C, F> = TLSDrain<FramedMessages, C, F>;
