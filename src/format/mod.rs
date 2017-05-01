// Macros used by childred
// Write separator
#[macro_export]
macro_rules! write_sp { ($io:expr) => ( write!($io, " ") ) }

// Write Rfc5424 NILVALUE
#[macro_export]
macro_rules! write_nilvalue { ($io:expr) => ( write!($io, "-") ) }

// Write end of message some server implementation need NULL Termination, some need LF
// some need both, so let's send both
#[macro_export]
macro_rules! write_eom { ($io:expr) => ( write!($io, "\n\0") ) }


mod rfc5424;
mod rfc3164;

use self::rfc3164::{Rfc3164, Rfc3164Short, Rfc3164Full};
use self::rfc5424::{Rfc5424, Rfc5424Short, Rfc5424Full};
use serializers::KsvSerializerUnquoted;

use slog::{Record, OwnedKVList, KV};
use std::io;
use std::panic::{UnwindSafe, RefUnwindSafe};
use std::marker::PhantomData;
use syslog::Facility;
use time::{FormatTimestamp, OmitTimestamp, Ts3164Local, Ts3164Utc, TsIsoLocal, TsIsoUtc};


/// Syslog header fields
#[derive(Debug)]
pub struct HeaderFields {
    hostname: Option<String>,
    process_name: Option<String>,
    pid: i32,
    facility: Facility,
}

impl HeaderFields {
    /// Syslog header fields constructor
    pub fn new(hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility)
               -> Self {
        HeaderFields {
            hostname: hostname,
            process_name: process_name,
            pid: pid,
            facility: facility,
        }
    }
}

/// Generic Syslog Header Formatter
pub trait FormatHeader: UnwindSafe + RefUnwindSafe + Send + Sync + 'static  {
    /// Associated `time::Timestamp`
    type Timestamp;

    /// Create Header Formatter
    fn new(fields: HeaderFields) -> Self;

    /// Format syslog header
    #[allow(dead_code)]
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKVList)
              -> io::Result<()>;
}

/// Plain record message
#[derive(Debug)]
pub struct MessageOnly;

/// Message with (key,sperator,value) serialized data
#[derive(Debug)]
pub struct MessageWithKsv;

/// Generic Syslog Message formatter
pub trait FormatMessage: UnwindSafe + RefUnwindSafe + Send + Sync + 'static  {
    /// Format syslog message
    fn format(io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKVList)
              -> io::Result<()>;
}

impl FormatMessage for MessageOnly {
    #[allow(unused_variables)]
    fn format(io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKVList)
              -> io::Result<()> {

        // MESSAGE
        write!(io, "{}", record.msg())?;

        Ok(())
    }
}

impl FormatMessage for MessageWithKsv {
    fn format(io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKVList)
              -> io::Result<()> {

        // MESSAGE
        write!(io, "{}", record.msg())?;

        // MESSAGE STRUCTURED_DATA
        let mut serializer = KsvSerializerUnquoted::new(io, "=");
        record.kv().serialize(record, &mut serializer)?;
        logger_values.serialize(record, &mut serializer)?;

        Ok(())
    }
}


/// Generic Syslog Formatter
#[derive(Debug, Clone)]
pub struct SyslogFormatter<H, M>
    where H: FormatHeader,
          H::Timestamp: FormatTimestamp,
          M: FormatMessage
{
    header: H,
    _message: PhantomData<M>,
}

/// Format syslog message
pub trait SyslogFormat: UnwindSafe + RefUnwindSafe + Send + Sync + 'static {
    /// Format Syslog Message
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKVList)
              -> io::Result<()>;
}

impl<H, M> SyslogFormatter<H, M>
    where H: FormatHeader,
          H::Timestamp: FormatTimestamp,
          M: FormatMessage
{
    ///
    pub fn new(hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility)
               -> Self {

        let header_fields = HeaderFields::new(hostname, process_name, pid, facility);
        let header = H::new(header_fields);

        SyslogFormatter {
            header: header,
            _message: PhantomData,
        }

    }
}

impl<H, M> SyslogFormat for SyslogFormatter<H, M>
    where H: FormatHeader,
          H::Timestamp: FormatTimestamp,
          M: FormatMessage
{
    /// Format syslog message
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKVList)
              -> io::Result<()> {

        // HEADER
        H::format(&self.header, io, record, logger_values)?;

        write_sp!(io)?; // SP

        // MESSAGE
        M::format(io, record, logger_values)?;

        // EOM
        write_eom!(io)?;

        Ok(())
    }
}

// RFC3164 formatter invariants

/// Rfc3164 message formatter without timestamp and hostname with Ksv serialized data
/// for logging to Unix domain socket only
pub type Rfc3164ShortKsv = SyslogFormatter<Rfc3164<OmitTimestamp, Rfc3164Short>, MessageWithKsv>;

/// Rfc3164 message formatter with Ksv serialized data
pub type Rfc3164FullKsv<T> = SyslogFormatter<Rfc3164<T, Rfc3164Full>, MessageWithKsv>;

/// Rfc13614, Ksv, Local TZ
pub type Rfc3164KsvTs3164Local = Rfc3164FullKsv<Ts3164Local>;

/// Rfc13614, Ksv, UTC
pub type Rfc3164KsvTs3164Utc = Rfc3164FullKsv<Ts3164Utc>;

/// Rfc13614, Ksv, ISO8601, Local TZ
pub type Rfc3164KsvTsIsoLocal = Rfc3164FullKsv<TsIsoLocal>;

/// Rfc13614, Ksv, ISO8601, UTC
pub type Rfc3164KsvTsIsoUtc = Rfc3164FullKsv<TsIsoUtc>;


// RFC5424 formatter invariants

/// Rfc5424 message formatter with Ksv serialized data
pub type Rfc5424Ksv<T, F> = SyslogFormatter<Rfc5424<T, F>, MessageWithKsv>;

/// Rfc5424 message formatter with RFC5424 structured data
pub type Rfc5424Native<T, F> = SyslogFormatter<Rfc5424<T, F>, MessageOnly>;

// SyslogFormatter invariants with timestamps

/// Rfc5424, Ksv, Local TZ
pub type Rfc5424KsvTsIsoLocal = Rfc5424Ksv<TsIsoLocal, Rfc5424Short>;

/// Rfc5424, Ksv, UTC
pub type Rfc5424KsvTsIsoUtc = Rfc5424Ksv<TsIsoUtc, Rfc5424Short>;

/// Rfc5424, Local TZ
pub type Rfc5424NativeTsIsoLocal = Rfc5424Native<TsIsoLocal, Rfc5424Full>;

/// Rfc5424, UTC
pub type Rfc5424NativeTsIsoUtc = Rfc5424Native<TsIsoUtc, Rfc5424Full>;
