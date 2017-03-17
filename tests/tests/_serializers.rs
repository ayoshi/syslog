#[test]
fn formatter_rfc3164_minimal_ksv() {
    let formatter = formatter!(HeaderRFC3164Minimal, MessageKSV);
    let buffer = emit_test_message_to_buffer(formatter);
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>"));
    assert!(buffer.as_string().contains("Test message 1 mk2=mv2 mk1=mv1 lk2=lv2 lk1=lv1"));
}

#[test]
fn formatter_rfc3164_ksv_tslocal() {
    let formatter = formatter!(HeaderRFC3164<Ts3164Local>, MessageKSV);
    let buffer = emit_test_message_to_buffer(formatter);
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>"));
    assert!(buffer.as_string().contains("Test message 1 mk2=mv2 mk1=mv1 lk2=lv2 lk1=lv1"));
}

#[test]
fn formatter_rfc3164_ksv_tsutc() {
    let formatter = formatter!(HeaderRFC3164<Ts3164Utc>, MessageKSV);
    let buffer = emit_test_message_to_buffer(formatter);
    println!("{:?}", buffer.as_vec());
    println!("{:?}", buffer.as_string());
    assert!(buffer.as_string().contains("<14>"));
    assert!(buffer.as_string().contains("Test message 1 mk2=mv2 mk1=mv1 lk2=lv2 lk1=lv1"));
}

#[test]
fn formatter_rfc5424_ksv() {
    let formatter = formatter!(HeaderRFC5424<TsIsoLocal>, MessageKSV);
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
fn formatter_rfc5424_native() {
    let formatter = formatter!(HeaderRFC5424<TsIsoUtc>, MessageRFC5424);
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
