use axum::{http::StatusCode, response::IntoResponse};
use std::fmt::Display;

pub mod auth;
pub mod session;

macro response($status:ident, $res:ident) {
    /// Return a response with the status code
    #[allow(dead_code)]
    pub fn $res(res: impl Display + 'static) -> Error {
        Error {
            status: StatusCode::$status,
            message: Some(Box::new(res)),
            ..Default::default()
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

#[derive(Default)]
pub struct Error {
    pub status: StatusCode,
    pub message: Option<Box<dyn Display>>,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.to_string()).into_response()
    }
}

impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        return Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: Some(Box::new(value)),
        };
    }
}

impl From<eyre::Report> for Error {
    fn from(value: eyre::Report) -> Self {
        return Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: Some(Box::new(value)),
        };
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.message
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or_default()
        )
    }
}
