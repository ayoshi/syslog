use errors::*;
use format::SyslogFormat;
use slog::{Drain, OwnedKVList, Record};
use std::io;
use std::io::{Write, Cursor};
use std::marker::PhantomData;
use std::net::{TcpStream, SocketAddr};
use std::panic::AssertUnwindSafe;
use std::sync::{Arc, Mutex};
use tls_client::{TlsClient, TlsSessionConfig, TlsClientConnected, TlsClientDisconnected};

/// Delimited messages
pub struct DelimitedMessages;

/// Framed messages
pub struct FramedMessages;

/// State: `TLSDisconnected`` for the TLS drain
#[derive(Debug)]
pub struct TLSDisconnected {
    addr: SocketAddr,
    session_config: TlsSessionConfig,
}

impl TLSDisconnected {
    /// Connect TLS stream
    pub fn connect(self) -> Result<TLSConnected> {

        let stream = TcpStream::connect(self.addr)
            .chain_err(|| ErrorKind::ConnectionFailure("Failed to connect socket"))?;
        let stream =
            TlsClient::<TlsClientDisconnected>::new()
                .configure(&self.session_config)?
                .connect(stream)
                .chain_err(|| ErrorKind::ConnectionFailure("Failed to establish TLS session"))?;

        Ok(TLSConnected {
               stream: Arc::new(AssertUnwindSafe(Mutex::new(stream))),
               addr: self.addr,
               session_config: self.session_config,
           })
    }
}

/// State: `TLSConnected` for the TLS drain
#[derive(Debug)]
pub struct TLSConnected {
    stream: Arc<AssertUnwindSafe<Mutex<TlsClient<TlsClientConnected>>>>,
    session_config: TlsSessionConfig,
    addr: SocketAddr,
}

impl TLSConnected {
    pub fn disconnect(self) -> Result<TLSDisconnected> {
        self.stream
            .lock()
            .map_err(|_| ErrorKind::DisconnectFailure("Couldn't acquire lock").into())
            .and_then(|mut s| s.disconnect())?;
        Ok(TLSDisconnected {
               addr: self.addr,
               session_config: self.session_config,
           })
    }
}

/// TLS drain
#[derive(Debug)]
pub struct TLSDrain<T, C, F>
    where F: SyslogFormat
{
    formatter: F,
    connection: C,
    _message_type: PhantomData<T>,
}

impl<T, F> TLSDrain<T, TLSDisconnected, F>
    where F: SyslogFormat
{
    /// TLSDrain constructor
    pub fn new(addr: SocketAddr,
               session_config: TlsSessionConfig,
               formatter: F)
               -> TLSDrain<T, TLSDisconnected, F> {
        TLSDrain::<T, TLSDisconnected, F> {
            formatter: formatter,
            connection: TLSDisconnected {
                addr: addr,
                session_config: session_config,
            },
            _message_type: PhantomData,
        }
    }

    /// Connect TLS stream
    pub fn connect(self) -> Result<TLSDrain<T, TLSConnected, F>> {
        Ok(TLSDrain::<T, TLSConnected, F> {
               formatter: self.formatter,
               connection: self.connection.connect()?,
               _message_type: PhantomData,
           })
    }
}

impl<T, F> TLSDrain<T, TLSConnected, F>
    where F: SyslogFormat
{
    /// Disconnect TLS stream, completing all operations
    pub fn disconnect(self) -> Result<TLSDrain<T, TLSDisconnected, F>> {
        Ok(TLSDrain::<T, TLSDisconnected, F> {
               formatter: self.formatter,
               connection: self.connection.disconnect()?,
               _message_type: PhantomData,
           })
    }
}

// RFC3164 messages over TLS don't require framed headers
impl<F> Drain for TLSDrain<DelimitedMessages, TLSConnected, F>
    where F: SyslogFormat
{
    type Err = io::Error;
    type Ok = ();

    #[allow(dead_code)]
    fn log(&self, info: &Record, logger_values: &OwnedKVList) -> io::Result<()> {

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
    where F: SyslogFormat
{
    type Err = io::Error;
    type Ok = ();

    fn log(&self, info: &Record, logger_values: &OwnedKVList) -> io::Result<()> {

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
