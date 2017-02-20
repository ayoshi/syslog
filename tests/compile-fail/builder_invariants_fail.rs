extern crate slog_syslog_ng;

use slog_syslog_ng::*;

fn main() {

        let config = syslog();
        let config = config.socket("/dev/log");
        //~^ no method named `socket`

        let config = syslog();
        let config = config.server("localhost:514");
        //~^ no method named `server`

        let config = syslog().uds();
        let config = config.server("localhost:514");
        //~^ no method named `server`

        let config = syslog().tcp().server("localhost:514");
        let config = config.socket("/dev/log");
        //~^ no method named `socket`

        let config = syslog().tcp().server("localhost:514");
        let config = config.socket("/dev/log");
        //~^ no method named `socket`

        let config = syslog().tcp();
        let config = config.mode(FormatMode::RFC5424);
        let config = config.socket("/dev/log");
        //~^ no method named `socket`

        let config = syslog().uds();
        let config = config.mode(FormatMode::RFC5424);
        let config = config.server("localhost:514");
        //~^ no method named `server`

}
