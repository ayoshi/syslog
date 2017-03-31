// Test requiring docker, work against live syslog instances
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate slog;
extern crate slog_stream;
extern crate slog_syslog_ng;
extern crate serde_json;

#[macro_use]
mod common;

#[cfg(test)]
#[cfg(feature="full-integration-env")]
mod tests {

    use common::{TestDrain, TestIoBuffer, filter_syslog_messages, reset_syslog_ng};
    use slog::{Logger, DrainExt, duplicate};
    use slog_syslog_ng::*;

    use std::{thread, time};

    use std::net::ToSocketAddrs;
    use std::path::PathBuf;

    include!("tests/_drains.rs");

}
