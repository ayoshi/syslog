use errors::*;
use format::SyslogFormat;
use parking_lot::Mutex;
use slog::{Drain, OwnedKVList, Record};
use std::io;
use std::net::Shutdown;
use std::os::unix::net::UnixDatagram;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

/// State: `UDSDisconnected`
#[derive(Default, Debug)]
pub struct UDSDisconnected {
    path_to_socket: PathBuf,
}

impl UDSDisconnected {
    /// Connect Unix domain socket
    fn connect(self) -> Result<UDSConnected> {
        let socket = UnixDatagram::unbound()
            .chain_err(|| ErrorKind::ConnectionFailure("Failed to connect socket"))?;
        Ok(UDSConnected {
               socket: Arc::new(Mutex::new(socket)),
               path_to_socket: self.path_to_socket,
           })
    }
}

/// State: `UDSConnected` for the UDS drain
#[derive(Debug)]
pub struct UDSConnected {
    socket: Arc<Mutex<UnixDatagram>>,
    path_to_socket: PathBuf,
}

impl UDSConnected {
    /// Disconnect Unix domain socket, completing all operations
    fn disconnect(self) -> Result<UDSDisconnected> {
        self.socket
            .try_lock_for(Duration::from_secs(super::LOCK_TRY_TIMEOUT))
            .ok_or_else(|| ErrorKind::DisconnectFailure("Timed out trying to acquire lock"))
            .and_then(|s| {
                          s.shutdown(Shutdown::Both)
                              .map_err(|_| ErrorKind::DisconnectFailure("Socket shutdown failed"))
                      })?;
        Ok(UDSDisconnected { path_to_socket: self.path_to_socket })
    }
}

/// Unix domain socket drain
#[derive(Debug)]
pub struct UDSDrain<C, F>
    where F: SyslogFormat
{
    formatter: F,
    connection: C,
}

impl<F> UDSDrain<UDSDisconnected, F>
    where F: SyslogFormat
{
    /// UDSDrain constructor
    pub fn new(path_to_socket: PathBuf, formatter: F) -> UDSDrain<UDSDisconnected, F> {
        UDSDrain::<UDSDisconnected, F> {
            formatter: formatter,
            connection: UDSDisconnected { path_to_socket: path_to_socket },
        }
    }

    /// Connect UDS socket
    pub fn connect(self) -> Result<UDSDrain<UDSConnected, F>> {
        Ok(UDSDrain::<UDSConnected, F> {
               formatter: self.formatter,
               connection: self.connection.connect()?,
           })
    }
}

impl<F> UDSDrain<UDSConnected, F>
    where F: SyslogFormat
{
    /// Disconnect UDS socket, completing all operations
    pub fn disconnect(self) -> Result<UDSDrain<UDSDisconnected, F>> {
        Ok(UDSDrain::<UDSDisconnected, F> {
               formatter: self.formatter,
               connection: self.connection.disconnect()?,
           })
    }
}

impl<F> Drain for UDSDrain<UDSConnected, F>
    where F: SyslogFormat
{
    type Err = io::Error;
    type Ok = ();

    fn log(&self, info: &Record, logger_values: &OwnedKVList) -> io::Result<()> {

        // Should be thread safe - redo the buffering
        let mut buf = Vec::<u8>::with_capacity(4096);

        self.formatter.format(&mut buf, info, logger_values)?;

        self.connection
            .socket
            .try_lock_for(Duration::from_secs(super::LOCK_TRY_TIMEOUT))
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Couldn't acquire lock"))
            .and_then(|s| s.send_to(buf.as_slice(), &self.connection.path_to_socket))?;

        Ok(())
    }
}
