mod config;
mod dao;
mod model;
mod service;
use anyhow::{Context, Result};
use config::Config;
use mongodb::bson::doc;
use tokio;
mod import;
const TABLE_SIZE: u8 = 10;
fn main() -> Result<()> {
    let config = Config::get().context("main 1")?;
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(start(config))
}

async fn start(c: config::Config) -> Result<()> {
    let database = dao::Mongo::new(&c.mongo.url, &c.mongo.database).await?;

    let col_code = database
        .collection::<model::Code>(&c.mongo.table_code)
        .await;
    let a = model::data::get_fund_code(&col_code).await?;
    let b = model::data::get_stock_code(&col_code).await?;
    println!("{:?}", a);
    println!("{:?}", b);
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

async fn pull() -> Result<()> {
    let by = dao::request::easy_get("http://api.waizaowang.com/doc/getStockHSADailyMarket?code=000001&startDate=2021-08-19&endDate=2021-08-19&fields=name,code,price,zdfd,zded,cjl,cje,zhfu,hslv,sjlv,ssdate,dsyl,jsyl,ttmsyl,ztj,dtj,jjia,lbi,zgj,zdj,jrkpj,zrspj,weibi,wpan,npan,roe,zgb,ltgb,ltsz,zsz,mgsy,zf05,zf10,zf20,zf60,zfy,zys,zystb,jzc,jlr,mlil,jlil,fzl,mgwfplr,mgjzc,mggjj,zljlr,cddlr,cddlc,cddjlr,ddlr,ddlc,ddjlr,zdlr,zdlc,zdjlr,xdlr,xdlc,xdjlr&export=1&token=febb869f0979d084c4a8d17ce45ea866").await?;
    let a = serde_json::from_slice::<model::data::DayMessageRsb>(&by)?;
    println!("{:?}", a);
    Ok(())
}
