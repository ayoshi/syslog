use config::SerializationFormat;
use serializers::KSVSerializer;

use slog::{Serializer, Record, OwnedKeyValueList};
use slog_stream::Format as StreamFormat;
use std::io;
use syslog::{Facility, Priority};
use time::TimestampFn;


pub struct Format3164 {
    hostname: Option<String>,
    process_name: Option<String>,
    pid: i32,
    facility: Facility,
    fn_timestamp: Box<TimestampFn>,
    serialization_format: SerializationFormat,
}

impl Format3164 {
    pub fn new(hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility,
               fn_timestamp: Box<TimestampFn>,
               serialization_format: SerializationFormat)
               -> Self {
        Format3164 {
            fn_timestamp: fn_timestamp,
            hostname: hostname,
            process_name: process_name,
            pid: pid,
            facility: facility,
            serialization_format: serialization_format,
        }
    }

    fn format_header(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        // PRIORITY
        write!(io,
               "<{}> ",
               Priority::new(self.facility, record.level().into()))?;

        // TIMESTAMP
        (self.fn_timestamp)(io)?;
        write!(io, " ")?;

        // HOSTNAME
        match self.hostname {
            Some(ref hostname) => write!(io, "{} ", hostname)?,
            None => {}
        }

        // TAG process_name[pid]:
        match self.process_name {
            Some(ref process_name) => write!(io, "{}[{}]: ", process_name, self.pid)?,
            None => write!(io, "[{}]: ", self.pid)?,
        }

        Ok(())
    }

    fn format_message_ksv(&self,
                      io: &mut io::Write,
                      record: &Record,
                      logger_values: &OwnedKeyValueList)
                      -> io::Result<()> {

        // MESSAGE
        write!(io, "{} ", record.msg())?;

        // MESSAGE STRUCTURED_DATA
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
