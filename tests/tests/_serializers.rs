// RFC3164

#[test]
fn formatter_rfc3164_minimal_ksv() {
    let formatter = formatter!(FormatRfc3164MinimalKsv);
    let buffer = emit_test_message_to_buffer(formatter);
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>"));
    assert!(buffer.as_string().contains("Test message 1 mk2=mv2 mk1=mv1 lk2=lv2 lk1=lv1"));
}

// KSV

// RFC3164 Timestamp

#[test]
fn formatter_rfc3164_ksv_ts3164_local() {
    let formatter = formatter!(FormatRfc3164KsvTs3164Local);
    let buffer = emit_test_message_to_buffer(formatter);
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>"));
    assert!(buffer.as_string().contains("Test message 1 mk2=mv2 mk1=mv1 lk2=lv2 lk1=lv1"));
}

#[test]
fn formatter_rfc3164_ksv_ts3164_utc() {
    let formatter = formatter!(FormatRfc3164KsvTs3164Utc);
    let buffer = emit_test_message_to_buffer(formatter);
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>"));
    assert!(buffer.as_string().contains("Test message 1 mk2=mv2 mk1=mv1 lk2=lv2 lk1=lv1"));
}

// ISO timestamp

#[test]
fn formatter_rfc3164_ksv_tsiso_local() {
    let formatter = formatter!(FormatRfc3164KsvTsIsoLocal);
    let buffer = emit_test_message_to_buffer(formatter);
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>"));
    assert!(buffer.as_string().contains("Test message 1 mk2=mv2 mk1=mv1 lk2=lv2 lk1=lv1"));
}

#[test]
fn formatter_rfc3164_ksv_tsiso_utc() {
    let formatter = formatter!(FormatRfc3164KsvTsIsoUtc);
    let buffer = emit_test_message_to_buffer(formatter);
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>"));
    assert!(buffer.as_string().contains("Test message 1 mk2=mv2 mk1=mv1 lk2=lv2 lk1=lv1"));
}

// RFC5424

// KSV

#[test]
fn formatter_rfc5424_ksv_tsiso_local() {
    let formatter = formatter!(FormatRfc5424KsvTsIsoLocal);
    let buffer = emit_test_message_to_buffer(formatter);
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>1"));
    assert!(buffer.as_string().contains("Test message 1"));
    assert!(buffer.as_string().contains("mk1=mv1"));
    assert!(buffer.as_string().contains("mk2=mv2"));
    assert!(buffer.as_string().contains("lk1=lv1"));
    assert!(buffer.as_string().contains("lk2=lv2"));
}

#[test]
fn formatter_rfc5424_ksv_tsiso_utc() {
    let formatter = formatter!(FormatRfc5424KsvTsIsoUtc);
    let buffer = emit_test_message_to_buffer(formatter);
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>1"));
    assert!(buffer.as_string().contains("Test message 1"));
    assert!(buffer.as_string().contains("mk1=mv1"));
    assert!(buffer.as_string().contains("mk2=mv2"));
    assert!(buffer.as_string().contains("lk1=lv1"));
    assert!(buffer.as_string().contains("lk2=lv2"));
}

// Native

#[test]
fn formatter_rfc5424_native_tsiso_local() {
    let formatter = formatter!(FormatRfc5424NativeTsIsoLocal);
    let buffer = emit_test_message_to_buffer(formatter);
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>1"));
    assert!(buffer.as_string().contains("Test message 1"));
    assert!(buffer.as_string().contains("[logger@"));
    assert!(buffer.as_string().contains("[msg@"));
    assert!(buffer.as_string().contains("mk2=mv2 mk1=mv1]"));
    assert!(buffer.as_string().contains("lk2=lv2 lk1=lv1]"));
}

#[test]
fn formatter_rfc5424_native_tsiso_utc() {
    let formatter = formatter!(FormatRfc5424NativeTsIsoUtc);
    let buffer = emit_test_message_to_buffer(formatter);
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>1"));
    assert!(buffer.as_string().contains("+00:00"));
    assert!(buffer.as_string().contains("Test message 1"));
    assert!(buffer.as_string().contains("[logger@"));
    assert!(buffer.as_string().contains("[msg@"));
    assert!(buffer.as_string().contains("mk2=mv2 mk1=mv1]"));
    assert!(buffer.as_string().contains("lk2=lv2 lk1=lv1]"));
}
