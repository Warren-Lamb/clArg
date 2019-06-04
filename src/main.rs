use std::process;
use std::env;

fn main() {
    println!("Hello, wod!");
}

mod tests {
	extern crate assert_cmd;

	use tests::assert_cmd::prelude::*;
	use std::process::Command;

	#[test]
	fn config_valid_args(){
		let mut cmd = Command::cargo_bin("clarg").unwrap();
		cmd.arg("r2d2")
		   .assert()
		   .success();
	}	
}