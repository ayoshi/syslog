/////! Syslog RFC3164 and RFC5424 formatter and drain for slog
/////!
/////! ```
/////! #[macro_use]
/////! extern crate slog;
/////! extern crate slog_syslog;
/////!
/////! use slog::*;
/////!
/////! fn main() {
/////!     let root = Logger::root(slog_term::streamer().build().fuse(),
///// o!("build-id" => "8dfljdf"));
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
///
/// Most of the newer syslog servers (syslog-ng, rsyslog)
/// support RFC5424, which allows to handle structured data properly.
///
/// All of the syslog server support RFC3164 (BSD format).
/// This format is the default for all the drains.
/// Structured data will be serialized as a part of the message.
pub enum FormatMode {
    /// RFC3164 (Older, BSD syslog format).
    ///
    /// Supported by all syslog daemons on all operating systems and platforms.
    RFC3164,
    /// RFC5424 Newer format (supported by rsyslog, syslog-ng and others).
    ///
    /// Allows for logging of structural data.
    RFC5424,
}


#[derive(PartialEq, Clone)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
/// Timestamp format
pub enum TimestampMode {
    /// Use timestamp in the local TZ.
    Local,
    /// Use UTC timestamp.
    UTC,
}


#[derive(PartialEq, Clone, Builder)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
/// Builder to configure Unix domain socket connection to syslog server.
pub struct UDSStreamer {
    /// Path to syslog socket.
    ///
    /// Will default to `/dev/log` on Linux and `/var/run/syslog` on MacOS.
    socket: PathBuf,
    /// Whether streamer should be synchronous or asynchronous.
    ///
    /// Default: `sync`.
    async: bool,
    /// Formatting mode [FormatMode](enum.FormatMode.html).
    ///
    /// Default: `RFC3164`.
    mode: FormatMode,
    /// Timestamp mode: [TimestampMode](enum.TimestampMode.html).
    ///
    /// Default: `Local`.
    timestamp: TimestampMode,
    /// Syslog facility [Facility](enum.Facility.html).
    ///
    /// Default: `LOG_USER`.
    facility: Facility,
}


impl UDSStreamerBuilder {
    pub fn connect(self) -> Result<bool, String> {
        Ok(true)
    }
}


#[derive(PartialEq, Clone, Builder)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
/// Builder to configure UDP connection to syslog server.
pub struct UDPStreamer {
    /// Syslog server host - should convert to
    /// [ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html).
    ///
    /// Default: `localhost:6514`
    server: String,
    /// Whether streamer should be synchronous or asynchronous.
    ///
    /// Default: `sync`.
    async: bool,
    /// Formatting mode [FormatMode](enum.FormatMode.html).
    ///
    /// Default: `RFC3164`
    mode: FormatMode,
    /// Timestamp mode: [TimestampMode](enum.TimestampMode.html).
    ///
    /// Default: `Local`.
    timestamp: TimestampMode,
    /// Syslog facility [Facility](enum.Facility.html).
    ///
    /// Default: `LOG_USER`.
    ///
    facility: Facility,
}

impl UDPStreamerBuilder {
    pub fn connect(self) -> Result<bool, String> {
        Ok(true)
    }
}


#[derive(PartialEq, Clone, Builder)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
/// Builder to configure TCP connection to syslog server.
pub struct TCPStreamer {
    /// Syslog server host - should convert to
    /// [ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html).
    ///
    /// Default: `localhost:6514`
    server: String,
    /// Whether streamer should be synchronous or asynchronous.
    ///
    /// Default: `sync`.
    async: bool,
    /// Formatting mode [FormatMode](enum.FormatMode.html).
    ///
    /// Default: `RFC3164`.
    mode: FormatMode,
    /// Timestamp mode [TimestampMode](enum.TimestampMode.html).
    ///
    /// Default: `Local`.
    timestamp: TimestampMode,
    /// Syslog facility [Facility](enum.Facility.html).
    ///
    /// Default: `LOG_USER`.
    facility: Facility,
}

impl TCPStreamerBuilder {
    pub fn connect(self) -> Result<bool, String> {
        Ok(true)
    }
}

/// Syslog configuration builder.
pub struct SyslogBuilder {
}

impl SyslogBuilder {

    pub fn new() -> SyslogBuilder {
        SyslogBuilder {}
    }

    /// Return Unix domain socket builder.
    pub fn uds(self) -> UDSStreamerBuilder {
        UDSStreamerBuilder::default()
            .socket("/dev/log")
            .async(false)
            .mode(FormatMode::RFC3164)
            .facility(Facility::LOG_USER)
            .timestamp(TimestampMode::Local)
            .to_owned()
    }

    /// Return UDP socket builder.
    pub fn udp(self) -> UDPStreamerBuilder {
        UDPStreamerBuilder::default()
            .server("localhost:514")
            .async(false)
            .mode(FormatMode::RFC3164)
            .facility(Facility::LOG_USER)
            .timestamp(TimestampMode::Local)
            .to_owned()
    }

    /// Return TCP socket builder.
    pub fn tcp(self) -> TCPStreamerBuilder {
        TCPStreamerBuilder::default()
            .server("localhost:6514")
            .async(false)
            .mode(FormatMode::RFC3164)
            .facility(Facility::LOG_USER)
            .timestamp(TimestampMode::Local)
            .to_owned()
    }

    /// Return unix domain socket without further configuration.
    /// By default will use the first working detected socket on the system,
    /// RFC3164 message format, and local timestamp
    pub fn connect() -> Result<bool, String> {
        Ok(true)
    }

}

/// Entry point to any further syslog configuration
pub fn syslog() -> SyslogBuilder {
    SyslogBuilder::new()
}
