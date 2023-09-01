use crate::{db::MongoDB, error::ServerError, ServeResult};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use mongodb::{bson::doc, options::IndexOptions, Collection, IndexModel};
use tracing::info;

pub struct Server<T> {
    pub collection: Collection<T>,
}

#[async_trait]
pub trait ServerInit: Send + Sync {
    async fn init(&self) -> ServeResult<()>;
}

#[async_trait]
impl<S, T> FromRequestParts<S> for Server<T>
where
    Server<T>: ServerInit,
    T: Send,
{
    type Rejection = ServerError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        //获取请求扩展中的数据库句柄
        let mongod = parts.extensions.get::<MongoDB>().unwrap();
        //通过类型名称设置集合
        let type_name = std::any::type_name::<T>();
        let type_name = type_name.split("::").last().unwrap();
        let collection = mongod.db.collection::<T>(type_name);
        let server = Self { collection };
        server.init().await?;
        Ok(server)
    }
}
