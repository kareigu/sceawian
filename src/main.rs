use anyhow::Result;
use tracing::{error, info};

mod repo;
use repo::RepositoryDetails;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    for file in std::path::Path::new("repos").read_dir()? {
        let file = match file {
            Ok(f) => f,
            Err(e) => {
                error!("Failed getting file: {}", e);
                continue;
            }
        };

        if let Some(ext) = file.path().extension() {
            if ext != "toml" {
                continue;
            }
        }

        let details = RepositoryDetails::read_from_file(file.path())?;
        info!("details: {:?}", details);

        let repo = details.fetch(format!("workspace/{}", details.name))?;
        details.mirror_to_target(&repo)?;
    }

    return Ok(());
}
