use sqlx::{PgPool, Error};
use std::env;
use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use serde_json::Value;

// pub async fn connect_db() -> Result<PgPool, Error> {
//     dotenv().ok(); // Carga las variables del .env
//     let host = env::var("DATABASE_HOST").expect("Falta DATABASE_HOST");
//     let port = env::var("DATABASE_PORT").expect("Falta DATABASE_PORT");
//     let user = env::var("DATABASE_USER").expect("Falta DATABASE_USER");
//     let password = env::var("DATABASE_PASSWORD").expect("Falta DATABASE_PASSWORD");
//     let dbname = env::var("DATABASE_NAME").expect("Falta DATABASE_NAME");
//
//     let db_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, dbname);
//     PgPool::connect(&db_url).await
// }

pub async fn connect_db() -> Result<PgPool, Error> {
    dotenv().ok(); // Carga las variables desde .env

    let database_url = env::var("DATABASE_URL").expect("Falta DATABASE_URL en .env");

    // Conectar con la base de datos
    PgPool::connect(&database_url).await
}

pub async fn create_or_update_account_and_session(
    google_name: &str,
    google_id: &str,
    google_email: &str,
    google_access_token: &str,
    google_access_token_expire: DateTime<Utc>,
    google_refresh_token: &str,
    google_profile_pic: Vec<u8>,
    pool: &PgPool
) -> Result<Value, Error> {
    let result: Value = sqlx::query_scalar!(
        "SELECT public.create_or_update_account_and_session_from_google_auth($1, $2, $3, $4, $5, $6, $7)",
        google_name,
        google_id,
        google_email,
        google_access_token,
        google_access_token_expire,
        google_refresh_token,
        google_profile_pic
    )
        .fetch_one(pool)
        .await?;

    Ok(result)
}

pub async fn update_account_name_from_id(
    account_id: i64,
    new_account_name: &str,
    pool: &PgPool
) -> Result<Value, Error> {
    let result: Value = sqlx::query_scalar!(
        "SELECT public.update_account_name_from_account_id($1, $2)",
        account_id,
        new_account_name
    )
        .fetch_one(pool)
        .await?;

    Ok(result)
}

pub async fn update_account_name_from_session(
    session_token: &str,
    new_account_name: &str,
    pool: &PgPool
) -> Result<Value, Error> {
    let result: Value = sqlx::query_scalar!(
        "SELECT public.update_account_name_from_account_session_token($1, $2)",
        session_token,
        new_account_name
    )
        .fetch_one(pool)
        .await?;

    Ok(result)
}

pub async fn get_account_details(
    session_token: &str,
    pool: &PgPool
) -> Result<Value, Error> {
    let result: Value = sqlx::query_scalar!(
        "SELECT public.get_account_details_from_session_token($1)",
        session_token
    )
        .fetch_one(pool)
        .await?;

    Ok(result)
}

pub async fn get_session_from_account_id(
    account_id: i64,
    pool: &PgPool
) -> Result<Value, Error> {
    let result: Value = sqlx::query_scalar!(
        "SELECT public.get_session_from_account_id($1)",
        account_id
    )
        .fetch_one(pool)
        .await?;

    Ok(result)
}

