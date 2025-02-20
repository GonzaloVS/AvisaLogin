mod config;
mod routes;
mod services;
mod models;

use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use dotenvy::dotenv;
use config::settings::AppConfig;
use config::google_creds::GoogleCredentials;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Cargar configuración
    let app_config = AppConfig::load();
    let google_credentials = GoogleCredentials::load_from_file(&app_config.google_credentials_path);

    // Conectar a PostgreSQL
    let pool = PgPool::connect(&app_config.database_url)
        .await
        .expect("Error conectando a PostgreSQL");

    println!("Google Client ID: {}", google_credentials.web.client_id);
    println!("Google Redirect URI: {}", google_credentials.web.redirect_uris[0]);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::config)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
