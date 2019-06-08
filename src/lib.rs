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
			None => return Err("No arguments provided"),
		};
		let argParts: Vec<&str> = availArgs.split("d").collect();
		if argParts.len() != 2  { 
			return Err("Must be in the form r<rolls>s<sides>");
		}
		let mut argParts = argParts.iter();
		let parseErr = "Must be in format rxdx where x > 0";
		let r = argParts.next().unwrap().parse::<u32>();
		let s = argParts.next().unwrap().parse::<u32>();
		if r.is_err() || s.is_err() {
			return Err(parseErr);
		}
		let r = r.unwrap();
		let s = s.unwrap();
		if r ==0 || s ==0 {
			return Err(parseErr);
		}
		Ok(Config{arg1: r,arg2: s})
	}
}