use std::collections::BTreeMap;

use docker_api::{
    models::ImageBuildChunk,
    opts::{ImageBuildOpts, ImageFilter, ImageListOpts, ImageName},
    Container, Docker,
};
use futures::{stream::StreamExt, Stream};

use crate::config::DockerConfig;

struct LanguageRunner {}

pub struct Runner {
    language: String,
    container: Container,
}
pub type RunnerRegistry = BTreeMap<String, Runner>;

pub async fn generate_registry(
    cfg: &DockerConfig,
    docker: &Docker,
) -> docker_api::Result<RunnerRegistry> {
    let languages = ["python"];

    let r_futures = languages.iter().map(|s| Runner::new(s, cfg, docker));
    let runners = futures::future::join_all(r_futures)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;
    Ok(languages
        .iter()
        .map(|s| s.to_string())
        .zip(runners)
        .collect())
}

impl Runner {
    pub async fn new(
        language: &str,
        cfg: &DockerConfig,
        docker: &Docker,
    ) -> docker_api::Result<Self> {
        let name = cfg.name_prefix.clone() + language;

        let images = docker.images();
        let filter = ImageFilter::Reference(name.clone(), None);
        let list_opts = ImageListOpts::builder().filter([filter]);
        let image_list = images.list(&list_opts.build()).await?;
        assert!(image_list.len() <= 1, "Docker Image names should be unique");

        // check if the image exists
        let image = match image_list.first() {
            Some(s) => images.get(&s.id),
            None => {
                tracing::info!("Building new docker image: `{name}`");
                let build_opts = ImageBuildOpts::builder(format!("src/runner/{language}"))
                    .tag(name)
                    .labels([(cfg.image_label.clone(), language.to_string())])
                    .memory(4096 * 4096);
                let stream = images.build(&build_opts.build());
                consume_image_build_stream(stream).await;

                todo!()
            }
        };
        todo!()
    }
}

async fn consume_image_build_stream(
    mut stream: impl Stream<Item = docker_api::Result<ImageBuildChunk>> + Unpin,
) {
    while let Some(s) = stream.next().await {
        match s {
            Ok(ImageBuildChunk::Update { stream }) => eprintln!("{}", stream.trim()),
            Ok(ImageBuildChunk::Error { error, .. }) => tracing::error!("{error}"),
            Ok(ImageBuildChunk::Digest { aux }) => eprintln!("{}", aux.id),
            Ok(ImageBuildChunk::PullStatus { progress, .. }) => {
                eprintln!("{}", progress.unwrap_or_default())
            }
            Err(e) => tracing::error!("{e}"),
        }
    }
}
