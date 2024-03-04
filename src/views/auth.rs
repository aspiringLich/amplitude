use super::{json, Result};
use entity::user;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserAvatar {
    pub name: String,
    pub avatar_url: Option<String>,
}

pub fn user_avatar(user: user::Model) -> Result {
    json(UserAvatar {
        name: user.name,
        avatar_url: user.avatar_url,
    })
}
