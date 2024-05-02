use std::time::Duration;

use serde::{de::Error, Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct Config {
    pub session: SessionConfig,
    pub docker: DockerConfig,
}

#[derive(Deserialize)]
pub struct DockerConfig {
    pub host: String,
    pub image_label: String,
    pub name_prefix: String,
    pub languages: Vec<String>,
}

#[derive(Deserialize)]
pub struct SessionConfig {
    #[serde(deserialize_with = "parse_duration")]
    pub expiration: Duration,
    #[serde(deserialize_with = "parse_duration")]
    pub expired_clear_interval: Duration,
}

#[derive(Deserialize)]
pub struct Secrets {
    pub google_auth: GoogleAuth,
}

#[derive(Deserialize)]
pub struct GoogleAuth {
    pub client_id: String,
    pub client_secret: String,
}

fn parse_duration<'de, D>(de: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(de)?;
    Ok(s.trim()
        .split_whitespace()
        .map(|s| {
            let first_digit = s
                .find(|c: char| c.is_ascii_alphabetic())
                .ok_or(D::Error::custom(
                    "No alphabetic character found in duration chunk",
                ))?;
            let (num, unit) = s.split_at(first_digit);
            let num: u64 = num
                .parse()
                .map_err(|_| D::Error::custom("Could not parse number in duration chunk"))?;
            let duration = match unit {
                "s" => Duration::from_secs(num),
                "m" => Duration::from_secs(num * 60),
                "h" => Duration::from_secs(num * 60 * 60),
                "d" => Duration::from_secs(num * 60 * 60 * 24),
                "w" => Duration::from_secs(num * 60 * 60 * 24 * 7),
                "M" => Duration::from_secs(num * 60 * 60 * 24 * 30),
                "y" => Duration::from_secs(num * 60 * 60 * 24 * 365),
                _ => {
                    return Err(D::Error::custom(format!(
                    "Unrecognized unit in duration chunk: {}; Valid units are s, m, h, d, w, M, y",
                    unit
                )))
                }
            };
            Ok(duration)
        })
        .collect::<Result<Vec<Duration>, _>>()?
        .into_iter()
        .sum())
}
