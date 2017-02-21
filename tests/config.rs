extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;
extern crate slog_stream;

#[cfg(test)]
mod tests {

    use std::net::{ToSocketAddrs, SocketAddr, IpAddr, Ipv4Addr};
    use std::path::PathBuf;
    use slog_syslog_ng::*;
    use slog::{Logger, Discard};
    use slog_stream::stream;
    // use slog::Level;

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
