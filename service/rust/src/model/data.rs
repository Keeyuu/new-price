use anyhow::{Context, Result};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, results, Collection};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    pub type_: String,
    pub data: SourceCode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SourceCode {
    pub code: String,
    pub name: String,
    pub market: i32,
    pub hsgt: i32,
    pub bk: String,
    pub roe: f32,
    pub zgb: f64,
    pub ltgb: f64,
    pub ltsz: f64,
    pub zsz: f64,
    pub ssdate: String,
}

impl SourceCode {
    pub fn to_code(self, type_: String) -> Code {
        Code {
            type_,
            data: self.into(),
        }
    }
}

pub async fn get_stock_code(c: &Collection<Code>) -> Result<Vec<Code>> {
    get_type_code(c, "stock").await
}

pub async fn get_fund_code(c: &Collection<Code>) -> Result<Vec<Code>> {
    get_type_code(c, "fund").await
}

async fn get_type_code(c: &Collection<Code>, s: &str) -> Result<Vec<Code>> {
    let mut cursor = c
        .find(doc! {"type_":s}, None)
        .await
        .context("get_stock_code err")?;
    let mut list: Vec<Code> = Vec::new();
    while let Some(code) = cursor.try_next().await? {
        list.push(code)
    }
    Ok(list)
}

pub async fn insert_many(l: Vec<Code>, c: Collection<Code>) -> Result<results::InsertManyResult> {
    c.insert_many(l, None).await.context("code insert_many err")
}
// doc! { "author": "George Orwell" }

pub async fn insert(i: Code, c: Collection<Code>) -> Result<results::InsertOneResult> {
    c.insert_one(i, None).await.context("code insert err")
}

impl Code {
    pub fn is_stock(&self) -> bool {
        self.type_ == "stock".to_string()
    }
    pub fn is_fund(&self) -> bool {
        self.type_ == "fund".to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DayMessageRsb {
    code: i32,
    message: String,
    data: Vec<DayMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DayMessage {
    code: String,
    name: String,
    ssdate: String,
    price: f64,
    zdfd: f64,
    zded: f64,
    cjl: f64,
    cje: f64,
    zhfu: f64,
    hslv: f64,
    sjlv: f64,
    dsyl: f64,
    jsyl: f64,
    ttmsyl: f64,
    jjia: f64,
    lbi: f64,
    zgj: f64,
    zdj: f64,
    jrkpj: f64,
    zrspj: f64,
    weibi: f64,
    wpan: f64,
    npan: f64,
    roe: f64,
    zgb: f64,
    ltgb: f64,
    ltsz: f64,
    zsz: f64,
    mgsy: f64,
    zfy: f64,
    zys: f64,
    zystb: f64,
    jzc: f64,
    jlr: f64,
    mlil: f64,
    jlil: f64,
    fzl: f64,
    mgwfplr: f64,
    zljlr: f64,
    cddlr: f64,
    cddlc: f64,
    cddjlr: f64,
    ddlr: f64,
    ddlc: f64,
    ddjlr: f64,
    zdlr: f64,
    zdlc: f64,
    zdjlr: f64,
    xdlr: f64,
    xdlc: f64,
    xdjlr: f64,
}
