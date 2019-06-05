use std::error::Error;

#[derive(Debug)]
pub struct  Config {
	pub arg1: u32,
	pub arg2: u32,
}

impl Config {
	pub fn new (mut args: std::env::Args) -> Result<Config, &'static str> {
		args.next();
		let availArgs = match args.next() {
			Some(arg) => arg,
			None => return Err("No args provided"),
		};
		Ok(Config{arg1: 0,arg2: 0})
	}
}