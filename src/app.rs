use std::path::Path;

use docker_api::Docker;
use handlebars::{Handlebars, Template, TemplateError};
use sea_orm::DatabaseConnection;

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
    
    pub fn get_generator(&self, name: &str) -> Option<&Template> {
        self.handlebars.get_template(&format!("{name}/generator"))
    }
    
    pub fn get_runner(&self, name: &str) -> Option<&Template> {
        self.handlebars.get_template(&format!("{name}/runner"))
    }
}
