use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use crate::services::{google_oauth, db};
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
}

#[get("/auth/callback")]
pub async fn auth_callback(
    query: web::Query<AuthRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let client_id = "TU_CLIENT_ID";
    let client_secret = "TU_CLIENT_SECRET";
    let redirect_uri = "https://api.wachatbot.com/klendathu/authcallback";

    match google_oauth::exchange_code_for_token(&query.code, client_id, client_secret, redirect_uri).await {
        Ok(token_response) => {
            match google_oauth::get_user_info(&token_response.access_token).await {
                Ok(user_info) => {
                    match db::save_user_session(&user_info, &token_response, &db_pool).await {
                        Ok(session_token) => HttpResponse::Ok().json(serde_json::json!({ "session_token": session_token })),
                        Err(_) => HttpResponse::InternalServerError().body("Error al crear la sesión"),
                    }
                }
                Err(_) => HttpResponse::InternalServerError().body("Error al obtener datos del usuario"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error al intercambiar código de autorización"),
    }
}
