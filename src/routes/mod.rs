use actix_web::web;
pub mod auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::auth_callback); // Registra la ruta de autenticación
    cfg.service(auth::login);
}