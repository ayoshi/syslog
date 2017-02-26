use std::net::{ToSocketAddrs, SocketAddr};
use std::path::PathBuf;
use syslog::Facility;

#[derive(Debug, PartialEq, Clone)]
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
    fn default() -> FormatMode {
        FormatMode::RFC3164
    }
}

#[derive(Debug, PartialEq, Clone)]
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
    /// RFC5424 format supports serialization of structured data natively
    /// (rsyslog, syslog-ng and others).
    /// When specified for RFC3164 will fall back to key=value
    ///
    /// This is the default setting - will fall back to key=value for RFC3164 and
    /// native format for RFC5424
    Native,
}

impl Default for SerializationFormat {
    fn default() -> SerializationFormat {
        SerializationFormat::Native
    }
}

#[derive(Debug, PartialEq, Clone)]
/// Timestamp timezone
///
/// By default, syslog expects timestamp in the local timezone (recommended by RFC3164),
/// Since RFC3164 timestamps don't contain timezone information
/// Newer syslog servers support RFC 3339/ISO 8601 formats, which allow client to
/// specify the timezone
pub enum TimestampTZ {
    /// Default: Use timestamp in the local TZ.
    Local,
    /// Use UTC timestamp.
    UTC,
}

impl Default for TimestampTZ {
    fn default() -> TimestampTZ {
        TimestampTZ::Local
    }
}


/// Timestamp format
///
/// By default, syslog expects timestamp in a RFC3164 format.
/// Newer syslog servers support RFC 3339/ISO 8601 formats,
/// which allow client to specify the timezone and use high precision timestamps
#[derive(Debug, PartialEq, Clone)]
pub enum TimestampFormat {
    /// RFC3164
    RFC3164,
    /// ISO8601
    ISO8601,
}

impl Default for TimestampFormat {
    fn default() -> TimestampFormat {
        TimestampFormat::RFC3164
    }
}

/// Empty configuration
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DefaultConfig {}


/// Unix domain socket specific configuration
#[derive(Debug, Clone, PartialEq)]
pub struct UDSConfig {
    /// Path to syslog socket.
    ///
    /// Default: `None`, will try to connect to
    /// `/dev/log` on Linux and `/var/run/syslog` on MacOS.
    pub socket: Option<PathBuf>,
}

impl Default for UDSConfig {
    fn default() -> UDSConfig {
        UDSConfig { socket: None }
    }
}

/// UDP specific configuration
#[derive(Debug, Clone, PartialEq)]
pub struct UDPConfig<S>
    where S: ToSocketAddrs
{
    /// Syslog server host - should convert to
    /// [ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html).
    ///
    /// Default: None. will try to connect to default ports on localhost
    pub server: Option<S>,
}

impl<S: ToSocketAddrs> UDPConfig<S>
    where S: ToSocketAddrs
{
    fn new(server: S) -> Self {
        UDPConfig { server: Some(server) }
    }
}

impl Default for UDPConfig<SocketAddr> {
    fn default() -> Self {
        UDPConfig { server: None }
    }
}

/// TCP specific configuration
#[derive(Debug, Clone, PartialEq)]
pub struct TCPConfig<S>
    where S: ToSocketAddrs
{
    /// Syslog server host - should convert to
    /// [ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html).
    ///
    /// Default: None. will try to connect to default ports on localhost
    pub server: Option<S>,
}

impl<S: ToSocketAddrs> TCPConfig<S>
    where S: ToSocketAddrs
{
    fn new(server: S) -> Self {
        TCPConfig { server: Some(server) }
    }
}

impl Default for TCPConfig<SocketAddr> {
    fn default() -> Self {
        TCPConfig { server: None }
    }
}

/// Syslog drain configuration
#[derive(Debug)]
pub struct SyslogConfig<T> {
    /// Connection type specific options
    pub connection_config: T,
    /// Whether streamer should be synchronous or asynchronous.
    ///
    /// Default: `sync`.
    pub async: bool,
    /// Formatting mode [FormatMode](enum.FormatMode.html).
    ///
    /// Default: `RFC3164`.
    pub mode: FormatMode,
    /// Timestamp format: [TimestampFormat](enum.TimestampFormat.html).
    ///
    /// Default: `RFC3164`.
    pub timestamp: TimestampFormat,
    /// Timezone format: [TimestampTZ](enum.TimestampTZ.html).
    ///
    /// Default: `Local`.
    pub timezone: TimestampTZ,
    /// Serialization format [SerializationFormat](enum.SerializationFormat.html)
    ///
    /// Default: `Native`
    pub serialization: SerializationFormat,
    /// Syslog facility [Facility](enum.Facility.html).
    ///
    /// Default: `LOG_USER`.
    pub facility: Facility,
    /// Hostname
    ///
    /// Default: `None` will be omitted for unix domain socket drain,
    /// autodetected in case of UDP or TCP drains
    pub hostname: Option<String>,
}

/// General syslog config, applies to all connection types
impl<T> SyslogConfig<T> {
    /// Constructor
    pub fn new() -> SyslogConfig<DefaultConfig> {
        SyslogConfig::default()
    }

