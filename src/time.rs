use chrono;

use config::{TimestampFormat, TimestampTZ};
use std::io;


/// Timestamp function type
pub type TimestampFn = Fn(&mut io::Write) -> io::Result<()> + Send + Sync;

/// RFC3164 local timestamp function used by `Format`
fn timestamp_local_rfc3164(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::Local::now().format("%b %d %T"))
}

/// RFC3164 UTC timestamp function used by `Format`
fn timestamp_utc_rfc3164(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::UTC::now().format("%b %d %T"))
}

/// ISO8601 local timestamp function used by `Format`
fn timestamp_local_iso8601(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::Local::now().to_rfc3339())
}

/// ISO8691 UTC timestamp function used by `Format`
fn timestamp_utc_iso8601(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::UTC::now().to_rfc3339())
}

/// Return timestamp function for timezone/timestamp format
pub fn timestamp(tf: TimestampFormat, tz: TimestampTZ) -> Box<TimestampFn> {
    let timestamp_fn = match (tf, tz) {
        (TimestampFormat::RFC3164, TimestampTZ::Local) => timestamp_local_rfc3164,
        (TimestampFormat::RFC3164, TimestampTZ::UTC) => timestamp_utc_rfc3164,
        (TimestampFormat::ISO8601, TimestampTZ::Local) => timestamp_local_iso8601,
        (TimestampFormat::ISO8601, TimestampTZ::UTC) => timestamp_utc_iso8601,
    };
    Box::new(timestamp_fn)
}
