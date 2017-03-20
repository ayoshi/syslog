//fn formatter_rfc3164_minimal_ksv()
//fn formatter_rfc3164_ksv_ts3164_local()
//fn formatter_rfc3164_ksv_ts3164_utc()
//fn formatter_rfc3164_ksv_tsiso_local()
//fn formatter_rfc3164_ksv_tsiso_utc()
//fn formatter_rfc5424_ksv_tsiso_local()
//fn formatter_rfc5424_ksv_tsiso_utc()
//fn formatter_rfc5424_native_tsiso_local()
//fn formatter_rfc5424_native_tsiso_utc()


generate_uds_tests!([uds_rfc3164_minimal_ksv,
                     Rfc3164MinimalKsv,
                     "/syslog-ng/socket_dgram_rfc3164_ksv"],
                    [uds_rfc3164_ts3164_local_ksv,
                     Rfc3164KsvTs3164Local,
                     "/syslog-ng/socket_dgram_rfc3164_ksv"],
                    [uds_rfc3164_ts3164_utc_ksv,
                     Rfc3164KsvTs3164Utc,
                     "/syslog-ng/socket_dgram_rfc3164_ksv"],
                    [uds_rfc3164_tsiso_local_ksv,
                     Rfc3164KsvTsIsoLocal,
                     "/syslog-ng/socket_dgram_rfc3164_ksv"],
                    [uds_rfc3164_tsiso_utc_ksv,
                     Rfc3164KsvTsIsoUtc,
                     "/syslog-ng/socket_dgram_rfc3164_ksv"],
                    [uds_rfc5424_tsiso_local_ksv,
                     Rfc5424KsvTsIsoLocal,
                     "/syslog-ng/socket_dgram_rfc5424_ksv"],
                    [uds_rfc5424_tsiso_utc_ksv,
                     Rfc5424KsvTsIsoUtc,
                     "/syslog-ng/socket_dgram_rfc5424_ksv"],
                    [uds_rfc5424_tsiso_local_native,
                     Rfc5424NativeTsIsoLocal,
                     "/syslog-ng/socket_dgram_rfc5424_native"],
                    [uds_rfc5424_tsiso_utc_native,
                     Rfc5424NativeTsIsoUtc,
                     "/syslog-ng/socket_dgram_rfc5424_native"]);

// RFC3164

// #[test]
// fn uds_rfc3164_ksv_ts3164_local() {
//     let dest = PathBuf::from("/syslog-ng/socket_dgram_rfc3164_ksv");
//     logger_emit!(UDSDrain,
//                  Rfc3164KsvTs3164Local,
//                  dest,
//                  "UDS Test message RFC3164 TS3164 LOCAL KSV");
// }

// // RFC5424
// #[test]
// fn uds_drain_rfc5424_native() {
//     let dest = PathBuf::from("/syslog-ng/socket_dgram_rfc5424_native");
//     logger_emit!(UDSDrain,
//                  Rfc5424NativeTsIsoLocal,
//                  dest,
//                  "UDS Test message RFC5424 ISO LOCAL NATIVE");
// }

// #[test]
// fn udp_drain_rfc3164_ksv_minimal() {
//     let dest = "syslog-ng:10514"
//         .to_socket_addrs()
//         .expect("Unable to resolve host, check that syslog-ng Docker service is up")
//         .collect::<Vec<_>>()
//                    [0];
//     logger_emit!(UDPDrain,
//                  Rfc3164MinimalKsv,
//                  dest,
//                  "UDP Test message RFC3164 MINIMAL KSV");
// }

// #[test]
// fn tcp_drain_rfc3164_ksv_utc() {
//     let dest = "syslog-ng:10601"
//         .to_socket_addrs()
//         .expect("Unable to resolve host, check that syslog-ng Docker service is up")
//         .collect::<Vec<_>>()
//                    [0];
//     logger_emit!(TCPDrain,
//                  Rfc3164KsvTsIsoUtc,
//                  dest,
//                  "TCP Test message RFC3164 ISO UTC KSV");
// }

// #[test]
// fn tcp_drain_rfc3164_ts3164_utc() {
//     let dest = "syslog-ng:20601"
//         .to_socket_addrs()
//         .expect("Unable to resolve host, check that syslog-ng Docker service is up")
//         .collect::<Vec<_>>()
//                    [0];
//     logger_emit!(TCPDrain,
//                  Rfc3164KsvTs3164Utc,
//                  dest,
//                  "TCP Test message RFC3164 TS3164 UTC KSV");
// }

// #[test]
// fn tcp_drain_rfc5424_iso_utc() {
//     let dest = "syslog-ng:22601"
//         .to_socket_addrs()
//         .expect("Unable to resolve host, check that syslog-ng Docker service is up")
//         .collect::<Vec<_>>()
//                    [0];
//     logger_emit!(TCPDrain,
//                  Rfc5424NativeTsIsoUtc,
//                  dest,
//                  "TCP Test message RFC5424 ISO UTC NATIVE");
// }
