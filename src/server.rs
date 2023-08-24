use crate::db::MongoDB;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    async_trait
};
use mongodb::Collection;

pub struct Server<T> {
    pub collection: Collection<T>,
}

#[async_trait]
impl<S, T> FromRequestParts<S> for Server<T> {
    type Rejection = StatusCode;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        //获取请求扩展中的数据库句柄
        let mongod = parts.extensions.get::<MongoDB>().unwrap();
        //通过类型名称设置集合
        let type_name = std::any::type_name::<T>();
        let type_name=type_name.split("::").last().unwrap();
        let collection = mongod.db.collection::<T>(type_name);
        Ok(Self { collection })
    }
}
