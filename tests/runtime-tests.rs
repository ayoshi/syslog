// Tests to be run
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

extern crate slog_syslog_ng;

#[macro_use]
extern crate slog;
extern crate slog_stream;


#[cfg(test)]
mod tests {

    use slog::{Logger, Record, OwnedKeyValueList, Drain, DrainExt};
    use slog_stream::Format as StreamFormat;
    use slog_syslog_ng::*;

    use std::{io, result};
    use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    use std::ops::DerefMut;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    include!("tests/_fixtures.rs");
    include!("tests/_basic.rs");
    include!("tests/_config.rs");
    include!("tests/_serializers.rs");


}
