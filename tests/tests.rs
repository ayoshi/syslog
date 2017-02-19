extern crate slog_syslog;

#[macro_use]
extern crate slog;
extern crate slog_stream;

#[cfg(test)]
mod tests {

    use slog_syslog::*;
    use std::path::PathBuf;
    use slog::{Logger, Discard};
    use slog_stream::stream;
    // use slog::Level;

    #[test]
    fn uds_config_default() {
        let config = syslog().uds();
        assert!(config.connection_config.socket.is_none());
        assert!(!config.async);
        assert!(config.timestamp == TimestampFormat::RFC3164);
        assert!(config.timezone == TimestampTZ::Local);
        assert!(config.serialization == SerializationFormat::Native);
        assert!(config.facility == Facility::LOG_USER);
    }

    #[test]
    fn uds_config_with_path() {
        let config = syslog().uds().socket("/dev/log").mode(FormatMode::RFC5424);
        assert!(config.connection_config.socket == Some(PathBuf::from("/dev/log")));
        assert!(config.mode == FormatMode::RFC5424);
    }

    #[test]
    fn udp_config_default() {
        let config = syslog().udp();
        assert!(config.connection_config.server == None);
        assert!(!config.async);
        assert!(config.timestamp == TimestampFormat::RFC3164);
        assert!(config.timezone == TimestampTZ::Local);
        assert!(config.serialization == SerializationFormat::Native);
        assert!(config.facility == Facility::LOG_USER);
    }

    #[test]
    fn tcp_config_default() {
        let config = syslog().tcp();
        assert!(config.connection_config.server == None);
        assert!(!config.async);
        assert!(config.timestamp == TimestampFormat::RFC3164);
        assert!(config.timezone == TimestampTZ::Local);
        assert!(config.serialization == SerializationFormat::Native);
        assert!(config.facility == Facility::LOG_USER);
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
        let config = syslog().connect();
        assert!(config.is_ok())
    }

    #[test]
    fn construct_priority() {
        Priority::new(Facility::LOG_USER, Severity::LOG_WARN);
    }

    #[test]
    fn phantom_type_builder_invariants() {
        let config = syslog();
        println!("{:?}", config);
        let config = config.mode(FormatMode::RFC5424);
        println!("{:?}", config);
        // let config = config.socket("/dev/log"); Compiler error
        let config = config.uds();
        let config = config.socket("/dev/log");
        println!("{:?}", config);
        // let config = config("localhost:514"); Compiler error

        let config = syslog().mode(FormatMode::RFC3164);
        println!("{:?}", config);
        let config = syslog().udp().server("localhost:514");
        println!("{:?}", config);

        let config = syslog().tcp().server("localhost:514");
        let config = config.mode(FormatMode::RFC5424);
        // let config = config.socket("/dev/log"); Compiler error
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
