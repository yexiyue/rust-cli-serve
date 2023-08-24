use std::fmt::Display;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, DateTime},
    results::InsertOneResult,
};
use serde::{Deserialize, Serialize};

use crate::server::Server;

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Normal,
}
impl Display for Role{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin=>write!(f,"Admin"),
            Role::Normal=>write!(f,"Normal")
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
    create_time: DateTime,
    role: Role,
}

impl Server<User> {
    pub async fn create_user(
        &self,
        username: String,
        password: String,
        role: Role,
    ) -> Result<InsertOneResult, Box<dyn std::error::Error>> {
        Ok(self
            .collection
            .insert_one(
                User {
                    username,
                    password,
                    role,
                    create_time: DateTime::now(),
                },
                None,
            )
            .await?)
    }

    pub async fn find_users(&self) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut result: Vec<User> = Vec::new();
        while let Some(user) = cursor.try_next().await? {
            result.push(user);
        }
        Ok(result)
    }
}
