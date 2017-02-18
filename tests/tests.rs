extern crate slog_syslog;

// #[macro_use]
extern crate slog;

#[cfg(test)]
mod tests {

    use slog_syslog::*;
    use std::path::PathBuf;
    // use slog::Level;

    #[test]
    fn uds_config_default() {
        let config = syslog().uds();
        assert!(config.socket.is_none());
        assert!(!config.async);
        assert!(config.timestamp == TimestampFormat::RFC3164);
        assert!(config.timezone == TimestampTZ::Local);
        assert!(config.serialization == SerializationFormat::Native);
        assert!(config.facility == Facility::LOG_USER);
    }

    #[test]
    fn uds_config_with_path() {
        let config = syslog().uds().socket("/dev/log").mode(FormatMode::RFC5424);
        assert!(config.socket == Some(PathBuf::from("/dev/log")));
        assert!(config.mode == FormatMode::RFC5424);
    }

    #[test]
    fn udp_config_default() {
        let config = syslog().udp();
        assert!(config.server == None);
        assert!(!config.async);
        assert!(config.timestamp == TimestampFormat::RFC3164);
        assert!(config.timezone == TimestampTZ::Local);
        assert!(config.serialization == SerializationFormat::Native);
        assert!(config.facility == Facility::LOG_USER);
    }

    #[test]
    fn tcp_config_default() {
        let config = syslog().tcp();
        assert!(config.server == None);
        assert!(!config.async);
        assert!(config.timestamp == TimestampFormat::RFC3164);
        assert!(config.timezone == TimestampTZ::Local);
        assert!(config.serialization == SerializationFormat::Native);
        assert!(config.facility == Facility::LOG_USER);
    }

    #[test]
    fn test_get_pid() {
        assert!(get_pid() > 1);
    }

    #[test]
    fn test_get_process_name() {
        assert!(get_process_name().is_some());
    }

    #[test]
    #[ignore]
    fn connect_to_default() {
        let config = syslog().connect();
        assert!(config.is_ok())
    }

    #[test]
    fn construct_priority() {
        Priority::new(Facility::LOG_USER, Severity::LOG_WARN);
    }

    //    #[test]
    //    #[ignore]
    //    fn get_local_socket() {
    //        println!("{:?}",
    //                 UnixDomainSocketStreamer::locate_default_uds_socket());
    //        assert!(false);
    //    }
}
