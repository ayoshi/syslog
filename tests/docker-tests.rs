// Test requiring docker, work against live syslog instances

extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;
extern crate slog_stream;

#[cfg(test)]
#[cfg(feature="full-integration-env")]
mod tests {

    use slog::{Logger, Record, OwnedKeyValueList, Drain, DrainExt, duplicate};
    use slog_stream::Format as StreamFormat;
    use slog_syslog_ng::*;

    use std::{io, result};
    use std::net::ToSocketAddrs;
    use std::ops::DerefMut;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    include!("tests/_fixtures.rs");
    include!("tests/_drains.rs");

}
