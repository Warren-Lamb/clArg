use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub arg1: u32,
    pub arg2: u32,
}

fn remove_first(s: &str) -> Option<&str> {
    s.chars().next().map(|c| &s[c.len_utf8()..])
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let availArgs = match args.next() {
            Some(arg) => arg,
            None => return Err("No arguments provided"),
        };
        let argParts: Vec<&str> = availArgs.split("s").collect();
        if argParts.len() != 2 {
            return Err("Must be in the form r<rolls>s<sides>");
        }
        let mut argParts = argParts.iter();
        let parseErr = "Must be in format rxsx where x > 0";
        let r = argParts.next().unwrap();
        let r = remove_first(r).unwrap().parse::<u32>();
        let s = argParts.next().unwrap().parse::<u32>();
        if r.is_err() || s.is_err() {
            return Err(parseErr);
        }
        let r = r.unwrap();
        let s = s.unwrap();
        if r == 0 || s == 0 {
            return Err(parseErr);
        }
        Ok(Config { arg1: r, arg2: s })
    }
}
