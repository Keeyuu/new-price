use mongodb::{options::ClientOptions, Client};
async fn test() -> std::result::Result<mongodb::Client, mongodb::error::Error> {
	let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
	client_options.app_name = Some("Rust App".to_string());
	let client = Client::with_options(client_options)?;
	let db = client.database("mydb");
	
	return Ok(client);
}
