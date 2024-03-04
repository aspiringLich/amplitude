use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use std::fmt::Display;

pub mod auth;

pub type Result = std::result::Result<Response<Body>, Error>;

macro response($status:ident, $res:ident) {
    /// Return a response with the status code
    pub fn $res(res: impl Display + 'static) -> Error {
        Error {
            status: StatusCode::$status,
            message: Box::new(res),
        }
    }
}

response!(OK, ok);
response!(CREATED, created);
response!(BAD_REQUEST, bad_request);
response!(UNAUTHORIZED, unauthorized);
response!(FORBIDDEN, forbidden);
response!(NOT_FOUND, not_found);
response!(INTERNAL_SERVER_ERROR, internal);

pub fn json<T: serde::Serialize>(data: T) -> Result {
    match serde_json::to_string(&data) {
        Ok(json) => Err(Error {
            status: StatusCode::OK,
            message: Box::new(json),
        }),
        Err(e) => Err(internal(e)),
    }
}

pub fn json_created<T: serde::Serialize>(data: T) -> Result {
    match serde_json::to_string(&data) {
        Ok(json) => Err(Error {
            status: StatusCode::CREATED,
            message: Box::new(json),
        }),
        Err(e) => Err(internal(e)),
    }
}

pub struct Error {
    pub status: StatusCode,
    pub message: Box<dyn Display>,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message.to_string()).into_response()
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        return Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: Box::new(value),
        };
    }
}

impl From<eyre::Report> for Error {
    fn from(value: eyre::Report) -> Self {
        return Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: Box::new(value),
        };
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
