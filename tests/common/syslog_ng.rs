use serde_json;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{thread, time};

// Empty messages.json and syslog-ng log files
pub fn reset_syslog_ng() {

    File::create("/syslog-ng/messages.json").expect("Failed to empty messages.json");
    File::create("/syslog-ng/syslog-ng").expect("Failed to empty syslog-ng");
}

// Fetch syslog-ng ouput
pub fn fetch_syslog_messages() -> Vec<serde_json::Value> {

    File::open("/syslog-ng/messages.json")
        .map(|f| BufReader::new(f))
        .expect("Couldn't open messages file")
        .lines()
        .map(|l| l.expect("Couldn't get line"))
        .map(|l| serde_json::from_str(l.as_str()).unwrap())
        .collect::<Vec<serde_json::Value>>()
}

// Fetch records for message in syslog-ng ouput
// Matching a filter
pub fn filter_syslog_messages(message: String) -> Vec<serde_json::Value> {

    File::open("/syslog-ng/messages.json")
        .map(|f| BufReader::new(f))
        .expect("Couldn't open messages file")
        .lines()
        .map(|l| l.expect("Couldn't get line"))
        // Replace required to remove syslogd escaping  of "
        .filter(|l| l.as_str().replace("\\","").contains(message.as_str()))
        .map(|l| serde_json::from_str(l.as_str()).unwrap())
        .collect::<Vec<serde_json::Value>>()
}

// Fetch and verify recieved message from syslog-ng
pub fn verify_syslog_ng_message(message: String) {

    // Timing issue here - we need to wait for logger to log,
    thread::sleep(time::Duration::from_millis(500));

    println!("---trying to find: {}---", message);
    let logged_messages = filter_syslog_messages(message);

    // Message is logged, once and only once
    assert_eq!(logged_messages.len(), 1);

    let ref logged_message = logged_messages[0];
    println!("{}", logged_message);
}
