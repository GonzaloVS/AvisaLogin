use std::collections::HashMap;
use config::{Config, File};
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub google_credentials_path: String,
}

impl AppConfig {
    pub fn load() -> Self {
        let settings = Config::builder()
            .add_source(config::File::with_name("config/settings"))
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

pub fn load_database_url() -> String {
    dotenv().ok(); // Carga .env automáticamente
    env::var("DATABASE_URL").expect("DATABASE_URL no está configurado en .env")
}
