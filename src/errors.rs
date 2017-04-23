use openssl;

error_chain! {

    foreign_links {
        Io(::std::io::Error);
        SslErrorStack(openssl::error::ErrorStack);
        Ssl(openssl::ssl::Error);
        SslHandshake(openssl::ssl::HandshakeError<::std::net::TcpStream>);
    }

    errors {
        DisconnectFailure (t: &'static str) {
            description("Failed to disconnect properly")
            display("Failed to disconnect properly: '{}' ", t)
        }
    }
}
