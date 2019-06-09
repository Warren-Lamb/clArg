use std::env;
use std::process;
extern crate clArg;
use clArg::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("{:?}",config);
}

mod tests {
    extern crate assert_cmd;

    use std::process::Command;
    use tests::assert_cmd::prelude::*;

    #[test]
    fn config_valid_args() {
        let mut cmd = Command::cargo_bin("clarg").unwrap();
        cmd.arg("r2s2").assert().success();
    }
    #[test]
    fn config_empty_args() {
        let mut cmd = Command::cargo_bin("clarg").unwrap();
        cmd.assert()
            .failure()
            .stderr("Problem parsing arguments: No arguments provided\n");
    }
    #[test]
    fn config_nonZero_args() {
        let mut cmd = Command::cargo_bin("clarg").unwrap();
        cmd.arg("r0s0")
            .assert()
            .failure()
            .stderr("Problem parsing arguments: Must be in format rxsx where x > 0\n");
    }
}
