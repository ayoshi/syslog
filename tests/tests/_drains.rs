const SOCK_3164_KSV: &'static str = "/syslog-ng/socket_dgram_rfc3164_ksv";
const SOCK_5424_KSV: &'static str = "/syslog-ng/socket_dgram_rfc5424_ksv";
const SOCK_5424_NATIVE: &'static str = "/syslog-ng/socket_dgram_rfc5424_native";

generate_uds_tests!([uds_rfc3164_minimal_ksv, Rfc3164MinimalKsv, SOCK_3164_KSV],
                    [uds_rfc3164_ts3164_local_ksv, Rfc3164KsvTs3164Local, SOCK_3164_KSV],
                    [uds_rfc3164_ts3164_utc_ksv, Rfc3164KsvTs3164Utc, SOCK_3164_KSV],
                    [uds_rfc3164_tsiso_local_ksv, Rfc3164KsvTsIsoLocal, SOCK_3164_KSV],
                    [uds_rfc3164_tsiso_utc_ksv, Rfc3164KsvTsIsoUtc, SOCK_3164_KSV],
                    [uds_rfc5424_tsiso_local_ksv, Rfc5424KsvTsIsoLocal, SOCK_5424_KSV],
                    [uds_rfc5424_tsiso_utc_ksv, Rfc5424KsvTsIsoUtc, SOCK_5424_KSV],
                    [uds_rfc5424_tsiso_local_native, Rfc5424NativeTsIsoLocal, SOCK_5424_NATIVE],
                    [uds_rfc5424_tsiso_utc_native, Rfc5424NativeTsIsoUtc, SOCK_5424_NATIVE]);

generate_udp_tests!([udp_rfc3164_minimal_ksv, Rfc3164MinimalKsv, "syslog-ng:10514"],
                    [udp_rfc3164_ts3164_local_ksv, Rfc3164KsvTs3164Local, "syslog-ng:10514"],
                    [udp_rfc3164_ts3164_utc_ksv, Rfc3164KsvTs3164Utc, "syslog-ng:10514"],
                    [udp_rfc3164_tsiso_local_ksv, Rfc3164KsvTsIsoLocal, "syslog-ng:10514"],
                    [udp_rfc3164_tsiso_utc_ksv, Rfc3164KsvTsIsoUtc, "syslog-ng:10514"],
                    [udp_rfc5424_tsiso_local_ksv, Rfc5424KsvTsIsoLocal, "syslog-ng:20514"],
                    [udp_rfc5424_tsiso_utc_ksv, Rfc5424KsvTsIsoUtc, "syslog-ng:20514"],
                    [udp_rfc5424_tsiso_local_native, Rfc5424NativeTsIsoLocal, "syslog-ng:22514"],
                    [udp_rfc5424_tsiso_utc_native, Rfc5424NativeTsIsoUtc, "syslog-ng:22514"]);

generate_tcp_tests!([tcp_rfc3164_ts3164_local_ksv, Rfc3164KsvTs3164Local, "syslog-ng:10601"],
                    [tcp_rfc3164_ts3164_utc_ksv, Rfc3164KsvTs3164Utc, "syslog-ng:10601"],
                    [tcp_rfc3164_tsiso_local_ksv, Rfc3164KsvTsIsoLocal, "syslog-ng:10601"],
                    [tcp_rfc3164_tsiso_utc_ksv, Rfc3164KsvTsIsoUtc, "syslog-ng:10601"],
                    [tcp_rfc5424_tsiso_local_ksv, Rfc5424KsvTsIsoLocal, "syslog-ng:20601"],
                    [tcp_rfc5424_tsiso_utc_ksv, Rfc5424KsvTsIsoUtc, "syslog-ng:20601"],
                    [tcp_rfc5424_tsiso_local_native, Rfc5424NativeTsIsoLocal, "syslog-ng:22601"],
                    [tcp_rfc5424_tsiso_utc_native, Rfc5424NativeTsIsoUtc, "syslog-ng:22601"]);
