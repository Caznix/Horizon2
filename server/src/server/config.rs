use anyhow::{Context, Result};
use env_logger::Logger;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    sync::{Arc, OnceLock},
};

lazy_static! {
    static ref SERVER_CONFIG: OnceLock<Arc<ServerConfig>> = OnceLock::new();
}

pub fn server_config() -> Result<Arc<ServerConfig>> {
    let config_str = fs::read_to_string("./server_config.json")
        .context("Failed to read server_config.json to string")?;

    let config: ServerConfig = serde_json::from_str(&config_str).unwrap_or_else(|e| {
        ServerConfig::new()
    });
    Ok(SERVER_CONFIG.get_or_init(|| Arc::new(config)).clone())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    players_per_pool: u32,
    num_thread_pools: u16,
}

impl ServerConfig {
    fn new() -> Self {
        Self {
            players_per_pool: 5000,
            num_thread_pools: 60,
        }
    }
    fn log_level() -> String {
        String::from("info")
    }
    
}
