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

#[derive(thiserror::Error, Debug)]
#[error("{msg}: mirroring failed: {source}")]
pub struct Error {
    msg: String,
    source: anyhow::Error,
}

impl RepositoryDetails {
    pub fn read_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let repo: Self = toml::from_str(&contents)?;
        Ok(repo)
    }

    pub fn fetch<P: AsRef<std::path::Path>>(&self, output_path: P) -> Result<(), Error> {
        if !output_path.as_ref().exists() {
            return self.clone_from_source(output_path);
        }

        info!(
            "{}: fetching latest changes from {}",
            self.name, self.source,
        );
        let mut fetch = git_cmd();
        fetch.current_dir(output_path);
        fetch.args(["fetch", "--prune", "origin"]);
        let exit = fetch
            .spawn()
            .and_then(|mut e| Ok(e.wait()?))
            .map_err(self.wrap_err())?;

        if !exit.success() {
            return Err(self.wrap_err()(anyhow::anyhow!(
                "git fetch exited with code {}",
                exit.code().unwrap_or(-999)
            )));
        }

        Ok(())
    }

    pub fn clone_from_source<P: AsRef<std::path::Path>>(
        &self,
        output_path: P,
    ) -> Result<(), Error> {
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
        let exit = clone
            .spawn()
            .and_then(|mut e| Ok(e.wait()?))
            .map_err(self.wrap_err())?;

        if !exit.success() {
            return Err(self.wrap_err()(anyhow::anyhow!(
                "git clone exited with code {}",
                exit.code().unwrap_or(-999)
            )));
        }

        Ok(())
    }

    pub fn mirror_to_target<P: AsRef<std::path::Path>>(&self, work_path: P) -> Result<(), Error> {
        info!("{}: pushing mirror to {}", self.name, self.target,);
        let mut push = git_cmd();
        push.current_dir(work_path);
        push.args(["push", "--mirror", &self.target]);
        let exit = push
            .spawn()
            .and_then(|mut e| Ok(e.wait()?))
            .map_err(self.wrap_err())?;

        if !exit.success() {
            return Err(self.wrap_err()(anyhow::anyhow!(
                "git push exited with code {}",
                exit.code().unwrap_or(-999)
            )));
        }

        Ok(())
    }

    fn wrap_err<E>(&self) -> impl FnOnce(E) -> Error + '_
    where
        E: Into<anyhow::Error>,
    {
        move |e| Error {
            msg: self.name.clone(),
            source: e.into(),
        }
    }
}
