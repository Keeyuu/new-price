use crate::config::Config;
use crate::model::data;
use anyhow::{Context, Result};
use chrono::prelude::*;
use chrono::Duration;
//-----------------------------------------------------------------------
pub trait Reckoner {
    type Output;
    type Input;
    fn reckon(&self, i: &Self::Input, o: &mut Self::Output) -> Result<()>;
}

//-----------------------------------------------------------------------
pub struct Calc {
    data: Vec<data::Day>,
    reckoners: Vec<Box<dyn Reckoner<Input = Vec<data::Day>, Output = data::day_result>>>,
    result: data::day_result,
    table: mongodb::Collection<data::day_result>,
    code: data::Code,
}

impl Calc {
    pub async fn new(
        code: data::Code,
        config: &Config,
        database: &mongodb::Database,
    ) -> Result<Self> {
        let data = code.get_all_day(config, database).await?;
        let table = database.collection::<data::day_result>(&config.mongo.table_day_result);
        Ok(Calc {
            data,
            reckoners: Vec::new(),
            result: data::day_result::new(),
            table,
            code: code,
        })
    }
    // ! first in last out so assure one in last
    pub fn append_reckoner(
        mut self,
        item: Box<dyn Reckoner<Input = Vec<data::Day>, Output = data::day_result>>,
    ) -> Self {
        self.reckoners.push(item);
        self
    }
    pub async fn save_result(&mut self) -> Result<()> {
        for item in self.reckoners.iter() {
            item.reckon(&self.data, &mut self.result)?;
        }
        self.result.code = self.code.code.clone();
        self.result.ddate = self.code.update_at.clone();
        println!("{:?}", &self.result.nodes.len());
        data::upsert_day_result(&self.result, &self.table).await?;
        Ok(())
    }
}
//-----------------------------------------------------------------------
pub struct ReckonNode_one {}

impl Reckoner for ReckonNode_one {
    type Output = data::day_result;
    type Input = Vec<data::Day>;
    fn reckon(&self, i: &Self::Input, o: &mut Self::Output) -> Result<()> {
        let mut index: usize = 1;
        while index < i.len() - 2 {
            let ref last = i[index - 1].close;
            let ref now = i[index].close;
            let ref next = i[index + 1].close;
            let mut node = data::node {
                point: *now,
                ddate: i[index].ddate,
                type_: String::from(data::PointL),
            };
            if now < last && now < next {
                let list = o.nodes.get_mut(&String::from("one"));
                match list {
                    None => {
                        let mut list = Vec::new();
                        list.push(node);
                        o.nodes.insert(String::from("one"), list);
                    }
                    Some(list) => {
                        list.push(node);
                    }
                }
            } else if now > last && now > next {
                node.type_ = String::from(data::PointT);
                let list = o.nodes.get_mut(&String::from("one"));
                match list {
                    None => {
                        let mut list = Vec::new();
                        list.push(node);
                        o.nodes.insert(String::from("one"), list);
                    }
                    Some(list) => {
                        list.push(node);
                    }
                }
            }
            index += 1;
        }
        self.filter(o);
        Ok(())
    }
}

impl ReckonNode_one {
    fn filter(&self, o: &mut data::day_result) {
        let node_one = o.nodes.get_mut(&String::from("one"));
        match node_one {
            None => return,
            Some(list) => {
                if list.len() <= 3 {
                    return;
                }
                let top = String::from(data::PointT);
                let low = String::from(data::PointL);
                let mut valids: Vec<data::node> = Vec::new();
                let mut cache: Vec<data::node> = Vec::new();
                let mut index = 1;
                let mut last = String::new();
                while index < list.len() - 1 {
                    index += 1;
                }
            }
        }
    }
}

pub struct ReckonNode_two {}

impl Reckoner for ReckonNode_two {
    type Output = data::day_result;
    type Input = Vec<data::Day>;
    fn reckon(&self, i: &Self::Input, o: &mut Self::Output) -> Result<()> {
        Ok(())
    }
}

//-----------------------------------------------------------------------

pub fn interval_day(start: i64, end: i64) -> u32 {
    let (y, m, d) = parse_date(start);
    let (y_, m_, d_) = parse_date(end);
    let mut interval: u32 = 0;
    let mut new_start = Utc.ymd(y, m, d) + Duration::days(1);
    let new_end = Utc.ymd(y_, m_, d_);
    while new_start < new_end {
        if valid_day(new_start) {
            interval += 1;
        }
        new_start = new_start + Duration::days(1);
    }
    interval
}

fn parse_date(i: i64) -> (i32, u32, u32) {
    let y = i / 10000;
    let m = i / 100;
    let m = m - y * 100;
    let d = i % 100;
    (y as i32, m as u32, d as u32)
}

fn valid_day(i: Date<Utc>) -> bool {
    match i.weekday() {
        Weekday::Sat | Weekday::Sun => false,
        _ => true,
    }
}
//-----------------------------------------------------------------------
#[test]
fn test_parse_data() {
    let date = parse_date(20211231);
    assert_eq!(date, (2021, 12, 31))
}
//fn test_valid_day() {
//    assert_eq!(false, valid_day(20210905));
//    assert_eq!(true, valid_day(20210906))
//}

#[test]
fn test_interval_day() {
    assert_eq!(22, interval_day(20210801, 20210901))
}
//-----------------------------------------------------------------------
