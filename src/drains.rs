use config::UDSConfig;
use format::SyslogFormat;
use slog::{Drain, Level, Record, OwnedKeyValueList, Never};
use slog_stream::Format as StreamFormat;
use std::cell::RefCell;
use std::io;
use std::net::ToSocketAddrs;
use std::os::unix::net::UnixDatagram;
use std::path::PathBuf;
use std;

pub struct UDSDrain<F>
    where F: StreamFormat
{
    socket: PathBuf,
    formatter: F,
}

impl<F> UDSDrain<F>
    where F: StreamFormat
{
    pub fn new(socket: PathBuf, formatter: F) -> Self {
        UDSDrain {
            socket: socket,
            formatter: formatter,
        }
    }
}

impl<F> Drain for UDSDrain<F>
    where F: StreamFormat
{
    type Error = Never;

    fn log(&self, info: &Record, logger_values: &OwnedKeyValueList) -> std::result::Result<(), Never> {

        let mut buf = Vec::<u8>::with_capacity(4096);
        self.formatter.format(&mut buf, info, logger_values).unwrap();

        let socket = UnixDatagram::unbound().unwrap();
        socket.send_to(buf.as_slice(), &self.socket).unwrap();
        Ok(())
    }
}
