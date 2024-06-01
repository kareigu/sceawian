use crate::utils;
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

    pub fn fetch<P: AsRef<std::path::Path>>(&self, output_path: P) -> Result<git2::Repository> {
        if !output_path.as_ref().exists() {
            return self.clone_from_source(output_path);
        }

        let repo = git2::Repository::open(output_path)?;
        for branch in repo.branches(Some(git2::BranchType::Local))? {
            let (branch, _) = branch?;
            let branch_name = branch.name()?.unwrap();

            info!("{}: fetching {} from origin", self.name, branch_name);
            repo.find_remote("origin")?.fetch(
                &[branch_name],
                Some(&mut utils::fetch_opts()),
                None,
            )?;

            let upstream = branch.upstream()?;
            info!(
                "{}: resetting {} to {}",
                self.name,
                branch_name,
                upstream.name()?.unwrap_or("invalid remote_branch_name")
            );
            repo.reset(
                &upstream.into_reference().peel(git2::ObjectType::Commit)?,
                git2::ResetType::Hard,
                None,
            )?;
        }
        Ok(repo)
    }

    pub fn clone_from_source<P: AsRef<std::path::Path>>(
        &self,
        output_path: P,
    ) -> Result<git2::Repository> {
        info!(
            "{}: cloning {} into {}",
            self.name,
            self.source,
            output_path
                .as_ref()
                .to_str()
                .unwrap_or("invalid output_path")
        );

        let mut repo_build = utils::repo_build();
        let repo = repo_build.clone(&self.source, output_path.as_ref())?;
        Ok(repo)
    }

    pub fn mirror_to_target(&self, repo: &git2::Repository) -> Result<()> {
        const TARGET_REMOTE_NAME: &str = "target";
        const CONFIG_VALUE: &str = "remote.target.mirror";
        if repo.find_remote(TARGET_REMOTE_NAME).is_ok() {
            repo.remote_delete(TARGET_REMOTE_NAME)?;
        }

        repo.remote_add_push(TARGET_REMOTE_NAME, "dev")?;
        repo.remote_set_pushurl(TARGET_REMOTE_NAME, Some(&self.target))?;
        repo.config()?.set_bool(CONFIG_VALUE, true)?;

        info!("{}: pushing to {}", self.name, self.target);
        repo.find_remote(TARGET_REMOTE_NAME)?
            .push(&["+refs/heads/dev"], Some(&mut utils::push_opts()))?;

        Ok(())
    }
}
