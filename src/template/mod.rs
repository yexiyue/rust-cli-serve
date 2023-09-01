use axum::{
    routing::{get, post},
    Router,
};
use mongodb::{bson::oid::ObjectId, bson::DateTime};
use serde::{Deserialize, Serialize};
pub mod controller;
pub mod server;
use bson::serde_helpers::deserialize_rfc3339_string_from_bson_datetime;
#[derive(Debug, Deserialize, Serialize)]
pub struct Template {
    #[serde(
        rename = "_id",
        serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string"
    )]
    id: ObjectId,
    value: String,
    npm_name: String,
    version: String,
    image: String,
    description: String,
    create_time: String,
}

pub fn template_router() -> Router {
    Router::new().nest(
        "/template",
        Router::new()
            .route("/", post(controller::create))
            .route("/list", get(controller::find))
            .route("/list/:id", get(controller::find_by_id)),
    )
}
