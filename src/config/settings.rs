use config::{Config as AppConfigLib, File};
use dotenvy::dotenv;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub google_credentials_path: String,
}

impl AppConfig {
    pub fn load() -> Self {

        dotenv().ok(); // Carga .env antes de leer settings.toml

        let settings = AppConfigLib::builder()
            .add_source(File::with_name("config/settings"))
            .build()
            .expect("Error cargando settings.toml");

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("DATABASE_URL no está definido en el entorno"));

        let google_credentials_path = settings.get::<String>("google.credentials_file")
            .unwrap_or_else(|_| panic!("No se encontró 'credentials_file' en settings.toml"));

        Self {
            database_url,
            google_credentials_path,
        }
    }
}
