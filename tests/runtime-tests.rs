// Tests to be run
extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;
extern crate slog_stream;


#[cfg(test)]
mod tests {

    use slog::{Logger, Record, OwnedKeyValueList, Drain, Never};
    use slog_stream;
    use slog_syslog_ng::*;

    use std;
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    include!("tests/_fixtures.rs");
    include!("tests/_basic.rs");
    include!("tests/_config.rs");
    include!("tests/_serializers.rs");


}
