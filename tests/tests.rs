extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;
extern crate slog_stream;

#[cfg(test)]
mod tests {

    use slog::{Logger, Record, OwnedKeyValueList, Drain, Never};
    use slog_stream;
    use slog_syslog_ng::*;
    use std;
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};
    // use slog::Level;

    type SharedIoVec = Arc<Mutex<Vec<u8>>>;

    // Test buffer to hold single message
    // Can be passed to TestDrain, and examined from outside
    #[derive(Clone)]
    struct TestIoBuffer {
        io: SharedIoVec,
    }

    impl TestIoBuffer {
        fn new(capacity: usize) -> TestIoBuffer {
            TestIoBuffer { io: Arc::new(Mutex::new(Vec::<u8>::with_capacity(capacity))) }
        }

        fn io(&self) -> SharedIoVec {
            self.io.clone()
        }

        fn as_vec(&self) -> Vec<u8> {
            self.io.lock().unwrap().clone()
        }

        fn as_string(&self) -> String {
            String::from_utf8(self.as_vec()).unwrap()
        }
    }

    // Test Drain which accepts a TestIoBuffer::io
    #[derive(Debug)]
    struct TestDrain<F>
        where F: slog_stream::Format
    {
        io: SharedIoVec,
        formatter: F,
    }

    impl<F> TestDrain<F>
        where F: slog_stream::Format
    {
        fn new(io: SharedIoVec, formatter: F) -> TestDrain<F> {
            TestDrain {
                io: io,
                formatter: formatter,
            }
        }
    }

    impl<F> Drain for TestDrain<F>
        where F: slog_stream::Format
    {
        type Error = Never;

        fn log(&self,
               record: &Record,
               values: &OwnedKeyValueList)
               -> std::result::Result<(), Never> {
            use std::ops::DerefMut;
            let mut io = self.io.lock().unwrap();
            self.formatter.format(io.deref_mut(), record, values).;
            Ok(())
        }
    }


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
        println!("{:?}", config);

        let config = syslog().mode(FormatMode::RFC3164);
        println!("{:?}", config);
        let config = syslog().udp().server("localhost:514");
        println!("{:?}", config);

        let config = syslog().tcp().server("localhost:514");
        let config = config.mode(FormatMode::RFC5424);
        println!("{:?}", config);

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 514);
        let config = syslog().tcp().server(addr);
        println!("{:?}", config);
    }

    #[test]
    fn kv_formatter() {

        pub struct TestFormatter;

        impl TestFormatter {
            pub fn new() -> Self {
                TestFormatter {}
            }
        }

        impl slog_stream::Format for TestFormatter {
            fn format(&self,
                      io: &mut std::io::Write,
                      rinfo: &Record,
                      logger_values: &OwnedKeyValueList)
                      -> std::io::Result<()> {
                write!(io, "{}", rinfo.msg())?;
                write!(io, " ")?;

                let mut serializer = KSVSerializer::new(io, "=".into());
                {
                    for (ref k, ref v) in logger_values.iter() {
                        v.serialize(rinfo, k, &mut serializer)?;
                    }
                }

                let mut io = serializer.finish();

                write!(io, " ")?;

                let mut serializer = KSVSerializer::new(io, "=".into());
                {
                    for &(ref k, ref v) in rinfo.values().iter() {
                        v.serialize(rinfo, k, &mut serializer)?;
                    }
                }

                Ok(())
            }
        }

        let buffer = TestIoBuffer::new(1024);
        let test_drain = TestDrain::new(buffer.io(), TestFormatter::new());
        let logger = Logger::root(test_drain, o!("lk" => "lv"));
        info!(logger, "Test message 1"; "mk" => "mv" );
        println!("{:?}", buffer.as_vec());
        println!("{:?}", buffer.as_string());
        assert!(buffer.as_string() == "Test message 1 lk=lv mk=mv");
    }
}

//    #[test]
//    #[ignore]
//    fn get_local_socket() {
//        println!("{:?}",
//                 UnixDomainSocketStreamer::locate_default_uds_socket());
//        assert!(false);
//    }
