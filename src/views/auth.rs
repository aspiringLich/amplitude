use crate::routes::Session;

use super::Error;
use axum::Json;
use entity::user;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserAvatar {
    pub name: String,
    pub avatar_url: Option<String>,
}

pub fn user_avatar(user: user::Model) -> Json<UserAvatar> {
    Json(UserAvatar {
        name: user.name,
        avatar_url: user.avatar_url,
    })
}
