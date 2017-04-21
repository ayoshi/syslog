use openssl;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        SslErrorStack(openssl::error::ErrorStack);
        SslError(openssl::ssl::Error);
        SslHandshake(openssl::ssl::HandshakeError<::std::net::TcpStream>);
    }
}
