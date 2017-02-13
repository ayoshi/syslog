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
use std::path::PathBuf;
use slog::Level;
use std::fmt;

#[macro_use]
extern crate derive_builder;


include!("_syslog.rs");


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


#[derive(PartialEq, Clone, Builder)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
pub struct UDSStreamer {
    /// Path to syslog socket
    ///
    /// Will default to `/dev/log` on Linux and `/var/run/syslog` on MacOS
    socket: PathBuf,
    /// Whether streamer should be synchronous or asynchronous
    ///
    /// Default: `sync`
    async: bool,
    /// Formatting mode [FormatMode](enum.FormatMode.html)
    ///
    /// Default: `RFC3164`
    mode: FormatMode,
    /// Timestamp mode: [TimestampMode](enum.TimestampMode.html)
    ///
    /// Default: `Local`
    timestamp: TimestampMode,
    /// Syslog facility [Facility](enum.Facility.html)
    ///
    /// Default: `LOG_USER`
    facility: Facility,
}


impl UDSStreamerBuilder {
    pub fn connect(self) -> Result<bool, String> {
       Ok(true)
    }
}


#[derive(PartialEq, Clone, Builder)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
pub struct UDPStreamer {
    /// Syslog server host
    ///
    /// Should convert to [ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html)
    ///
    /// Default: `localhost:6514`
    server: String,
    /// Whether streamer should be synchronous or asynchronous
    ///
    /// Default: `sync`
    async: bool,
    /// Formatting mode [FormatMode](enum.FormatMode.html)
    ///
    /// Default: `RFC3164`
    mode: FormatMode,
    /// Timestamp mode: [TimestampMode](enum.TimestampMode.html)
    ///
    /// Default: `Local`
    timestamp: TimestampMode,
    /// Syslog facility [Facility](enum.Facility.html)
    ///
    /// Default: `LOG_USER`
    ///
    facility: Facility,
}


#[derive(PartialEq, Clone, Builder)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
pub struct TCPStreamer {
    /// Syslog server host
    ///
    /// Should convert to [ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html)
    ///
    /// Default: `localhost:6514`
    server: String,
    /// Whether streamer should be synchronous or asynchronous
    ///
    /// Default: `sync`
    async: bool,
    /// Formatting mode [FormatMode](enum.FormatMode.html)
    ///
    /// Default: `RFC3164`
    mode: FormatMode,
    /// Timestamp mode [TimestampMode](enum.TimestampMode.html)
    ///
    /// Default: `Local`
    timestamp: TimestampMode,
    /// Syslog facility [Facility](enum.Facility.html)
    ///
    /// Default: `LOG_USER`
    facility: Facility,
}


pub fn syslog_socket() -> UDSStreamerBuilder {
    UDSStreamerBuilder::default()
        .socket("/dev/log")
        .async(false)
        .mode(FormatMode::RFC3164)
        .facility(Facility::LOG_USER)
        .timestamp(TimestampMode::Local)
        .to_owned()
}


pub fn syslog_udp() -> UDPStreamerBuilder {
    UDPStreamerBuilder::default()
        .server("localhost:514")
        .async(false)
        .mode(FormatMode::RFC3164)
        .facility(Facility::LOG_USER)
        .timestamp(TimestampMode::Local)
        .to_owned()
}


pub fn syslog_tcp() -> TCPStreamerBuilder {
    TCPStreamerBuilder::default()
        .server("localhost:6514")
        .async(false)
        .mode(FormatMode::RFC3164)
        .facility(Facility::LOG_USER)
        .timestamp(TimestampMode::Local)
        .to_owned()
}