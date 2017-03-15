extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;
extern crate slog_term;

use slog_syslog_ng::*;

#[cfg(test)]
mod tests {

    use slog::{Logger, Record, OwnedKeyValueList, Drain, Never, Discard, DrainExt, duplicate};
    use slog_syslog_ng::*;
    use slog_term;
    use std;
    use std::net::SocketAddr;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    #[test]
    fn uds_drain_rfc3164_minimal() {
        let formatter = SyslogFormat::<HeaderRFC3164Minimal, MessageKSV>::new(None,
                                                                              Some("test"
                                                                                  .to_owned()),
                                                                              12345,
                                                                              Facility::LOG_USER);
        let console_drain = slog_term::streamer().full().build();
        let test_drain =
            UDSDrain::new(PathBuf::from("/tmp/rsyslog/rfc3164"), formatter).connect().unwrap();

        // let test_drain =
        //     UDSDrain::new(PathBuf::from("/var/run/syslog"), formatter).connect().unwrap();

        println!("{:?}", test_drain);
        let logger = Logger::root(duplicate(console_drain, test_drain).fuse(),
                                  o!("key1" => "value1", "key2" => "value2"));

        info!(logger, "Test message RFC3164 minimal"; "mk1" => "mv1", "mk2" => "mv2" );
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

        let console_drain = slog_term::streamer().full().build();
        let test_drain =
            UDSDrain::new(PathBuf::from("/tmp/rsyslog/rfc3164"), formatter).connect().unwrap();

        // let test_drain =
        //     UDSDrain::new(PathBuf::from("/var/run/syslog"), formatter).connect().unwrap();

        println!("{:?}", test_drain);

        let logger = Logger::root(duplicate(console_drain, test_drain).fuse(),
                                  o!("key1" => "value1", "key2" => "value2"));

        info!(logger, "Test message RFC3164 full"; "mk1" => "mv1", "mk2" => "mv2" );
        assert!(false);

    }

    #[test]
    fn uds_drain_rfc5424() {
        let formatter = SyslogFormat::<HeaderRFC5424<Timestamp<TimestampISO8601,
                                                               TimestampLocal>>,
                                       MessageRFC5424>::new(None,
                                                            Some("test".to_owned()),
                                                            12345,
                                                            Facility::LOG_USER);

        let console_drain = slog_term::streamer().full().build();
        let test_drain =
            UDSDrain::new(PathBuf::from("/tmp/rsyslog/rfc3164"), formatter).connect().unwrap();

        // let test_drain =
        //     UDSDrain::new(PathBuf::from("/var/run/syslog"), formatter).connect().unwrap();

        println!("{:?}", test_drain);

        let logger = Logger::root(duplicate(console_drain, test_drain).fuse(),
                                  o!("key1" => "value1", "key2" => "value2"));

        info!(logger, "Test message RFC5424"; "mk1" => "mv1", "mk2" => "mv2" );
        assert!(false);

    }

    #[test]
    fn udp_drain_rfc3164_minimal() {
        let formatter = SyslogFormat::<HeaderRFC3164Minimal, MessageKSV>::new(None,
                                                                              Some("test"
                                                                                  .to_owned()),
                                                                              12345,
                                                                              Facility::LOG_USER);

        let console_drain = slog_term::streamer().full().build();
        let test_drain = UDPDrain::new(SocketAddr::from_str("192.168.99.100:10514").unwrap(),
                                       formatter)
            .connect()
            .unwrap();

        println!("{:?}", test_drain);
        let logger = Logger::root(duplicate(console_drain, test_drain).fuse(),
                                  o!("key1" => "value1", "key2" => "value2"));

        info!(logger, "Test message RFC3164 minimal"; "mk1" => "mv1", "mk2" => "mv2" );
        assert!(false);

    }


}
