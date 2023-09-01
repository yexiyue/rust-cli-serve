use super::server::User;
use crate::{server::Server, user::server::Role, ServeResult};
use axum::{response::IntoResponse, Json};
use bcrypt::hash;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct UserParams {
    username: String,
    password: String,
    role: usize,
}

pub async fn create_user(
    server: Server<User>,
    Json(user): Json<UserParams>,
) -> ServeResult<impl IntoResponse> {
    let password = user.password;
    // Hash password
    let password = hash(password, bcrypt::DEFAULT_COST).unwrap();
    let res = server
        .create_user(
            user.username,
            password,
            match user.role {
                0 => Role::Normal,
                1 => Role::Admin,
                _ => unimplemented!(),
            },
        )
        .await?;
    Ok(Json::from(res))
}

pub async fn get_user(server: Server<User>) -> impl IntoResponse {
    let res = server.find_users().await.unwrap();
    Json::from(res)
}

pub async fn login(server: Server<User>, Json(user): Json<UserParams>) -> impl IntoResponse {}
