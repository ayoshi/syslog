// Test requiring docker, work against live syslog instances
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

#[macro_use]
extern crate slog;
extern crate slog_syslog_ng;
extern crate serde_json;

#[macro_use]
mod common;

#[cfg(test)]
#[cfg(feature="full-integration-env")]
mod tests {

    use common::{TestDrain, TestIoBuffer, verify_syslog_ng_message};
    use slog::{Logger, Duplicate, Drain};
    use slog_syslog_ng::*;

    use std::net::ToSocketAddrs;
    use std::path::PathBuf;

    include!("tests/drains.rs");

}
