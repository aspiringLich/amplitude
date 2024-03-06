use crate::{app::AppState, views::session::Session};

use super::{internal, Error};
use axum::{
    http::{header::CONTENT_TYPE, StatusCode},
    response::{AppendHeaders, IntoResponse, Response},
};
use entity::user;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserAvatar {
    pub name: String,
    pub avatar_url: Option<String>,
}

impl UserAvatar {
    pub fn new(user: &user::Model) -> Self {
        Self {
            name: user.name.clone(),
            avatar_url: user.avatar_url.clone(),
        }
    }
}

pub async fn login(
    state: &AppState,
    session: Session,
    user: &user::Model,
    code: StatusCode,
) -> Result<Response, Error> {
    Ok((
        code,
        session.get_or_add(state, user.user_id).await?,
        AppendHeaders([(CONTENT_TYPE, "application/json")]),
        serde_json::to_string(&UserAvatar::new(user)).map_err(internal)?,
    )
        .into_response())
}
