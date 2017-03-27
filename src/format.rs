use serializers::{KsvSerializerQuotedValue, KsvSerializerUnquoted};

use slog::{Record, OwnedKeyValueList};
use slog_stream::Format as StreamFormat;
use std::io;
use std::marker::PhantomData;
use syslog::{Facility, Priority};
use time::{FormatTimestamp, OmitTimestamp, Ts3164Local, Ts3164Utc, TsIsoLocal, TsIsoUtc};

// Write separator
macro_rules! write_sp { ($io:expr) => ( write!($io, " ") ) }

// Write RFC5424 NILVALUE
macro_rules! write_nilvalue { ($io:expr) => ( write!($io, "-") ) }

// Write end of message some server implementation need NULL Termination, some need LF
// some need both, so let's send both
macro_rules! write_eom { ($io:expr) => ( write!($io, "\n\0") ) }

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
pub trait FormatHeader {
    /// Associated `time::Timestamp`
    type Timestamp;

    /// Syslog Header formatter constructor
    fn new(fields: HeaderFields) -> Self;

    /// Format syslog header
    #[allow(dead_code)]
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()>;
}

/// RFC3164 Minimal header (PRIORITY HOSTNAME TAG)
pub struct Minimal;

/// RFC3164 Full header (PRIORITY TIMESTAMP TAG)
pub struct Full;

pub trait Rfc3164Header {}

impl Rfc3164Header for Minimal {}
impl Rfc3164Header for Full {}


/// RFC3164 Header
#[derive(Debug)]
pub struct HeaderRFC3164<T, F>
    where T: FormatTimestamp,
          F: Rfc3164Header
{
    fields: HeaderFields,
    _timestamp: PhantomData<T>,
    _header_format: PhantomData<F>,
}

impl<T, F> HeaderRFC3164<T, F>
    where T: FormatTimestamp,
          F: Rfc3164Header
{
    fn format_prioriy(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        let priority = Priority::new(self.fields.facility, record.level().into());
        write!(io, "<{}>", priority)?;
        Ok(())
    }

    fn format_tag(&self, io: &mut io::Write) -> io::Result<()> {
        match self.fields.process_name {
            Some(ref process_name) => write!(io, "{}[{}]:", process_name, self.fields.pid)?,
            None => write!(io, "[{}]:", self.fields.pid)?,
        }
        Ok(())
    }

    fn format_timestamp(&self, io: &mut io::Write) -> io::Result<()> {
        T::format(io)?;
        Ok(())
    }

}

impl FormatHeader for HeaderRFC3164<OmitTimestamp, Minimal> {

    type Timestamp = OmitTimestamp;

    fn new(fields: HeaderFields) -> Self {
        HeaderRFC3164::<OmitTimestamp, Minimal> {
            fields: fields,
            _timestamp: PhantomData,
            _header_format: PhantomData,
        }
    }

    #[allow(unused_variables)]
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        // PRIORITY
        HeaderRFC3164::format_prioriy(&self, io, record)?;

        // TAG process_name[pid]:
        self.format_tag(io)?;

        write_sp!(io)?;

        Ok(())
    }
}

impl<T> FormatHeader for HeaderRFC3164<T, Full>
    where T: FormatTimestamp
{

    type Timestamp = T;

    fn new(fields: HeaderFields) -> Self {
        HeaderRFC3164::<T, Full> {
            fields: fields,
            _timestamp: PhantomData,
            _header_format: PhantomData,
        }
    }

    #[allow(unused_variables)]
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        // PRIORITY
        self.format_prioriy(io, record)?;

        // TIMESTAMP
        self.format_timestamp(io)?;
        // write_sp!(io)?;

        // HOSTNAME
        if let Some(ref hostname) = self.fields.hostname {
            write!(io, "{}", hostname)?
        }
        write_sp!(io)?;

        // TAG process_name[pid]:
        self.format_tag(io)?;
        write_sp!(io)?;

        Ok(())
    }
}

/// RFC5424 Header
#[derive(Debug)]
pub struct HeaderRFC5424<T> {
    fields: HeaderFields,
    _timestamp: PhantomData<T>,
}


impl<T> FormatHeader for HeaderRFC5424<T>
    where T: FormatTimestamp
{
    type Timestamp = T;

    fn new(fields: HeaderFields) -> Self {
        HeaderRFC5424::<T> {
            fields: fields,
            _timestamp: PhantomData,
        }
    }

    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        // <PRIORITY>VERSION
        write!(io,
               "<{}>1",
               Priority::new(self.fields.facility, record.level().into()))?;

        // SP
        write_sp!(io)?;

        // TIMESTAMP (ISOTIMESTAMP)
        T::format(io)?;

        // SP
        write_sp!(io)?;

        // HOSTNAME
        match self.fields.hostname {
            Some(ref hostname) => write!(io, "{}", hostname)?,
            None => write_nilvalue!(io)?,
        }

        // SP
        write_sp!(io)?;

        // APPLICATION
        match self.fields.process_name {
            Some(ref process_name) => write!(io, "{}", process_name)?,
            None => write_nilvalue!(io)?,
        }

        // SP
        write_sp!(io)?;

        // PID
        write!(io, "{}", self.fields.pid)?;
        write_sp!(io)?;

        // MESSAGEID
        write!(io, "{}", record.level().to_string())?;

        // SP
        write_sp!(io)?;

        // MESSAGE STRUCTURED_DATA
        write!(io, "{}", "[")?;
        write!(io, "{}{}", "msg@", record.line())?;
        let mut serializer = KsvSerializerQuotedValue::new(io, "=");
        for &(k, v) in record.values().iter().rev() {
            serializer.emit_delimiter()?;
            v.serialize(record, k, &mut serializer)?;
        }
        let mut io = serializer.finish();
        write!(io, "{}", "]")?;

        write!(io, "{}", "[")?;
        write!(io, "{}{}", "logger@", record.line())?;
        let mut serializer = KsvSerializerQuotedValue::new(io, "=");
        for (k, v) in logger_values.iter() {
            serializer.emit_delimiter()?;
            v.serialize(record, k, &mut serializer)?;
        }
        let mut io = serializer.finish();
        write!(io, "{}", "]")?;

        // SP
        write_sp!(io)?;

        Ok(())
    }
}

