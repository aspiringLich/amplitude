use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {}

#[derive(Deserialize)]
pub struct Secrets {
    pub google_auth: GoogleAuth,
}

#[derive(Deserialize)]
pub struct GoogleAuth {
    pub client_id: String,
    pub client_secret: String,
}