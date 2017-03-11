// use config::{SerializationFormat, FormatMode};
use serializers::KSVSerializer;

use slog::{Record, OwnedKeyValueList};
use slog_stream::Format as StreamFormat;
use std::io;
use syslog::{Facility, Priority};
use time::TimestampFn;

// Write separator
macro_rules! write_separator { ($io:expr) => ( write!($io, " ") ) }
// Write RFC5424 NILVALUE
macro_rules! write_nilvalue { ($io:expr) => ( write!($io, "-") ) }
// Write end of message
macro_rules! write_eom { ($io:expr) => ( write!($io, "\0") ) }


// All fields that are present in header
pub struct HeaderFields {
    hostname: Option<String>,
    process_name: Option<String>,
    pid: i32,
    facility: Facility,
    fn_timestamp: Box<TimestampFn>,
}

impl HeaderFields {
    pub fn new(hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility,
               fn_timestamp: Box<TimestampFn>)
               -> Self {
        HeaderFields {
            hostname: hostname,
            process_name: process_name,
            pid: pid,
            facility: facility,
            fn_timestamp: fn_timestamp,
        }
    }
}

pub trait FormatHeader {
    fn new(config: HeaderFields) -> Self;
    fn format(&self, io: &mut io::Write, record: &Record) -> io::Result<()>;
}

pub struct HeaderRFC3164 {
    config: HeaderFields,
}

impl HeaderRFC3164 {
}

pub struct HeaderRFC5424 {
    config: HeaderFields,
}

impl FormatHeader for HeaderRFC3164 {

    fn new(config: HeaderFields) -> Self {
        HeaderRFC3164 { config: config }
    }

    fn format(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        // PRIORITY
        write!(io,
               "<{}>",
               Priority::new(self.config.facility, record.level().into()))?;
        write_separator!(io)?;

        // TIMESTAMP
        (self.config.fn_timestamp)(io)?;
        write_separator!(io)?;

        // HOSTNAME
        if let Some(ref hostname) = self.config.hostname {
            write!(io, "{}", hostname)?
        }
        write_separator!(io)?;

        // TAG process_name[pid]:
        match self.config.process_name {
            Some(ref process_name) => write!(io, "{}[{}]:", process_name, self.config.pid)?,
            None => write!(io, "[{}]:", self.config.pid)?,
        }
        write_separator!(io)?;

        Ok(())
    }
}

impl FormatHeader for HeaderRFC5424 {

    fn new(config: HeaderFields) -> Self {
        HeaderRFC5424 { config: config }
    }

    fn format(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        // <PRIORITY>VERSION
        write!(io,
               "<{}>1",
               Priority::new(self.config.facility, record.level().into()))?;
        write_separator!(io)?;

        // TIMESTAMP (ISOTIMESTAMP)
        (self.config.fn_timestamp)(io)?;
        write_separator!(io)?;

        // HOSTNAME
        match self.config.hostname {
            Some(ref hostname) => write!(io, "{}", hostname)?,
            None => write_nilvalue!(io)?,
        }
        write_separator!(io)?;

        // APPLICATION
        match self.config.process_name {
            Some(ref process_name) => write!(io, "{}", process_name)?,
            None => write_nilvalue!(io)?,
        }
        write_separator!(io)?;

        // PID
        write!(io, "{}", self.config.pid)?;
        write_separator!(io)?;

        // MESSAGEID
        write_nilvalue!(io)?;
        write_separator!(io)?;

        Ok(())
    }
}

pub trait FormatMessage {
    fn new() -> Self;
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()>;
}

pub struct MessageRFC5424 {}
pub struct MessageKSV {}

impl MessageRFC5424 {

    fn format_sd_element(&self,
                         io: &mut io::Write,
                         sd_id: String,
                         f: &Fn(&mut io::Write) -> io::Result<()>)
                         -> io::Result<()> {
        write!(io, "{}", "[")?;
        write!(io, "{}", sd_id)?;
        write_separator!(io)?;
        f(io)?;
        write!(io, "{}", "]")?;
        Ok(())
    }
}

impl FormatMessage for MessageRFC5424 {

    fn new() -> Self {
        MessageRFC5424 {}
    }

    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        // MESSAGE STRUCTURED_DATA
        self.format_sd_element(io,
                               format!("{}@{}", "logger", record.line()),
                               &|io| {
                let mut serializer = KSVSerializer::new(io, "=");
                for &(k, v) in record.values().iter().rev() {
                    v.serialize(record, k, &mut serializer)?;
                }
                Ok(())
            })?;

        self.format_sd_element(io,
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

    fn new() -> Self {
        MessageKSV {}
    }

    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        // MESSAGE
        write!(io, "{}", record.msg())?;
        write_separator!(io)?;

        // MESSAGE STRUCTURED_DATA
        let mut serializer = KSVSerializer::new(io, "=");

        for &(k, v) in record.values().iter().rev() {
            v.serialize(record, k, &mut serializer)?;
        }

        let mut io = serializer.finish();
        write_separator!(io)?;

        let mut serializer = KSVSerializer::new(io, "=");

        for (k, v) in logger_values.iter() {
            v.serialize(record, k, &mut serializer)?;
        }

        Ok(())
    }
}

/// Generic Syslog Formatter
pub struct SyslogFormat<H, M>
    where H: FormatHeader,
          M: FormatMessage {
    header: H,
    message: M
}


impl <H, M> SyslogFormat<H, M>
    where H: FormatHeader + Send + Sync,
          M: FormatMessage + Send + Sync {
    ///
    pub fn new(hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility,
               fn_timestamp: Box<TimestampFn>)
               -> Self {

        let header_fields = HeaderFields::new(hostname, process_name, pid, facility, fn_timestamp);
        let header = H::new(header_fields);
        let message = M::new();

        SyslogFormat {
            header: header,
            message: message,
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
        self.message.format(io, record, logger_values)?;
        write_eom!(io)?;
        Ok(())
    }
}

impl <H,M>StreamFormat for SyslogFormat<H,M>
    where H: FormatHeader + Send + Sync,
          M: FormatMessage + Send + Sync {
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        self.format(io, record, logger_values)?;
        Ok(())
    }
}
