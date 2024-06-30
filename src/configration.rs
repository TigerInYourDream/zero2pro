use serde::Deserialize;

#[derive(Deserialize)]
pub struct AppSetting {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<AppSetting, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("config.toml", config::FileFormat::Toml))
        .build()?;
    settings.try_deserialize()
}
