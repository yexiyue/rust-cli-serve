use std::{borrow::BorrowMut, fmt::Display, sync::Arc};

use axum::{async_trait, http::StatusCode, response::IntoResponse};
use chrono::Local;
use futures::lock::Mutex;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    options::{FindOptions, IndexOptions},
    results::InsertOneResult,
    IndexModel,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    error::ServerError,
    server::{Server, ServerInit},
    ServeResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Normal,
}
impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::Normal => write!(f, "Normal"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string",
        rename = "_id"
    )]
    id: ObjectId,
    username: String,
    password: String,
    create_time: String,
    role: Role,
}

static FIRST: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(true)));

#[async_trait]
impl ServerInit for Server<User> {
    async fn init(&self) -> ServeResult<()> {
        let mut first = FIRST.lock().await;
        let first = first.borrow_mut();
        if **first {
            **first = false;
            let res = self
                .collection
                .create_index(
                    IndexModel::builder()
                        .keys(doc! {"username":1})
                        .options(IndexOptions::builder().unique(true).build())
                        .build(),
                    None,
                )
                .await
                .map_err(|err| {
                    info!("{err:?}");
                    ServerError(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to create index".to_string(),
                    )
                })?;
            return Ok(());
        }
        Ok(())
    }
}

impl Server<User> {
    pub async fn create_user(
        &self,
        username: String,
        password: String,
        role: Role,
    ) -> ServeResult<InsertOneResult> {
        Ok(self
            .collection
            .insert_one(
                User {
                    id: ObjectId::new(),
                    username,
                    password,
                    role,
                    create_time: Local::now().to_rfc3339(),
                },
                None,
            )
            .await
            .map_err(|_| {
                ServerError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to insert user".to_string(),
                )
            })?)
    }

    pub async fn find_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let cursor = self.collection.find(None, None).await?;
        let users = cursor.try_collect::<Vec<User>>().await?;
        Ok(users)
    }

    pub async fn find_user_by_username(&self, username: String) -> ServeResult<Option<User>> {
        let user = self
            .collection
            .find_one(doc! {"username":username}, None)
            .await
            .map_err(|_| ServerError(StatusCode::NOT_FOUND, "Not Found User".to_string()))?;
        Ok(user)
    }
}
