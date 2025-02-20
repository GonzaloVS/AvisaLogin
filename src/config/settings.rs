use std::collections::HashMap;
use config::{Config as AppConfigLib, File};


#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub google_credentials_path: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let settings = AppConfigLib::builder()
            .add_source(File::with_name("config/settings"))
            .build()
            .expect("Error cargando settings.toml");

        let database_settings: HashMap<String, String> = settings.get("database").unwrap();
        let google_settings: HashMap<String, String> = settings.get("google").unwrap();

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            database_settings["user"],
            database_settings["password"],
            database_settings["host"],
            database_settings["port"],
            database_settings["dbname"]
        );

        Self {
            database_url,
            google_credentials_path: google_settings["credentials_file"].clone(),
        }
    }
}
