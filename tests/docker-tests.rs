// Test requiring docker, work against live syslog instances

extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;

extern crate slog_term;
extern crate slog_stream;

// TODO: Get IP straight from DOCKER_ENV

#[cfg(test)]
mod tests {

    use slog::{Logger, Record, OwnedKeyValueList, Drain, Never, Discard, DrainExt, duplicate};
    use slog_stream;
    use slog_syslog_ng::*;
    use slog_term;

    use std;
    use std::net::{SocketAddr, IpAddr, Ipv4Addr, ToSocketAddrs};
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::sync::{Arc, Mutex};

    include!("tests/_fixtures.rs");

    include!("tests/_drains.rs");

}
