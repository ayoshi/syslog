/// Default syslog unix domain socket locations on different
/// operating systems
pub const SYSLOG_DEFAULT_UDS_LOCATIONS: &'static [&'static str] =
    &["/dev/log", "/var/run/syslog", "/var/run/log"];

/// Syslog Severity
#[allow(non_camel_case_types)]
#[derive(PartialEq, Copy, Clone)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
pub enum Severity {
    LOG_EMERG = 0,
    LOG_ALERT,
    LOG_CRIT,
    LOG_ERR,
    LOG_WARN,
    LOG_NOTICE,
    LOG_INFO,
    LOG_DEBUG,
}

impl From<slog::Level> for Severity {
    fn from(level: slog::Level) -> Severity {
        match level {
            Level::Critical => Severity::LOG_CRIT,
            Level::Error => Severity::LOG_ERR,
            Level::Warning => Severity::LOG_WARN,
            Level::Info => Severity::LOG_INFO,
            Level::Debug => Severity::LOG_DEBUG,
            Level::Trace => Severity::LOG_DEBUG,
        }
    }
}

impl FromStr for Severity {
    type Err = ();
    fn from_str(s: &str) -> Result<Severity, ()> {
        let result = match &s.to_lowercase()[..] {
            "log_emerg" | "emerg" | "panic" => Severity::LOG_EMERG,
            "log_alert" | "alert" => Severity::LOG_ALERT,
            "log_crit" | "crit" | "critical" => Severity::LOG_CRIT,
            "log_err" | "err" | "error" => Severity::LOG_ERR,
            "log_warn" | "warn" | "warning" => Severity::LOG_WARN,
            "log_notice" | "notice" => Severity::LOG_NOTICE,
            "log_info" | "info" => Severity::LOG_INFO,
            "log_debug" | "debug" => Severity::LOG_DEBUG,
            _ => return Err(()),
        };
        Ok(result)
    }
}

/// Syslog Facility
#[allow(non_camel_case_types)]
#[derive(PartialEq, Copy, Clone)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
pub enum Facility {
    LOG_KERN = 0,
    LOG_USER = 1,
    LOG_MAIL = 2,
    LOG_DAEMON = 3,
    LOG_AUTH = 4,
    LOG_SYSLOG = 5,
    LOG_LPR = 6,
    LOG_NEWS = 7,
    LOG_UUCP = 8,
    LOG_CRON = 9,
    LOG_AUTHPRIV = 10,
    LOG_FTP = 11,
    LOG_LOCAL0 = 16,
    LOG_LOCAL1 = 17,
    LOG_LOCAL2 = 18,
    LOG_LOCAL3 = 19,
    LOG_LOCAL4 = 20,
    LOG_LOCAL5 = 21,
    LOG_LOCAL6 = 22,
    LOG_LOCAL7 = 23,
}

impl Facility {
    /// Return &str variants, for using in commandline and configuration parsing
    pub fn variants() -> [&'static str; 19] {
        ["kern", "user", "mail", "daemon", "auth", "syslog", "lpr", "news", "uucp", "cron", "ftp",
         "local0", "local1", "local2", "local3", "local4", "local5", "local6", "local7"]
    }
}

impl FromStr for Facility {
    type Err = ();
    fn from_str(s: &str) -> Result<Facility, ()> {
        let result = match &s.to_lowercase()[..] {
            "log_kern" | "kern" => Facility::LOG_KERN,
            "log_user" | "user" => Facility::LOG_USER,
            "log_mail" | "mail" => Facility::LOG_MAIL,
            "log_daemon" | "daemon" => Facility::LOG_DAEMON,
            "log_auth" | "auth" => Facility::LOG_AUTH,
            "log_syslog" | "syslog" => Facility::LOG_SYSLOG,
            "log_lpr" | "lpr" => Facility::LOG_LPR,
            "log_news" | "news" => Facility::LOG_NEWS,
            "log_uucp" | "uucp" => Facility::LOG_UUCP,
            "log_cron" | "cron" => Facility::LOG_CRON,
            "log_authpriv" | "authpriv" => Facility::LOG_AUTHPRIV,
            "log_ftp" | "ftp" => Facility::LOG_FTP,
            "log_local0" | "local0" => Facility::LOG_LOCAL0,
            "log_local1" | "local1" => Facility::LOG_LOCAL1,
            "log_local2" | "local2" => Facility::LOG_LOCAL2,
            "log_local3" | "local3" => Facility::LOG_LOCAL3,
            "log_local4" | "local4" => Facility::LOG_LOCAL4,
            "log_local5" | "local5" => Facility::LOG_LOCAL5,
            "log_local6" | "local6" => Facility::LOG_LOCAL6,
            "log_local7" | "local7" => Facility::LOG_LOCAL7,
            _ => return Err(()),
        };
        Ok(result)
    }
}

#[derive(PartialEq, Copy, Clone)]
#[cfg_attr(not(feature = "release"), derive(Debug))]
pub struct Priority(u8);

impl Priority {
    pub fn new(facility: Facility, severity: Severity) -> Priority {
        let facility = facility as u8;
        let severity = severity as u8;
        Priority(facility << 3 | severity)
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
