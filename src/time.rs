
use chrono;

use config::{TimestampFormat, TimestampTZ};
use std::io;


/// Timestamp function type
pub type TimestampFn = Fn(&mut io::Write) -> io::Result<()> + Send + Sync;

/// RFC3164 local timestamp function used by `Format`
pub fn timestamp_local_rfc3164(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::Local::now().format("%b %d %T"))
}

/// RFC3164 UTC timestamp function used by `Format`
pub fn timestamp_utc_rfc3164(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::UTC::now().format("%b %d %T"))
}

/// ISO8601 local timestamp function used by `Format`
pub fn timestamp_local_iso8601(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::Local::now().to_rfc3339())
}

/// ISO8691 UTC timestamp function used by `Format`
pub fn timestamp_utc_iso8601(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::UTC::now().to_rfc3339())
}
