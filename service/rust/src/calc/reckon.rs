use crate::model::data;
use anyhow::Result;
//-----------------------------------------------------------------------
pub struct Calc {
    data: Vec<data::Day>,
    reckoners: Vec<Box<dyn Reckoner<Input = Vec<data::Day>, Output = data::day_result>>>,
    result: Option<data::day_result>,
    table: mongodb::Collection<data::day_result>,
}

impl Calc {
    pub fn new(data: Vec<data::Day>, table: mongodb::Collection<data::day_result>) -> Self {
        Calc {
            data,
            reckoners: Vec::new(),
            result: None,
            table,
        }
    }
    pub fn append_reckoner(
        &mut self,
        item: Box<dyn Reckoner<Input = Vec<data::Day>, Output = data::day_result>>,
    ) {
        self.reckoners.push(item);
    }
    pub async fn save_result(mut self) -> Result<()> {
        // ! fnOnce be careful
        match self.result {
            None => {
                let mut tmp: Option<data::day_result> = None;
                for item in self.reckoners.pop() {
                    tmp = Some(item.reckon(&self.data));
                }
                if let Some(data) = tmp {
                    data::upsert_day_result(data, self.table).await?
                }
                println!("{}", "err no reckoners")
            }
            Some(res) => data::upsert_day_result(res, self.table).await?,
        }
        Ok(())
    }
}
//-----------------------------------------------------------------------
pub trait Reckoner {
    type Output;
    type Input;
    fn reckon(&self, i: &Self::Input) -> Self::Output;
}

pub struct ReckonInitNode {}

impl Reckoner for ReckonInitNode {
    type Output = data::day_result;
    type Input = Vec<data::Day>;
    fn reckon(&self, i: &Self::Input) -> Self::Output {
        Self::Output::new()
    }
}

//-----------------------------------------------------------------------
