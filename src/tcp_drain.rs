use errors::*;
use slog::{Drain, OwnedKeyValueList, Record};
use slog_stream::Format as StreamFormat;
use std::io;
use std::io::{Write, Cursor};
use std::marker::PhantomData;
use std::net::{Shutdown, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};


/// Delimited messages
pub struct DelimitedMessages;

/// Framed messages
pub struct FramedMessages;

/// State: `TCPDisconnected`` for the TCP drain
#[derive(Debug)]
pub struct TCPDisconnected {
    addr: SocketAddr,
}

impl TCPDisconnected {
    /// Connect TCP stream
    fn connect(self) -> Result<TCPConnected> {
        let stream = TcpStream::connect(self.addr)
            .chain_err(|| ErrorKind::ConnectionFailure("Failed to connect socket"))?;
        Ok(TCPConnected {
               stream: Arc::new(Mutex::new(stream)),
               addr: self.addr,
           })
    }
}

/// State: `TCPConnected` for the TCP drain
#[derive(Debug)]
pub struct TCPConnected {
    stream: Arc<Mutex<TcpStream>>,
    addr: SocketAddr,
}

impl TCPConnected {
    /// Disconnect TCP stream, completing all operations
    // TODO try to chain errors properly
    fn disconnect(self) -> Result<TCPDisconnected> {
        self.stream
            .lock()
            .map_err(|_| ErrorKind::DisconnectFailure("Couldn't acquire lock"))
            .and_then(|s| {
                          s.shutdown(Shutdown::Both)
                              .map_err(|_| ErrorKind::DisconnectFailure("Socket shutdown failed"))
                      })?;
        Ok(TCPDisconnected { addr: self.addr })
    }
}

/// TCP drain
#[derive(Debug)]
pub struct TCPDrain<T, C, F>
    where F: StreamFormat
{
    formatter: F,
    connection: C,
    _message_type: PhantomData<T>,
}

impl<T, F> TCPDrain<T, TCPDisconnected, F>
    where F: StreamFormat
{
    /// TCPDrain constructor
    pub fn new(addr: SocketAddr, formatter: F) -> TCPDrain<T, TCPDisconnected, F> {
        TCPDrain::<T, TCPDisconnected, F> {
            formatter: formatter,
            connection: TCPDisconnected { addr: addr },
            _message_type: PhantomData,
        }
    }

    /// Connect TCP stream
    pub fn connect(self) -> Result<TCPDrain<T, TCPConnected, F>> {
        Ok(TCPDrain::<T, TCPConnected, F> {
               formatter: self.formatter,
               connection: self.connection.connect()?,
               _message_type: PhantomData,
           })
    }
}

impl<T, F> TCPDrain<T, TCPConnected, F>
    where F: StreamFormat
{
    /// Disconnect TCP stream, completing all operations
    pub fn disconnect(self) -> Result<TCPDrain<T, TCPDisconnected, F>> {
        Ok(TCPDrain::<T, TCPDisconnected, F> {
               formatter: self.formatter,
               connection: self.connection.disconnect()?,
               _message_type: PhantomData,
           })
    }
}

// RFC3164 messages over TCP don't require framed headers
impl<F> Drain for TCPDrain<DelimitedMessages, TCPConnected, F>
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
impl<F> Drain for TCPDrain<FramedMessages, TCPConnected, F>
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

/// `TCPDrain` sending delimited messages
/// RFC3164 over TCP is generally used this way
/// but some servers accepting RFC5424 work with it too
pub type TCPDrainDelimited<C, F> = TCPDrain<DelimitedMessages, C, F>;

/// `TCPDrain` sending framed messages
/// Mostly for sending RFC5424 messages - rsyslog, syslog-ng will use this format
pub type TCPDrainFramed<C, F> = TCPDrain<FramedMessages, C, F>;
