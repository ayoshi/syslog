use openssl;

error_chain! {

    foreign_links {
        Io(::std::io::Error);
        SslErrorStack(openssl::error::ErrorStack);
        Ssl(openssl::ssl::Error);
        SslHandshake(openssl::ssl::HandshakeError<::std::net::TcpStream>);
    }

    errors {
        ConnectionFailure (t: &'static str) {
            description("Failed to connect to socket")
                display("Failed to connect to socket: '{}' ", t)
        }
        DisconnectFailure (t: &'static str) {
            description("Failed to disconnect properly")
            display("Failed to disconnect properly: '{}' ", t)
        }
    }
}
