use std::env;

use anyhow::Context;

const DATABASE_URL_KEY: &str = "DATABASE_URL";

const SERVER_PORT_KEY: &str = "SERVER_PORT";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub server_port: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {
        let server_port = load_env(SERVER_PORT_KEY)?;
        let database_url = load_env(DATABASE_URL_KEY)?;

        Ok(Config {
            server_port,
            database_url,
        })
    }
}

fn load_env(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("failed to load environment variable {}", key))
}
