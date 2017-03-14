// use config::UDSConfig;
// use format::SyslogFormat;
use slog::{Drain, Record, OwnedKeyValueList, Never};
use slog_stream::Format as StreamFormat;
use std;
// use std::cell::RefCell;
use std::io;
use std::os::unix::net::UnixDatagram;
use std::path::PathBuf;
use std::net::Shutdown;

#[derive(Default, Debug)]
pub struct DrainDisconnected {}

#[derive(Debug)]
pub struct DrainConnected {
    socket: UnixDatagram,
}

#[derive(Debug)]
pub struct UDSDrain<C, F>
    where F: StreamFormat
{
    path_to_socket: PathBuf,
    formatter: F,
    connection: C,
}

impl<F> UDSDrain<DrainDisconnected, F>
    where F: StreamFormat
{
    pub fn new(path_to_socket: PathBuf, formatter: F) -> UDSDrain<DrainDisconnected, F> {
        UDSDrain::<DrainDisconnected, F> {
            path_to_socket: path_to_socket,
            formatter: formatter,
            connection: DrainDisconnected {},
        }
    }
}

impl<F> UDSDrain<DrainDisconnected, F>
    where F: StreamFormat
{
    pub fn connect(self) -> Result<UDSDrain<DrainConnected, F>, io::Error> {
        let socket = UnixDatagram::unbound()?;
        Ok(UDSDrain::<DrainConnected, F> {
            path_to_socket: self.path_to_socket,
            formatter: self.formatter,
            connection: DrainConnected { socket: socket },
        })
    }
}

impl<F> UDSDrain<DrainConnected, F>
    where F: StreamFormat
{
    pub fn disconnect(self) -> Result<(), io::Error> {
        self.connection.socket.shutdown(Shutdown::Both)
    }
}


impl<F> Drain for UDSDrain<DrainConnected, F>
    where F: StreamFormat
{
    type Error = Never;

    fn log(&self,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> std::result::Result<(), Never> {

        let mut buf = Vec::<u8>::with_capacity(4096);
        self.formatter.format(&mut buf, info, logger_values).unwrap();
        self.connection.socket.send_to(buf.as_slice(), &self.path_to_socket).unwrap();
        Ok(())
    }
}
