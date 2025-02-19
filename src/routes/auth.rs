use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::services::{google_oauth, db};
use sqlx::PgPool;
use crate::models::user::UserData;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<UserData>,
}

#[derive(Debug, Serialize)]
pub struct UserData {
    pub id: i32,
    pub username: String,
    pub email: String,
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

#[post("/auth/login")]
pub async fn login(
    login_data: web::Json<LoginRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query!(
        "SELECT id, username, email FROM users WHERE username = $1 AND password = crypt($2, password)",
        login_data.username,
        login_data.password
    )
        .fetch_optional(db_pool.get_ref())
        .await;

    match result {
        Ok(Some(user)) => {
            HttpResponse::Ok().json(LoginResponse {
                success: true,
                message: "Inicio de sesión exitoso".to_string(),
                user: Some(UserData {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                }),
            })
        }
        Ok(None) => HttpResponse::Unauthorized().json(LoginResponse {
            success: false,
            message: "Credenciales incorrectas".to_string(),
            user: None,
        }),
        Err(_) => HttpResponse::InternalServerError().json(LoginResponse {
            success: false,
            message: "Error del servidor".to_string(),
            user: None,
        }),
    }
}
