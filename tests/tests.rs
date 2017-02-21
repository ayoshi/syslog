extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;
extern crate slog_stream;

#[cfg(test)]
mod tests {

    use std::net::{ToSocketAddrs, SocketAddr, IpAddr, Ipv4Addr};
    use std::path::PathBuf;
    use slog_syslog_ng::*;
    use slog::{Logger, Discard};
    use slog_stream::stream;
    // use slog::Level;

    #[test]
    fn get_pid_gt_one() {
        assert!(get_pid() > 1);
    }

    #[test]
    fn get_process_name_some() {
        assert!(get_process_name().is_some());
    }

    #[test]
    fn get_host_name_ok() {
        let hostname = get_host_name();
        println!("{:?}", hostname);
        assert!(hostname.is_ok());
    }


    // #[test]
    // #[ignore]
    // fn connect_to_default() {
    //     let config = syslog().connect();
    //     assert!(config.is_ok())
    // }

    #[test]
    fn construct_priority() {
        Priority::new(Facility::LOG_USER, Severity::LOG_WARN);
    }

    #[test]
    fn builder_invariants() {

        let config = syslog();
        println!("{:?}", config);
        let config = config.mode(FormatMode::RFC5424);
        println!("{:?}", config);

        let config = config.uds();
        let config = config.socket("/dev/log");
        println!("{:?}", config);
        let config = config.socket(PathBuf::from("/dev/log"));

        let config = syslog().mode(FormatMode::RFC3164);
        println!("{:?}", config);
        let config = syslog().udp().server("localhost:514");
        println!("{:?}", config);

        let config = syslog().tcp().server("localhost:514");
        let config = config.mode(FormatMode::RFC5424);
        println!("{:?}", config);

        let addr =SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 514);
        let config = syslog().tcp().server(addr);
        println!("{:?}", config);
    }

    #[test]
    fn kv_formatter() {
        let out = String::new();
        // let serializer =
        // let formatter = Format::(
        //     mode: FormatMode::RFC3164
        //     fn_timestamp: Box<timestamp_utc()>,
        //     hostname: "localhost".to_string(),
        //     process_name: test,
        //     serializer: &KV,
        //                          pid: i32,
        //                          facility: Facility
        // );

        // let log = Logger::root(
        //     stream(out, )
        //     , o!("version" => env!("CARGO_PKG_VERSION"))
        // );
    }

    //    #[test]
    //    #[ignore]
    //    fn get_local_socket() {
    //        println!("{:?}",
    //                 UnixDomainSocketStreamer::locate_default_uds_socket());
    //        assert!(false);
    //    }
}
