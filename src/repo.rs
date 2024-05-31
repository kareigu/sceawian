use anyhow::Result;
use git2::Repository;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RepositoryDetails {
    pub name: String,
    pub source: String,
    pub target: String,
}

impl RepositoryDetails {
    pub fn read_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let repo: Self = toml::from_str(&contents)?;
        return Ok(repo);
    }
}
