use crate::routes::Session;

use super::{internal, Error};
use axum::{
    http::{header::CONTENT_TYPE, StatusCode},
    response::{AppendHeaders, IntoResponse, Response},
};
use entity::user;
use sea_orm::DatabaseConnection;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserAvatar<'a> {
    pub name: &'a str,
    pub avatar_url: Option<&'a str>,
}

pub async fn login(
    db: &DatabaseConnection,
    session: Session,
    user: &user::Model,
    code: StatusCode,
) -> Result<Response, Error> {
    Ok((
        code,
        session.get_or_add(db, user.user_id).await?,
        AppendHeaders([(CONTENT_TYPE, "application/json")]),
        serde_json::to_string(&UserAvatar {
            name: &user.name,
            avatar_url: user.avatar_url.as_ref().map(String::as_str),
        })
        .map_err(internal)?,
    )
        .into_response())
}
