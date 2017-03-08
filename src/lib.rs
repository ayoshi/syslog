/////! Syslog RFC3164 and RFC5424 formatter and drain for slog
/////!
/////! ```
/////! #[macro_use]
/////! extern crate slog;
/////! extern crate slog_syslog;
/////!
/////! use slog::*;
/////!
/////! fn main() {
/////!     let root = Logger::root(slog_term::streamer().build().fuse(),
///// o!("build-id" => "8dfljdf"));
/////! }
/////! ```
/////
/////

#![warn(missing_docs)]
extern crate slog;
extern crate chrono;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate slog_stream;


pub mod config;
pub mod syslog;
pub mod posix;

mod time;
mod format;
mod serializers;

pub use self::config::*;
pub use self::posix::{get_pid, get_process_name, get_host_name};
pub use self::syslog::*;
pub use self::serializers::*;
pub use self::format::*;
pub use self::time::*;


/// Entry point to any further syslog configuration
pub fn syslog() -> SyslogConfig<DefaultConfig> {
    SyslogConfig::<DefaultConfig>::new()
}
