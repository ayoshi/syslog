
use config::SerializationFormat;
use config::FormatMode;
use drains::TimestampFn;
use serializers::KSVSerializer;

use slog::{Level, Serializer, Record, OwnedKeyValueList};
use slog_stream::Format as StreamFormat;
use std::io;
use syslog::{Facility, Priority};


pub struct Format3164 {
    mode: FormatMode,
    fn_timestamp: Box<TimestampFn>,
    hostname: Option<String>,
    process_name: Option<String>,
    pid: i32,
    facility: Facility,
    serialization_format: SerializationFormat,
}

impl Format3164 {
    pub fn new(mode: FormatMode,
               fn_timestamp: Box<TimestampFn>,
               hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility,
               serialization_format: SerializationFormat)
               -> Self {
        Format3164 {
            mode: mode,
            fn_timestamp: fn_timestamp,
            hostname: hostname,
            process_name: process_name,
            pid: pid,
            facility: facility,
            serialization_format: serialization_format,
        }
    }

    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        write!(io,
               "<{}> ",
               Priority::new(self.facility, record.level().into()))?;

        (self.fn_timestamp)(io)?;

        write!(io, " ")?;

        match self.hostname {
            Some(ref hostname) => write!(io, "{} ", hostname)?,
            None => {}
        }

        match self.process_name {
            Some(ref process_name) => write!(io, "{}[{}]: ", process_name, self.pid)?,
            None => write!(io, "[{}]: ", self.pid)?,
        }

        write!(io, "{} ", record.msg())?;

        let mut serializer = KSVSerializer::new(io, "=");

        for &(k, v) in record.values().iter().rev() {
            v.serialize(record, k, &mut serializer)?;
        }

        let mut io = serializer.finish();

        write!(io, " ")?;

        let mut serializer = KSVSerializer::new(io, "=");

        for (k, v) in logger_values.iter() {
            v.serialize(record, k, &mut serializer)?;
        }

        let mut io = serializer.finish();

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
