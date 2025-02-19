use sqlx::{PgPool, Error};
use std::env;
use chrono::Utc;
use dotenvy::dotenv;
use crate::services::google_oauth::{GoogleAuthResponse, GoogleUserInfo};

pub async fn connect_db() -> Result<PgPool, Error> {
    dotenv().ok(); // Carga las variables del .env
    let host = env::var("DATABASE_HOST").expect("Falta DATABASE_HOST");
    let port = env::var("DATABASE_PORT").expect("Falta DATABASE_PORT");
    let user = env::var("DATABASE_USER").expect("Falta DATABASE_USER");
    let password = env::var("DATABASE_PASSWORD").expect("Falta DATABASE_PASSWORD");
    let dbname = env::var("DATABASE_NAME").expect("Falta DATABASE_NAME");

    let db_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, dbname);
    PgPool::connect(&db_url).await
}

pub async fn save_user_session(
    user: &GoogleUserInfo,
    token: &GoogleAuthResponse,
    pool: &PgPool
) -> Result<String, Error> {
    let session_token = uuid::Uuid::new_v4().to_string();
    let session_expire = Utc::now() + chrono::Duration::days(7);

    let result = sqlx::query!(
        r#"
        INSERT INTO sessions (google_id, email, access_token, refresh_token, session_token, session_expire)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (google_id) DO UPDATE
        SET access_token = EXCLUDED.access_token, refresh_token = EXCLUDED.refresh_token, session_token = EXCLUDED.session_token, session_expire = EXCLUDED.session_expire
        RETURNING session_token
        "#,
        user.id,
        user.email,
        token.access_token,
        token.refresh_token.as_deref(),
        session_token,
        session_expire
    )
        .fetch_one(pool)
        .await?;

    Ok(result.session_token)
}
