use slog::{Drain, OwnedKeyValueList, Record};
use slog_stream::Format as StreamFormat;
use std::io;
use std::io::{Write, Cursor};
use std::marker::PhantomData;
use std::net::{TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
// use tls_client::{TlsClient, TLSSessionConfig, make_config};
use tls_client::{TlsClient, TlsSessionConfig, TlsClientConnected, TlsClientDisconnected};

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
    stream: Arc<Mutex<TlsClient<TlsClientConnected>>>,
    addr: SocketAddr,
}

/// TLS drain
#[derive(Debug)]
pub struct TLSDrain<T, C, F>
    where F: StreamFormat
{
    formatter: F,
    connection: C,
    session_config: TlsSessionConfig,
    _message_type: PhantomData<T>,
}

impl<T, F> TLSDrain<T, TLSDisconnected, F>
    where F: StreamFormat
{
    /// TLSDrain constructor
    pub fn new(addr: SocketAddr,
               session_config: TlsSessionConfig,
               formatter: F)
               -> TLSDrain<T, TLSDisconnected, F> {
        TLSDrain::<T, TLSDisconnected, F> {
            formatter: formatter,
            connection: TLSDisconnected { addr: addr },
            session_config: session_config,
            _message_type: PhantomData,
        }
    }

    /// Connect TLS stream
    pub fn connect(self) -> io::Result<TLSDrain<T, TLSConnected, F>> {

        // TODO convert errors properly
        let stream = TcpStream::connect(self.connection.addr)?;
        let stream = TlsClient::<TlsClientDisconnected>::new()
            .configure(&self.session_config)
            .connect(stream).map_err(|e| io::Error::last_os_error())?;

        Ok(TLSDrain::<T, TLSConnected, F> {
               formatter: self.formatter,
               connection: TLSConnected {
                   stream: Arc::new(Mutex::new(stream)),
                   addr: self.connection.addr,
               },
               session_config: self.session_config,
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
               session_config: self.session_config,
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
