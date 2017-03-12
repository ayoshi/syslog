use chrono;
use std::io;
use std::marker::PhantomData;


/// Timestamp in local TZ
pub struct TimestampLocal {}

/// UTC timestamp
pub struct TimestampUTC {}

/// RFC3164 compatible timestamp
pub struct TimestampRFC3164 {}

/// ISO8601 timestamp
pub struct TimestampISO8601 {}

/// Generic timestamp formatter
pub trait FormatTimestamp {
    /// Format timestamp in a given format
    fn format(&mut io::Write) -> io::Result<()>;
}

/// Timestamp
#[derive(Default)]
pub struct Timestamp<TZ, TF> {
    _tz: PhantomData<TZ>,
    _tf: PhantomData<TF>,
}

impl FormatTimestamp for Timestamp<TimestampRFC3164, TimestampLocal> {
    fn format(io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::Local::now().format("%b %d %T"))
    }
}

impl FormatTimestamp for Timestamp<TimestampRFC3164, TimestampUTC> {
    fn format(io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::UTC::now().format("%b %d %T"))
    }
}

impl FormatTimestamp for Timestamp<TimestampISO8601, TimestampLocal> {
    fn format(io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::Local::now().to_rfc3339())
    }
}

impl FormatTimestamp for Timestamp<TimestampISO8601, TimestampUTC> {
    fn format(io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::UTC::now().to_rfc3339())
    }
}
