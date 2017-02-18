//use std::net::{ToSocketAddrs};

//
// extern crate slog_stream;
extern crate chrono;
extern crate libc;

use libc::getpid;
use std::{io, env, ffi};

// extern crate hostname;
// extern crate thread_local;
// extern crate unix_socket;
//
//use std::io::Write;
// use unix_socket::UnixDatagram;
//
// use slog::Record;
// use slog::ser;
// use slog::{Level, OwnedKeyValueList};
//
// use hostname::get_hostname;
//
// use slog_stream::Format as StreamFormat;
// use slog_stream::{stream, async_stream};


// /include!("_format.rs");
//
// thread_local! {
//    static TL_BUF: cell::RefCell<Vec<u8>> = cell::RefCell::new(Vec::with_capacity(128));
//
//
// //// Get process name and pid
// /fn get_process_name() -> Option<String> {
// /    env::current_exe()
// /        .ok()
// /        .as_ref()
// /        .map(Path::new)
// /        .and_then(Path::file_name)
// /        .and_then(ffi::OsStr::to_str)
// /        .map(String::from)
// /}
// /
// /fn get_pid() -> i32 {
// /    unsafe { getpid() }
// /}
// /
// //// Timestamp function type
// pub type TimestampFn = Fn(&mut io::Write) -> io::Result<()> + Send + Sync;
// /
//
// // Formatting mode

//    }
//

//    /// Build the streamer
//    pub fn build(self) -> Box<slog::Drain<Error = io::Error> + Send + Sync> {
//        // FIX: the builder can fail, we need a way to fail safely
//    }
//
//
//    impl fmt::Debug for SyslogStreamer {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f,
//               "SyslogStreamer {{ async: {:?}, mode: {:?}, facility: {:?} }}",
//               self.async,
//               self.mode,
//               self.facility)
//    }
//
//
// impl Default for SyslogStreamer {
//    fn default() -> Self {
//        Self::new()
//    }
//
//
// // Build `slog_stream::Streamer`/`slog_stream::AsyncStreamer` that
// // will output logging records to syslog
// pub fn syslog_streamer() -> SyslogStreamer {
//    SyslogStreamer::new()
//

/// Use the UTC time zone for the timestamp
//fn use_utc_timestamp(mut self) -> Self {
//    self.fn_timestamp = Box::new(self.timestamp_utc);
//self
//}
//
///// Use the local time zone for the timestamp (default)
//fn use_local_timestamp(mut self) -> Self {
//    self.fn_timestamp = Box::new(self.timestamp_local);
//self
//}

/// Default local timestamp function used by `Format`
pub fn timestamp_local(io: &mut io::Write) -> io::Result<()> {
write!(io, "{}", chrono::Local::now().to_rfc3339())
}

/// Default UTC timestamp function used by `Format`
pub fn timestamp_utc(io: &mut io::Write) -> io::Result<()> {
write!(io, "{}", chrono::UTC::now().to_rfc3339())
}

/// Check for existence of domain sockets
pub fn locate_default_uds_socket() -> Result<PathBuf, String> {
    SYSLOG_DEFAULT_UDS_LOCATIONS.iter()
        .map(PathBuf::from)
        .find(|p| p.exists())
        .ok_or(format!("Couldn't find socket file (tried {:?})",
                       SYSLOG_DEFAULT_UDS_LOCATIONS))
}

 // Get process name
pub fn get_process_name() -> Option<String> {
    env::current_exe()
        .ok()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(ffi::OsStr::to_str)
        .map(String::from)
}

// Get pid
pub fn get_pid() -> i32 {
    unsafe { getpid() }
}
