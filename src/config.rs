use figment::{
    Figment,
    providers::{Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 4000,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let config: Config = Default::default();
        Figment::new()
            .merge(Serialized::defaults(config))
            .merge(Toml::file("config.toml"))
            .extract()
            .unwrap()
    }
}
