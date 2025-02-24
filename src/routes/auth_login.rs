use actix_web::{post, web, HttpResponse, Responder};
use base64::{Engine as Base64Engine};
use base64::engine::general_purpose;
use serde::{Deserialize};
use crate::models::user::{UserData, UserResponse};
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[post("/auth/login")]
pub async fn login(
    login_data: web::Json<LoginRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    // let result = sqlx::query_as::<_, UserData>(
    //     "SELECT * FROM private.account WHERE google_email = $1 AND google_access_token = $2"
    // )
    //     .bind(&login_data.username)
    //     .bind(&login_data.password)
    //     .fetch_optional(db_pool.get_ref())
    //     .await;

    let result = sqlx::query_as::<_, UserData>(
        "SELECT * FROM private.account WHERE google_email = $1 AND password_hash = crypt($2, password_hash)"
    )
        .bind(&login_data.username)
        .bind(&login_data.password)
        .fetch_optional(db_pool.get_ref())
        .await;


    match result {
        Ok(Some(user)) => {
            // Convertimos `UserData` a `UserResponse`
            let user_response = UserResponse {
                account_id: user.account_id,
                account_name: user.account_name,
                google_email: user.google_email,
                google_profile_pic: user.google_profile_pic.as_ref().map(|pic| general_purpose::STANDARD.encode(pic)),
            };

            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Inicio de sesión exitoso",
                "user": user_response
            }))
        }
        Ok(None) => HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Credenciales incorrectas"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "message": "Error del servidor"
        })),
    }
}
