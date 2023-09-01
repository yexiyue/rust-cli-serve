use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{request::Parts, Request, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{error::ServerError, ServeResult, server::Server, user::server::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    username: String,
    role: String,
    exp: usize,
}

static TOKEN_KEY: &str = "yexiyue666";

impl Claims {
    pub fn new(username: String, role: String) -> Self {
        Self {
            username,
            role,
            // 默认验证过期时间
            exp: 2000000000, //required
        }
    }

    pub fn encode(&self) -> ServeResult<String> {
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(TOKEN_KEY.as_ref()),
        ).map_err(|_| ServerError(StatusCode::INTERNAL_SERVER_ERROR, "Failed to encode token".to_string()))?;
        Ok(token)
    }

    pub fn decode(token: &str) -> ServeResult<Self> {
        let claims = decode::<Self>(
            token,
            &DecodingKey::from_secret(TOKEN_KEY.as_ref()),
            &Validation::default(),
        ).map_err(|_| ServerError(StatusCode::INTERNAL_SERVER_ERROR, "Failed to decode token".to_string()))?;
        Ok(claims.claims)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims {
    type Rejection = ServerError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(token)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .unwrap();
        let token = token.token();
        Ok(Self::decode(token)?)
    }
}
const WHITE_LIST: [&str; 2] = ["/user/login", "/template/list"];

pub async fn authorization_middleware<B>(
    user:Server<User>,
    req: Request<B>,
    next: Next<B>,
) -> ServeResult<Response>{
    for &i in WHITE_LIST.iter() {
        if req.uri().path().starts_with(i) {
            return Ok(next.run(req).await);
        }
    }
    let token = req.headers().typed_get::<Authorization<Bearer>>();
    if let Some(Authorization(bearer)) = token {
        let token = bearer.token();
        let claims=Claims::decode(token)?;
        let res=user.find_user_by_username(claims.username).await?;
        if res.is_none(){
            return Err(ServerError(StatusCode::UNAUTHORIZED,"用户不存在".to_string()));
        }
    } else {
        return Err(ServerError(StatusCode::UNAUTHORIZED,"未经许可".to_string()));
    };

    Ok(next.run(req).await)
}
