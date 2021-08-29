use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Read;
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub mongo: String,
	pub test: String,
}

impl Config {
	pub fn get() -> Result<Config> {
		let mut file = OpenOptions::new().read(true).open("config.yaml")?;
		let mut s = String::new();
		file.read_to_string(&mut s)?;
		let a = serde_yaml::from_str::<Config>(&s)?;
		Ok(a)
	}
}
