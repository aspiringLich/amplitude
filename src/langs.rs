use std::{
    fs,
    path::{Path, PathBuf},
};

use eyre::ensure;
use serde::Deserialize;

#[derive(Debug)]
pub struct Languages {
    langs: Vec<LangInfo>,
}

impl Languages {
    pub fn new() -> eyre::Result<Self> {
        Ok(Self {
            langs: fs::read_dir("languages")?
                .filter_map(|entry| -> Option<LangInfo> {
                    let entry = entry.ok()?;
                    let path = entry.path();
                    let filename = entry.file_name();
                    if path.is_file() {
                        return None;
                    }

                    LangInfo::new(filename.to_str()?).ok()
                })
                .collect(),
        })
    }
}

impl std::ops::Deref for Languages {
    type Target = Vec<LangInfo>;

    fn deref(&self) -> &Self::Target {
        &self.langs
    }
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LangType {
    Scripting,
    Compiled,
    Config,
    Markup,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LangInfo {
    #[serde(default)]
    pub name: String,
    pub r#type: LangType,
}

impl LangInfo {
    pub fn new(name: &str) -> Result<Self, eyre::Error> {
        let dir_string = format!("languages/{name}");
        let dir = Path::new(&dir_string);

        ensure!(
            dir.is_dir(),
            format!("Expected directory `{dir_string}` to exist")
        );
        ensure!(dir.join("Dockerfile").is_file());
        ensure!(dir.join("generator.hbs").is_file());
        ensure!(dir.join("runner.hbs").is_file());

        let config = dir.join("config.json");
        ensure!(config.is_file());

        let config = fs::read_to_string(config)?;
        let mut config: LangInfo = serde_json::from_str(&config)?;

        config.name = name.into();

        Ok(config)
    }

    pub fn dockerfile_path(&self) -> PathBuf {
        let name = &self.name;
        return format!("languages/{name}/Dockerfile").into();
    }

    pub fn generator_path(&self) -> PathBuf {
        let name = &self.name;
        return format!("languages/{name}/generator.hbs").into();
    }

    pub fn runner_path(&self) -> PathBuf {
        let name = &self.name;
        return format!("languages/{name}/runner.hbs").into();
    }
}
