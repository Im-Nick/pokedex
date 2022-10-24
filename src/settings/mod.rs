use config::{Config, ConfigError, File};
use serde::Deserialize;
#[derive(Deserialize, Debug, Clone)]
pub struct Api {
    pub pokemon_api: String,
    pub yoda_api: String,
    pub shakespeare_api: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub api: Api,
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("config/default"))
            // Use env file
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        builder.try_deserialize()
    }
}
