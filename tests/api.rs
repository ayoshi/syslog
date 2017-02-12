extern crate slog_syslog;

#[cfg(test)]
mod tests {

    use slog_syslog::*;

    #[test]
    fn domain_socket_drain_default() {
        let drain =domain_socket_drain().build();
        assert!(drain.is_ok());
    }
}
