// All invariants as types
// 1. Shortens tests
// 2. Helps to catch untested invariants as compiler warnings

// Message Invariants
type Rfc3164MinimalKsv = SyslogFormat<HeaderRFC3164Minimal, MessageKSV>;
type Rfc3164Ksv<T> = SyslogFormat<HeaderRFC3164<T>, MessageKSV>;
type Rfc5424Ksv<T> = SyslogFormat<HeaderRFC5424<T>, MessageKSV>;
type Rfc5424Native<T> = SyslogFormat<HeaderRFC5424<T>, MessageRFC5424>;

// Cartesian product

// RFC3164
type Rfc3164KsvTs3164Local = Rfc3164Ksv<Ts3164Local>;
type Rfc3164KsvTs3164Utc = Rfc3164Ksv<Ts3164Utc>;
type Rfc3164KsvTsIsoLocal = Rfc3164Ksv<TsIsoLocal>;
type Rfc3164KsvTsIsoUtc = Rfc3164Ksv<TsIsoUtc>;

// RFC 5424
type Rfc5424KsvTsIsoLocal = Rfc5424Ksv<TsIsoLocal>;
type Rfc5424KsvTsIsoUtc = Rfc5424Ksv<TsIsoUtc>;
type Rfc5424NativeTsIsoLocal = Rfc5424Native<TsIsoLocal>;
type Rfc5424NativeTsIsoUtc = Rfc5424Native<TsIsoUtc>;


// Formater fixture
macro_rules! formatter(
        ($format: ident) => (
            $format::new(
                None, Some("test" .to_owned()), 12345, Facility::LOG_USER)
    ));

// Emit message Fixture
macro_rules! logger_emit(
    ($drain: ident, $header: ty, $message: ty, $dest: expr, $event: expr) => {{

        let buffer = TestIoBuffer::new(1024);
        let introspection_drain = TestDrain::new(buffer.io(), formatter!($header, $message));

        let test_drain = $drain::new($dest, formatter!($header, $message))
            .connect().expect("couldn't connect to socket");

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
