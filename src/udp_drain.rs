use slog::{Drain, OwnedKeyValueList, Record};
use slog_stream::Format as StreamFormat;
use std::io;
use std::net::{UdpSocket, SocketAddr};

/// State: UDPConnected for the UDP drain
#[derive(Debug)]
pub struct UDPDisconnected {
    addr: SocketAddr,
}

/// State: UDPConnected for the UDP drain
#[derive(Debug)]
pub struct UDPConnected {
    socket: UdpSocket,
    addr: SocketAddr,
}


/// UDP socket drain
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
    pub fn connect(self) -> io::Result<UDPDrain<UDPConnected, F>> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        Ok(UDPDrain::<UDPConnected, F> {
            formatter: self.formatter,
            connection: UDPConnected {
                socket: socket,
                addr: self.connection.addr,
            },
        })
    }
}

impl<F> UDPDrain<UDPConnected, F>
    where F: StreamFormat
{
    /// Disconnect UDP socket, completing all operations
    pub fn disconnect(self) -> io::Result<UDPDrain<UDPDisconnected, F>> {
        Ok(UDPDrain::<UDPDisconnected, F> {
            formatter: self.formatter,
            connection: UDPDisconnected { addr: self.connection.addr },
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
        self.connection.socket.send_to(buf.as_slice(), &self.connection.addr)?;

        Ok(())
    }
}
