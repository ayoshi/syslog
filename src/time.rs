use chrono;
use std::io;
use std::marker::PhantomData;

/// Timestamp in local TZ
#[derive(Debug)]
pub struct TimestampLocal {}

/// UTC timestamp
#[derive(Debug)]
pub struct TimestampUTC {}

/// RFC3164 compatible timestamp
#[derive(Debug)]
pub struct TimestampRFC3164 {}

/// ISO8601 timestamp
#[derive(Debug)]
pub struct TimestampISO8601 {}

/// Omitted timestamp
#[derive(Debug)]
pub struct OmitTimestamp {}

/// Generic timestamp formatter
pub trait FormatTimestamp {
    /// Format timestamp in a given format
    fn format(&mut io::Write) -> io::Result<()>;
}

// All timestamp Invariants

/// ISO8601(RFC339) timestamp in UTC
pub type TsIsoUtc = Timestamp<TimestampISO8601, TimestampUTC>;

/// ISO8601(RFC339) timestamp in local TZ
pub type TsIsoLocal = Timestamp<TimestampISO8601, TimestampLocal>;

/// RFC3164 timestamp in UTC
pub type Ts3164Utc = Timestamp<TimestampRFC3164, TimestampUTC>;

/// RFC3164 timestamp in local TZ
pub type Ts3164Local = Timestamp<TimestampRFC3164, TimestampLocal>;

/// Timestamp
#[derive(Default, Debug)]
pub struct Timestamp<TZ, TF> {
    _tz: PhantomData<TZ>,
    _tf: PhantomData<TF>,
}

impl FormatTimestamp for Ts3164Local {
    fn format(io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::Local::now().format("%b %d %T"))
    }
}

impl FormatTimestamp for Ts3164Utc {
    fn format(io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::UTC::now().format("%b %d %T"))
    }
}

impl FormatTimestamp for TsIsoLocal {
    fn format(io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.6f%:z"))
    }
}

impl FormatTimestamp for TsIsoUtc {
    fn format(io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::UTC::now().format("%Y-%m-%dT%H:%M:%S%.6f%:z"))
    }
}

impl FormatTimestamp for OmitTimestamp {
    fn format(_: &mut io::Write) -> io::Result<()> {
        Ok(())
    }
}
