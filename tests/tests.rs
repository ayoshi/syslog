extern crate slog_syslog;

#[macro_use]
extern crate slog;

#[cfg(test)]
mod tests {

    use slog_syslog::*;
    use slog::{Record, RecordStatic};
    use slog::Level;
    use slog::OwnedKeyValueList;

    #[test]
    fn uds_drain_default() {
        let drain = syslog().uds().build();
        assert!(drain.is_ok());
    }

    #[test]
    fn udp_drain_default() {
        let drain = syslog().udp().build();
        assert!(drain.is_ok());
    }

    #[test]
    fn tcp_drain_default() {
        let drain = syslog().tcp().build();
        assert!(drain.is_ok());
    }

    #[test]
    fn test_get_pid() {
        assert!(get_pid() > 1);
    }

    #[test]
    fn test_get_process_name() {
        assert!(get_process_name().is_some());
    }

    #[test]
    #[ignore]
    fn connect_to_default() {
        let drain = syslog().connect();
        assert!(drain.is_ok())
    }

    #[test]
    fn kv_serializer() {

        let rs = RecordStatic {
            level: Level::Info,
            file: "filepath",
            line: 11192,
            column: 0,
            function: "function",
            module: "modulepath",
            target: "target"
        };

        let record = record!(
            Level::Info,
            "message",
            "a=b"
        );
        // let logger_values = OwnedKeyValueList::root(None)
        println!("{:?}", record.values().iter().rev().collect());

        let mut w = Vec::new();

        let mut serializer = KVSerializer::new(&mut w);

        for &(k, v) in record.values().iter().rev() {
            v.serialize(record, k, &mut serializer).unwrap();
        }

        // for (k, v) in logger_values.iter() {
        //     v.serialize(record, k, &mut serializer)?;
        // }

        let w = serializer.finish();
        println!("{:?}", w);
        assert!(false)

    }
    //    #[test]
    //    #[ignore]
    //    fn get_local_socket() {
    //        println!("{:?}",
    //                 UnixDomainSocketStreamer::locate_default_uds_socket());
    //        assert!(false);
    //    }
}
