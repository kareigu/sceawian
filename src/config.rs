use anyhow::Result;
use serde::Deserialize;
use tracing::warn;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Interval(u64);

impl Default for Interval {
    fn default() -> Self {
        Self(40)
    }
}

impl From<Interval> for u64 {
    fn from(val: Interval) -> u64 {
        val.0
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub update_interval: Interval,
    #[serde(default = "default_repos")]
    pub repos: String,
    #[serde(default = "default_task_count")]
    pub task_count: usize,
}

impl Config {
    pub fn read_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let contents = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                warn!("couldn't load `config.toml`, using defaults: {}", e);
                return Ok(Self::default());
            }
        };
        Ok(toml::from_str(&contents)?)
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        writeln!(f, "update_interval = {} seconds", self.update_interval.0)?;
        writeln!(f, "repos = {}", self.repos)?;
        writeln!(f, "task_count = {}", self.task_count)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            update_interval: Interval::default(),
            repos: default_repos(),
            task_count: default_task_count(),
        }
    }
}

fn default_repos() -> String {
    "repos".to_string()
}

fn default_task_count() -> usize {
    4
}
