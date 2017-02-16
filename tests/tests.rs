extern crate slog_syslog;

// #[macro_use]
extern crate slog;

#[cfg(test)]
mod tests {

    use slog_syslog::*;
    // use slog::Level;

    #[test]
    fn uds_drain_default() {
        let drain = syslog().uds().build();
        assert!(drain.is_ok());
    }

    #[test]
    fn udp_drain_default() {
        let drain = syslog().udp().build();
        assert!(drain.is_ok());
    }

    #[test]
    fn tcp_drain_default() {
        let drain = syslog().tcp().build();
        assert!(drain.is_ok());
    }

    #[test]
    fn test_get_pid() {
        assert!(get_pid() > 1);
    }

    #[test]
    fn test_get_process_name() {
        assert!(get_process_name().is_some());
    }

    #[test]
    #[ignore]
    fn connect_to_default() {
        let drain = syslog().connect();
        assert!(drain.is_ok())
    }

    #[test]
    fn construct_priority() {
        Priority::new(Facility::LOG_USER, Severity::LOG_WARN);
    }

    //    #[test]
    //    #[ignore]
    //    fn get_local_socket() {
    //        println!("{:?}",
    //                 UnixDomainSocketStreamer::locate_default_uds_socket());
    //        assert!(false);
    //    }
}
