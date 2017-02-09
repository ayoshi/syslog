//! Unix terminal formatter and drain for slog-rs
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_term;
//!
//! use slog::*;
//!
//! fn main() {
//!     let root = Logger::root(slog_term::streamer().build().fuse(), o!("build-id" => "8dfljdf"));
//! }
//! ```
#![warn(missing_docs)]

extern crate slog;
extern crate slog_stream;
extern crate chrono;
extern crate thread_local;

use std::{io, fmt, sync, cell};
use std::io::Write;

use slog::Record;
use slog::ser;
use slog::{Level, OwnedKeyValueList};
use slog_stream::Format as StreamFormat;
use slog_stream::{stream, async_stream};

thread_local! {
    static TL_BUF: cell::RefCell<Vec<u8>> = cell::RefCell::new(Vec::with_capacity(128));
}

/// Syslog severity
#[derive(Debug)]
pub enum Severity {
    Emerg = 0,
    Alert,
    Crit,
    Err,
    Warn,
    Notice,
    Info,
    Debug,
}

/// Syslog facility
#[derive(Debug)]
pub enum Facility {
    KERN = 0,
    USER = 1,
    MAIL = 2,
    DAEMON = 3,
    AUTH = 4,
    SYSLOG = 5,
    LPR = 6,
    NEWS = 7,
    UUCP = 8,
    CRON = 9,
    AUTHPRIV = 10,
    FTP = 11,
    LOCAL0 = 16,
    LOCAL1 = 17,
    LOCAL2 = 18,
    LOCAL3 = 19,
    LOCAL4 = 20,
    LOCAL5 = 21,
    LOCAL6 = 22,
    LOCAL7 = 23,
}

#[derive(Debug)]
pub struct Priority(u8);

impl Priority {
    pub fn new( facility: Facility, severity: Severity) -> Priority {
        let facility = facility as u8;
        let severity = severity as u8;
        Priority(facility << 3 | severity)
    }
}

//impl<'a> Into<u8> for &'a Priority {
//    fn into(self) -> u8 {
//      let facility = self.facility as u8;
//      let severity = self.severity as u8;
//      facility << 3 | severity
//    }
//}

/// Translate from level to severity
fn level_to_severity(level: slog::Level) -> Severity {
    match level {
        Level::Critical => Severity::Crit,
        Level::Error => Severity::Err,
        Level::Warning => Severity::Warn,
        Level::Info => Severity::Notice,
        Level::Debug => Severity::Info,
        Level::Trace => Severity::Debug,
    }
}

/// Timestamp function type
pub type TimestampFn = Fn(&mut io::Write) -> io::Result<()> + Send + Sync;

/// Formatting mode
pub enum FormatMode {
    /// Compact logging format
    RFC3164,
    /// Full logging format
    RFC5424,
}

/// Protocol
pub enum Protocol {
    /// Log to Unix socket
    UnixSocket,
    /// Log over TCP
    TCP,
    /// Log over UDP
    UDP,
}

/// Full formatting with optional color support
pub struct Format {
    mode: FormatMode,
    fn_timestamp: Box<TimestampFn>,
}

impl Format {
    pub fn new(mode: FormatMode, fn_timestamp: Box<TimestampFn>) -> Self {
        Format {
            mode: mode,
            fn_timestamp: fn_timestamp,
        }
    }

    /// Format a field
    fn fmt_msg(&self,
               io: &mut io::Write,
               f: &Fn(&mut io::Write) -> io::Result<()>)
               -> io::Result<()> {
        f(io)
    }
    /// Format a key
    fn fmt_key(&self,
               io: &mut io::Write,
               f: &Fn(&mut io::Write) -> io::Result<()>)
               -> io::Result<()> {
        f(io)
    }
    /// Format a separator
    fn fmt_separator(&self,
                     io: &mut io::Write,
                     f: &Fn(&mut io::Write) -> io::Result<()>)
                     -> io::Result<()> {
        f(io)
    }
    /// Format a value
    fn fmt_value(&self,
                 io: &mut io::Write,
                 f: &Fn(&mut io::Write) -> io::Result<()>)
                 -> io::Result<()> {
        f(io)
    }
    /// Format a timestamp
    fn fmt_timestamp(&self,
                     io: &mut io::Write,
                     f: &Fn(&mut io::Write) -> io::Result<()>)
                     -> io::Result<()> {
        f(io)
    }
    /// Format a level
    fn fmt_level(&self,
                 io: &mut io::Write,
                 f: &Fn(&mut io::Write) -> io::Result<()>)
                 -> io::Result<()> {
        f(io)
    }

