use mongodb::{options::ClientOptions, Client, Database};
use std::error::Error;
use tracing::info;

#[derive(Debug,Clone)]
pub struct MongoDB {
    pub db: Database,
}

impl MongoDB {
    pub async fn init() -> Result<Self, Box<dyn Error>> {
        let client_options =
            ClientOptions::parse("mongodb://yexiyue:123456@localhost:27017/cli_db").await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("cli_db");
        let name=db.name();
        info!("Connected to MongoDB db is {name}");
        Ok(Self { db })
    }
}
