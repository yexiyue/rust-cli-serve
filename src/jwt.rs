use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{request::Parts, Request, StatusCode},
    middleware::Next,
    response::Response,
    response::IntoResponse,
    RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::info;

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

    pub fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let token = encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(TOKEN_KEY.as_ref()),
        )?;
        Ok(token)
    }

    pub fn decode(token: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let claims = decode::<Self>(
            token,
            &DecodingKey::from_secret(TOKEN_KEY.as_ref()),
            &Validation::default(),
        )?;
        Ok(claims.claims)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims {
    type Rejection = StatusCode;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(token)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .unwrap();
        let token = token.token();
        Ok(Self::decode(token).unwrap())
    }
}
const WHITE_LIST: [&str; 2] = ["/user/login", "/template"];

pub async fn authorization_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    for &i in WHITE_LIST.iter() {
        if req.uri().path().starts_with(i) {
            return Ok(next.run(req).await);
        }
    }
    let token = req.headers().typed_get::<Authorization<Bearer>>();
    if let Some(Authorization(bearer)) = token {
        let token = bearer.token();
        info!("authorization_middleware {token:?}");
    } else {
        return Ok((StatusCode::UNAUTHORIZED, "Unauthorized Token").into_response());
    };

    Ok(next.run(req).await)
}
