use anyhow::{Context, Result};
use mongodb::{options::ClientOptions, Client};

struct Mongo {
	database: mongodb::Database,
}

impl Mongo {
	async fn new(url: &String, database: &String) -> Result<Mongo> {
		let mut client_options = ClientOptions::parse(url).await?;
		client_options.app_name = Some("Rust Price Service".to_string());
		let client = Client::with_options(client_options)?;
		Ok(Mongo {
			database: client.database(database),
		})
	}
	async fn collection(&mut self, collection: &String) {}
}
