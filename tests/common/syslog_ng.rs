use serde_json;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        .filter(|l| l.as_str().contains(message.as_str()))
        .map(|l| serde_json::from_str(l.as_str()).unwrap())
        .collect::<Vec<serde_json::Value>>()
}
