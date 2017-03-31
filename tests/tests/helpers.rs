#[test]
fn get_pid_gt_one() {
    assert!(get_pid() > 1);
}

#[test]
fn get_process_name_some() {
    assert!(get_process_name().is_some());
}

#[test]
fn get_host_name_ok() {
    let hostname = get_host_name();
    println!("{:?}", hostname);
    assert!(hostname.is_ok());
}

// #[test]
// #[ignore]
// fn connect_to_default() {
//     let config = syslog().connect();
//     assert!(config.is_ok())
// }


//    #[test]
//    #[ignore]
//    fn get_local_socket() {
//        println!("{:?}",
//                 UnixDomainSocketStreamer::locate_default_uds_socket());
//        assert!(false);
//    }
