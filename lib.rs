/////! Syslog RFC3164 and RFC5424 formatter and drain for slog
/////!
/////! ```
/////! #[macro_use]
/////! extern crate slog;
/////! extern crate slog_term;
/////!
/////! use slog::*;
/////!
/////! fn main() {
/////!     let root = Logger::root(slog_term::streamer().build().fuse(), o!("build-id" => "8dfljdf"));
/////! }
/////! ```
///// ![warn(missing_docs)]
/////
extern crate slog;

use std::str::FromStr;
use slog::Level;
use std::fmt;
//
// extern crate slog_stream;
// extern crate chrono;
// extern crate libc;
// extern crate hostname;
// extern crate thread_local;
// extern crate unix_socket;
//
// use libc::getpid;
// /use std::io::Write;
// use std::{io, fmt, cell, env, ffi};
// use std::path::Path;
// use unix_socket::UnixDatagram;
//
// use slog::Record;
// use slog::ser;
// use slog::{Level, OwnedKeyValueList};
//
// use hostname::get_hostname;
//
// use slog_stream::Format as StreamFormat;
// use slog_stream::{stream, async_stream};

#[macro_use]
extern crate derive_builder;

include!("_syslog.rs");
// /include!("_format.rs");
//
// // By default the following locations are checked for sockets, in order
// /pub const SYSLOG_SOCKET_LOCATIONS: &'static [&'static str] = &["/dev/log", "/var/run/syslog"];
//
// thread_local! {
//    static TL_BUF: cell::RefCell<Vec<u8>> = cell::RefCell::new(Vec::with_capacity(128));
//
//
// //// Get process name and pid
// /fn get_process_name() -> Option<String> {
// /    env::current_exe()
// /        .ok()
// /        .as_ref()
// /        .map(Path::new)
// /        .and_then(Path::file_name)
// /        .and_then(ffi::OsStr::to_str)
// /        .map(String::from)
// /}
// /
// /fn get_pid() -> i32 {
// /    unsafe { getpid() }
// /}
// /
// /fn get_syslog_socket<'a>() -> Option<String> {
// /    vec!["/dev/log", "/var/run/syslog"]
// /        .iter()
// /        .find(|s| Path::new(s).exists())
// /        .map(|s| s.to_string())
// /}
// /
// //// Timestamp function type
// pub type TimestampFn = Fn(&mut io::Write) -> io::Result<()> + Send + Sync;
// /
//
// // Formatting mode

#[derive(PartialEq, Clone)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
/// Syslog message format
pub enum FormatMode {
    /// RFC3164 (Older, BSD syslog format)
    /// Supported by all syslog daemons on all operating systems and platforms
    RFC3164,
    /// RFC5424 Newer format (supported by rsyslog, syslog-ng and others)
    /// Allows for logging of structural data
    RFC5424,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
pub enum TimestampMode {
    /// Use timestamp in the local TZ
    Local,
    /// Use UTC timestamp
    UTC,
}
// //// Protocol
// /#[derive(Debug)]
// /pub enum Protocol {
// /    /// Log to Unix socket
// /    UnixSocket,
// /    /// Log over TCP
// /    TCP,
// /    /// Log over UDP
// /    UDP,
// /}
//
//
// // Streamer builder
// pub struct SyslogStreamer {
//    async: bool,
//    mode: FormatMode,
//    syslog_socket: Option<String>,
//    facility: Facility,
//    fn_timestamp: Box<TimestampFn>,
//
//
// impl SyslogStreamer {
//    /// New `StreamerBuilder`
//    pub fn new() -> Self {
//        SyslogStreamer {
//            async: false,
//            mode: FormatMode::RFC3164,
//            facility: Facility::LOG_USER,
//            fn_timestamp: Box::new(timestamp_local),
//        }
//    }
//
//    /// Output using RFC5424
//    pub fn rfc5424(mut self) -> Self {
//        self.mode = FormatMode::RFC5424;
//        self
//    }
//
//    /// Output using RFC3164 (default)
//    pub fn rfc3164(mut self) -> Self {
//        self.mode = FormatMode::RFC3164;
//        self
//    }
//
//    /// Syslog facility
//    /// Default: LOG_USER
//    pub fn facility(mut self, facility: Facility) -> Self {
//        self.facility = facility;
//        self
//    }
//
//    /// Use asynchronous streamer
//    pub fn async(mut self) -> Self {
//        self.async = true;
//        self
//    }
//
//    /// Use synchronous streamer (default)
//    pub fn sync(mut self) -> Self {
//        self.async = false;
//        self
//    }
//
//    /// Use the UTC time zone for the timestamp
//    pub fn use_utc_timestamp(mut self) -> Self {
//        self.fn_timestamp = Box::new(self.timestamp_utc);
//        self
//    }
//
//    /// Use the local time zone for the timestamp (default)
//    pub fn use_local_timestamp(mut self) -> Self {
//        self.fn_timestamp = Box::new(self.timestamp_local);
//        self
//    }
//
//    /// Provide a custom function to generate the timestamp
//    pub fn use_custom_timestamp<F>(mut self, f: F) -> Self
//        where F: Fn(&mut io::Write) -> io::Result<()> + 'static + Send + Sync
//    {
//        self.fn_timestamp = Box::new(f);
//        self
//    }
//
//    ///// Default local timestamp function used by `Format`
//    pub fn timestamp_local(io: &mut io::Write) -> io::Result<()> {
//        write!(io, "{}", chrono::Local::now().to_rfc3339())
//    }
//
//    /// Default UTC timestamp function used by `Format`
//    pub fn timestamp_utc(io: &mut io::Write) -> io::Result<()> {
//        write!(io, "{}", chrono::UTC::now().to_rfc3339())
//    }
//
//
//    /// Build the streamer
//    pub fn build(self) -> Box<slog::Drain<Error = io::Error> + Send + Sync> {
//        // FIX: the builder can fail, we need a way to fail safely
//    }
//
//
//    impl fmt::Debug for SyslogStreamer {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f,
//               "SyslogStreamer {{ async: {:?}, mode: {:?}, facility: {:?} }}",
//               self.async,
//               self.mode,
//               self.facility)
//    }
//
//
// impl Default for SyslogStreamer {
//    fn default() -> Self {
//        Self::new()
//    }
//
//
// // Build `slog_stream::Streamer`/`slog_stream::AsyncStreamer` that
// // will output logging records to syslog
// pub fn syslog_streamer() -> SyslogStreamer {
//    SyslogStreamer::new()
//


#[derive(PartialEq, Clone, Builder)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
pub struct DomainSocketStreamer {
    /// Whether streamer should be synchronous or asynchronous
    async: bool,
    /// Formatting mode [FormatMode](enum.FormatMode.html)
    mode: FormatMode,
    /// Timestamp mode [TimestampMode](enum.TimestampMode.html)
    timestamp: TimestampMode,
    /// Optional: path to syslog socket `Option<String>`
    /// Will default to `/dev/log` on Linux and `/var/run/syslog` on MacOS
    syslog_socket: Option<String>,
    facility: Facility,
}

pub fn domain_socket_drain() -> DomainSocketStreamerBuilder {
    DomainSocketStreamerBuilder::default()
        .async(false)
        .mode(FormatMode::RFC3164)
        .facility(Facility::LOG_USER)
        .syslog_socket(None)
        .timestamp(TimestampMode::UTC)
        .to_owned()
}

