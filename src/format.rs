// use config::SerializationFormat;
use serializers::KSVSerializer;

use slog::{Record, OwnedKeyValueList};
use slog_stream::Format as StreamFormat;
use std::io;
use syslog::{Facility, Priority};
use time::TimestampFn;

// RFC5424 NILVALUE
const NILVALUE: &'static str = "-";

// Write separator
macro_rules! write_separator { ($i:expr) => ( write!($i, " ")? ) }


/// RFC3164 formatter
pub struct Format3164 {
    hostname: Option<String>,
    process_name: Option<String>,
    pid: i32,
    facility: Facility,
    fn_timestamp: Box<TimestampFn>,
}

impl Format3164 {
    /// Return an instance of RFC3164 compatible formatter
    pub fn new(hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility,
               fn_timestamp: Box<TimestampFn>)
               -> Self {
        Format3164 {
            fn_timestamp: fn_timestamp,
            hostname: hostname,
            process_name: process_name,
            pid: pid,
            facility: facility,
        }
    }

    fn format_header(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        // PRIORITY
        write!(io,
               "<{}>",
               Priority::new(self.facility, record.level().into()))?;
        write_separator!(io);

        // TIMESTAMP
        (self.fn_timestamp)(io)?;
        write_separator!(io);

        // HOSTNAME
        match self.hostname {
            Some(ref hostname) => write!(io, "{}", hostname)?,
            None => {}
        }
        write_separator!(io);

        // TAG process_name[pid]:
        match self.process_name {
            Some(ref process_name) => write!(io, "{}[{}]:", process_name, self.pid)?,
            None => write!(io, "[{}]:", self.pid)?,
        }
        write_separator!(io);

        Ok(())
    }

    fn format_message_ksv(&self,
                          io: &mut io::Write,
                          record: &Record,
                          logger_values: &OwnedKeyValueList)
                          -> io::Result<()> {

        // MESSAGE
        write!(io, "{}", record.msg())?;
        write_separator!(io);

        // MESSAGE STRUCTURED_DATA
        let mut serializer = KSVSerializer::new(io, "=");

        for &(k, v) in record.values().iter().rev() {
            v.serialize(record, k, &mut serializer)?;
        }

        let mut io = serializer.finish();
        write_separator!(io);

        let mut serializer = KSVSerializer::new(io, "=");

        for (k, v) in logger_values.iter() {
            v.serialize(record, k, &mut serializer)?;
        }

        Ok(())
    }

    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        // HEADER
        self.format_header(io, record)?;

        // MESSAGE
        self.format_message_ksv(io, record, logger_values)?;

        // EOL
        write!(io, "\n")?;

        Ok(())
    }
}

/// RFC5424 formatter

pub struct Format5424 {
    hostname: Option<String>,
    process_name: Option<String>,
    pid: i32,
    facility: Facility,
    fn_timestamp: Box<TimestampFn>,
}

impl Format5424 {


    /// Return an instance of RFC5424(IETF) compatible formatter
    pub fn new(hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility,
               fn_timestamp: Box<TimestampFn>)
               -> Self {
        Format5424 {
            fn_timestamp: fn_timestamp,
            hostname: hostname,
            process_name: process_name,
            pid: pid,
            facility: facility,
        }
    }

    fn format_header(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        // <PRIORITY>VERSION
        write!(io,
               "<{}>1",
               Priority::new(self.facility, record.level().into()))?;
        write_separator!(io);

        // TIMESTAMP (ISOTIMESTAMP)
        (self.fn_timestamp)(io)?;
        write_separator!(io);

        // HOSTNAME
        match self.hostname {
            Some(ref hostname) => write!(io, "{}", hostname)?,
            None => write!(io, "{}", NILVALUE)?
        }
        write_separator!(io);

        // APPLICATION
        match self.process_name {
            Some(ref process_name) => write!(io, "{}", process_name)?,
            None => write!(io, "{}", NILVALUE)?
        }
        write_separator!(io);

        // PID
        write!(io, "{}", self.pid)?;
        write_separator!(io);

        // MESSAGEID
        write!(io, "{}", NILVALUE)?;
        write_separator!(io);

        Ok(())
    }

    fn format_message_ksv(&self,
                          io: &mut io::Write,
                          record: &Record,
                          logger_values: &OwnedKeyValueList)
                          -> io::Result<()> {

        // MESSAGE
        write!(io, "{}", record.msg())?;
        write_separator!(io);

        // MESSAGE STRUCTURED_DATA
        let mut serializer = KSVSerializer::new(io, "=");

        for &(k, v) in record.values().iter().rev() {
            v.serialize(record, k, &mut serializer)?;
        }

        let mut io = serializer.finish();
        write_separator!(io);

        let mut serializer = KSVSerializer::new(io, "=");

        for (k, v) in logger_values.iter() {
            v.serialize(record, k, &mut serializer)?;
        }

        Ok(())
    }

    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        // HEADER
        self.format_header(io, record)?;

        // MESSAGE
        self.format_message_ksv(io, record, logger_values)?;

        // EOL
        write!(io, "\n")?;

        Ok(())
    }
}

impl StreamFormat for Format3164 {
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        // match self.mode {
        //     FormatMode::RFC3164 => self.format(io, record, logger_values),
        //     FormatMode::RFC5424 => self.format_rfc5424(io, record, logger_values),
        self.format(io, record, logger_values)
    }
}

impl StreamFormat for Format5424 {
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        // match self.mode {
        //     FormatMode::RFC3164 => self.format(io, record, logger_values),
        //     FormatMode::RFC5424 => self.format_rfc5424(io, record, logger_values),
        self.format(io, record, logger_values)
    }
}
