// All timestamp Invariants

// 1. Shortens tests
// 2. Helps to catch untested invariants as compiler warnings

type TsIsoUtc = Timestamp<TimestampISO8601, TimestampUTC>;
type TsIsoLocal = Timestamp<TimestampISO8601, TimestampLocal>;
type Ts3164Utc = Timestamp<TimestampRFC3164, TimestampUTC>;
type Ts3164Local = Timestamp<TimestampRFC3164, TimestampLocal>;

// TODO Add all header x timestamp invariant types

// Formater fixture
macro_rules! formatter(
        ($header: ty, $message: ty) => (
            SyslogFormat::<$header, $message>::new(
                None, Some("test" .to_owned()), 12345, Facility::LOG_USER)
    ));

// Emit message Fixture
macro_rules! logger_emit(
    ($drain: ident, $header: ty, $message: ty, $dest: expr, $event: expr) => {{

        let buffer = TestIoBuffer::new(1024);
        let introspection_drain = TestDrain::new(buffer.io(), formatter!($header, $message));

        let test_drain = $drain::new($dest, formatter!($header, $message))
            .connect()
            .unwrap();

        println!("{:?}", test_drain);
        let logger = Logger::root(duplicate(introspection_drain, test_drain).fuse(),
                                  o!("lk1" => "lv1", "lk2" => "lv2"));
            info!(logger, $event; "mk1" => "mv1", "mk2" => "mv2")
    }});

type SharedIoVec = Arc<Mutex<Vec<u8>>>;

// Test buffer to hold single message
// Can be passed to TestDrain, and examined from outside
#[allow(dead_code)]
#[derive(Clone)]
struct TestIoBuffer {
    io: SharedIoVec,
}

impl TestIoBuffer {
    #[allow(dead_code)]
    fn new(capacity: usize) -> TestIoBuffer {
        TestIoBuffer { io: Arc::new(Mutex::new(Vec::<u8>::with_capacity(capacity))) }
    }

    fn io(&self) -> SharedIoVec {
        self.io.clone()
    }

    #[allow(dead_code)]
    fn as_vec(&self) -> Vec<u8> {
        self.io.lock().unwrap().clone()
    }

    #[allow(dead_code)]
    fn as_string(&self) -> String {
        String::from_utf8(self.as_vec()).unwrap()
    }
}

// Test Drain which accepts a TestIoBuffer::io
#[derive(Debug)]
#[allow(dead_code)]
struct TestDrain<F>
    where F: StreamFormat
{
    io: SharedIoVec,
    formatter: F,
}

impl<F> TestDrain<F>
    where F: StreamFormat
{
    fn new(io: SharedIoVec, formatter: F) -> TestDrain<F> {
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
#[allow(dead_code)]
fn emit_test_message_to_buffer<F>(formatter: F) -> TestIoBuffer
    where F: StreamFormat + 'static
{
    let buffer = TestIoBuffer::new(1024);
    let test_drain = TestDrain::new(buffer.io(), formatter);
    let logger = Logger::root(test_drain.fuse(), o!("lk1" => "lv1", "lk2" => "lv2"));
    info!(logger, "Test message 1"; "mk1" => "mv1", "mk2" => "mv2" );
    return buffer;
}
