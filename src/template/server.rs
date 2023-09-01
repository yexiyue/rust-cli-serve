use std::{borrow::BorrowMut, cell::RefCell, rc::Rc, sync::Arc};

use super::Template;
use crate::{
    error::ServerError,
    server::{Server, ServerInit},
    ServeResult,
};
use axum::{async_trait, http::StatusCode};
use chrono::Local;
use futures::{lock::Mutex, TryStreamExt};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::IndexOptions,
    results::InsertOneResult,
    IndexModel,
};
use once_cell::sync::Lazy;
use tracing::error;

static FIRST: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(true)));

#[async_trait]
impl ServerInit for Server<Template> {
    async fn init(&self) -> ServeResult<()> {
        let mut one = FIRST.lock().await;
        let one = one.borrow_mut();
        if **one {
            **one = false;
            self.collection
                .create_index(
                    IndexModel::builder()
                        .keys(doc! {"npm_name": 1})
                        .options(IndexOptions::builder().unique(true).build())
                        .build(),
                    None,
                )
                .await
                .map_err(|_| {
                    ServerError(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to create Template some index".to_string(),
                    )
                })?;

            self.collection
                .create_index(
                    IndexModel::builder()
                        .keys(doc! {"value": 1})
                        .options(IndexOptions::builder().unique(true).build())
                        .build(),
                    None,
                )
                .await
                .map_err(|_| {
                    ServerError(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "failed to create Template some index".to_string(),
                    )
                })?;
            return Ok(());
        }

        Ok(())
    }
}

impl Server<Template> {
    pub async fn create_template(
        &self,
        value: String,
        npm_name: String,
        version: String,
        image: String,
        description: String,
    ) -> ServeResult<InsertOneResult> {
        let res = self
            .collection
            .insert_one(
                Template {
                    id: ObjectId::new(),
                    value,
                    npm_name,
                    version,
                    image,
                    description,
                    create_time: Local::now().to_rfc3339(),
                },
                None,
            )
            .await
            .map_err(|_| {
                ServerError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to insert template".to_string(),
                )
            })?;

        Ok(res)
    }

    pub async fn find(&self) -> ServeResult<Vec<Template>> {
        let cursor = self.collection.find(None, None).await.map_err(|_| {
            ServerError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to find template".to_string(),
            )
        })?;

        let templates = cursor.try_collect::<Vec<Template>>().await.map_err(|e| {
            error!("error {e:#?}");
            ServerError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to collect template".to_string(),
            )
        })?;

        Ok(templates)
    }

    pub async fn find_by_id(&self, id: String) -> ServeResult<Option<Template>> {
        let template = self
            .collection
            .find_one(
                doc! {
                    "_id":id
                },
                None,
            )
            .await
            .map_err(|_| ServerError(StatusCode::INTERNAL_SERVER_ERROR, "".to_string()))?;

        Ok(template)
    }
}
