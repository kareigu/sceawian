use crate::utils::git_cmd;
use anyhow::Result;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct RepositoryDetails {
    pub name: String,
    pub source: String,
    pub target: String,
}

impl std::fmt::Display for RepositoryDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "name: {}", self.name)?;
        writeln!(f, "source: {}", self.source)?;
        writeln!(f, "target: {}", self.target)?;

        Ok(())
    }
}

impl RepositoryDetails {
    pub fn read_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let repo: Self = toml::from_str(&contents)?;
        Ok(repo)
    }

    pub fn fetch<P: AsRef<std::path::Path>>(&self, output_path: P) -> Result<()> {
        if !output_path.as_ref().exists() {
            return self.clone_from_source(output_path);
        }

        info!(
            "{}: fetching latest changes from {}",
            self.name, self.source,
        );
        let mut fetch = git_cmd();
        fetch.current_dir(output_path);
        fetch.args(["fetch", "origin"]);
        fetch.spawn()?.wait()?;

        Ok(())
    }

    pub fn clone_from_source<P: AsRef<std::path::Path>>(&self, output_path: P) -> Result<()> {
        info!(
            "{}: cloning {} into {}",
            self.name,
            self.source,
            output_path
                .as_ref()
                .to_str()
                .unwrap_or("invalid output_path")
        );

        let mut clone = git_cmd();
        clone.args([
            "clone",
            "--mirror",
            &self.source,
            output_path.as_ref().to_str().unwrap(),
        ]);
        clone.spawn()?.wait()?;

        Ok(())
    }

    pub fn mirror_to_target<P: AsRef<std::path::Path>>(&self, work_path: P) -> Result<()> {
        info!("{}: pushing mirror to {}", self.name, self.target,);
        let mut push = git_cmd();
        push.current_dir(work_path);
        push.args(["push", "--mirror", &self.target]);
        push.spawn()?.wait()?;

        Ok(())
    }
}
