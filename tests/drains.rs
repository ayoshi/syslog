extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;

use slog_syslog_ng::*;

#[cfg(test)]
mod tests {

    use slog::{Logger, Record, OwnedKeyValueList, Drain, Never};
    use slog_syslog_ng::*;
    use std;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};
    use std::net::{SocketAddr};

    #[test]
    fn uds_drain_rfc3164_minimal() {
        let formatter = SyslogFormat::<HeaderRFC3164Minimal, MessageKSV>::new(None,
                                                                              Some("test"
                                                                                  .to_owned()),
                                                                              12345,
                                                                              Facility::LOG_USER);

        let test_drain =
            UDSDrain::new(PathBuf::from("/tmp/rsyslog/rfc3164"), formatter).connect().unwrap();
        // let test_drain = UDSDrain::new(PathBuf::from("/var/run/syslog"), formatter);
        println!("{:?}", test_drain);

        // let logger = Logger::root(test_drain, o!("lk1" => "lv1", "lk2" => "lv2"));
        // info!(logger, "Test message RFC3164 minimal"; "mk1" => "mv1", "mk2" => "mv2" );
        assert!(false);

    }

    #[test]
    fn uds_drain_rfc3164_full() {
        let formatter = SyslogFormat::<HeaderRFC3164<Timestamp<TimestampRFC3164,
                                                               TimestampLocal>>,
                                       MessageKSV>::new(None,
                                                        Some("test".to_owned()),
                                                        12345,
                                                        Facility::LOG_USER);

        let test_drain =
            UDSDrain::new(PathBuf::from("/tmp/rsyslog/rfc3164"), formatter).connect().unwrap();
        // let test_drain = UDSDrain::new(PathBuf::from("/var/run/syslog"), formatter);

        let logger = Logger::root(test_drain, o!("lk1" => "lv1", "lk2" => "lv2"));
        info!(logger, "Test message RFC3164 full"; "mk1" => "mv1", "mk2" => "mv2" );
        assert!(false);

    }

    // #[test]
    // fn uds_drain_rfc5424() {
    //     let formatter = SyslogFormat::<HeaderRFC5424<Timestamp<TimestampISO8601,
    //                                                            TimestampLocal>>,
    //                                    MessageRFC5424>::new(None,
    //                                                         Some("test".to_owned()),
    //                                                         12345,
    //                                                         Facility::LOG_USER);

    //     let test_drain =
    //         UDSDrain::new(PathBuf::from("/tmp/rsyslog/rfc5424"), formatter).connect().unwrap();
    //     // let test_drain = UDSDrain::new(PathBuf::from("/var/run/syslog"), formatter);

    //     let logger = Logger::root(test_drain, o!("lk1" => "lv1", "lk2" => "lv2"));
    //     info!(logger, "Test message RFC5424"; "mk1" => "mv1", "mk2" => "mv2" );
    //     assert!(false);

    // }

    // #[test]
    // fn udp_drain_rfc3164_minimal() {
    //     let formatter = SyslogFormat::<HeaderRFC3164Minimal, MessageKSV>::new(None,
    //                                                                           Some("test"
    //                                                                               .to_owned()),
    //                                                                           12345,
    //                                                                           Facility::LOG_USER);

    //     let test_drain = UDPDrain::new(SocketAddr::from_str("192.168.99.100:10514").unwrap(),
    //                                    formatter)
    //         .connect()
    //         .unwrap();
    //     // let test_drain = UDSDrain::new(PathBuf::from("/var/run/syslog"), formatter);
    //     println!("{:?}", test_drain);

    //     let logger = Logger::root(test_drain, o!("lk1" => "lv1", "lk2" => "lv2"));
    //     info!(logger, "Test message RFC3164 minimal"; "mk1" => "mv1", "mk2" => "mv2" );
    //     assert!(false);

    // }


}
