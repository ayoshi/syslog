extern crate slog_syslog_ng;

extern crate slog;
extern crate slog_stream;

#[cfg(test)]
mod tests {

    use slog_syslog_ng::*;
    use std::path::PathBuf;

    #[test]
    fn uds_config_default() {
        let config = syslog().uds();
        assert!(config.connection_config.socket.is_none());
        assert!(!config.async);
        assert!(config.timestamp == TimestampFormat::RFC3164);
        assert!(config.timezone == TimestampTZ::Local);
        assert!(config.serialization == SerializationFormat::Native);
        assert!(config.facility == Facility::LOG_USER);
    }

    #[test]
    fn uds_config_with_path() {
        let config = syslog().uds().socket("/dev/log").mode(FormatMode::RFC5424);
        assert!(config.connection_config.socket == Some(PathBuf::from("/dev/log")));
        assert!(config.mode == FormatMode::RFC5424);
    }

    #[test]
    fn udp_config_default() {
        let config = syslog().udp();
        assert!(config.connection_config.server == None);
        assert!(!config.async);
        assert!(config.timestamp == TimestampFormat::RFC3164);
        assert!(config.timezone == TimestampTZ::Local);
        assert!(config.serialization == SerializationFormat::Native);
        assert!(config.facility == Facility::LOG_USER);
    }

    #[test]
    fn tcp_config_default() {
        let config = syslog().tcp();
        assert!(config.connection_config.server == None);
        assert!(!config.async);
        assert!(config.timestamp == TimestampFormat::RFC3164);
        assert!(config.timezone == TimestampTZ::Local);
        assert!(config.serialization == SerializationFormat::Native);
        assert!(config.facility == Facility::LOG_USER);
    }
}
