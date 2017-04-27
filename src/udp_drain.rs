
use errors::*;
use parking_lot::Mutex;
use slog::{Drain, OwnedKeyValueList, Record};
use slog_stream::Format as StreamFormat;
use std::io;
use std::net::{UdpSocket, SocketAddr};
use std::sync::Arc;
use std::time::Duration;


/// State: `UDPDisconnected`` for the UDP drain
#[derive(Debug)]
pub struct UDPDisconnected {
    addr: SocketAddr,
}

impl UDPDisconnected {
    /// Connect UDP stream
    fn connect(self) -> Result<UDPConnected> {
        let socket = UdpSocket::bind("0.0.0.0:0")
            .chain_err(|| ErrorKind::ConnectionFailure("Failed to connect socket"))?;
        Ok(UDPConnected {
               socket: Arc::new(Mutex::new(socket)),
               addr: self.addr,
           })
    }
}

/// State: `UDPConnected` for the UDP drain
#[derive(Debug)]
pub struct UDPConnected {
    socket: Arc<Mutex<UdpSocket>>,
    addr: SocketAddr,
}

impl UDPConnected {
    /// Disconnect UDP stream, completing all operations
    fn disconnect(self) -> Result<UDPDisconnected> {
        self.socket
            .try_lock_for(Duration::from_secs(super::LOCK_TRY_TIMEOUT))
            .ok_or_else(|| ErrorKind::DisconnectFailure("Timed out trying to acquire lock"))?;
        Ok(UDPDisconnected { addr: self.addr })
    }
}

/// UDP drain
#[derive(Debug)]
pub struct UDPDrain<C, F>
    where F: StreamFormat
{
    formatter: F,
    connection: C,
}

impl<F> UDPDrain<UDPDisconnected, F>
    where F: StreamFormat
{
    /// UDPDrain constructor
    pub fn new(addr: SocketAddr, formatter: F) -> UDPDrain<UDPDisconnected, F> {
        UDPDrain::<UDPDisconnected, F> {
            formatter: formatter,
            connection: UDPDisconnected { addr: addr },
        }
    }

    /// Connect UDP socket
    pub fn connect(self) -> Result<UDPDrain<UDPConnected, F>> {
        Ok(UDPDrain::<UDPConnected, F> {
               formatter: self.formatter,
               connection: self.connection.connect()?,
           })
    }
}

impl<F> UDPDrain<UDPConnected, F>
    where F: StreamFormat
{
    /// Disconnect UDP socket, completing all operations
    pub fn disconnect(self) -> Result<UDPDrain<UDPDisconnected, F>> {
        Ok(UDPDrain::<UDPDisconnected, F> {
               formatter: self.formatter,
               connection: self.connection.disconnect()?,
           })
    }
}

impl<F> Drain for UDPDrain<UDPConnected, F>
    where F: StreamFormat
{
    type Error = io::Error;

    fn log(&self, info: &Record, logger_values: &OwnedKeyValueList) -> io::Result<()> {

        // Should be thread safe - redo the buffering
        let mut buf = Vec::<u8>::with_capacity(4096);

        self.formatter.format(&mut buf, info, logger_values)?;
        self.connection
            .socket
            .try_lock_for(Duration::from_secs(super::LOCK_TRY_TIMEOUT))
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Couldn't acquire lock"))
            .and_then(|s| s.send_to(buf.as_slice(), &self.connection.addr))?;

        Ok(())
    }
}
