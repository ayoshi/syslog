extern crate libc;

use libc::getpid;
use std::{env, ffi};
use std::path::{PathBuf, Path};

use syslog::SYSLOG_DEFAULT_UDS_LOCATIONS;


/// Check for existence of domain sockets
pub fn locate_default_uds_socket() -> Result<PathBuf, String> {
    SYSLOG_DEFAULT_UDS_LOCATIONS.iter()
        .map(PathBuf::from)
        .find(|p| p.exists())
        .ok_or_else(|| {
                        format!("Couldn't find socket file (tried {:?})",
                                SYSLOG_DEFAULT_UDS_LOCATIONS)
                    })
}

/// Get current process name
pub fn get_process_name() -> Option<String> {
    env::current_exe()
        .ok()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(ffi::OsStr::to_str)
        .map(String::from)
}

/// Get current proccess pid
pub fn get_pid() -> i32 {
    unsafe { getpid() }
}

/// Get local hostname
pub fn get_host_name() -> Result<String, String> {

    extern "C" {
        pub fn gethostname(name: *mut libc::c_char, size: libc::size_t) -> libc::c_int;
    }

    let len = u8::max_value() as usize;
    let mut buf = vec![0; len];

    let err = unsafe { gethostname(buf.as_mut_ptr() as *mut libc::c_char, len as libc::size_t) };

    match err {
        0 => {
            // If we have NULL, terminate, otherwise take whole buffer
            let actual_len = buf.iter().position(|byte| *byte == 0).unwrap_or(len);
            // trim the hostname to the actual len
            String::from_utf8(buf.split_at(actual_len).0.to_vec()).map_err(|err| err.to_string())
        }
        _ => Err("Couldn't get my own hostname".to_string()),
    }
}
