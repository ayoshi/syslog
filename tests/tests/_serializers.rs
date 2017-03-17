#[test]
fn ksv_serializer() {

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

            let mut serializer = KSVSerializer::new(io, "=".into());
            for (k, v) in logger_values.iter() {
                v.serialize(rinfo, k, &mut serializer)?;
            }

            for &(k, v) in rinfo.values().iter() {
                v.serialize(rinfo, k, &mut serializer)?;
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
fn formatter_rfc3164_minimal_ksv() {
    let formatter = formatter!(HeaderRFC3164Minimal, MessageKSV);

    let buffer = TestIoBuffer::new(1024);
    let test_drain = TestDrain::new(buffer.io(), formatter);
    let logger = Logger::root(test_drain, o!("lk1" => "lv1", "lk2" => "lv2"));
    info!(logger, "Test message 1"; "mk1" => "mv1", "mk2" => "mv2" );
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>"));
    assert!(buffer.as_string().contains("Test message 1 mk2=mv2 mk1=mv1 lk2=lv2 lk1=lv1"));
}

#[test]
fn formatter_rfc3164_ksv_tslocal() {
    let formatter = formatter!(HeaderRFC3164<Ts3164Local>, MessageKSV);

    let buffer = TestIoBuffer::new(1024);
    let test_drain = TestDrain::new(buffer.io(), formatter);
    let logger = Logger::root(test_drain, o!("lk1" => "lv1", "lk2" => "lv2"));
    info!(logger, "Test message 1"; "mk1" => "mv1", "mk2" => "mv2" );
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>"));
    assert!(buffer.as_string().contains("Test message 1 mk2=mv2 mk1=mv1 lk2=lv2 lk1=lv1"));
}

#[test]
fn formatter_rfc3164_ksv_tsutc() {
    let formatter = formatter!(HeaderRFC3164<Ts3164Utc>, MessageKSV);

    let buffer = TestIoBuffer::new(1024);
    let test_drain = TestDrain::new(buffer.io(), formatter);
    let logger = Logger::root(test_drain, o!("lk1" => "lv1", "lk2" => "lv2"));
    info!(logger, "Test message 1"; "mk1" => "mv1", "mk2" => "mv2" );
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>"));
    assert!(buffer.as_string().contains("Test message 1 mk2=mv2 mk1=mv1 lk2=lv2 lk1=lv1"));
}

#[test]
fn formatter_rfc5424_ksv() {
    let formatter = formatter!(HeaderRFC5424<TsIsoLocal>, MessageKSV);

    let buffer = TestIoBuffer::new(1024);
    let test_drain = TestDrain::new(buffer.io(), formatter);
    let logger = Logger::root(test_drain, o!("lk" => "lv"));
    info!(logger, "Test message 1"; "mk" => "mv" );
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>1"));
    assert!(buffer.as_string().contains("Test message 1 mk=mv lk=lv"));
}

#[test]
fn formatter_rfc5424_native() {
    let formatter = formatter!(HeaderRFC5424<TsIsoUtc>, MessageRFC5424);

    let buffer = TestIoBuffer::new(1024);
    let test_drain = TestDrain::new(buffer.io(), formatter);
    let logger = Logger::root(test_drain, o!("lk" => "lv"));
    info!(logger, "Test message 1"; "mk" => "mv" );
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>1"));
    assert!(buffer.as_string().contains("+00:00"));
    assert!(buffer.as_string().contains("Test message 1"));
    assert!(buffer.as_string().contains("[logger@"));
    assert!(buffer.as_string().contains("[msg@"));
    assert!(buffer.as_string().contains("mk=mv]"));
    assert!(buffer.as_string().contains("lk=lv]"));
}