    // Returns `true` if message was not empty
    fn print_msg_header(&self, io: &mut io::Write, record: &Record) -> io::Result<bool> {
        try!(self.fmt_timestamp(io, &*self.fn_timestamp));
        try!(self.fmt_level(io, &|io: &mut io::Write| write!(io, " {} ", record.level().as_short_str())));
        try!(self.fmt_msg(io, &|io: &mut io::Write| write!(io, "{}", record.msg())));
        Ok(true)
    }

    fn format_rfc5424(&self,
                      io: &mut io::Write,
                      record: &Record,
                      logger_values: &OwnedKeyValueList)
                      -> io::Result<()> {

        /// format RFC 5424 structured data as `([id (name="value")*])*`
        /// / pub fn format_5424_structured_data(&self, data: StructuredData) -> String {
        /// /if data.is_empty() {
        /// /"-".to_string()
        /// /} else {
        /// /let mut res = String::new();
        /// /for (id, params) in data.iter() {
        /// /res = res + "["+id;
        /// /for (name,value) in params.iter() {
        /// /res = res + " " + name + "=\"" + value + "\"";
        /// /}
        /// /res = res + "]";
        /// /}
        /// /
        /// /res
        /// /}
        /// /}
        ///
        /// // format a message as a RFC 5424 log message
        /// pub fn format_5424<T: fmt::Display>(&self, severity:Severity, message_id: i32, data: StructuredData, message: T) -> String {
        /// let f =  format!("<{}> {} {} {} {} {} {} {} {}",
        /// self.encode_priority(severity, self.facility),
        /// 1, // version
        /// time::now_utc().rfc3339(),
        /// self.hostname.as_ref().map(|x| &x[..]).unwrap_or("localhost"),
        /// self.process, self.pid, message_id,
        /// self.format_5424_structured_data(data), message);
        /// return f;
        ///
        ///
        ///        let mut comma_needed = try!(self.print_msg_header(io,  record));
        let mut serializer = Serializer::new(io);

        for &(k, v) in record.values().iter().rev() {
            try!(v.serialize(record, k, &mut serializer));
        }

        for (k, v) in logger_values.iter() {
            try!(v.serialize(record, k, &mut serializer));
        }

        let mut io = serializer.finish();

        try!(write!(io, "\n"));

        Ok(())
    }


    fn format_rfc3164(&self,
                      io: &mut io::Write,
                      record: &Record,
                      logger_values: &OwnedKeyValueList)
                      -> io::Result<()> {
        // "<{priority}> {timestamp} {host} {tag} {msg}";
        let format = "<{}> {} {} {} {}";

        //        pub fn format_3164<T: fmt::Display>(&self, severity:Severity, message: T) -> String {
        //        if let Some(ref hostname) = self.hostname {
        //        format!("<{}>{} {} {}[{}]: {}",
        //        self.encode_priority(severity, self.facility),
        //        time::now().strftime("%b %d %T").unwrap(),
        //        hostname, self.process, self.pid, message)
        //        } else {
        //        format!("<{}>{} {}[{}]: {}",
        //        self.encode_priority(severity, self.facility),
        //        time::now().strftime("%b %d %T").unwrap(),
        //        self.process, self.pid, message)
        //        }
        //        }

        let mut comma_needed = try!(self.print_msg_header(io, record));
        let mut serializer = Serializer::new(io);

        for &(k, v) in record.values().iter().rev() {
            if comma_needed {
                try!(serializer.print_comma());
            }
            try!(v.serialize(record, k, &mut serializer));
            comma_needed |= true;
        }

        for (k, v) in logger_values.iter() {
            if comma_needed {
                try!(serializer.print_comma());
            }
            try!(v.serialize(record, k, &mut serializer));
            comma_needed |= true;
        }

        let mut io = serializer.finish();

        try!(write!(io, "\n"));

        Ok(())

    }
}

struct Serializer<W> {
    io: W,
}

impl<W: io::Write> Serializer<W> {
    fn new(io: W) -> Self {
        Serializer { io: io }
    }

    fn print_comma(&mut self) -> io::Result<()> {
        try!(write!(self.io, ", "));
        Ok(())
    }

    fn finish(self) -> W {
        self.io
    }
}

macro_rules! s(
    ($s:expr, $k:expr, $v:expr) => {
        try!(write!($s.io, "{}", $k));
        try!(write!($s.io, "="));
        try!(write!($s.io, "{}", $v));
    };
);


impl<W: io::Write> slog::ser::Serializer for Serializer<W> {
    fn emit_none(&mut self, key: &str) -> ser::Result {
        s!(self, key, "None");
        Ok(())
    }
    fn emit_unit(&mut self, key: &str) -> ser::Result {
        s!(self, key, "()");
        Ok(())
    }

