use std::collections::BTreeMap;

use docker_api::{
    models::ImageBuildChunk,
    opts::{
        ContainerCreateOpts, ImageBuildOpts, ImageFilter, ImageListOpts, NetworkCreateOpts,
        NetworkFilter, NetworkListOpts,
    },
    Container, Docker,
};
use futures::{stream::StreamExt, Stream};
use handlebars::Handlebars;
use uuid::Uuid;

use crate::config::DockerConfig;

mod exec;

pub struct Runner {
    pub image_id: String,
    pub network_id: String,
    pub container_name_base: String,
}

pub type RunnerRegistry = BTreeMap<String, Runner>;

pub async fn generate_registry(
    cfg: &DockerConfig,
    docker: &Docker,
    handlebars: &mut Handlebars<'_>,
) -> docker_api::Result<RunnerRegistry> {
    let network_name = cfg.name_prefix.clone() + "network";

    let filter = NetworkFilter::Name(network_name.clone());
    let list_opts = NetworkListOpts::builder().filter([filter]);
    let network_list = docker.networks().list(&list_opts.build()).await?;
    assert!(
        network_list.len() <= 1,
        "Docker Network names should be unique"
    );

    // get the network id, building a new network if necessary
    let network_id = match network_list.into_iter().next() {
        Some(net) => net.id.expect("Network has an ID"),
        None => {
            tracing::info!("Creating new docker network: `{network_name}`");
            let create_opts = NetworkCreateOpts::builder(network_name).internal(true);
            let net = docker.networks().create(&create_opts.build()).await?;
            net.id().to_string()
        }
    };

    let r_futures = cfg
        .languages
        .iter()
        .map(|s| Runner::new(docker, cfg, s, &network_id));
    let runners = futures::future::join_all(r_futures)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    // register handlebars templates
    for lang in cfg.languages.iter() {
        handlebars
            .register_template_file(
                &format!("{lang}/generator"),
                &format!("src/runner/{lang}/generator.hbs"),
            )
            .expect("Failed to register template file");
        handlebars
            .register_template_file(
                &format!("{lang}/runner"),
                &format!("src/runner/{lang}/runner.hbs"),
            )
            .expect("Failed to register template file");
    }

    let registry = cfg.languages.iter().cloned().zip(runners).collect();
    Ok(registry)
}

impl Runner {
    pub async fn new(
        docker: &Docker,
        cfg: &DockerConfig,
        language: &str,
        network_id: &str,
    ) -> docker_api::Result<Self> {
        let name = cfg.name_prefix.clone() + language;

        let images = docker.images();
        let filter = ImageFilter::Reference(name.clone(), None);
        let list_opts = ImageListOpts::builder().filter([filter]);
        let image_list = images.list(&list_opts.build()).await?;
        assert!(image_list.len() <= 1, "Docker Image names should be unique");

        // get the image id, building a new image if necessary
        let image_id = match image_list.into_iter().next() {
            Some(s) => s.id,
            None => {
                tracing::info!("Building new docker image: `{name}`");
                let build_opts = ImageBuildOpts::builder(format!("src/runner/{language}"))
                    .tag(&name)
                    .labels([(cfg.image_label.clone(), language.to_string())]);
                let stream = images.build(&build_opts.build());
                let mut id = String::new();
                consume_image_build_stream(stream, &mut id).await;
                assert!(!id.is_empty(), "Expected `Aux` from image build stream");

                id
            }
        };

        Ok(Self {
            image_id,
            network_id: network_id.to_owned(),
            container_name_base: cfg.name_prefix.clone() + language + "-",
        })
    }

    async fn create_container(&self, docker: &Docker) -> docker_api::Result<Container> {
        let uuid = Uuid::now_v7();
        let opts = ContainerCreateOpts::builder()
            .image(&self.image_id)
            .cpus(1.0)
            .memory(16384 * 1024)
            .network_mode(&self.network_id)
            .privileged(false)
            .name(self.container_name_base.clone() + &uuid.to_string());
        let container = docker.containers().create(&opts.build()).await?;

        Ok(container)
    }
}

async fn consume_image_build_stream(
    mut stream: impl Stream<Item = docker_api::Result<ImageBuildChunk>> + Unpin,
    id: &mut String,
) {
    while let Some(s) = stream.next().await {
        match s {
            Ok(ImageBuildChunk::Update { stream }) => eprintln!("{}", stream.trim()),
            Ok(ImageBuildChunk::Error { error, .. }) => tracing::error!("{error}"),
            Ok(ImageBuildChunk::Digest { aux }) => {
                eprintln!("{}", aux.id);
                *id = aux.id;
            }
            Ok(ImageBuildChunk::PullStatus { progress, .. }) => {
                eprintln!("{}", progress.unwrap_or_default())
            }
            Err(e) => tracing::error!("{e}"),
        }
    }
}
