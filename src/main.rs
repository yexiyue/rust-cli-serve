use axum::Router;
use rust_server::{jwt::Claims, user::server::Role};
use tracing_subscriber;
use tracing::{warn,info};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    rust_server::start().await?;
    Ok(())
}
