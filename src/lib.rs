use crate::{jwt::authorization_middleware, user::user_routes};
use axum::{middleware, Extension, Router, Server};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing::info;
pub mod db;
pub mod error;
pub mod jwt;
pub mod server;
pub mod template;
pub mod user;

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .merge(user_routes())
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn(authorization_middleware))
        .layer(Extension(db::MongoDB::init().await?));
    let addr = SocketAddr::new("127.0.0.1".parse()?, 3000);

    Server::bind(&addr).serve(app.into_make_service()).await?;
    info!("Server started at {}", &addr);
    Ok(())
}
