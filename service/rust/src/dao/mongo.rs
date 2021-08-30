use crate::model::Code;
use anyhow::{Context, Result};
use mongodb::{options, options::ClientOptions, Client};
use std::borrow::Borrow;
pub struct Mongo {
	database: mongodb::Database,
}

impl Mongo {
	pub async fn new(url: &String, database: &String) -> Result<Mongo> {
		let mut client_options = ClientOptions::parse(url)
			.await
			.context("Mongo new parse err")?;
		client_options.app_name = Some("Rust Price Service".to_string());
		let client = Client::with_options(client_options).context("Mongo get client err")?;
		Ok(Mongo {
			database: client.database(database),
		})
	}
	pub async fn collection<T>(&self, name: &String) -> mongodb::Collection<T> {
		self.database.collection::<T>(name)
	}
	pub async fn insert<T: serde::Serialize>(
		&self,
		name: &String,
		doc: impl Borrow<T>,
		options: impl Into<Option<options::InsertOneOptions>>,
	) -> Result<()> {
		let a = self.collection::<T>(name).await;
		a.insert_one(doc, options).await?;
		Ok(())
	}
}


