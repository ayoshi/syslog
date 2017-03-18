#[test]
fn uds_drain_rfc3164_ksv_minimal() {
    let dest = PathBuf::from("/syslog-ng/socket_dgram_rfc3164_ksv");
    logger_emit!(UDSDrain,
                 HeaderRFC3164Minimal,
                 MessageKSV,
                 dest,
                 "UDS Test message RFC3164 MINIMAL KSV");
}

#[test]
fn uds_drain_rfc3164_ksv_full() {
    let dest = PathBuf::from("/syslog-ng/socket_dgram_rfc3164_ksv");
    logger_emit!(
            UDSDrain,
            HeaderRFC3164<Ts3164Local>,
            MessageKSV,
            dest,
            "UDS Test message RFC3164 TS3164 LOCAL KSV"
        );
}

#[test]
fn uds_drain_rfc5424_native() {
    let dest = PathBuf::from("/syslog-ng/socket_dgram_rfc5424_native");
    logger_emit!(
            UDSDrain,
            HeaderRFC3164<TsIsoLocal>,
            MessageRFC5424,
            dest,
            "UDS Test message RFC5424 ISO LOCAL NATIVE"
        );
}

#[test]
fn udp_drain_rfc3164_ksv_minimal() {
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
            "UDP Test message RFC3164 MINIMAL KSV"
        );
}

#[test]
fn tcp_drain_rfc3164_ksv_utc() {
    let dest = "syslog-ng:10601"
        .to_socket_addrs()
        .expect("Unable to resolve host, check that syslog-ng Docker service is up")
        .collect::<Vec<_>>()
        [0];
    logger_emit!(
        UDPDrain,
        HeaderRFC3164<TsIsoUtc>,
        MessageKSV,
        dest,
        "TCP Test message RFC3164 ISO UTC KSV"
    );
}

#[test]
fn tcp_drain_rfc3164_ts3164_utc() {
    let dest = "syslog-ng:20601"
        .to_socket_addrs()
        .expect("Unable to resolve host, check that syslog-ng Docker service is up")
        .collect::<Vec<_>>()
        [0];
    logger_emit!(
        UDPDrain,
        HeaderRFC3164<Ts3164Utc>,
        MessageKSV,
        dest,
        "TCP Test message RFC3164 TS3164 UTC KSV"
    );
}

#[test]
fn tcp_drain_rfc5424_iso_utc() {
    let dest = "syslog-ng:22601"
        .to_socket_addrs()
        .expect("Unable to resolve host, check that syslog-ng Docker service is up")
        .collect::<Vec<_>>()
        [0];
    logger_emit!(
        UDPDrain,
        HeaderRFC5424<TsIsoUtc>,
        MessageRFC5424,
        dest,
        "TCP Test message RFC5424 ISO UTC NATIVE"
    );
}