    fn emit_bool(&mut self, key: &str, val: bool) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_char(&mut self, key: &str, val: char) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_usize(&mut self, key: &str, val: usize) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_isize(&mut self, key: &str, val: isize) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_u8(&mut self, key: &str, val: u8) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_i8(&mut self, key: &str, val: i8) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_u16(&mut self, key: &str, val: u16) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_i16(&mut self, key: &str, val: i16) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_u32(&mut self, key: &str, val: u32) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_i32(&mut self, key: &str, val: i32) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_f32(&mut self, key: &str, val: f32) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_u64(&mut self, key: &str, val: u64) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_i64(&mut self, key: &str, val: i64) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_f64(&mut self, key: &str, val: f64) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_str(&mut self, key: &str, val: &str) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_arguments(&mut self, key: &str, val: &fmt::Arguments) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
}

impl StreamFormat for Format {
    fn format(&self,
              io: &mut io::Write,
              record: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        match self.mode {
            FormatMode::RFC3164 => self.format_rfc3164(io, record, logger_values),
            FormatMode::RFC5424 => self.format_rfc5424(io, record, logger_values),
        }
    }
}

const TIMESTAMP_FORMAT: &'static str = "%b %d %H:%M:%S%.3f";

/// Default local timestamp function used by `Format`
///
/// The exact format used, is still subject to change.
pub fn timestamp_local(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::Local::now().format(TIMESTAMP_FORMAT))
}

/// Default UTC timestamp function used by `Format`
///
/// The exact format used, is still subject to change.
pub fn timestamp_utc(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::UTC::now().format(TIMESTAMP_FORMAT))
}

/// Streamer builder
pub struct SyslogStreamer {
    async: bool,
    mode: FormatMode,
    proto: Protocol,
    //    hostname: &'static str,
    fn_timestamp: Box<TimestampFn>,
}

impl SyslogStreamer {
    /// New `StreamerBuilder`
    pub fn new() -> Self {
        SyslogStreamer {
            async: false,
            proto: Protocol::UnixSocket,
            mode: FormatMode::RFC3164,
            //            hostname: &"-",
            fn_timestamp: Box::new(timestamp_local),
        }
    }

    /// Output using RFC5424
    pub fn rfc5424(mut self) -> Self {
        self.mode = FormatMode::RFC5424;
        self
    }

    /// Output using RFC3164 (default)
    pub fn rfc3164(mut self) -> Self {
        self.mode = FormatMode::RFC3164;
        self
    }

    /// Output to UNIX socket (default)
    pub fn unix_socket(mut self) -> Self {
        self.proto = Protocol::UnixSocket;
        self
    }

    /// Output over TCP
    pub fn tcp(mut self) -> Self {
        self.proto = Protocol::TCP;
        self
    }

    /// Output over UDP
    pub fn udp(mut self) -> Self {
        self.proto = Protocol::UDP;
        self
    }

    /// Use asynchronous streamer
    pub fn async(mut self) -> Self {
        self.async = true;
        self
    }

    /// Use synchronous streamer (default)
    pub fn sync(mut self) -> Self {
        self.async = false;
        self
    }

    /// Use the UTC time zone for the timestamp
    pub fn use_utc_timestamp(mut self) -> Self {
        self.fn_timestamp = Box::new(timestamp_utc);
        self
    }

    /// Use the local time zone for the timestamp (default)
    pub fn use_local_timestamp(mut self) -> Self {
        self.fn_timestamp = Box::new(timestamp_local);
        self
    }

    /// Provide a custom function to generate the timestamp
    pub fn use_custom_timestamp<F>(mut self, f: F) -> Self
        where F: Fn(&mut io::Write) -> io::Result<()> + 'static + Send + Sync
    {
        self.fn_timestamp = Box::new(f);
        self
    }

    /// Specify syslog client hostname (optional)
    //    pub fn hostname(mut self, hostname: &str) -> Self {
    //        self.hostname = hostname;
    //        self
    //    }
    /// Build the streamer
    pub fn build(self) -> Box<slog::Drain<Error = io::Error> + Send + Sync> {
        let format = Format::new(self.mode, self.fn_timestamp);

        let io = Box::new(io::stdout()) as Box<io::Write + Send>;

        if self.async {
            Box::new(async_stream(io, format))
        } else {
            Box::new(stream(io, format))
        }
    }
}

impl Default for SyslogStreamer {
    fn default() -> Self {
        Self::new()
    }
}

/// Build `slog_stream::Streamer`/`slog_stream::AsyncStreamer` that
/// will output logging records to syslog
pub fn syslog_streamer() -> SyslogStreamer {
    SyslogStreamer::new()
}
