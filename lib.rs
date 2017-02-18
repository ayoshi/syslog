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
use slog::{Level};
use std::fmt;

#[macro_use]
extern crate derive_builder;

extern crate serde_json;

include!("_syslog.rs");
include!("_drains.rs");
include!("_serializers.rs");


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
///  Structured data serialization format
///
/// All of the newer syslog servers (syslog-ng, rsyslog), and log analisys tools
/// support for two formats of structured data serialization:
/// key=value, and CEE (@cee: prefix in message followed by JSON)
/// Encoding all the keys directly in the message
///
/// Those serialization formats can be supported both in RFC3164 and RFC5424 formats,
/// though RFC5424 supports native way for structured data serialization
pub enum SerializationFormat {
    /// key=value This format is the default for RFC3164.
    ///
    KV,
    /// CEE serialization format
    ///
    CEE,
    /// Supported only in RFC5424 Newer format (supported by rsyslog, syslog-ng and others).
    /// When specified for RFC3164 will fall back to key=value
    ///
    /// This is the default setting - will fall back to key=value for RFC3164 and
    /// native format for RFC5424
    Default,
}


#[derive(PartialEq, Clone)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
/// Timestamp timezone
///
/// By default, syslog expects timestamp in the local timezone (recommended by RFC3164),
/// Since RFC3164 timestamps don't contain timezone information
/// Newer syslog servers support RFC 3339/ISO 8601 formats, which allow client to specify the timezone
pub enum TimestampTZ {
    /// Default: Use timestamp in the local TZ.
    Local,
    /// Use UTC timestamp.
    UTC,
}


/// Timestamp format
///
/// By default, syslog expects timestamp in a RFC3164 format.
/// Newer syslog servers support RFC 3339/ISO 8601 formats,
/// which allow client to specify the timezone and use high precision timestamps
#[derive(PartialEq, Clone)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
pub enum TimestampFormat {
    RFC3164,
    ISO8601
}


#[derive(PartialEq, Clone, Builder)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
/// Builder to configure Unix domain socket connection to syslog server.
pub struct UDSStreamerConfig {
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
    /// Timestamp format: [TimestampFormat](enum.TimestampFormat.html).
    ///
    /// Default: `RFC3164`.
    timestamp: TimestampFormat,
    /// Timezone format: [TimestampTZ](enum.TimestampTZ.html).
    ///
    /// Default: `Local`.
    timezone: TimestampTZ,
    /// Syslog facility [Facility](enum.Facility.html).
    ///
    /// Default: `LOG_USER`.
    facility: Facility,
}


impl UDSStreamerConfigBuilder {
    pub fn connect(self) -> Result<bool, String> {
        Ok(true)
    }
}


#[derive(PartialEq, Clone, Builder)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
/// Builder to configure UDP connection to syslog server.
pub struct UDPStreamerConfig {
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
    /// Timestamp format: [TimestampFormat](enum.TimestampFormat.html).
    ///
    /// Default: `RFC3164`.
    timestamp: TimestampFormat,
    /// Timezone format: [TimestampTZ](enum.TimestampTZ.html).
    ///
    /// Default: `Local`.
    timezone: TimestampTZ,
    /// Syslog facility [Facility](enum.Facility.html).
    ///
    /// Default: `LOG_USER`.
    ///
    facility: Facility,
}

impl UDPStreamerConfigBuilder {
    pub fn connect(self) -> Result<bool, String> {
        Ok(true)
    }
}


#[derive(PartialEq, Clone, Builder)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
/// Builder to configure TCP connection to syslog server.
pub struct TCPStreamerConfig {
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
    /// Timestamp format [TimestampFormat](enum.TimestampFormat.html).
    ///
    /// Default: `RFC3164`.
    timestamp: TimestampFormat,
    /// Timezone format: [TimeoneFormat](enum.TimeoneFormat.html).
    ///
    /// Default: `Local`.
    timezone: TimestampTZ,
    /// Syslog facility [Facility](enum.Facility.html).
    ///
    /// Default: `LOG_USER`.
    facility: Facility,
}

impl TCPStreamerConfigBuilder {
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
    pub fn uds(self) -> UDSStreamerConfigBuilder {
        UDSStreamerConfigBuilder::default()
            .socket("/dev/log")
            .async(false)
            .mode(FormatMode::RFC3164)
            .facility(Facility::LOG_USER)
            .timestamp(TimestampFormat::RFC3164)
            .timezone(TimestampTZ::Local)
            .to_owned()
    }

    /// Return UDP socket builder.
    pub fn udp(self) -> UDPStreamerConfigBuilder {
        UDPStreamerConfigBuilder::default()
            .server("localhost:514")
            .async(false)
            .mode(FormatMode::RFC3164)
            .facility(Facility::LOG_USER)
            .timestamp(TimestampFormat::RFC3164)
            .timezone(TimestampTZ::Local)
            .to_owned()
    }

    /// Return TCP socket builder.
    pub fn tcp(self) -> TCPStreamerConfigBuilder {
        TCPStreamerConfigBuilder::default()
            .server("localhost:6514")
            .async(false)
            .mode(FormatMode::RFC3164)
            .facility(Facility::LOG_USER)
            .timestamp(TimestampFormat::RFC3164)
            .timezone(TimestampTZ::Local)
            .to_owned()
    }

    /// Connect unix domain socket drain without further configuration.
    /// By default will use the first working detected socket on the system,
    /// RFC3164 message format, and a local timestamp
    pub fn connect(self) -> Result<bool, String> {
        Ok(true)
    }

}

/// Entry point to any further syslog configuration
pub fn syslog() -> SyslogBuilder {
    SyslogBuilder::new()
}
