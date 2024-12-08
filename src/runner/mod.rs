use std::{
    collections::BTreeMap,
    fs,
    time::{Duration, SystemTime},
};

use docker_api::{
    models::{ImageBuildChunk, ImageSummary},
    opts::{
        ContainerCreateOpts, ImageBuildOpts, ImageFilter, ImageListOpts, NetworkCreateOpts,
        NetworkFilter, NetworkListOpts,
    },
    Container, Docker,
};
use futures::{stream::StreamExt, Stream};
use uuid::Uuid;

use crate::{
    app::Templates,
    config::DockerConfig,
    langs::{LangInfo, Languages},
};

pub mod exec;

pub struct Runner {
    pub image_id: String,
    pub network_id: String,
    pub container_name_prefix: String,
    pub lang: LangInfo,
}

pub type RunnerRegistry = BTreeMap<String, Runner>;

pub async fn generate_registry(
    cfg: &DockerConfig,
    docker: &Docker,
    langs: &Languages,
    templates: &mut Templates,
) -> eyre::Result<RunnerRegistry> {
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

    let r_futures = langs
        .iter()
        .map(|s| Runner::new(docker, cfg, s, &network_id));
    let runners = futures::future::join_all(r_futures)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    // register template files
    for lang in langs.iter() {
        templates.register_lang(lang)?;
    }

    let registry: BTreeMap<String, Runner> = langs
        .iter()
        .map(|l| &l.name)
        .cloned()
        .zip(runners)
        .collect();
    Ok(registry)
}

impl Runner {
    pub async fn new(
        docker: &Docker,
        cfg: &DockerConfig,
        lang: &LangInfo,
        network_id: &str,
    ) -> docker_api::Result<Self> {
        let lang_name = &lang.name;
        let image_name = cfg.name_prefix.clone() + lang_name;

        let images = docker.images();
        let filter = ImageFilter::Reference(image_name.clone(), None);
        let list_opts = ImageListOpts::builder().filter([filter]);
        let image_list = images.list(&list_opts.build()).await?;
        assert!(image_list.len() <= 1, "Docker Image names should be unique");

        // get the image id, building a new image if necessary
        let path = format!("languages/{lang_name}");
        let age = fs::metadata(format!("{path}/Dockerfile"))
            .expect("Dockerfile exists")
            .modified()
            .unwrap();
        let image_age =
            |s: &ImageSummary| SystemTime::UNIX_EPOCH + Duration::from_secs(s.created as u64);

        let image_id = match image_list.into_iter().next() {
            Some(s) if image_age(&s) > age => s.id,
            _ => {
                tracing::info!("Building new docker image: `{image_name}`");
                let build_opts = ImageBuildOpts::builder(path)
                    .tag(&image_name)
                    .labels([(cfg.image_label.clone(), lang_name.to_string())]);
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
            container_name_prefix: cfg.container_name_prefix.clone(),
            lang: lang.clone(),
        })
    }

    async fn create_container(&self, docker: &Docker) -> docker_api::Result<Container> {
        let uuid = Uuid::now_v7();
        // TODO: put this into configuration
        let opts = ContainerCreateOpts::builder()
            .image(&self.image_id)
            .cpus(1.0)
            .memory(16384 * 1024)
            .network_mode(&self.network_id)
            .privileged(false)
            .name(self.container_name_prefix.clone() + &self.lang.name + &uuid.to_string()); // container names must be unique
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
