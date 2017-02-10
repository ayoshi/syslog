//! Syslog RFC3164 and RFC5424 formatter and drain for slog
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
extern crate libc;
extern crate hostname;
extern crate thread_local;

use libc::getpid;
use std::str::FromStr;
use std::io::Write;
use std::{io, fmt, sync, cell};
use std::{env,os,ffi};
use std::path::Path;

use slog::Record;
use slog::ser;
use slog::{Level, OwnedKeyValueList};

use hostname::get_hostname;

use slog_stream::Format as StreamFormat;
use slog_stream::{stream, async_stream};

include!("_syslog.rs");
include!("_format.rs");

/// By default the following locations are checked for sockets, in order
pub const SYSLOG_SOCKET_LOCATIONS: &'static [&'static str] = &["/dev/log", "/var/run/syslog"];

thread_local! {
    static TL_BUF: cell::RefCell<Vec<u8>> = cell::RefCell::new(Vec::with_capacity(128));
}

/// Get process name and pid
fn get_process_name() -> Option<String> {
    env::current_exe().ok().as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(ffi::OsStr::to_str)
        .map(String::from)
}

fn get_pid() -> i32 {
    unsafe { getpid() }
}

fn get_syslog_socket<'a>() -> Option<String> {
    vec!["/dev/log", "/var/run/syslog"]
        .iter()
        .find(|s| Path::new(s).exists())
        .map(|s| s.to_string())
}

/// Timestamp function type
pub type TimestampFn = Fn(&mut io::Write) -> io::Result<()> + Send + Sync;

/// Default local timestamp function used by `Format`
pub fn timestamp_local(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::Local::now().to_rfc3339())
}

/// Default UTC timestamp function used by `Format`
pub fn timestamp_utc(io: &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::UTC::now().to_rfc3339())
}

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


/// Streamer builder
pub struct SyslogStreamer {
    async: bool,
    mode: FormatMode,
    proto: Protocol,
    hostname: Option<String>,
    syslog_socket: Option<String>,
    syslog_host: Option<String>,
    syslog_port: Option<u8>,
    facility: Facility,
    fn_timestamp: Box<TimestampFn>,
}

impl SyslogStreamer {
    /// New `StreamerBuilder`
    pub fn new()-> Self {
        SyslogStreamer {
            async: false,
            proto: Protocol::UnixSocket,
            mode: FormatMode::RFC3164,
            hostname: get_hostname(),
            syslog_socket: None,
            syslog_host: None,
            syslog_port: None,
            facility: Facility::LOG_USER,
            fn_timestamp: Box::new(timestamp_local),
        }
    }

    /// Set own hostname
    pub fn hostname<S>(mut self, hostname: S) -> Self
        where S: Into<String> {
        self.hostname = Some(hostname.into());
        self
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

    /// UNIX domain socket address
    /// Default: will try those in order: '/dev/log', '/var/run/syslog'
    pub fn syslog_socket<S>(mut self, path: S) -> Self
        where S: Into<String> {
        self.syslog_socket = Some(path.into());
        self
    }

    /// Syslog server host
    /// Default: localhost
    pub fn syslog_host<S>(mut self, host: S) -> Self
        where S: Into<String> {
        self.syslog_host = Some(host.into());
        self
    }

    /// Syslog server port
    /// Default: 514 for UDP, 6514 for TCP
    pub fn syslog_port(mut self, port: u8) -> Self {
        self.syslog_port = Some(port);
        self
    }

    /// Syslog facility
    /// Default: LOG_USER
    pub fn facility(mut self, facility: Facility) -> Self {
        self.facility = facility;
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

    /// Build the streamer
    pub fn build(self) -> Box<slog::Drain<Error = io::Error> + Send + Sync> {
        let process_name = get_process_name().unwrap_or("".into());
        let pid = get_pid();
        let hostname = self.hostname.unwrap_or("".into());
        let format = Format::new(
            self.mode,
            self.fn_timestamp,
            hostname,
            process_name,
            pid,
            self.facility);

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
