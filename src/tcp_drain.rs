use slog::{Drain, OwnedKeyValueList, Record};
use slog_stream::Format as StreamFormat;
use std::io;
use std::io::Write;
use std::net::{Shutdown, TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};

/// State: TCPConnected for the TCP drain
#[derive(Debug)]
pub struct TCPDisconnected {
    addr: SocketAddr,
}

/// State: TCPConnected for the TCP drain
#[derive(Debug)]
pub struct TCPConnected {
    stream: Arc<Mutex<TcpStream>>,
    addr: SocketAddr,
}


/// TCP drain
#[derive(Debug)]
pub struct TCPDrain<C, F>
    where F: StreamFormat
{
    formatter: F,
    connection: C,
}

impl<F> TCPDrain<TCPDisconnected, F>
    where F: StreamFormat
{
    /// TCPDrain constructor
    pub fn new(addr: SocketAddr, formatter: F) -> TCPDrain<TCPDisconnected, F> {
        TCPDrain::<TCPDisconnected, F> {
            formatter: formatter,
            connection: TCPDisconnected { addr: addr },
        }
    }

    /// Connect TCP stream
    pub fn connect(self) -> io::Result<TCPDrain<TCPConnected, F>> {
        let stream = TcpStream::connect(self.connection.addr)?;
        Ok(TCPDrain::<TCPConnected, F> {
            formatter: self.formatter,
            connection: TCPConnected {
                stream: Arc::new(Mutex::new(stream)),
                addr: self.connection.addr,
            },
        })
    }
}

impl<F> TCPDrain<TCPConnected, F>
    where F: StreamFormat
{
    /// Disconnect TCP stream, completing all operations
    pub fn disconnect(self) -> io::Result<TCPDrain<TCPDisconnected, F>> {
        self.connection.stream.lock()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Couldn't acquire lock"))
            .and_then(|s| s.shutdown(Shutdown::Both))?;
        Ok(TCPDrain::<TCPDisconnected, F> {
            formatter: self.formatter,
            connection: TCPDisconnected { addr: self.connection.addr },
        })
    }
}

impl<F> Drain for TCPDrain<TCPConnected, F>
    where F: StreamFormat
{
    type Error = io::Error;

    fn log(&self, info: &Record, logger_values: &OwnedKeyValueList) -> io::Result<()> {

        // Should be thread safe - redo the buffering
        let mut buf = Vec::<u8>::with_capacity(4096);

        self.formatter.format(&mut buf, info, logger_values)?;
        self.connection.stream.lock()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Couldn't acquire lock"))
            .and_then(|mut s| s.write(buf.as_slice()))?;

        Ok(())
    }
}
