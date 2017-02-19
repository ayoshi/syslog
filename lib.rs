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
use std::path::{PathBuf, Path};
use slog::{Level};
use std::fmt;

extern crate serde_json;

include!("_config.rs");
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

impl Default for FormatMode {
    fn default() -> FormatMode { FormatMode::RFC3164 }
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
    /// Most of the log analisys tools also support embedding JSON directly in RFC3164 messages
    /// after the `@cee:` prefix
    CEE,
    /// RFC5424 format supports serialization of structured data natively (rsyslog, syslog-ng and others).
    /// When specified for RFC3164 will fall back to key=value
    ///
    /// This is the default setting - will fall back to key=value for RFC3164 and
    /// native format for RFC5424
    Native,
}

impl Default for SerializationFormat {
    fn default() -> SerializationFormat { SerializationFormat::Native }
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

impl Default for TimestampTZ {
    fn default() -> TimestampTZ { TimestampTZ::Local}
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

impl Default for TimestampFormat {
    fn default() -> TimestampFormat { TimestampFormat::RFC3164 }
}

/// Syslog configuration builder.
pub struct SyslogBuilder {
}

impl SyslogBuilder {

    pub fn new() -> SyslogBuilder {
        SyslogBuilder {}
    }

    /// Return Unix domain socket config.
    pub fn uds(self) -> UDSStreamerConfig {
        UDSStreamerConfig::default()
    }

    /// Return UDP socket config.
    pub fn udp(self) -> UDPStreamerConfig {
        UDPStreamerConfig::default()
    }

    /// Return TCP socket config.
    pub fn tcp(self) -> TCPStreamerConfig {
        TCPStreamerConfig::default()
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