    /// Whether streamer should be synchronous or asynchronous.
    ///
    /// Default: `sync`.
    pub fn async<VALUE: Into<bool>>(mut self, value: VALUE) -> Self {
        self.async = value.into();
        self
    }
    /// Formatting mode [FormatMode](enum.FormatMode.html).
    ///
    /// Default: `RFC3164`.
    pub fn mode<VALUE: Into<FormatMode>>(mut self, value: VALUE) -> Self {
        self.mode = value.into().clone();
        self
    }

    /// Timestamp format: [TimestampFormat](enum.TimestampFormat.html).
    ///
    /// Default: `RFC3164`.
    pub fn timestamp<VALUE: Into<TimestampFormat>>(mut self, value: VALUE) -> Self {
        self.timestamp = value.into().clone();
        self
    }

    /// Timezone format: [TimestampTZ](enum.TimestampTZ.html).
    ///
    /// Default: `Local`.
    pub fn timezone<VALUE: Into<TimestampTZ>>(mut self, value: VALUE) -> Self {
        self.timezone = value.into().clone();
        self
    }

    /// Serialization format [SerializationFormat](enum.SerializationFormat.html)
    pub fn serialization<VALUE: Into<SerializationFormat>>(mut self, value: VALUE) -> Self {
        self.serialization = value.into();
        self
    }

    /// Syslog facility [Facility](enum.Facility.html).
    ///
    /// Default: `LOG_USER`.
    pub fn facility<VALUE: Into<Facility>>(mut self, value: VALUE) -> Self {
        self.facility = value.into();
        self
    }

    /// Hostname
    ///
    /// Default: `None` will be omitted for unix domain socket drain,
    /// autodetected in case of UDP or TCP drains
    pub fn hostname<VALUE: Into<String>>(mut self, value: VALUE) -> Self {
        self.hostname = Some(value.into());
        self
    }

    pub fn connection_config<C>(self, connection_config: C) -> SyslogConfig<C> {
        SyslogConfig {
            connection_config: connection_config,
            async: self.async,
            mode: self.mode,
            timestamp: self.timestamp,
            timezone: self.timezone,
            serialization: self.serialization,
            facility: self.facility,
            hostname: self.hostname,
        }
    }
}

impl Default for SyslogConfig<DefaultConfig> {
    fn default() -> SyslogConfig<DefaultConfig> {
        SyslogConfig {
            connection_config: DefaultConfig {},
            async: false,
            mode: FormatMode::default(),
            timestamp: TimestampFormat::default(),
            timezone: TimestampTZ::default(),
            serialization: SerializationFormat::default(),
            facility: Facility::default(),
            hostname: None,
        }
    }
}

impl SyslogConfig<DefaultConfig> {
    /// Set config to UDS
    pub fn uds(self) -> SyslogConfig<UDSConfig> {
        let config = self.connection_config(UDSConfig::default());
        config
    }

    /// Set config to UDP
    pub fn udp(self) -> SyslogConfig<UDPConfig<SocketAddr>> {
        let config = self.connection_config(UDPConfig::default());
        config
    }


    /// Set config to TCP
    pub fn tcp(self) -> SyslogConfig<TCPConfig<SocketAddr>> {
        let config = self.connection_config(TCPConfig::default());
        config
    }

    /// Try to connect without further configuration.
    ///
    /// It will attempt to connect unix domain socket,
    /// then try to fall back on UDP and then TCP
    /// By default will use the first working detected socket on the system,
    /// and in case of UDP and TCP standart ports (514, 6514)
    ///
    /// Defaults:
    /// RFC3164 message format,
    /// key=value serialiation and a timestamp in RFC3164 format
    /// in a local timezone
    pub fn connect(self) -> Result<bool, String> {
        Ok(true)
    }
}

impl SyslogConfig<UDSConfig> {
    /// Path to syslog socket.
    ///
    /// Will default to `/dev/log` on Linux and `/var/run/syslog` on MacOS.
    pub fn socket<VALUE: Into<PathBuf>>(mut self, value: VALUE) -> Self {
        self.connection_config.socket = Some(value.into());
        self
    }

    /// Connect unix domain socket drain
    pub fn connect(self) -> Result<bool, String> {
        Ok(true)
    }
}

impl SyslogConfig<UDPConfig<SocketAddr>> {
    /// Syslog server host - should convert to
    /// [ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html).
    ///
    /// Default: `None`, will try to connect to default ports on localhost
    pub fn server<S>(self, server: S) -> SyslogConfig<UDPConfig<S>>
        where S: ToSocketAddrs
    {
        let config = self.connection_config(UDPConfig::new(server));
        config
    }

    /// Connect UDP drain
    pub fn connect(self) -> Result<bool, String> {
        Ok(true)
    }
}

impl SyslogConfig<TCPConfig<SocketAddr>> {
    /// Syslog server host - should convert to
    /// [ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html).
    ///
    /// Default: `None`, will try to connect to default ports on localhost
    pub fn server<S>(self, server: S) -> SyslogConfig<TCPConfig<S>>
        where S: ToSocketAddrs
    {
        let config = self.connection_config(TCPConfig::new(server));
        config
    }

    /// Connect TCP drain
    pub fn connect(self) -> Result<bool, String> {
        Ok(true)
    }
}
