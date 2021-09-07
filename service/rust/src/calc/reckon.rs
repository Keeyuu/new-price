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
}

impl Calc {
    pub fn new(data: Vec<data::Day>, table: mongodb::Collection<data::day_result>) -> Self {
        Calc {
            data,
            reckoners: Vec::new(),
            result: data::day_result::new(),
            table,
        }
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
