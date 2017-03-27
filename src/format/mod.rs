mod rfc3164;
mod rfc5424;

use self::rfc3164::{Rfc3164, Rfc3164Short, Rfc3164Full};
use serializers::{KsvSerializerQuotedValue, KsvSerializerUnquoted};

use slog::{Record, OwnedKeyValueList};
use slog_stream::Format as StreamFormat;
use std::io;
use std::marker::PhantomData;
use syslog::{Facility, Priority};
use time::{FormatTimestamp, OmitTimestamp, Ts3164Local, Ts3164Utc, TsIsoLocal, TsIsoUtc};

// Write separator
macro_rules! write_sp { ($io:expr) => ( write!($io, " ") ) }

// Write Rfc5424 NILVALUE
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

    /// Format syslog header
    #[allow(dead_code)]
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()>;
}

/// Plain record message
#[derive(Debug)]
pub struct MessageOnly;

/// Message with (key,sperator,value) serialized data
#[derive(Debug)]
pub struct MessageWithKsv;

/// Generic Syslog Message formatter
pub trait FormatMessage {
    /// Format syslog message
    fn format(io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()>;
}

impl FormatMessage for MessageOnly {
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

impl FormatMessage for MessageWithKsv {
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
          H::Timestamp: FormatTimestamp + Send + Sync,
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

        write_sp!(io)?; // SP

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
