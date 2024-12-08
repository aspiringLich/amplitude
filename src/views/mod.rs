use axum::{http::StatusCode, response::IntoResponse};
use std::fmt::Display;

pub mod auth;
pub mod session;

macro response($status:ident, $res:ident) {
    /// Return a response with the status code
    #[allow(dead_code)]
    pub fn $res(res: impl std::fmt::Debug + Display + 'static + Sync + Send) -> Error {
        Error {
            status: StatusCode::$status,
            report: Some(eyre::eyre!(res)),
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
    pub report: Option<eyre::Error>,
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
            report: Some(eyre::eyre!(value)),
        };
    }
}

impl From<eyre::Report> for Error {
    fn from(value: eyre::Report) -> Self {
        return Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            report: Some(value),
        };
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(report) = &self.report {
            write!(f, "{:?}", report)
        } else {
            Ok(())
        }
    }
}
