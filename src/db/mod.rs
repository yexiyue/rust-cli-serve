use mongodb::{options::ClientOptions, Client};
use std::error::Error;
use tracing::info;

pub async fn init()->Result<Client,Box<dyn Error>>{
    let client_option=ClientOptions::parse("mongodb://yexiyue:123456@127.0.0.1:27017/cli_db").await.expect("failed to parse");
    let client=Client::with_options(client_option)?;
    info!("connect to mongodb success");
    for name in client.list_database_names(None, None).await?{
        info!("{}",name);
    }
    Ok(client)
}