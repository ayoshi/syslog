type TsIsoUtc = Timestamp<TimestampISO8601, TimestampUTC>;
type TsIosLocal = Timestamp<TimestampISO8601, TimestampLocal>;
type Ts3164Utc = Timestamp<TimestampRFC3164, TimestampUTC>;
type Ts3164Local = Timestamp<TimestampRFC3164, TimestampLocal>;


#[test]
fn uds_drain_rfc3164_minimal() {
    let dest = PathBuf::from("/syslog-ng/socket_dgram");
    logger_emit!(UDSDrain,
                 HeaderRFC3164Minimal,
                 MessageKSV,
                 dest,
                 "Test message RFC3164 minimal");
}

#[test]
fn uds_drain_rfc3164_full() {
    let dest = PathBuf::from("/syslog-ng/socket_dgram");
    logger_emit!(
            UDSDrain,
            HeaderRFC3164<Ts3164Local>,
            MessageKSV,
            dest,
            "Test message RFC3164 full"
        );
}

#[test]
fn uds_drain_rfc5424() {
    let dest = PathBuf::from("/syslog-ng/socket_dgram");
    logger_emit!(
            UDSDrain,
            HeaderRFC3164<TsIosLocal>,
            MessageRFC5424,
            dest,
            "Test message RFC5424 Native"
        );
}

#[test]
fn udp_drain_rfc3164_minimal() {
    let dest = "syslog-ng:10514"
        .to_socket_addrs()
        .expect("Unable to resolve host, check that syslog-ng Docker service is up")
        .collect::<Vec<_>>()
                   [0];
    logger_emit!(
            UDPDrain,
            HeaderRFC3164Minimal,
            MessageKSV,
            dest,
            "Test message RFC3164 minimal"
        );
}
