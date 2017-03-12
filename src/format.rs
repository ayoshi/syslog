// use fields::{SerializationFormat, FormatMode};

use serializers::KSVSerializer;

use slog::{Record, OwnedKeyValueList};
use slog_stream::Format as StreamFormat;
use std::io;
use std::marker::PhantomData;
use syslog::{Facility, Priority};
use time::FormatTimestamp;

// Write separator
macro_rules! write_separator { ($io:expr) => ( write!($io, " ") ) }

// Write RFC5424 NILVALUE
macro_rules! write_nilvalue { ($io:expr) => ( write!($io, "-") ) }

// Write end of message
macro_rules! write_eom { ($io:expr) => ( write!($io, "\0") ) }


/// Syslog header fields
pub struct HeaderFields<T> {
    hostname: Option<String>,
    process_name: Option<String>,
    pid: i32,
    facility: Facility,
    _timestamp: PhantomData<T>,
}

impl<T> HeaderFields<T>
    where T: FormatTimestamp
{
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
            _timestamp: PhantomData,
        }
    }
}

/// Generic Syslog Header Formatter
pub trait FormatHeader<T> {
    /// Syslog Header formatter constructor
    fn new(fields: HeaderFields<T>) -> Self;

    /// Format syslog header
    fn format(&self, io: &mut io::Write, record: &Record) -> io::Result<()>;
}

/// RFC3164 Header
pub struct HeaderRFC3164<T> {
    fields: HeaderFields<T>,
}

/// RFC5424 Header
pub struct HeaderRFC5424<T> {
    fields: HeaderFields<T>,
}

impl<T> FormatHeader<T> for HeaderRFC3164<T>
    where T: FormatTimestamp
{
    fn new(fields: HeaderFields<T>) -> Self {
        HeaderRFC3164::<T> { fields: fields }
    }

    fn format(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        // PRIORITY
        write!(io,
               "<{}>",
               Priority::new(self.fields.facility, record.level().into()))?;
        write_separator!(io)?;

        // TIMESTAMP
        T::format(io)?;
        write_separator!(io)?;

        // HOSTNAME
        if let Some(ref hostname) = self.fields.hostname {
            write!(io, "{}", hostname)?
        }
        write_separator!(io)?;

        // TAG process_name[pid]:
        match self.fields.process_name {
            Some(ref process_name) => write!(io, "{}[{}]:", process_name, self.fields.pid)?,
            None => write!(io, "[{}]:", self.fields.pid)?,
        }
        write_separator!(io)?;

        Ok(())
    }
}

impl<T> FormatHeader<T> for HeaderRFC5424<T>
    where T: FormatTimestamp
{
    fn new(fields: HeaderFields<T>) -> Self {
        HeaderRFC5424::<T> { fields: fields }
    }

    fn format(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        // <PRIORITY>VERSION
        write!(io,
               "<{}>1",
               Priority::new(self.fields.facility, record.level().into()))?;
        write_separator!(io)?;

        // TIMESTAMP (ISOTIMESTAMP)
        T::format(io)?;
        write_separator!(io)?;

        // HOSTNAME
        match self.fields.hostname {
            Some(ref hostname) => write!(io, "{}", hostname)?,
            None => write_nilvalue!(io)?,
        }
        write_separator!(io)?;

        // APPLICATION
        match self.fields.process_name {
            Some(ref process_name) => write!(io, "{}", process_name)?,
            None => write_nilvalue!(io)?,
        }
        write_separator!(io)?;

        // PID
        write!(io, "{}", self.fields.pid)?;
        write_separator!(io)?;

        // MESSAGEID
        write_nilvalue!(io)?;
        write_separator!(io)?;

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
#[derive(Default)]
pub struct MessageRFC5424 {}

/// KSV Serialized message
#[derive(Default)]
pub struct MessageKSV {}

impl MessageRFC5424 {
    fn format_sd_element(io: &mut io::Write,
                         sd_id: String,
                         f: &Fn(&mut io::Write) -> io::Result<()>)
                         -> io::Result<()> {
        write!(io, "{}", "[")?;
        write!(io, "{}", sd_id)?;
        f(io)?;
        write!(io, "{}", "]")?;
        Ok(())
    }
}

impl FormatMessage for MessageRFC5424 {
    fn format(io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        // MESSAGE STRUCTURED_DATA
        MessageRFC5424::format_sd_element(io,
                                          format!("{}@{}", "logger", record.line()),
                                          &|io| {
            let mut serializer = KSVSerializer::new(io, "=");
            for &(k, v) in record.values().iter().rev() {
                v.serialize(record, k, &mut serializer)?;
            }
            Ok(())
        })?;

        MessageRFC5424::format_sd_element(io,
                                          format!("{}@{}", "msg", record.line()),
                                          &|io| {
            let mut serializer = KSVSerializer::new(io, "=");
            for (k, v) in logger_values.iter() {
                v.serialize(record, k, &mut serializer)?;
            }
            Ok(())
        })?;

        write_separator!(io)?;

        // MESSAGE
        write!(io, "{}", record.msg())?;
        write_separator!(io)?;

        Ok(())
    }
}

impl FormatMessage for MessageKSV {
    fn format(io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        // MESSAGE
        write!(io, "{}", record.msg())?;

        // MESSAGE STRUCTURED_DATA
        let mut serializer = KSVSerializer::new(io, "=");

        for &(k, v) in record.values().iter().rev() {
            v.serialize(record, k, &mut serializer)?;
        }

        for (k, v) in logger_values.iter() {
            v.serialize(record, k, &mut serializer)?;
        }

        Ok(())
    }
}

/// Generic Syslog Formatter
pub struct SyslogFormat<H, M, T>
    where H: FormatHeader<T>,
          M: FormatMessage,
          T: FormatTimestamp
{
    header: H,
    _message: PhantomData<M>,
    _timestamp: PhantomData<T>,
}


impl<H, M, T> SyslogFormat<H, M, T>
    where H: FormatHeader<T> + Send + Sync,
          M: FormatMessage + Send + Sync,
          T: FormatTimestamp + Send + Sync
{
    ///
    pub fn new(hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility)
               -> Self {

        let header_fields = HeaderFields::<T>::new(hostname, process_name, pid, facility);
        let header = H::new(header_fields);
        // let message = M::default();

        SyslogFormat {
            header: header,
            _message: PhantomData,
            _timestamp: PhantomData,
        }

    }

    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        // HEADER
        self.header.format(io, record)?;
        // MESSAGE
        M::format(io, record, logger_values)?;
        write_eom!(io)?;
        Ok(())
    }
}

impl<H, M, T> StreamFormat for SyslogFormat<H, M, T>
    where H: FormatHeader<T> + Send + Sync,
          M: FormatMessage + Send + Sync,
          T: FormatTimestamp + Send + Sync
{
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        self.format(io, record, logger_values)?;
        Ok(())
    }
}
