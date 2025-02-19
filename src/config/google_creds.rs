use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct GoogleCredentials {
    pub web: WebCredentials,
}

#[derive(Debug, Deserialize)]
pub struct WebCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
}

impl GoogleCredentials {
    pub fn load_from_file(path: &str) -> Self {
        let file_contents = fs::read_to_string(Path::new(path))
            .expect("Error al leer credentials.json");
        serde_json::from_str(&file_contents).expect("Error al parsear credentials.json")
    }
}
