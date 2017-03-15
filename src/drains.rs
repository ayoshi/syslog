use slog::{Drain, OwnedKeyValueList, Record};
use slog_stream::Format as StreamFormat;
use std::io;
use std::net::{Shutdown, UdpSocket, TcpStream, SocketAddr};
use std::os::unix::net::UnixDatagram;
use std::path::PathBuf;

/// State: UDSDisconnected
#[derive(Default, Debug)]
pub struct UDSDisconnected {
    path_to_socket: PathBuf,
}

/// State: UDSConnected for the UDS drain
#[derive(Debug)]
pub struct UDSConnected {
    socket: UnixDatagram,
    path_to_socket: PathBuf,
}

/// Unix domain socket drain
#[derive(Debug)]
pub struct UDSDrain<C, F>
    where F: StreamFormat
{
    formatter: F,
    connection: C,
}

impl<F> UDSDrain<UDSDisconnected, F>
    where F: StreamFormat
{
    /// UDSDrain constructor
    pub fn new(path_to_socket: PathBuf, formatter: F) -> UDSDrain<UDSDisconnected, F> {
        UDSDrain::<UDSDisconnected, F> {
            formatter: formatter,
            connection: UDSDisconnected { path_to_socket: path_to_socket },
        }
    }

    /// Bind UDS socket
    pub fn connect(self) -> io::Result<UDSDrain<UDSConnected, F>> {
        let socket = UnixDatagram::unbound()?;
        Ok(UDSDrain::<UDSConnected, F> {
            formatter: self.formatter,
            connection: UDSConnected {
                socket: socket,
                path_to_socket: self.connection.path_to_socket,
            },
        })
    }
}

impl<F> UDSDrain<UDSConnected, F>
    where F: StreamFormat
{
    /// Disconnect UDS socket, completing all operations
    pub fn disconnect(&mut self) -> io::Result<()> {
        self.connection.socket.shutdown(Shutdown::Both)
    }
}

// TODO: temporary disabled: https://github.com/rust-lang/rust/issues/38868
// impl <F>Drop for UDSDrain<UDSConnected, F>
//     where F: StreamFormat {
//     fn drop(&mut self) {
//         self.disconnect();
//     }
// }


impl<F> Drain for UDSDrain<UDSConnected, F>
    where F: StreamFormat
{
    type Error = io::Error;

    fn log(&self, info: &Record, logger_values: &OwnedKeyValueList) -> io::Result<()> {

        // Should be thread safe - redo the buffering
        let mut buf = Vec::<u8>::with_capacity(4096);

        self.formatter.format(&mut buf, info, logger_values)?;
        self.connection.socket.send_to(buf.as_slice(), &self.connection.path_to_socket)?;

        Ok(())
    }
}

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

    /// Bind UDP socket
    pub fn connect(self) -> io::Result<UDPDrain<UDPConnected, F>> {
        let socket = UdpSocket::bind("0.0.0.0:31245")?; // TODO: Fix binding
        socket.connect(self.connection.addr)?;
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
    pub fn disconnect(&mut self) -> io::Result<()> {
        Ok(()) // TODO: Fix disconnection
    }
}
