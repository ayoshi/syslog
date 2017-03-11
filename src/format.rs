use config::{SerializationFormat, FormatMode};
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


trait FormatHeader {
    fn format(&self, io: &mut io::Write, record: &Record) -> io::Result<()>;
}

trait FormatMessage {
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()>;
}

struct HeaderRFC3164 {
    hostname: Option<String>,
    process_name: Option<String>,
    pid: i32,
    facility: Facility,
    fn_timestamp: Box<TimestampFn>,
}

impl HeaderRFC3164 {
    pub fn new(hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility,
               fn_timestamp: Box<TimestampFn>)
               -> Self {
        HeaderRFC3164 {
            hostname: hostname,
            process_name: process_name,
            pid: pid,
            facility: facility,
            fn_timestamp: fn_timestamp,
        }
    }
}

struct HeaderRFC5424 {
    hostname: Option<String>,
    process_name: Option<String>,
    pid: i32,
    facility: Facility,
    fn_timestamp: Box<TimestampFn>,
}

impl HeaderRFC5424 {
    /// Return an instance of syslog formatter
    pub fn new(hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility,
               fn_timestamp: Box<TimestampFn>)
               -> Self {
        HeaderRFC5424 {
            hostname: hostname,
            process_name: process_name,
            pid: pid,
            facility: facility,
            fn_timestamp: fn_timestamp,
        }
    }
}

impl FormatHeader for HeaderRFC3164 {
    fn format(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        // PRIORITY
        write!(io,
               "<{}>",
               Priority::new(self.facility, record.level().into()))?;
        write_separator!(io)?;

        // TIMESTAMP
        (self.fn_timestamp)(io)?;
        write_separator!(io)?;

        // HOSTNAME
        if let Some(ref hostname) = self.hostname { write!(io, "{}", hostname)? }
        write_separator!(io)?;

        // TAG process_name[pid]:
        match self.process_name {
            Some(ref process_name) => write!(io, "{}[{}]:", process_name, self.pid)?,
            None => write!(io, "[{}]:", self.pid)?,
        }
        write_separator!(io)?;

        Ok(())
    }
}

impl FormatHeader for HeaderRFC5424 {
    fn format(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        // <PRIORITY>VERSION
        write!(io,
               "<{}>1",
               Priority::new(self.facility, record.level().into()))?;
        write_separator!(io)?;

        // TIMESTAMP (ISOTIMESTAMP)
        (self.fn_timestamp)(io)?;
        write_separator!(io)?;

        // HOSTNAME
        match self.hostname {
            Some(ref hostname) => write!(io, "{}", hostname)?,
            None => write_nilvalue!(io)?,
        }
        write_separator!(io)?;

        // APPLICATION
        match self.process_name {
            Some(ref process_name) => write!(io, "{}", process_name)?,
            None => write_nilvalue!(io)?,
        }
        write_separator!(io)?;

        // PID
        write!(io, "{}", self.pid)?;
        write_separator!(io)?;

        // MESSAGEID
        write_nilvalue!(io)?;
        write_separator!(io)?;

        Ok(())
    }
}

enum SyslogHeaderFormat {
    RFC3164(HeaderRFC3164),
    RFC5424(HeaderRFC5424),
}

impl FormatHeader for SyslogHeaderFormat {
    fn format(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        match *self {
            SyslogHeaderFormat::RFC3164(ref header) => header.format(io, record),
            SyslogHeaderFormat::RFC5424(ref header) => header.format(io, record),
        }
    }
}

struct MessageRFC5424 {}
struct MessageKSV {}

impl MessageKSV {
    pub fn new() -> Self {
        MessageKSV {}
    }
}

impl MessageRFC5424 {
    pub fn new() -> Self {
        MessageRFC5424 {}
    }

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

enum SyslogMessageFormat {
    KSV(MessageKSV),
    RFC5424(MessageRFC5424),
}

impl FormatMessage for SyslogMessageFormat {
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        match *self {
            SyslogMessageFormat::KSV(ref message) => message.format(io, record, logger_values),
            SyslogMessageFormat::RFC5424(ref message) => message.format(io, record, logger_values),
        }
    }
}

/// Generic Syslog Formatter
pub struct SyslogFormat {
    header: SyslogHeaderFormat,
    message: SyslogMessageFormat,
}


impl SyslogFormat {
    ///
    pub fn new(hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility,
               fn_timestamp: Box<TimestampFn>,
               mode: FormatMode,
               serialization_format: SerializationFormat)
               -> Self {

        let (header, message) = match (mode, serialization_format) {
            (FormatMode::RFC3164, SerializationFormat::KSV) |
            (FormatMode::RFC3164, SerializationFormat::Native) => {
                (SyslogHeaderFormat::RFC3164(HeaderRFC3164::new(hostname,
                                                                process_name,
                                                                pid,
                                                                facility,
                                                                fn_timestamp)),
                 SyslogMessageFormat::KSV(MessageKSV::new()))
            }
            (FormatMode::RFC5424, SerializationFormat::Native) => {
                (SyslogHeaderFormat::RFC5424(HeaderRFC5424::new(hostname,
                                                                process_name,
                                                                pid,
                                                                facility,
                                                                fn_timestamp)),
                 SyslogMessageFormat::RFC5424(MessageRFC5424::new()))
            }
            (FormatMode::RFC5424, SerializationFormat::KSV) => {
                (SyslogHeaderFormat::RFC5424(HeaderRFC5424::new(hostname,
                                                                process_name,
                                                                pid,
                                                                facility,
                                                                fn_timestamp)),
                 SyslogMessageFormat::KSV(MessageKSV::new()))
            }
            (FormatMode::RFC3164, SerializationFormat::CEE) => {
                (SyslogHeaderFormat::RFC5424(HeaderRFC5424::new(hostname,
                                                                process_name,
                                                                pid,
                                                                facility,
                                                                fn_timestamp)),
                 SyslogMessageFormat::KSV(MessageKSV::new()))
            }
            (FormatMode::RFC5424, SerializationFormat::CEE) => {
                (SyslogHeaderFormat::RFC5424(HeaderRFC5424::new(hostname,
                                                                process_name,
                                                                pid,
                                                                facility,
                                                                fn_timestamp)),
                 SyslogMessageFormat::KSV(MessageKSV::new()))
            }
        };

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

impl StreamFormat for SyslogFormat {
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        self.format(io, record, logger_values)?;
        Ok(())
    }
}