/// Generic Syslog Message formatter
pub trait FormatMessage {
    /// Format syslog message
    fn format(io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()>;
}

/// RFC5424 structured data message
#[derive(Debug)]
pub struct MessageRFC5424;

/// Ksv Serialized message
#[derive(Debug)]
pub struct MessageKsv;

impl FormatMessage for MessageRFC5424 {
    #[allow(unused_variables)]
    fn format(io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        // MESSAGE
        write!(io, "{}", record.msg())?;

        Ok(())
    }
}

impl FormatMessage for MessageKsv {
    fn format(io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        // MESSAGE
        write!(io, "{}", record.msg())?;

        // MESSAGE STRUCTURED_DATA
        let mut serializer = KsvSerializerUnquoted::new(io, "=");

        for &(k, v) in record.values().iter().rev() {
            serializer.emit_delimiter()?;
            v.serialize(record, k, &mut serializer)?;
        }

        for (k, v) in logger_values.iter() {
            serializer.emit_delimiter()?;
            v.serialize(record, k, &mut serializer)?;
        }

        Ok(())
    }
}

// SyslogFormatter invariants

/// RFC3164 message formatter without timestamp and hostname with Ksv serialized data
/// for logging to Unix domain socket only
pub type Rfc3164MinimalKsv = SyslogFormatter<HeaderRFC3164<OmitTimestamp, Minimal>, MessageKsv>;

/// RFC3164 message formatter with Ksv serialized data
pub type Rfc3164Ksv<T> = SyslogFormatter<HeaderRFC3164<T, Full>, MessageKsv>;

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
pub trait SyslogFormat {
    // type FormatHeader;
    // type FormatMessage;

    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()>;
}

impl<H, M> SyslogFormatter<H, M>
    where H: FormatHeader + Send + Sync,
          H::Timestamp: FormatTimestamp,
          M: FormatMessage + Send + Sync
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
    where H: FormatHeader + Send + Sync,
          H::Timestamp: FormatTimestamp + Send + Sync,
          M: FormatMessage + Send + Sync
{
    /// Format syslog message
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        // HEADER
        H::format(&self.header, io, record, logger_values)?;

        // MESSAGE
        M::format(io, record, logger_values)?;

        // EOM
        write_eom!(io)?;

        Ok(())
    }
}

impl<H, M> StreamFormat for SyslogFormatter<H, M>
    where H: FormatHeader + Send + Sync,
          H::Timestamp: FormatTimestamp + Send + Sync,
          M: FormatMessage + Send + Sync
{
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        (self as &SyslogFormat).format(io, record, logger_values)?;
        Ok(())
    }
}

/// RFC5424 message formatter with Ksv serialized data
pub type Rfc5424Ksv<T> = SyslogFormatter<HeaderRFC5424<T>, MessageKsv>;

/// RFC5424 message formatter with RFC5424 structured data
pub type Rfc5424Native<T> = SyslogFormatter<HeaderRFC5424<T>, MessageRFC5424>;

// SyslogFormatter invariants with timestamps

// RFC3164

/// RFC13614, Ksv, Local TZ
pub type Rfc3164KsvTs3164Local = Rfc3164Ksv<Ts3164Local>;

/// RFC13614, Ksv, UTC
pub type Rfc3164KsvTs3164Utc = Rfc3164Ksv<Ts3164Utc>;

/// RFC13614, Ksv, ISO8601, Local TZ
pub type Rfc3164KsvTsIsoLocal = Rfc3164Ksv<TsIsoLocal>;

/// RFC13614, Ksv, ISO8601, UTC
pub type Rfc3164KsvTsIsoUtc = Rfc3164Ksv<TsIsoUtc>;

// RFC 5424

/// RFC5424, Ksv, Local TZ
pub type Rfc5424KsvTsIsoLocal = Rfc5424Ksv<TsIsoLocal>;

/// RFC5424, Ksv, UTC
pub type Rfc5424KsvTsIsoUtc = Rfc5424Ksv<TsIsoUtc>;

/// RFC5424, Local TZ
pub type Rfc5424NativeTsIsoLocal = Rfc5424Native<TsIsoLocal>;

/// RFC5424, UTC
pub type Rfc5424NativeTsIsoUtc = Rfc5424Native<TsIsoUtc>;
