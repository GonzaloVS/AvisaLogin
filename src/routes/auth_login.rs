use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::{PgPool, Row}; // Importamos Row para manejar resultados sin tipado explícito

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
    let result = sqlx::query(
        "SELECT account_id, account_name, google_email, google_profile_pic_url, status, suspension_reason, message
         FROM public.check_login($1, $2)",
    )
        .bind(&login_data.username)
        .bind(&login_data.password)
        .fetch_optional(db_pool.get_ref())
        .await;

    match result {
        Ok(Some(row)) => {
            let account_id: i32 = row.get("account_id");
            let account_name: String = row.get("account_name");
            let google_email: Option<String> = row.get("google_email");
            let google_profile_pic_url: Option<String> = row.get("google_profile_pic_url");
            let status: String = row.get("status");
            let suspension_reason: Option<String> = row.get("suspension_reason");
            let message: Option<String> = row.get("message");

            if message.as_deref() == Some("Login exitoso") {
                HttpResponse::Ok().json(serde_json::json!({
                    "success": true,
                    "message": message.unwrap_or_else(|| "Login exitoso".to_string()),
                    "user": {
                        "account_id": account_id,
                        "account_name": account_name,
                        "google_email": google_email,
                        "google_profile_pic_url": google_profile_pic_url,
                        "status": status,
                        "suspension_reason": suspension_reason
                    }
                }))
            } else {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "success": false,
                    "message": "Usuario no encontrado o contraseña incorrecta"
                }))
            }
        }
        Ok(None) => HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Usuario no encontrado o contraseña incorrecta"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "message": "Error del servidor"
        })),
    }
}
