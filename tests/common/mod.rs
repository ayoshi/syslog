#![allow(dead_code)]
pub mod syslog_ng;

pub use self::syslog_ng::verify_syslog_ng_message;
use slog::{Logger, Record, OwnedKVList, Drain};
use slog_syslog_ng::SyslogFormat;

use std::{io, panic};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};


pub type SharedIoVec = Arc<Mutex<Vec<u8>>>;

// Test buffer to hold single message
// Can be passed to TestDrain, and examined from outside
#[allow(dead_code)]
#[derive(Clone)]
pub struct TestIoBuffer {
    io: SharedIoVec,
}

impl TestIoBuffer {
    pub fn new(capacity: usize) -> TestIoBuffer {
        TestIoBuffer { io: Arc::new(Mutex::new(Vec::<u8>::with_capacity(capacity))) }
    }

    pub fn io(&self) -> SharedIoVec {
        self.io.clone()
    }

    #[allow(dead_code)]
    pub fn as_vec(&self) -> Vec<u8> {
        self.io.lock().unwrap().clone()
    }

    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        String::from_utf8(self.as_vec()).unwrap()
    }
}

// Test Drain which accepts a TestIoBuffer::io
#[derive(Debug)]
#[allow(dead_code)]
pub struct TestDrain<F>
    where F: SyslogFormat + panic::RefUnwindSafe + Send + Sync
{
    io: SharedIoVec,
    formatter: F,
}

impl<F> TestDrain<F>
    where F: SyslogFormat + panic::RefUnwindSafe + Send + Sync
{
    pub fn new(io: SharedIoVec, formatter: F) -> TestDrain<F> {
        TestDrain {
            io: io,
            formatter: formatter,
        }
    }
}

impl<F> Drain for TestDrain<F>
    where F: SyslogFormat + panic::RefUnwindSafe + Send + Sync
{
    type Err = io::Error;
    type Ok = ();

    fn log(&self, record: &Record, values: &OwnedKVList) -> io::Result<()> {
        let mut io = self.io.lock().unwrap();
        self.formatter
            .format(io.deref_mut(), record, values)
            .unwrap_or(());
        Ok(())
    }
}

// Create test buffer for introspection, and
// log defined message to the test drain, returning buffer
pub fn emit_test_message_to_buffer<F>(formatter: F) -> TestIoBuffer
    where F: SyslogFormat + panic::RefUnwindSafe + Send + Sync
{
    let buffer = TestIoBuffer::new(1024);
    let test_drain = TestDrain::new(buffer.io(), formatter);
    let logger = Logger::root(test_drain.fuse(), o!("lk1" => "lv1", "lk2" => "lv2"));
    info!(logger, "Test message 1"; "mk1" => "mv1", "mk2" => "mv2" );
    return buffer;
}

// Formater fixture
#[macro_export]
macro_rules! formatter(
        ($format: ident) => (
            $format::new(
                None, Some("test" .to_owned()), 12345, Facility::LOG_USER)
    ));

// Emit message Fixture
#[macro_export]
macro_rules! emit_and_verify(
    ($test_drain: ident, $format: ident, $dest: expr, $event: expr) => {{

        let buffer = TestIoBuffer::new(1024);
        let introspection_drain = TestDrain::new(buffer.io(), formatter!($format));
        let drain_type = format!("{:?}", $test_drain);

        let logger = Logger::root(
            Duplicate::new(introspection_drain, $test_drain).fuse(),
            o!("lk1" => "lv1", "lk2" => "lv2")
        );

        info!(logger, "{} {:?} {}", $event, drain_type, stringify!($format);
              "mk1" => "mv1", "mk2" => "mv2");

        println!("{} \n-> {:?} \n-> {}", buffer.as_string(), buffer.as_vec(), drain_type);

        // Hate it, need better way to match on unique message in syslog-ng
        verify_syslog_ng_message(drain_type + &format!("\" {}", stringify!($format)));
    }});

// Generate tests for unix socket drain
#[macro_export]
macro_rules! uds_tests {
    ($([$name:ident, $format:ident, $path:expr]),*) =>
        ($(
            #[test]
            fn $name() {
                let dest = PathBuf::from($path);

                let test_drain = UDSDrain::new(dest.clone(), formatter!($format))
                    .connect().expect("couldn't connect to socket");

                emit_and_verify!(test_drain, $format, dest, "Test message");
            }
        )*)
}

// Generate tests for UDP drain
#[macro_export]
macro_rules! udp_tests {
    ($([$name:ident, $format:ident, $addr:expr]),*) =>
        ($(
            #[test]
            fn $name() {
                let dest = $addr
                    .to_socket_addrs()
                    .expect(format!("Couldn't to connect to {}", $addr).as_str())
                    .collect::<Vec<_>>()
                    [0];

                let test_drain = UDPDrain::new(dest.clone(), formatter!($format))
                    .connect().expect("couldn't connect to socket");

                emit_and_verify!(test_drain, $format, dest, "Test message");
            }
        )*)
}

// Generate tests for TCP drain
#[macro_export]
macro_rules! tcp_delimited_tests {
    ($([$name:ident, $format:ident, $addr:expr]),*) =>
        ($(
            #[test]
            fn $name() {
                let dest = $addr
                    .to_socket_addrs()
                    .expect(format!("Couldn't to connect to {}", $addr).as_str())
                    .collect::<Vec<_>>()
                    [0];

                let test_drain = TCPDrainDelimited::new(dest.clone(), formatter!($format))
                    .connect().expect("couldn't connect to socket");

                emit_and_verify!(test_drain, $format, dest, "Test message");
            }
        )*)
}

// Generate tests for TCP drain
#[macro_export]
macro_rules! tcp_framed_tests {
    ($([$name:ident, $format:ident, $addr:expr]),*) =>
        ($(
            #[test]
            fn $name() {
                let dest = $addr
                    .to_socket_addrs()
                    .expect(format!("Couldn't to connect to {}", $addr).as_str())
                    .collect::<Vec<_>>()
                    [0];
                let test_drain = TCPDrainFramed::new(dest.clone(), formatter!($format))
                    .connect().expect("couldn't connect to socket");

                emit_and_verify!(test_drain, $format, dest, "Test message");
            }
        )*)
}

// Generate tests for TLS drain
#[macro_export]
macro_rules! tls_framed_tests {
    ($([$name:ident, $format:ident, $addr:expr]),*) =>
        ($(
            #[test]
            fn $name() {
                let dest = $addr
                    .to_socket_addrs()
                    .expect(format!("Couldn't to connect to {}", $addr).as_str())
                    .collect::<Vec<_>>()
                    [0];
                let tls_session_config = TlsSessionConfig {
                    domain: String::from("syslog-ng"),
                    ca_file: Some(PathBuf::from("/syslog-ng/cacert.pem")),
                    private_key_file: None,
                    certs_file: None,
                    no_verify: false,
                };
                let test_drain = TLSDrainFramed::new(
                    dest, tls_session_config, formatter!($format))
                    .connect().expect("couldn't connect to socket");

                emit_and_verify!(test_drain, $format, dest, "Test message");
            }
        )*)
}
