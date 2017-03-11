
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
    pub fn new() -> Self {
        SyslogStreamer {
            async: false,
            proto: Protocol::UnixSocket,
            mode: FormatMode::RFC3164,
            hostname: None,
            syslog_socket: None,
            syslog_host: None,
            syslog_port: None,
            facility: Facility::LOG_USER,
            fn_timestamp: Box::new(timestamp_local),
        }
    }

    /// Set own hostname
    pub fn hostname<S>(mut self, hostname: S) -> Self
        where S: Into<String>
    {
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
        where S: Into<String>
    {
        self.syslog_socket = Some(path.into());
        self
    }

    /// Syslog server host
    /// Default: deduce hostname, if it fails empty hostname
    pub fn syslog_host<S>(mut self, host: S) -> Self
        where S: Into<String>
    {
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
        // FIX: the builder can fail, we need a way to fail safely
        let process_name = get_process_name();
        let pid = get_pid();
        let hostname = self.hostname.or(get_hostname());
        let format = Format::new(self.mode,
                                 self.fn_timestamp,
                                 hostname,
                                 process_name,
                                 pid,
                                 self.facility);

        let syslog_socket = self.syslog_socket.or(get_syslog_socket());

        // Connect to socket
        let mut socket_stream = match  UnixDatagram::bind(&Path::new("/var/run/syslog")) {
            Err(_) => panic!("Couldn't connect to socket"),
            Ok(stream) => stream,
        };

        let io = Box::new(socket_stream) as Box<io::Write + Send>;

        if self.async {
            Box::new(async_stream(io, format))
        } else {
            Box::new(stream(io, format))
        }
    }
}

impl fmt::Debug for SyslogStreamer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "SyslogStreamer {{ async: {:?}, proto: {:?}, mode: {:?}, hostname: {:?}, \
                syslog_socket: {:?}, syslog_host: {:?}, syslog_port: {:?}, facility: {:?} }}",
               self.async,
               self.proto,
               self.mode,
               self.hostname,
               self.syslog_socket,
               self.syslog_host,
               self.syslog_port,
               self.facility)
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