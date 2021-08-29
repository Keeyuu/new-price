mod config;
mod db;
mod model;
use anyhow::Result;
use config::Config;
use tokio;
const TABLE_SIZE: usize = 10;
fn main() -> Result<()> {
    let config = Config::get()?;
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(start(config))
}

async fn start(c: config::Config) -> Result<()> {
    let database = db::Mongo::new(&c.mongo.url, &c.mongo.database).await?;
    // let code = database.collection(&c.mongo.table_code).await;
    let a = model::Code {
        type_: String::from("ccc"),
    };
    database.insert(&c.mongo.table_code, a, None).await?;
    // code.insert_many(a, None).await?;
    Ok(())
}
