extern crate slog_syslog;

#[cfg(test)]
mod tests {

    use slog_syslog::*;

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
    //    #[test]
    //    #[ignore]
    //    fn get_local_socket() {
    //        println!("{:?}",
    //                 UnixDomainSocketStreamer::locate_default_uds_socket());
    //        assert!(false);
    //    }
}
