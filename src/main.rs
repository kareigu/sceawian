use anyhow::Result;

mod repo;
use repo::RepositoryDetails;

#[tokio::main]
async fn main() -> Result<()> {
    let details = RepositoryDetails::read_from_file("repos/sceawian.toml")?;
    println!("{:?}", details);
    return Ok(());
}
