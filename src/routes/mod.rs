use actix_web::web;

pub mod auth_google;
pub mod auth_login;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(auth_google::auth_callback); // Ruta de autenticación Google
    cfg.service(auth_login::login);         // Ruta de login con BD
}
