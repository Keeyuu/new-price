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
            let ref last = i[index - 1];
            let ref now = i[index];
            let ref next = i[index + 1];
            //println!("{:?},{:?},{:?}", last, now, next);
            println!("now:{}", index);
            index += 1;
        }
        Ok(())
    }
}

pub struct ReckonNode_2 {}

impl Reckoner for ReckonNode_2 {
    type Output = data::day_result;
    type Input = Vec<data::Day>;
    fn reckon(&self, i: &Self::Input, o: &mut Self::Output) -> Result<()> {
        Ok(())
    }
}
