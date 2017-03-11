use config::{SerializationFormat, FormatMode};
use serializers::KSVSerializer;

use slog::{Record, OwnedKeyValueList, Serializer};
use slog_stream::Format as StreamFormat;
use std::io;
use syslog::{Facility, Priority};
use time::TimestampFn;

// Write separator
macro_rules! write_separator { ($i:expr) => ( write!($i, " ") ) }
// Write RFC5424 NILVALUE
macro_rules! write_nilvalue { ($i:expr) => ( write!($i, "-") ) }
// Write end of message
macro_rules! write_eom { ($i:expr) => ( write!($i, "\0") ) }


/// Syslog formatter
pub struct Format {
    hostname: Option<String>,
    process_name: Option<String>,
    pid: i32,
    facility: Facility,
    fn_timestamp: Box<TimestampFn>,
}

impl Format {
    /// Return an instance of syslog formatter
    pub fn new(hostname: Option<String>,
               process_name: Option<String>,
               pid: i32,
               facility: Facility,
               fn_timestamp: Box<TimestampFn>)
               -> Self {
        Format {
            hostname: hostname,
            process_name: process_name,
            pid: pid,
            facility: facility,
            fn_timestamp: fn_timestamp,
        }
    }

    fn format_header_rfc3164(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
        // PRIORITY
        write!(io,
               "<{}>",
               Priority::new(self.facility, record.level().into()))?;
        write_separator!(io)?;

        // TIMESTAMP
        (self.fn_timestamp)(io)?;
        write_separator!(io)?;

        // HOSTNAME
        match self.hostname {
            Some(ref hostname) => write!(io, "{}", hostname)?,
            None => {}
        }
        write_separator!(io)?;

        // TAG process_name[pid]:
        match self.process_name {
            Some(ref process_name) => write!(io, "{}[{}]:", process_name, self.pid)?,
            None => write!(io, "[{}]:", self.pid)?,
        }
        write_separator!(io)?;

        Ok(())
    }

    fn format_message_ksv(&self,
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

    // fn format_rfc3164_ksv(&self,
    //                       io: &mut io::Write,
    //                       record: &Record,
    //                       logger_values: &OwnedKeyValueList)
    //                       -> io::Result<()> {

    //     // HEADER
    //     self.format_header_rfc3164(io, record)?;

    //     // MESSAGE
    //     self.format_message_ksv(io, record, logger_values)?;

    //     // EOM
    //     write_eom!(io)?;

    //     Ok(())
    // }

    fn format_header_rfc5424(&self, io: &mut io::Write, record: &Record) -> io::Result<()> {
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

    fn _format_sd_element(&self,
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

    fn format_message_rfc5424(&self,
                                     io: &mut io::Write,
                                     record: &Record,
                                     logger_values: &OwnedKeyValueList)
                                     -> io::Result<()> {

        // MESSAGE STRUCTURED_DATA

        // write!(io, "{}", "[")?;

        self._format_sd_element(io,
                                format!("{}@{}", "logger", self.pid),
                                &|io| {
                let mut serializer = KSVSerializer::new(io, "=");
                for &(k, v) in record.values().iter().rev() {
                    v.serialize(record, k, &mut serializer)?;
                }
                Ok(())
            })?;

        self._format_sd_element(io,
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

    // fn format_rfc5424_native(&self,
    //                          io: &mut io::Write,
    //                          record: &Record,
    //                          logger_values: &OwnedKeyValueList)
    //                          -> io::Result<()> {

    //     // HEADER
    //     self.format_header_rfc5424(io, record)?;

    //     // MESSAGE
    //     self.format_message_native_rfc5424(io, record, logger_values)?;

    //     // EOM
    //     write_eom!(io)?;

    //     Ok(())
    // }

    // fn format_rfc5424_ksv(&self,
    //                       io: &mut io::Write,
    //                       record: &Record,
    //                       logger_values: &OwnedKeyValueList)
    //                       -> io::Result<()> {

    //     // HEADER
    //     self.format_header_rfc5424(io, record)?;

    //     // MESSAGE
    //     self.format_message_ksv(io, record, logger_values)?;

    //     // EOM
    //     write_eom!(io)?;

    //     Ok(())
    // }
}

// impl StreamFormat for Format {
//     fn format(&self,
//               io: &mut io::Write,
//               record: &Record,
//               logger_values: &OwnedKeyValueList)
//               -> io::Result<()> {
//         match self.mode {
//             FormatMode::RFC3164 => self.format_rfc3164(io, record, logger_values),
//             FormatMode::RFC5424 => self.format_rfc5424(io, record, logger_values),
//         }
//     }
// }

pub type HeaderWriterFn = Fn(&mut io::Write, &Record) -> io::Result<()> + Send + Sync;


pub type MessageWriterFn =
    Fn(&mut io::Write, &Record, &OwnedKeyValueList) -> io::Result<()> + Send + Sync;


struct SyslogFormat {
    write_header: Box<HeaderWriterFn>,
    write_message: Box<MessageWriterFn>,
}

impl SyslogFormat {

    pub fn new(
        hostname: Option<String>,
        process_name: Option<String>,
        pid: i32,
        facility: Facility,
        fn_timestamp: Box<TimestampFn>,
        mode: FormatMode,
        serialization_format: SerializationFormat
    ) -> Self {

        let formatter = Format::new(
            hostname,
            process_name,
            pid,
            facility,
            fn_timestamp,
        );

        let (write_header, write_message) = match (mode, serialization_format) {
            (FormatMode::RFC3164, SerializationFormat::KSV) => (Box::new(formatter.format_header_rfc3164), Box::new(formatter.format_message_ksv)),
        };

        SyslogFormat {
            write_header: write_header,
            write_message: write_message
        }

    }

    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        // HEADER
        (self.write_header)(io, record)?;
        // MESSAGE
        (self.write_message)(io, record, logger_values)?;
        write_eom!(io)?;
        Ok(())
    }
}

// impl StreamFormat for SyslogFormat {


// }
