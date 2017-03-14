#![warn(missing_docs)]
//!
//! TODO Syslog drain for slog
//!

extern crate slog;
extern crate chrono;
extern crate libc;
extern crate serde;
extern crate serde_json;
extern crate slog_stream;

/// TODO
pub mod config;
/// TODO
pub mod syslog;
/// TODO
pub mod posix;

mod time;
mod format;
mod serializers;
mod drains;

pub use self::config::*;
pub use self::format::*;
pub use self::posix::{get_pid, get_process_name, get_host_name};
pub use self::serializers::*;
pub use self::syslog::*;
pub use self::time::*;
pub use self::drains::*;


/// Entry point to any further syslog configuration
pub fn syslog() -> SyslogConfig<DefaultConfig> {
    SyslogConfig::<DefaultConfig>::new()
}
