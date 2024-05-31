use anyhow::Result;
use tracing::info;

mod repo;
use repo::RepositoryDetails;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let details = RepositoryDetails::read_from_file("repos/sceawian.toml")?;
    info!("details: {:?}", details);

    let repo = details.fetch(format!("workspace/{}", details.name))?;
    details.mirror_to_target(&repo)?;
    return Ok(());
}
