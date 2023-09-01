use crate::template::template_router;
use crate::{jwt::authorization_middleware, user::user_routes};
use axum::{middleware, routing::get_service, Extension, Router, Server};
use error::ServerError;
use once_cell::sync::Lazy;
use std::net::SocketAddr;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::CorsLayer;
use tracing::info;
pub mod db;
pub mod error;
pub mod jwt;
pub mod server;
pub mod template;
pub mod upload;
pub mod user;
pub type ServeResult<T> = Result<T, ServerError>;

static ADDR: Lazy<SocketAddr> = Lazy::new(|| SocketAddr::new("127.0.0.1".parse().unwrap(), 3000));
pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .merge(user_routes())
        .merge(template_router())
        .merge(upload::upload_router())
        .layer(CorsLayer::permissive())
        .layer(middleware::from_fn(authorization_middleware))
        .layer(CatchPanicLayer::new())
        .layer(Extension(db::MongoDB::init().await?))
        .fallback_service(Router::new().nest_service(
            "/",
            get_service(tower_http::services::ServeDir::new("public")),
        ));

    info!("Server started at {:?}", &ADDR.to_string());
    Server::bind(&ADDR).serve(app.into_make_service()).await?;
    Ok(())
}
