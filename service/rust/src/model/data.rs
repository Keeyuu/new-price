use crate::config::Config;
use anyhow::{Context, Result};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, results, Collection};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Code {
    pub code: String,
    pub update_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Day {
    pub date: String,
    pub code: String,
    pub ddate: i64,
    #[serde(default = "default_resource_f64")]
    pub open: f64,
    #[serde(default = "default_resource_f64")]
    pub high: f64,
    #[serde(default = "default_resource_f64")]
    pub low: f64,
    #[serde(default = "default_resource_f64")]
    pub close: f64,
    #[serde(default = "default_resource_f64")]
    pub preclose: f64,
    #[serde(default = "default_resource_f64")]
    pub volume: f64,
    #[serde(default = "default_resource_f64")]
    pub amount: f64,
    #[serde(default = "default_resource_f64")]
    pub turn: f64,
    #[serde(default = "default_resource_f64")]
    pub pctChg: f64,
    #[serde(default = "default_resource_f64")]
    pub peTTM: f64,
    #[serde(default = "default_resource_f64")]
    pub pbMRQ: f64,
    pub adjustflag: u8,
    pub tradestatus: u8,
    pub isST: u8,
}

fn default_resource_f64() -> f64 {
    0.0
}

pub async fn get_all_code(config: &Config, database: &mongodb::Database) -> Result<Vec<Code>> {
    let col_code = database.collection::<Code>(&config.mongo.table_code);
    let mut cursor = col_code.find(None, None).await?;
    let mut list: Vec<Code> = Vec::new();
    while let Some(code) = cursor.try_next().await? {
        list.push(code)
    }
    Ok(list)
}

pub async fn get_all_day(
    code: &String,
    config: &Config,
    database: &mongodb::Database,
) -> Result<Vec<Day>> {
    let col_code = database.collection::<Day>(&config.mongo.table_day);
    let mut cursor = col_code.find(doc! {"code":code}, None).await?;
    let mut list: Vec<Day> = Vec::new();
    while let Some(code) = cursor.try_next().await? {
        list.push(code)
    }
    Ok(list)
}
//pub struct Code {
//    pub type_: String,
//    pub data: SourceCode,
//}

//#[derive(Debug, Serialize, Deserialize, Clone)]
//pub struct SourceCode {
//    pub code: String,
//    pub name: String,
//    pub market: i32,
//    pub hsgt: i32,
//    pub bk: String,
//    pub roe: f32,
//    pub zgb: f64,
//    pub ltgb: f64,
//    pub ltsz: f64,
//    pub zsz: f64,
//    pub ssdate: String,
//}

//impl SourceCode {
//    pub fn to_code(self, type_: String) -> Code {
//        Code {
//            type_,
//            data: self.into(),
//        }
//    }
//}

//pub async fn get_stock_code(c: &Collection<Code>) -> Result<Vec<Code>> {
//    get_type_code(c, "stock").await
//}

//pub async fn get_fund_code(c: &Collection<Code>) -> Result<Vec<Code>> {
//    get_type_code(c, "fund").await
//}

//async fn get_type_code(c: &Collection<Code>, s: &str) -> Result<Vec<Code>> {
//    let mut cursor = c
//        .find(doc! {"type_":s}, None)
//        .await
//        .context("get_stock_code err")?;
//    let mut list: Vec<Code> = Vec::new();
//    while let Some(code) = cursor.try_next().await? {
//        list.push(code)
//    }
//    Ok(list)
//}

//pub async fn insert_many(l: Vec<Code>, c: Collection<Code>) -> Result<results::InsertManyResult> {
//    c.insert_many(l, None).await.context("code insert_many err")
//}
//// doc! { "author": "George Orwell" }

//pub async fn insert(i: Code, c: Collection<Code>) -> Result<results::InsertOneResult> {
//    c.insert_one(i, None).await.context("code insert err")
//}

//impl Code {
//    pub fn is_stock(&self) -> bool {
//        self.type_ == "stock".to_string()
//    }
//    pub fn is_fund(&self) -> bool {
//        self.type_ == "fund".to_string()
//    }
//}

//#[derive(Debug, Serialize, Deserialize, Default)]
//pub struct DayMessageRsb {
//    code: i32,
//    message: String,
//    data: Vec<DayMessage>,
//}

//#[derive(Debug, Serialize, Deserialize, Clone)]
//pub struct DayMessage {
//    code: String,
//    name: String,
//    ssdate: String,
//    price: f64,
//    zdfd: f64,
//    zded: f64,
//    cjl: f64,
//    cje: f64,
//    zhfu: f64,
//    hslv: f64,
//    sjlv: f64,
//    dsyl: f64,
//    jsyl: f64,
//    ttmsyl: f64,
//    jjia: f64,
//    lbi: f64,
//    zgj: f64,
//    zdj: f64,
//    jrkpj: f64,
//    zrspj: f64,
//    weibi: f64,
//    wpan: f64,
//    npan: f64,
//    roe: f64,
//    zgb: f64,
//    ltgb: f64,
//    ltsz: f64,
//    zsz: f64,
//    mgsy: f64,
//    zfy: f64,
//    zys: f64,
//    zystb: f64,
//    jzc: f64,
//    jlr: f64,
//    mlil: f64,
//    jlil: f64,
//    fzl: f64,
//    mgwfplr: f64,
//    zljlr: f64,
//    cddlr: f64,
//    cddlc: f64,
//    cddjlr: f64,
//    ddlr: f64,
//    ddlc: f64,
//    ddjlr: f64,
//    zdlr: f64,
//    zdlc: f64,
//    zdjlr: f64,
//    xdlr: f64,
//    xdlc: f64,
//    xdjlr: f64,
//}
