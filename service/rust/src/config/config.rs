use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Read;
#[derive(Debug, Serialize, Deserialize)]
enum Config {
	None,
	Config { mongo: String, test: String },
}

impl Config {
	fn init() -> Config {
		// serde_yaml::from_str(s: &str)
		let mut file = OpenOptions::new().read(true).open("./config.yaml");
		match file {
			Ok(ref mut fd) => {
				let mut s = String::new();
				if let Err(err) = fd.read_to_string(&mut s) {
					return Config::None;
				}
			}
			Err(fd) => {}
		}
		Config::None
	}
}

fn get_config() -> Config {
	return Config::None;
}

// fn init() -> Result<config, serde_yaml::Error> {}
