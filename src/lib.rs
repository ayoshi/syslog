#![warn(missing_docs)]
//!
//! TODO Syslog drain for slog
//!

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate slog;
extern crate chrono;
extern crate libc;
extern crate serde;
extern crate serde_json;
// extern crate slog_stream;
extern crate openssl;
extern crate parking_lot;

#[macro_use]
extern crate error_chain;

/// TODO
pub mod config;
/// TODO
pub mod syslog;
/// TODO
pub mod posix;

pub mod errors;

mod time;
mod format;
mod serializers;
mod uds_drain;
mod udp_drain;
mod tcp_drain;
mod tls_drain;
mod tls_client;

pub use self::config::*;
pub use self::format::*;
pub use self::posix::{get_pid, get_process_name, get_host_name};
pub use self::serializers::*;
pub use self::syslog::*;
pub use self::tcp_drain::*;
pub use self::time::*;
pub use self::tls_client::TlsSessionConfig;
pub use self::tls_drain::*;
pub use self::udp_drain::*;
pub use self::udp_drain::*;
pub use self::uds_drain::*;

/// Timeout in seconds for tyring to acquire lock on streams
const LOCK_TRY_TIMEOUT: u64 = 3;



/// Entry point to any further syslog configuration
pub fn syslog() -> SyslogConfig<DefaultConfig> {
    SyslogConfig::<DefaultConfig>::new()
}
