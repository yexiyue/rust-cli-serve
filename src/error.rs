use axum::{response::{IntoResponse, Response}, http::StatusCode};

pub enum ServerError{
    UserServerError
}

impl IntoResponse for ServerError{
    fn into_response(self) ->Response {
        match self{
            Self::UserServerError=>{
                (StatusCode::INTERNAL_SERVER_ERROR,"unhandled server error").into_response()
            }
        }
    }
}