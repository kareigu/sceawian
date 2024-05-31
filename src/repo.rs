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

    pub fn fetch<P: AsRef<std::path::Path>>(&self, output_path: P) -> Result<git2::Repository> {
        if !output_path.as_ref().exists() {
            return self.clone_from_source(output_path);
        }

        let repo = git2::Repository::open(output_path)?;
        for branch in repo.branches(Some(git2::BranchType::Local))? {
            let (branch, _) = branch?;
            let branch_name = branch.name()?.unwrap();
            repo.find_remote("origin")?
                .fetch(&[branch_name], None, None)?;

            let upstream = branch.upstream()?;
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
        let repo = Repository::clone(&self.source, output_path)?;
        return Ok(repo);
    }
}
