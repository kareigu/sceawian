use anyhow::Result;
use serde::Deserialize;
use tracing::warn;

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Interval(u64);

impl Default for Interval {
    fn default() -> Self {
        Self(20)
    }
}

impl Into<u64> for Interval {
    fn into(self) -> u64 {
        self.0
    }
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub update_interval: Interval,
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
        writeln!(f, "update_interval = {} seconds", self.update_interval.0)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            update_interval: Interval::default(),
        }
    }
}
