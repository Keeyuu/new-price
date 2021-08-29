use crate::model::Code;
use anyhow::{Context, Result};
use mongodb::{options::ClientOptions, Client};
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
	pub async fn collection(&self, name: &String) -> mongodb::Collection<Code> {
		self.database.collection::<Code>(name)
	}
}
