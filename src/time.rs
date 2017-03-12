use chrono;
use std::marker::PhantomData;
use config::{TimestampFormat, TimestampTZ};
use std::io;


pub struct TimestampLocal {}
pub struct TimestampUTC {}

pub struct TimestampRFC3164 {}
pub struct TimestampISO8601 {}

pub trait FormatTimestamp {

    fn format (&mut io::Write) -> io::Result<()>;
}

#[derive(Default)]
pub struct Timestamp<TZ, TF>
{
    _tz: PhantomData<TZ>,
    _tf: PhantomData<TF>
}

impl FormatTimestamp for Timestamp<TimestampRFC3164, TimestampLocal> {

    fn format (io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::Local::now().format("%b %d %T"))
    }
}

impl FormatTimestamp for Timestamp<TimestampRFC3164, TimestampUTC> {

    fn format (io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::UTC::now().format("%b %d %T"))
    }
}

impl FormatTimestamp for Timestamp<TimestampISO8601, TimestampLocal> {

    fn format (io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::Local::now().to_rfc3339())
    }
}

impl FormatTimestamp for Timestamp<TimestampISO8601, TimestampUTC> {

    fn format (io: &mut io::Write) -> io::Result<()> {
        write!(io, "{}", chrono::UTC::now().to_rfc3339())
    }
}
