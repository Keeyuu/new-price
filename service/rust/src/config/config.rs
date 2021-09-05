use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Read;
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub mongo: Mongo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mongo {
	pub url: String,
	pub database: String,
	pub table_code: String,
	pub table_day: String,
}

impl Config {
	pub fn get() -> Result<Config> {
		let mut file = OpenOptions::new()
			.read(true)
			.open("config.yaml")
			.context("Config open file err")?;
		let mut s = String::new();
		file.read_to_string(&mut s)
			.context("Config open read err")?;
		let a = serde_yaml::from_str::<Config>(&s).context("Config convert yaml err")?;
		Ok(a)
	}
}
