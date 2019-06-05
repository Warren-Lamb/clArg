use std::process;
use std::env;
extern crate clArg;
use clArg::Config;


fn main() {
	let config = Config::new(env::args()).unwrap_or_else(|err| {
		eprintln!("Problem parsing arguments: {}", err);
	    process::exit(1);
     });
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
	#[test]
	fn config_empty_args(){
		let mut cmd = Command::cargo_bin("clarg").unwrap();
		cmd.assert()
			.failure()
			.stderr("Problem parsing arguments: No arguments provided\n");
	}
	#[test]
	fn config_nonZero_args(){
		let mut cmd = Command::cargo_bin("clarg").unwrap();
		cmd.arg("r0d0")
			.assert()
			.failure()
			.stderr("Problem parsing argments: Must be in format rxdx where x > 0\n");
	}
}