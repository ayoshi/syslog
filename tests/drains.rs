extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;
extern crate slog_term;

use slog_syslog_ng::*;

// TODO: Get IP straight from DOCKER_ENV
// TODO: Factor out fixtures

#[cfg(test)]
mod tests {

    use slog::{Logger, Record, OwnedKeyValueList, Drain, Never, Discard, DrainExt, duplicate};
    use slog_syslog_ng::*;
    use slog_term;
    use std;
    use std::net::SocketAddr;
    use std::path::PathBuf;
    use std::str::FromStr;
    use std::sync::{Arc, Mutex};

    type TsIsoUtc = Timestamp<TimestampISO8601, TimestampUTC>;
    type TsIosLocal = Timestamp<TimestampISO8601, TimestampLocal>;
    type Ts3164Utc = Timestamp<TimestampRFC3164, TimestampUTC>;
    type Ts3164Local = Timestamp<TimestampRFC3164, TimestampLocal>;

    // Formater fixture
    macro_rules! formatter(
        ($header: ty, $message: ty) => (
            SyslogFormat::<$header, $message>::new(
                None, Some("test" .to_owned()), 12345, Facility::LOG_USER)
    ));

    // Emit message Fixture
    macro_rules! logger_emit(
        ($drain: ident, $header: ty, $message: ty, $dest: expr, $event: expr) => {{
        let console_drain = slog_term::streamer().full().build();
        let test_drain = $drain::new($dest, formatter!($header, $message))
            .connect()
            .unwrap();

        println!("{:?}", test_drain);
        let logger = Logger::root(duplicate(console_drain, test_drain).fuse(),
                                  o!("lk1" => "lv1", "lk2" => "lv2"));
            info!(logger, $event; "mk1" => "mv1", "mk2" => "mv2")
        }});

    #[test]
    fn uds_drain_rfc3164_minimal() {
        let dest = PathBuf::from("/tmp/syslog/rfc3164");
        logger_emit!(UDSDrain,
                     HeaderRFC3164Minimal,
                     MessageKSV,
                     dest,
                     "Test message RFC3164 minimal");
        assert!(false);
    }

    #[test]
    fn uds_drain_rfc3164_full() {
        let dest = PathBuf::from("/tmp/syslog/rfc3164");
        logger_emit!(
            UDSDrain,
            HeaderRFC3164<Ts3164Local>,
            MessageKSV,
            dest,
            "Test message RFC3164 full"
        );
        assert!(false);
    }

    #[test]
    fn uds_drain_rfc5424() {
        let dest = PathBuf::from("/tmp/syslog/rfc5424");
        logger_emit!(
            UDSDrain,
            HeaderRFC3164<TsIosLocal>,
            MessageRFC5424,
            dest,
            "Test message RFC5424 Native"
        );
        assert!(false);
    }

    #[test]
    fn udp_drain_rfc3164_minimal() {
        let dest = SocketAddr::from_str("192.168.99.100:10514").unwrap();
        logger_emit!(
            UDPDrain,
            HeaderRFC3164Minimal,
            MessageKSV,
            dest,
            "Test message RFC3164 minimal"
        );
        assert!(false);
    }
}
