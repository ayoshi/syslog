extern crate compiletest_rs as compiletest;

use std::env;
use std::path::PathBuf;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::default_config();

    let cfg_mode = mode.parse().expect("Invalid mode");

    // Support for overriding target dir through environment vars
    let target_dir = match env::var("CARGO_TARGET_DIR") {
        Ok(dir) => dir,
        Err(_) => "target".to_owned()
    };

    let rustflags = format!("-L {target}/debug/ -L {target}/debug/deps", target=target_dir);

    config.target_rustcflags = Some(rustflags);
    if let Ok(name) = env::var::<&str>("TESTNAME") {
        let s: String = name.to_owned();
        config.filter = Some(s)
    }
    config.mode = cfg_mode;
    config.src_base = PathBuf::from(format!("tests/{}", mode));

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail");
    run_mode("run-fail");
}
