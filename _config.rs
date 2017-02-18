#[derive(Debug, PartialEq, Clone, Default)]
pub struct UDSStreamerConfig {
    /// Path to syslog socket.
    ///
    /// Will default to `/dev/log` on Linux and `/var/run/syslog` on MacOS.
    pub socket: Option<PathBuf>,
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
}

impl UDSStreamerConfig {
    pub fn new(
        socket: Option<PathBuf>,
        async: bool,
        mode: FormatMode,
        timestamp: TimestampFormat,
        timezone: TimestampTZ,
        serialization: SerializationFormat,
        facility: Facility,
    ) -> UDSStreamerConfig {
        UDSStreamerConfig {
            socket: socket,
            async: async,
            mode: mode,
            timestamp: timestamp,
            timezone: timezone,
            serialization: serialization,
            facility: facility
        }
    }
    /// Path to syslog socket.
    ///
    /// Will default to `/dev/log` on Linux and `/var/run/syslog` on MacOS.
    pub fn socket<VALUE: Into<PathBuf>>(mut self, value: VALUE) ->  Self {
        self.socket = Some(value.into());
        self
    }
    /// Whether streamer should be synchronous or asynchronous.
    ///
    /// Default: `sync`.
    pub fn async<VALUE: Into<bool>>(mut self, value: VALUE) ->  Self {
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
