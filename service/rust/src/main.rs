mod config;
mod dao;
mod model;
use anyhow::Result;
use config::Config;
use mongodb::bson::doc;
use tokio;
const TABLE_SIZE: usize = 10;
fn main() -> Result<()> {
    let config = Config::get()?;
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(start(config))
}

async fn start(c: config::Config) -> Result<()> {
    dao::test().await?;
    // let database = dao::Mongo::new(&c.mongo.url, &c.mongo.database).await?;
    // let a = model::Code {
    //     type_: String::from("ccc"),
    // };
    // let eee = database
    // .collection::<model::Code>(&c.mongo.table_code)
    // .await;
    // let filter = doc! { "type_": "ccc" };
    // let bbb = eee.find_one(filter, None).await?;
    // println!("{:?}", bbb);
    // c.find();
    // database.insert(&c.mongo.table_code, a, None).await?;
    Ok(())
}
