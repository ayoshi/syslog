use chrono;

use config::{TimestampFormat, TimestampTZ};
use std::io;


/// Timestamp function type
pub type TimestampFn = Fn(&mut io::Write) -> io::Result<()> + Send + Sync;

/// Timestamp configuration
pub struct TimestampConfig {
    tf: TimestampFormat,
    tz: TimestampTZ,
}

impl TimestampConfig {
    ///
    pub fn new(tf: TimestampFormat, tz: TimestampTZ) -> Self {
        TimestampConfig { tf: tf, tz: tz }
    }

    /// RFC3164 local timestamp function used by `Format`
    fn _local_rfc3164(io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::Local::now().format("%b %d %T"))
    }

    /// RFC3164 UTC timestamp function used by `Format`
    fn _utc_rfc3164(io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::UTC::now().format("%b %d %T"))
    }

    /// ISO8601 local timestamp function used by `Format`
    fn _local_iso8601(io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::Local::now().to_rfc3339())
    }

    /// ISO8691 UTC timestamp function used by `Format`
    fn _utc_iso8601(io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::UTC::now().to_rfc3339())
    }

    /// Return boxed timestamp function for timezone/timestamp format
    /// Function writes timestamp in specifed format into any `io` stream
    pub fn timestamp_fn(self) -> Box<TimestampFn> {
        let timestamp_fn = match (self.tf, self.tz) {
            (TimestampFormat::RFC3164, TimestampTZ::Local) => TimestampConfig::_local_rfc3164,
            (TimestampFormat::RFC3164, TimestampTZ::UTC) => TimestampConfig::_utc_rfc3164,
            (TimestampFormat::ISO8601, TimestampTZ::Local) => TimestampConfig::_local_iso8601,
            (TimestampFormat::ISO8601, TimestampTZ::UTC) => TimestampConfig::_utc_iso8601,
        };
        Box::new(timestamp_fn)
    }
}
