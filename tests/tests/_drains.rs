const SOCK_3164_KSV: &'static str = "/syslog-ng/socket_dgram_rfc3164_ksv";
const SOCK_5424_KSV: &'static str = "/syslog-ng/socket_dgram_rfc5424_ksv";
const SOCK_5424_NATIVE: &'static str = "/syslog-ng/socket_dgram_rfc5424_native";

const UDP_3164_KSV: &'static str = "syslog-ng:10514";
const UDP_5424_KSV: &'static str = "syslog-ng:20514";
const UDP_5424_NATIVE: &'static str = "syslog-ng:22514";

const TCP_3164_KSV: &'static str = "syslog-ng:10601";
const TCP_5424_KSV: &'static str = "syslog-ng:20601";
const TCP_5424_NATIVE: &'static str = "syslog-ng:22601";

uds_tests!([uds_rfc3164_minimal_ksv, Rfc3164MinimalKsv, SOCK_3164_KSV],
           [uds_rfc3164_ts3164_local_ksv, Rfc3164KsvTs3164Local, SOCK_3164_KSV],
           [uds_rfc3164_ts3164_utc_ksv, Rfc3164KsvTs3164Utc, SOCK_3164_KSV],
           [uds_rfc3164_tsiso_local_ksv, Rfc3164KsvTsIsoLocal, SOCK_3164_KSV],
           [uds_rfc3164_tsiso_utc_ksv, Rfc3164KsvTsIsoUtc, SOCK_3164_KSV],
           [uds_rfc5424_tsiso_local_ksv, Rfc5424KsvTsIsoLocal, SOCK_5424_KSV],
           [uds_rfc5424_tsiso_utc_ksv, Rfc5424KsvTsIsoUtc, SOCK_5424_KSV],
           [uds_rfc5424_tsiso_local_native, Rfc5424NativeTsIsoLocal, SOCK_5424_NATIVE],
           [uds_rfc5424_tsiso_utc_native, Rfc5424NativeTsIsoUtc, SOCK_5424_NATIVE]);

udp_tests!([udp_rfc3164_minimal_ksv, Rfc3164MinimalKsv, UDP_3164_KSV],
           [udp_rfc3164_ts3164_local_ksv, Rfc3164KsvTs3164Local, UDP_3164_KSV],
           [udp_rfc3164_ts3164_utc_ksv, Rfc3164KsvTs3164Utc, UDP_3164_KSV],
           [udp_rfc3164_tsiso_local_ksv, Rfc3164KsvTsIsoLocal, UDP_3164_KSV],
           [udp_rfc3164_tsiso_utc_ksv, Rfc3164KsvTsIsoUtc, UDP_3164_KSV],
           [udp_rfc5424_tsiso_local_ksv, Rfc5424KsvTsIsoLocal, UDP_5424_KSV],
           [udp_rfc5424_tsiso_utc_ksv, Rfc5424KsvTsIsoUtc, UDP_5424_KSV],
           [udp_rfc5424_tsiso_local_native, Rfc5424NativeTsIsoLocal, UDP_5424_NATIVE],
           [udp_rfc5424_tsiso_utc_native, Rfc5424NativeTsIsoUtc, UDP_5424_NATIVE]);

tcp_delimited_tests!([tcp_rfc3164_ts3164_local_ksv, Rfc3164KsvTs3164Local, TCP_3164_KSV],
                     [tcp_rfc3164_ts3164_utc_ksv, Rfc3164KsvTs3164Utc, TCP_3164_KSV],
                     [tcp_rfc3164_tsiso_local_ksv, Rfc3164KsvTsIsoLocal, TCP_3164_KSV],
                     [tcp_rfc3164_tsiso_utc_ksv, Rfc3164KsvTsIsoUtc, TCP_3164_KSV]);

tcp_framed_tests!([tcp_rfc5424_tsiso_local_ksv, Rfc5424KsvTsIsoLocal, TCP_5424_KSV],
                  [tcp_rfc5424_tsiso_utc_ksv, Rfc5424KsvTsIsoUtc, TCP_5424_KSV],
                  [tcp_rfc5424_tsiso_local_native, Rfc5424NativeTsIsoLocal, TCP_5424_NATIVE],
                  [tcp_rfc5424_tsiso_utc_native, Rfc5424NativeTsIsoUtc, TCP_5424_NATIVE]);
