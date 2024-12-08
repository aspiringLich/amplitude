use std::path::Path;

use docker_api::Docker;
use handlebars::{Handlebars, Template, TemplateError};
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::{
    config::{Config, Secrets},
    langs::{LangInfo, Languages},
    runner::RunnerRegistry,
};

pub struct AppState {
    pub config: Config,
    pub secrets: Secrets,
    pub db: DatabaseConnection,
    pub docker: Docker,
    pub runner_registry: RunnerRegistry,
    pub templates: Templates,
    pub langs: Languages,
}

pub struct Templates {
    handlebars: Handlebars<'static>,
}

impl Templates {
    pub fn new(handlebars: Handlebars<'static>) -> Self {
        return Self { handlebars };
    }

    pub fn register_lang(&mut self, lang: &LangInfo) -> Result<(), TemplateError> {
        let name = &lang.name;
        self.handlebars
            .register_template_file(&format!("{name}/generator"), lang.generator_path())?;
        self.handlebars
            .register_template_file(&format!("{name}/runner"), lang.runner_path())?;

        Ok(())
    }

    pub fn render_generator<T>(
        &self,
        lang: &LangInfo,
        data: &T,
    ) -> Result<String, handlebars::RenderError>
    where
        T: Serialize,
    {
        let name = &lang.name;
        self.handlebars.render(&format!("{name}/generator"), data)
    }
}
