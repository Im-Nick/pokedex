use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            // Use env file
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        builder.try_deserialize()
    }
}
