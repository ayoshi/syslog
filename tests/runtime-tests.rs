// Tests to be run
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;
extern crate slog_stream;

#[macro_use]
mod common;


#[cfg(test)]
mod tests {

    // use common::*;
    use common::{emit_test_message_to_buffer};
    use slog_syslog_ng::*;

    use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    use std::path::PathBuf;

    // include!("tests/_fixtures.rs");
    include!("tests/_basic.rs");
    include!("tests/_config.rs");
    include!("tests/_serializers.rs");


}
