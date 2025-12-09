use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub application: ApplicationConfig,
    pub db: DatabaseConfig,
}
#[derive(Deserialize)]
pub struct ApplicationConfig {
    pub host: String,
    pub port: u16,
}
#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub name: String,
}

pub fn read_config() -> Result<ServerConfig, ConfigError> {
    let additional_config_path = std::env::var("SHOP_ENVIROMENT").unwrap_or("local".into());

    config::Config::builder()
        .add_source(config::File::with_name("configuration/base.toml"))
        .add_source(config::File::with_name(&format!(
            "configuration/{additional_config_path}.toml"
        )))
        .add_source(config::Environment::with_prefix("SHOP"))
        .build()?
        .try_deserialize()
}
