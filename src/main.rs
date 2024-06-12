use anyhow::Result;
use tracing::{error, info};

mod repo;
use repo::RepositoryDetails;
mod config;
use config::Config;
mod utils;

async fn run_actions(path: std::path::PathBuf) -> Result<RepositoryDetails> {
    let details = RepositoryDetails::read_from_file(&path)?;
    info!("details {}: {}", path.display(), details);

    let output_path = format!("workspace/{}", details.name);
    details.fetch(&output_path)?;
    details.mirror_to_target(output_path)?;
    Ok(details)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::read_from_file("config.toml")?;
    info!("using config values: {}", config);

    let mut handles = tokio::task::JoinSet::new();

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(
        config.update_interval.into(),
    ));
    let mut prev_time = tokio::time::Instant::now();

    loop {
        let time = interval.tick().await;
        let duration = time.duration_since(prev_time).as_secs();
        info!("updating mirrors after waiting for {} seconds", duration);
        prev_time = time;

        let repos_dir = std::path::Path::new(&config.repos).read_dir()?;
        let paths = repos_dir
            .filter(|file| match file {
                Ok(file) => file.path().extension().map_or(false, |ext| ext == "toml"),
                Err(e) => {
                    error!("failed getting file: {}", e);
                    false
                }
            })
            .map(|file| file.expect("somehow error didn't get filtered").path())
            .collect::<Vec<std::path::PathBuf>>();

        for path in paths {
            handles.spawn(tokio::time::timeout(
                tokio::time::Duration::from_secs(config.update_interval.into()),
                run_actions(path),
            ));
        }

        while let Some(res) = handles.join_next().await {
            match res {
                Err(e) => error!("joining task failed: {}", e),
                Ok(Err(e)) => error!("task timed out after {}", e),
                Ok(Ok(Err(e))) => error!("{}", e),
                Ok(Ok(Ok(details))) => info!("{}: mirroring finished", details.name),
            }
        }
    }
}
