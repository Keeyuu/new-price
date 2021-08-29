use crate::config::Config;
use crate::dao::Mongo;
use crate::model::{Code, SourceCode};
use anyhow::{Context, Result};
use std::fs::OpenOptions;
use std::io::Read;

pub async fn import_code(c: &Config, d: &Mongo) -> Result<()> {
	let mut fd_code = OpenOptions::new()
		.read(true)
		.open("code.json")
		.context("1")?;
	let mut fd_fund = OpenOptions::new()
		.read(true)
		.open("fund.json")
		.context("2")?;
	let mut buf_code = String::new();
	let mut buf_fund = String::new();
	fd_code.read_to_string(&mut buf_code).context("3")?;
	fd_fund.read_to_string(&mut buf_fund).context("4")?;
	let a = serde_json::from_str::<Vec<SourceCode>>(&buf_code).context("5")?;
	let b = serde_json::from_str::<Vec<SourceCode>>(&buf_fund).context("6")?;
	// !导数据前注意清库
	import(c, d, &a, "stock".to_string()).await?;
	import(c, d, &b, "fund".to_string()).await?;
	println!("{}", "data import ok");
	Ok(())
}

async fn import(c: &Config, d: &Mongo, list: &[SourceCode], type_: String) -> Result<()> {
	let col_code = d.collection::<Code>(&c.mongo.table_code).await;
	for i in list.into_iter() {
		col_code
			.insert_one(i.clone().to_code(type_.clone()), None)
			.await
			.context("import 1")?;
	}
	Ok(())
}
