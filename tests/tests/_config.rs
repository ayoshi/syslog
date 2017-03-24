#[test]
fn uds_config_default() {
    let config = syslog().uds();
    assert!(config.connection_config.socket.is_none());
    assert!(!config.async);
    assert_eq!(config.timestamp, TimestampFormat::RFC3164);
    assert_eq!(config.timezone, TimestampTZ::Local);
    assert_eq!(config.serialization, SerializationFormat::Native);
    assert_eq!(config.facility, Facility::LOG_USER);
}

#[test]
fn uds_config_with_path() {
    let config = syslog().uds().socket("/dev/log").mode(FormatMode::RFC5424);
    assert_eq!(config.connection_config.socket,
               Some(PathBuf::from("/dev/log")));
    assert_eq!(config.mode, FormatMode::RFC5424);
}

#[test]
fn udp_config_default() {
    let config = syslog().udp();
    assert_eq!(config.connection_config.server, None);
    assert!(!config.async);
    assert_eq!(config.timestamp, TimestampFormat::RFC3164);
    assert_eq!(config.timezone, TimestampTZ::Local);
    assert_eq!(config.serialization, SerializationFormat::Native);
    assert_eq!(config.facility, Facility::LOG_USER);
}

#[test]
fn tcp_config_default() {
    let config = syslog().tcp();
    assert_eq!(config.connection_config.server, None);
    assert!(!config.async);
    assert_eq!(config.timestamp, TimestampFormat::RFC3164);
    assert_eq!(config.timezone, TimestampTZ::Local);
    assert_eq!(config.serialization, SerializationFormat::Native);
    assert_eq!(config.facility, Facility::LOG_USER);
}
