// use format::SyslogFormatter;

use slog::{Drain, Record, OwnedKeyValueList};
use slog_stream::Format as StreamFormat;
use std;
// use std::cell::RefCell;
use std::io;
use std::net::Shutdown;
use std::os::unix::net::UnixDatagram;
use std::path::PathBuf;

/// State: Connected for the drain
#[derive(Default, Debug)]
pub struct Disconnected {}

/// State: Disconnected for the drain
#[derive(Debug)]
pub struct Connected {
    socket: UnixDatagram,
}

/// Unix domain socket drain
#[derive(Debug)]
pub struct UDSDrain<C, F>
    where F: StreamFormat
{
    path_to_socket: PathBuf,
    formatter: F,
    connection: C,
}

impl<F> UDSDrain<Disconnected, F>
    where F: StreamFormat
{
    /// UDSDrain constructor
    pub fn new(path_to_socket: PathBuf, formatter: F) -> UDSDrain<Disconnected, F> {
        UDSDrain::<Disconnected, F> {
            path_to_socket: path_to_socket,
            formatter: formatter,
            connection: Disconnected {},
        }
    }
}

impl<F> UDSDrain<Disconnected, F>
    where F: StreamFormat
{
    /// Bind UDS socket
    pub fn connect(self) -> Result<UDSDrain<Connected, F>, io::Error> {
        let socket = UnixDatagram::unbound()?;
        Ok(UDSDrain::<Connected, F> {
            path_to_socket: self.path_to_socket,
            formatter: self.formatter,
            connection: Connected { socket: socket },
        })
    }
}

impl<F> UDSDrain<Connected, F>
    where F: StreamFormat
{
    /// Disconnect UDS socket, completing all operations
    pub fn disconnect(&mut self) -> Result<(), io::Error> {
        self.connection.socket.shutdown(Shutdown::Both)
    }
}

// TODO: temporary disabled: https://github.com/rust-lang/rust/issues/38868
// impl <F>Drop for UDSDrain<Connected, F>
//     where F: StreamFormat {
//     fn drop(&mut self) {
//         self.disconnect();
//     }
// }


impl<F> Drain for UDSDrain<Connected, F>
    where F: StreamFormat
{
    type Error = io::Error;

    fn log(&self,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> std::result::Result<(), io::Error> {

        // Should be thread safe - redo the buffering
        let mut buf = Vec::<u8>::with_capacity(4096);

        self.formatter.format(&mut buf, info, logger_values)?;
        self.connection.socket.send_to(buf.as_slice(), &self.path_to_socket)?;

        Ok(())
    }
}
