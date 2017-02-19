/// Empty configuration
#[derive(Debug, Clone, PartialEq)]
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

/// UDP specific configuration
#[derive(Debug, Clone, PartialEq)]
pub struct UDPConfig {
    /// Syslog server host - should convert to
    /// [ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html).
    ///
    /// Default: None. will try to connect to
    /// `localhost:514`
    pub server: Option<String>,
}

/// TCP specific configuration
#[derive(Debug, Clone, PartialEq)]
pub struct TCPConfig {
    /// Syslog server host - should convert to
    /// [ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html).
    ///
    /// Default: None, will try to connect to
    /// `localhost:6514`
    pub server: Option<String>,
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
    // PhantomData marker, to specialize builder depending on connection type
}

/// General syslog config, applies to all connection types
impl<T> SyslogConfig<T> {

    // Default constructor
    pub fn new() -> SyslogConfig<DefaultConfig> {
        SyslogConfig::default()
    }

    /// Whether streamer should be synchronous or asynchronous.
    ///
    /// Default: `sync`.
    pub fn async<VALUE: Into<bool>>(mut self, value: VALUE) -> Self {
        self.async = value.into().clone();
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
        self.facility = value.into().clone();
        self
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
        }
    }
}

impl SyslogConfig<DefaultConfig> {

    /// Set config to UDS
    pub fn uds(self) -> SyslogConfig<UDSConfig> {
        SyslogConfig {
            connection_config: UDSConfig { socket: None },
            async: self.async,
            mode: self.mode,
            timestamp: self.timestamp,
            timezone: self.timezone,
            serialization: self.serialization,
            facility: self.facility,
        }
    }

    /// Set config to UDP
    pub fn udp(self) -> SyslogConfig<UDPConfig> {
        SyslogConfig {
            connection_config: UDPConfig { server: None },
            async: self.async,
            mode: self.mode,
            timestamp: self.timestamp,
            timezone: self.timezone,
            serialization: self.serialization,
            facility: self.facility,
        }
    }

    /// Set config to TCP
    pub fn tcp(self) -> SyslogConfig<TCPConfig> {
        SyslogConfig {
            connection_config: TCPConfig { server: None },
            async: self.async,
            mode: self.mode,
            timestamp: self.timestamp,
            timezone: self.timezone,
            serialization: self.serialization,
            facility: self.facility,
        }
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

impl SyslogConfig<UDPConfig> {
    /// Syslog server host - should convert to
    /// [ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html).
    ///
    /// Default: `None`, will try to connect to
    /// `localhost:514`
    pub fn server<VALUE: Into<String>>(mut self, value: VALUE) -> Self {
        self.connection_config.server = Some(value.into());
        self
    }

    /// Connect UDP drain
    pub fn connect(self) -> Result<bool, String> {
        Ok(true)
    }
}

impl SyslogConfig<TCPConfig> {
    /// Syslog server host - should convert to
    /// [ToSocketAddrs](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html).
    ///
    /// Default: None
    /// will try to connect to `localhost:6514`
    pub fn server<VALUE: Into<String>>(mut self, value: VALUE) -> Self {
        self.connection_config.server = Some(value.into());
        self
    }

    /// Connect TCP drain
    pub fn connect(self) -> Result<bool, String> {
        Ok(true)
    }
}
