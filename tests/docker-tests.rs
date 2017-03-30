// Test requiring docker, work against live syslog instances

extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;
extern crate slog_stream;

extern crate serde_json;

#[cfg(test)]
#[cfg(feature="full-integration-env")]
mod tests {

    use serde_json;
    use slog::{Logger, Record, OwnedKeyValueList, Drain, DrainExt, duplicate};
    use slog_stream::Format as StreamFormat;
    use slog_syslog_ng::*;

    use std::{io, result, thread, time};
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    use std::net::ToSocketAddrs;
    use std::ops::DerefMut;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    include!("tests/_fixtures.rs");
    include!("tests/_drains.rs");

    // Empty messages.json and syslog-ng log files
    fn reset_syslog_ng() {

        File::create("/syslog-ng/messages.json").expect("Failed to empty messages.json");
        File::create("/syslog-ng/syslog-ng").expect("Failed to empty syslog-ng");
    }

    // Fetch syslog-ng ouput
    fn fetch_syslog_messages() -> Vec<serde_json::Value> {

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
    fn filter_syslog_messages(message: String) -> Vec<serde_json::Value> {

        File::open("/syslog-ng/messages.json")
            .map(|f| BufReader::new(f))
            .expect("Couldn't open messages file")
            .lines()
            .map(|l| l.expect("Couldn't get line"))
            .filter(|l| l.as_str().contains(message.as_str()))
            .map(|l| serde_json::from_str(l.as_str()).unwrap())
            .collect::<Vec<serde_json::Value>>()
    }


}
