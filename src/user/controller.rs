use axum::{response::IntoResponse, Json};
use tracing::info;

use crate::{server::Server, user::server::Role};

use super::server::User;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserParams {
    username: String,
    password: String,
    role: usize,
}

pub async fn create_user(server: Server<User>, Json(user): Json<UserParams>) -> impl IntoResponse {
    info!("{:?}", user);
    let res = server
        .create_user(
            user.username,
            user.password,
            match user.role {
                0 => Role::Normal,
                1 => Role::Admin,
                _ => unimplemented!(),
            },
        )
        .await
        .unwrap();
    Json::from(res)
}

pub async fn get_user(server: Server<User>) -> impl IntoResponse {
    let res = server.find_users().await.unwrap();
    Json::from(res)
}
