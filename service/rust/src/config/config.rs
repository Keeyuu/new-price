use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Read;
#[derive(Debug, Serialize, Deserialize)]
pub enum Config {
	None,
	Config { mongo: String, test: String },
}

impl Config {
	pub fn new() -> Config {
		let mut file = OpenOptions::new().read(true).open("config.yaml");
		match file {
			Ok(ref mut fd) => {
				let mut s = String::new();
				if let Err(err) = fd.read_to_string(&mut s) {
					println!("{}", err);
					return Config::None;
				}
				let a = serde_yaml::from_str::<Config>(&s);
				// if let Ok(c) = serde_yaml::from_str(&s) {};
			}
			Err(fd) => {
				println!("{}", fd);
			}
		}
		Config::None
	}
}

pub fn get_config() -> Config {
	return Config::None;
}

// fn init() -> Result<config, serde_yaml::Error> {}
