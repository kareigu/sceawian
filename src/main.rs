use anyhow::Result;
use tracing::info;
use tracing_subscriber;

mod repo;
use repo::RepositoryDetails;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let details = RepositoryDetails::read_from_file("repos/sceawian.toml")?;
    info!("details: {:?}", details);

    let _repo = details.fetch(format!("workspace/{}", details.name))?;
    return Ok(());
}
