use anyhow::Error;

pub struct Config {
}

impl Config {
    pub fn parse(toml: &str) -> Result<Self, Error> {
        Ok(Config {})
    }
}
