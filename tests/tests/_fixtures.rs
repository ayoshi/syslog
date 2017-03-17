// Formater fixture
macro_rules! formatter(
        ($header: ty, $message: ty) => (
            SyslogFormat::<$header, $message>::new(
                None, Some("test" .to_owned()), 12345, Facility::LOG_USER)
    ));

// Emit message Fixture
macro_rules! logger_emit(
        ($drain: ident, $header: ty, $message: ty, $dest: expr, $event: expr) => {{
        let console_drain = slog_term::streamer().full().build();
        let test_drain = $drain::new($dest, formatter!($header, $message))
            .connect()
            .unwrap();

        println!("{:?}", test_drain);
        let logger = Logger::root(duplicate(console_drain, test_drain).fuse(),
                                  o!("lk1" => "lv1", "lk2" => "lv2"));
            info!(logger, $event; "mk1" => "mv1", "mk2" => "mv2")
        }});

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

    fn log(&self, record: &Record, values: &OwnedKeyValueList) -> std::result::Result<(), Never> {
        use std::ops::DerefMut;
        let mut io = self.io.lock().unwrap();
        self.formatter.format(io.deref_mut(), record, values).unwrap_or(());
        Ok(())
    }
}
