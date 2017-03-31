// Tests to be run
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;
extern crate slog_stream;
extern crate serde_json;

#[macro_use]
mod common;


#[cfg(test)]
mod tests {

    // use common::*;

    use common::emit_test_message_to_buffer;
    use slog_syslog_ng::*;

    use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    use std::path::PathBuf;

    include!("tests/helpers.rs");
    include!("tests/config.rs");
    include!("tests/serializers.rs");


}
