mod config;
mod dao;
mod model;
mod service;
use anyhow::{Context, Result};
use config::Config;
use mongodb::bson::doc;
use tokio;
mod import;
const TABLE_SIZE: usize = 10;
fn main() -> Result<()> {
    let config = Config::get().context("main 1")?;
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(start(config))
}

async fn start(c: config::Config) -> Result<()> {
    // dao::test().await?;
    let database = dao::Mongo::new(&c.mongo.url, &c.mongo.database).await?;
    // let a = model::Code {
    //     type_: String::from("ccc"),
    // };
    let col_code = database
        .collection::<model::Code>(&c.mongo.table_code)
        .await;
    let list = model::code::get_stock_code(col_code).await?;
    println!("has get list len{}", list.len());
    // import::import_code(&c, &database)
    //     .await
    //     .context("start 1")?;
    // let filter = doc! { "type_": "ccc" };
    // let bbb = eee.find_one(filter, None).await?;
    // println!("{:?}", bbb);
    // c.find();
    // database.insert(&c.mongo.table_code, a, None).await?;
    Ok(())
}
