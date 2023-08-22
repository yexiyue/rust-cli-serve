use axum::{routing::post, Router};

use self::controller::{create_user, get_user};

pub mod controller;
pub mod server;

pub fn user_routes() -> Router {
    Router::new().route("/user", post(create_user).get(get_user))
}
