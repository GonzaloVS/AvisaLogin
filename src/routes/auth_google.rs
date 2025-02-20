use actix_web::{get, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;

use crate::services::db::create_or_update_account_and_session;
use crate::services::google_oauth::{exchange_code_for_token, get_user_info};

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String, //Parámetro "code" recibido en la URL
}

#[get("/auth/callback")]
pub async fn auth_callback(
    query: web::Query<AuthRequest>,  //Extrae code automáticamente desde la URL
    db_pool: web::Data<PgPool>,     //Pool de conexión a la base de datos
) -> impl Responder {
    let client_id = "TU_CLIENT_ID";
    let client_secret = "TU_CLIENT_SECRET";
    let redirect_uri = "https://api.wachatbot.com/klendathu/authcallback";

    //Intercambiar code por access_token
    match exchange_code_for_token(&query.code, client_id, client_secret, redirect_uri).await {
        Ok(token_response) => {
            //Obtener la información del usuario desde Google
            match get_user_info(&token_response.access_token).await {
                Ok(user_info) => {
                    //Llamar a la función en PostgreSQL para guardar/actualizar la cuenta
                    match create_or_update_account_and_session(
                        &user_info.name,
                        &user_info.id,
                        &user_info.email,
                        &token_response.access_token,
                        Utc::now() + chrono::Duration::days(7),
                        token_response.refresh_token.as_deref().unwrap_or(""),
                        vec![],  // No se usa imagen de perfil aquí
                        db_pool.get_ref()
                    ).await {
                        //Si la función devuelve un JSON con session_token, responder con éxito
                        Ok(response) => HttpResponse::Ok().json(response),

                        //Error al llamar a PostgreSQL
                        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                            "status": "error",
                            "message": "Error al crear o actualizar la cuenta"
                        })),
                    }
                }
                //Error al obtener datos del usuario desde Google
                Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "message": "Error al obtener información del usuario desde Google"
                })),
            }
        }
        //Error en la autenticación con Google
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Error al intercambiar código de autorización con Google"
        })),
    }
}
