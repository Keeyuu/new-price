use crate::config::Config;
use crate::model::data;
use anyhow::{Context, Result};
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
    pub fn append_reckoner(
        &mut self,
        item: Box<dyn Reckoner<Input = Vec<data::Day>, Output = data::day_result>>,
    ) {
        self.reckoners.push(item);
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
pub struct ReckonNode_1 {}

impl Reckoner for ReckonNode_1 {
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

impl ReckonNode_1 {
    fn filter(&self, o: &mut data::day_result) {}
}

pub struct ReckonNode_2 {}

impl Reckoner for ReckonNode_2 {
    type Output = data::day_result;
    type Input = Vec<data::Day>;
    fn reckon(&self, i: &Self::Input, o: &mut Self::Output) -> Result<()> {
        Ok(())
    }
}
