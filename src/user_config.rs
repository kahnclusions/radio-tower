use serde::Deserialize;
use std::{fs, io};
use tracing::debug;

#[derive(Debug, Clone, Deserialize)]
pub struct UserConfig {
    pub transmission_url: String,
    pub poll_frequency_ms: usize,
}

#[tracing::instrument]
pub fn load_config(user_config_path: &str) -> Result<UserConfig, io::Error> {
    if fs::metadata(user_config_path).is_ok() {
        let config_string = fs::read_to_string(&user_config_path)?;
        let user_config: UserConfig = toml::from_str(&config_string).unwrap();
        debug!("Loaded config from {:#?}", user_config_path);
        Ok(user_config)
    } else {
        debug!("Loaded default config");
        Ok(UserConfig {
            transmission_url: "http://localhost:9091/transmission/rpc".to_string(),
            poll_frequency_ms: 2000,
        })
    }
}
