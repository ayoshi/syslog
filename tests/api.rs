extern crate slog_syslog;

#[cfg(test)]
mod tests {

    use slog_syslog::*;

    #[test]
    fn unix_domain_socket_drain_default() {
        let drain =unix_domain_socket_drain().build();
        assert!(drain.is_ok());
    }
}
