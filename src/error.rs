use std::{
    convert::Infallible,
    fmt::{self, Debug, Display, Formatter},
    ops::{ControlFlow, FromResidual, Try},
};

use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub trait IntoStatusResult {
    type Output;

    fn status_result(self) -> StatusResult<Self::Output>;
}

impl<T: IntoResponse> IntoStatusResult for T {
    type Output = Response<Body>;

    fn status_result(self) -> StatusResult<Self::Output> {
        let res = self.into_response();
        let status = res.status();
        if status.is_informational() || status.is_success() {
            StatusResult::Ok(res)
        } else {
            StatusResult::Err(StatusError(res))
        }
    }
}

impl<T: IntoResponse> IntoResponse for StatusResult<T> {
    fn into_response(self) -> Response<Body> {
        match self {
            StatusResult::Ok(response) => response.into_response(),
            StatusResult::Err(err) => err.0,
        }
    }
}

pub enum StatusResult<T> {
    Ok(T),
    Err(StatusError),
}

impl<T: Debug> Debug for StatusResult<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StatusResult::Ok(value) => write!(f, "Ok({:?})", value),
            StatusResult::Err(err) => write!(f, "Err({:?})", err),
        }
    }
}

#[derive(Debug)]
pub struct StatusError(Response<Body>);

pub trait StatusContext<T> {
    fn err_context<D>(self, status: StatusCode, context: D) -> StatusResult<T>
    where
        D: Display + Send + Sync + 'static;

    fn err_status(self, status: StatusCode) -> Result<T, StatusError>;
}

impl<T, E> StatusContext<T> for Result<T, E>
where
    E: Display,
{
    fn err_context<D>(self, status: StatusCode, context: D) -> StatusResult<T>
    where
        D: Display + Send + Sync + 'static,
    {
        match self {
            Ok(value) => StatusResult::Ok(value),
            Err(error) => StatusResult::Err(StatusError((
                status,
                format!("{context}:\n{error}"),
            ).into_response())),
        }
    }

    fn err_status(self, status: StatusCode) -> Result<T, StatusError> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(StatusError((status, error.to_string()).into_response())),
        }
    }
}

impl<T> Try for StatusResult<T> {
    type Output = T;
    type Residual = StatusResult<Infallible>;

    fn from_output(output: Self::Output) -> Self {
        StatusResult::Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            StatusResult::Ok(output) => ControlFlow::Continue(output),
            StatusResult::Err(err) => ControlFlow::Break(StatusResult::Err(err)),
        }
    }
}

impl<T> FromResidual<StatusResult<Infallible>> for StatusResult<T> {
    fn from_residual(residual: StatusResult<Infallible>) -> Self {
        match residual {
            StatusResult::Ok(_) => unreachable!(),
            StatusResult::Err(err) => StatusResult::Err(err),
        }
    }
}
