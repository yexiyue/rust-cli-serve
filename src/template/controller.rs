use axum::extract::Path;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

use crate::server::Server;
use crate::ServeResult;

use super::Template;

#[derive(Debug, serde::Deserialize)]
pub struct CreateTemplate {
    value: String,
    npm_name: String,
    version: String,
    image: String,
    description: String,
}
pub async fn create(
    server: Server<Template>,
    Json(CreateTemplate {
        value,
        npm_name,
        version,
        image,
        description,
    }): Json<CreateTemplate>,
) -> ServeResult<impl IntoResponse> {
    let template = server
        .create_template(value, npm_name, version, image, description)
        .await?;
    Ok(Json(json!({ "template": template })))
}

pub async fn find(server: Server<Template>) -> ServeResult<impl IntoResponse> {
    let templates = server.find().await?;
    Ok(Json(json!({ "templates": templates })))
}

pub async fn find_by_id(
    server: Server<Template>,
    Path(id): Path<String>,
) -> ServeResult<impl IntoResponse> {
    let template = server.find_by_id(id).await?;
    Ok(Json(json!({ "template": template })))
}
