use std::net::SocketAddr;

use axum::{Extension, Router, Server};
use tracing::info;

use crate::user::user_routes;

pub mod db;
pub mod error;
pub mod server;
pub mod template;
pub mod user;

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .merge(user_routes())
        .layer(Extension(db::MongoDB::init().await?));
    let addr = SocketAddr::new("127.0.0.1".parse()?, 3000);

    Server::bind(&addr).serve(app.into_make_service()).await?;
    info!("Server started at {}", &addr);
    Ok(())
}
