use errors::*;
use format::SyslogFormat;
use parking_lot::Mutex;
use slog::{Drain, OwnedKVList, Record};
use std::io;
use std::io::{Write, Cursor};
use std::marker::PhantomData;
use std::net::{Shutdown, TcpStream, SocketAddr};
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use std::time::Duration;


/// Delimited messages
#[derive(Debug)]
pub struct DelimitedMessages;

/// Framed messages
#[derive(Debug)]
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
               stream: Arc::new(AssertUnwindSafe(Mutex::new(stream))),
               addr: self.addr,
           })
    }
}

/// State: `TCPConnected` for the TCP drain
#[derive(Debug)]
pub struct TCPConnected {
    stream: Arc<AssertUnwindSafe<Mutex<TcpStream>>>,
    addr: SocketAddr,
}

impl TCPConnected {
    /// Disconnect TCP stream, completing all operations
    fn disconnect(self) -> Result<TCPDisconnected> {
        self.stream
            .try_lock_for(Duration::from_secs(super::LOCK_TRY_TIMEOUT))
            .ok_or_else(|| ErrorKind::DisconnectFailure("Timed out trying to acquire lock"))
            .and_then(|s| {
                          s.shutdown(Shutdown::Both)
                              .map_err(|_| ErrorKind::DisconnectFailure("Socket shutdown failed"))
                      })?;
        Ok(TCPDisconnected { addr: self.addr })
    }

    fn send(&self, bytes: &[u8]) -> io::Result<usize> {
        self.stream
            .try_lock_for(Duration::from_secs(super::LOCK_TRY_TIMEOUT))
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Couldn't acquire lock"))
            .and_then(|mut s| s.write(bytes))
    }

    fn flush(&self) -> io::Result<()> {
        self.stream
            .try_lock_for(Duration::from_secs(super::LOCK_TRY_TIMEOUT))
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Couldn't acquire lock"))
            .and_then(|mut s| s.flush())
    }
}

/// TCP drain
#[derive(Debug)]
pub struct TCPDrain<T, C, F>
    where F: SyslogFormat
{
    formatter: F,
    connection: C,
    _message_type: PhantomData<T>,
}

impl<T, F> TCPDrain<T, TCPDisconnected, F>
    where F: SyslogFormat
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
    where F: SyslogFormat
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
    where F: SyslogFormat
{
    type Err = io::Error;
    type Ok = ();

    #[allow(dead_code)]
    fn log(&self, info: &Record, logger_values: &OwnedKVList) -> io::Result<()> {

        // Should be thread safe - redo the buffering
        let mut buf = Vec::<u8>::with_capacity(4096);

        self.formatter.format(&mut buf, info, logger_values)?;
        self.connection.send(buf.as_slice())?;

        Ok(())
    }
}

// RFC5424 messages require framed delimition, first we need to send
// the length of the message in octets
impl<F> Drain for TCPDrain<FramedMessages, TCPConnected, F>
    where F: SyslogFormat
{
    type Err = io::Error;
    type Ok = ();

    fn log(&self, info: &Record, logger_values: &OwnedKVList) -> io::Result<()> {

        // Should be thread safe - redo the buffering
        // 1. Could we use simply Vec.len()?
        // 2. format! is slow, there was another way of doing it
        let mut buf = Cursor::new(Vec::<u8>::with_capacity(10));

        self.formatter.format(&mut buf, info, logger_values)?;
        let length = buf.position();

        self.connection.send(format!("{} ", length).as_bytes())?;
        self.connection.send(buf.into_inner().as_slice())?;

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
