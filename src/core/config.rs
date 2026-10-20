use anyhow::{Context, Result};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub server_port: u16,
    pub env: String,
}

impl Settings {
    pub fn load() -> Result<Arc<Self>> {
        dotenv().ok();

        let settings = Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:///tmp/service.db".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("SERVER_PORT must be a number")?,
            env: env::var("APP_ENV").unwrap_or_else(|_| "development".to_string()),
        };

        Ok(Arc::new(settings))
    }
}
