use slog::{Logger, Record, OwnedKeyValueList, Drain, DrainExt};
use slog_stream::Format as StreamFormat;
// use slog_syslog_ng::*;

use std::{io, result};
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
        self.io
            .lock()
            .unwrap()
            .clone()
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
    where F: StreamFormat
{
    io: SharedIoVec,
    formatter: F,
}

impl<F> TestDrain<F>
    where F: StreamFormat
{
    pub fn new(io: SharedIoVec, formatter: F) -> TestDrain<F> {
        TestDrain {
            io: io,
            formatter: formatter,
        }
    }
}

impl<F> Drain for TestDrain<F>
    where F: StreamFormat
{
    type Error = io::Error;

    fn log(&self, record: &Record, values: &OwnedKeyValueList) -> result::Result<(), Self::Error> {
        let mut io = self.io.lock().unwrap();
        self.formatter.format(io.deref_mut(), record, values).unwrap_or(());
        Ok(())
    }
}

// Create test buffer for introspection, and
// log defined message to the test drain, returning buffer
pub fn emit_test_message_to_buffer<F>(formatter: F) -> TestIoBuffer
    where F: StreamFormat + 'static
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
macro_rules! logger_emit(
    ($drain: ident, $format: ident, $dest: expr, $event: expr) => {{

        let buffer = TestIoBuffer::new(1024);
        let introspection_drain = TestDrain::new(buffer.io(), formatter!($format));

        let test_drain = $drain::new($dest.clone(), formatter!($format))
            .connect().expect("couldn't connect to socket");

        let logger = Logger::root(duplicate(introspection_drain, test_drain).fuse(),
                                  o!("lk1" => "lv1", "lk2" => "lv2"));

        info!(logger, $event; "mk1" => "mv1", "mk2" => "mv2");

        println!("{:?}", buffer.as_vec());
        println!("{:?}", buffer.as_string());
        println!("{} -> {:?} -> {:?}", buffer.as_string(), buffer.as_vec(), $dest);
    }});

// Fetch and verify recieved message from syslog-ng
#[macro_export]
macro_rules! verify_syslog_ng_message {
    ($message: ident) =>
        (
            // Timing issue here - we need to wait for logger to log,
            thread::sleep(time::Duration::from_millis(500));

            let logged_messages = filter_syslog_messages($message);

            // Message is logged, once and only once
            assert_eq!(logged_messages.len(), 1);

            let ref logged_message = logged_messages[0];
            println!("{}", logged_message);
        )
}

// Generate tests for unix socket drain
#[macro_export]
macro_rules! uds_tests {
    ($([$name:ident, $format:ident, $path:expr]),*) =>
        ($(
            #[test]
            fn $name() {
                let dest = PathBuf::from($path);
                let message = format!(
                    "{} {} message to {}",
                    stringify!(UDSDrain),
                    stringify!($format),
                    $path);
                logger_emit!(UDSDrain, $format, dest, message);
                verify_syslog_ng_message!(message);
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
                let message = format!(
                    "{} {} message to {}",
                    stringify!(UDPDrain),
                    stringify!($format),
                    $addr);
                logger_emit!(UDPDrain, $format, dest, message);
                verify_syslog_ng_message!(message);
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
                let message = format!(
                    "{} {} message to {}",
                    stringify!(TCPDrainDelimited),
                    stringify!($format),
                    $addr);
                logger_emit!(TCPDrainDelimited, $format, dest, message);
                verify_syslog_ng_message!(message);
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
                let message = format!(
                    "{} {} message to {}",
                    stringify!(TCPDrainFramed),
                    stringify!($format),
                    $addr);
                logger_emit!(TCPDrainFramed, $format, dest, message);
                verify_syslog_ng_message!(message);
            }
        )*)
}
