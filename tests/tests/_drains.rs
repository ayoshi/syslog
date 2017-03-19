#[test]
fn uds_drain_rfc3164_ksv_minimal() {
    let dest = PathBuf::from("/syslog-ng/socket_dgram_rfc3164_ksv");
    logger_emit!(UDSDrain,
                 Rfc3164MinimalKsv,
                 dest,
                 "UDS Test message RFC3164 MINIMAL KSV");
}

#[test]
fn uds_drain_rfc3164_ksv_full() {
    let dest = PathBuf::from("/syslog-ng/socket_dgram_rfc3164_ksv");
    logger_emit!(UDSDrain,
                 Rfc3164KsvTs3164Local,
                 dest,
                 "UDS Test message RFC3164 TS3164 LOCAL KSV");
}

#[test]
fn uds_drain_rfc5424_native() {
    let dest = PathBuf::from("/syslog-ng/socket_dgram_rfc5424_native");
    logger_emit!(UDSDrain,
                 Rfc5424NativeTsIsoLocal,
                 dest,
                 "UDS Test message RFC5424 ISO LOCAL NATIVE");
}

#[test]
fn udp_drain_rfc3164_ksv_minimal() {
    let dest = "syslog-ng:10514"
        .to_socket_addrs()
        .expect("Unable to resolve host, check that syslog-ng Docker service is up")
        .collect::<Vec<_>>()
                   [0];
    logger_emit!(UDPDrain,
                 Rfc3164MinimalKsv,
                 dest,
                 "UDP Test message RFC3164 MINIMAL KSV");
}

#[test]
fn tcp_drain_rfc3164_ksv_utc() {
    let dest = "syslog-ng:10601"
        .to_socket_addrs()
        .expect("Unable to resolve host, check that syslog-ng Docker service is up")
        .collect::<Vec<_>>()
                   [0];
    logger_emit!(TCPDrain,
                 Rfc3164KsvTsIsoUtc,
                 dest,
                 "TCP Test message RFC3164 ISO UTC KSV");
}

#[test]
fn tcp_drain_rfc3164_ts3164_utc() {
    let dest = "syslog-ng:20601"
        .to_socket_addrs()
        .expect("Unable to resolve host, check that syslog-ng Docker service is up")
        .collect::<Vec<_>>()
                   [0];
    logger_emit!(TCPDrain,
                 Rfc3164KsvTs3164Utc,
                 dest,
                 "TCP Test message RFC3164 TS3164 UTC KSV");
}

#[test]
fn tcp_drain_rfc5424_iso_utc() {
    let dest = "syslog-ng:22601"
        .to_socket_addrs()
        .expect("Unable to resolve host, check that syslog-ng Docker service is up")
        .collect::<Vec<_>>()
                   [0];
    logger_emit!(TCPDrain,
                 Rfc5424NativeTsIsoUtc,
                 dest,
                 "TCP Test message RFC5424 ISO UTC NATIVE");
}
