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
            self.formatter.format(io.deref_mut(), record, values);
            Ok(())
        }
    }


    #[test]
    fn kvs_serializer() {

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

    #[test]
    fn formatter_rfc3164() {

        let mut formatter = Format3164::new(None,
                                            Some("test".to_owned()),
                                            12345,
                                            Facility::LOG_USER,
                                            Box::new(timestamp_local_rfc3164),
                                            SerializationFormat::KSV);

        let buffer = TestIoBuffer::new(1024);
        let test_drain = TestDrain::new(buffer.io(), formatter);
        let logger = Logger::root(test_drain, o!("lk" => "lv"));
        info!(logger, "Test message 1"; "mk" => "mv" );
        println!("{:?}", buffer.as_vec());
        println!("{:?}", buffer.as_string());
        assert!(buffer.as_string() == "Test message 1 lk=lv mk=mv");
    }

}
