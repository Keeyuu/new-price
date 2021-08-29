mod config;
mod db;
use anyhow::Result;
use config::Config;
const TABLE_SIZE: usize = 10;
fn main() -> Result<()> {
    println!("Hello, world!");
    let config = Config::get()?;
    println!("{}", config.mongo.table_code);
    Ok(())
}
