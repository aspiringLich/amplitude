use crate::{langs::LangType, runner::exec::GeneratorResult};

use super::*;

pub fn routes() -> Router<AppState> {
    Router::new().route("/gen", post(gen))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExecRequest {
    pub content: String,
    pub language: String,
    pub inputs: Vec<Type>,
    pub output: Type,
    pub hidden_cases: u16,
    pub visible_cases: u16,
    pub generate_cases: u16,
}

async fn gen(
    mut session: Session,
    State(state): State<AppState>,
    Json(req): Json<ExecRequest>,
) -> Result<Json<GeneratorResult>, Error> {
    if req.generate_cases == 0 {
        return Err(bad_request("Skipping generation of 0 cases"))
    }
    if session.get(&state.db).await.is_err() {
        return Err(forbidden("Invalid session"));
    }

    let lang = &req.language;
    let lang_info = state
        .langs
        .iter()
        .find(|l| l.name == *lang)
        .ok_or_else(|| not_found(format!("Unknown language: `{lang}`")))?;

    if lang_info.r#type != LangType::Scripting {
        return Err(bad_request(format!(
            "Language `{lang}` is not a scripting language"
        )));
    }

    let res = state.runner_registry[lang]
        .run_generator(&state.templates, &state.docker, &req)
        .await
        .map_err(internal)?;
    Ok(Json(res))
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Int,
    Float,
    String,
    // #[serde(untagged)]
    // Array(Box<Type>, Option<u16>),
    // TODO: array support
    // map support?
}
