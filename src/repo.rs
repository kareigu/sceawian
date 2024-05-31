use anyhow::Result;
use git2::Repository;
use serde::Deserialize;
use tracing::info;

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

    pub fn fetch<P: AsRef<std::path::Path>>(&self, output_path: P) -> Result<git2::Repository> {
        if !output_path.as_ref().exists() {
            return self.clone_from_source(output_path);
        }

        let repo = git2::Repository::open(output_path)?;
        for branch in repo.branches(Some(git2::BranchType::Local))? {
            let (branch, _) = branch?;
            let branch_name = branch.name()?.unwrap();

            info!("fetching {} from origin", branch_name);
            repo.find_remote("origin")?
                .fetch(&[branch_name], None, None)?;

            let upstream = branch.upstream()?;
            info!(
                "resetting {} to {}",
                branch_name,
                upstream
                    .name()?
                    .unwrap_or_else(|| "invalid remote_branch_name")
            );
            repo.reset(
                &upstream.into_reference().peel(git2::ObjectType::Commit)?,
                git2::ResetType::Hard,
                None,
            )?;
        }
        return Ok(repo);
    }

    pub fn clone_from_source<P: AsRef<std::path::Path>>(
        &self,
        output_path: P,
    ) -> Result<git2::Repository> {
        info!(
            "cloning {} into {}",
            self.source,
            output_path
                .as_ref()
                .to_str()
                .unwrap_or_else(|| "invalid output_path")
        );

        let mut callbacks = git2::RemoteCallbacks::default();
        callbacks.credentials(|_, username, _| git2::Cred::ssh_key_from_agent(username.unwrap()));
        let mut fetch_opts = git2::FetchOptions::default();
        fetch_opts.remote_callbacks(callbacks);
        let mut repo_build = git2::build::RepoBuilder::new();
        repo_build.fetch_options(fetch_opts);
        let repo = repo_build.clone(&self.source, output_path.as_ref())?;
        return Ok(repo);
    }
}
