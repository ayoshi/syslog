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

    #[test]
    fn drain() {
        let formatter = SyslogFormat::<HeaderRFC3164Minimal,
                                       MessageKSV>::new(None,
                                                        Some("test".to_owned()),
                                                        12345,
                                                        Facility::LOG_USER);

        // let test_drain = UDSDrain::new(PathBuf::from("/tmp/rsyslog/log"), formatter);
        let test_drain = UDSDrain::new(PathBuf::from("/var/run/syslog"), formatter);

        let logger = Logger::root(test_drain, o!("lk1" => "lv1", "lk2" => "lv2"));
        info!(logger, "Test message 1"; "mk1" => "mv1", "mk2" => "mv2" );
        assert!(false);

    }
}
