mod config;
mod db;
use config::Config;
fn main() {
    println!("Hello, world!");
    let a = Config::get();
   
}
