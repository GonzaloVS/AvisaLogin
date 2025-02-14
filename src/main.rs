use actix_web::{web, App, HttpServer, HttpResponse, Responder, Error};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use chrono::{Utc, Duration};
use uuid::Uuid;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    google_id: String,
    account_name: String,
    google_email: String,
    google_access_token: String,
    google_access_token_expire_dateutc: String,
    google_refresh_token: Option<String>,
    google_profile_pic: String,
}

#[derive(Debug, Serialize)]
struct SessionResponse {
    session_token: String,
    session_expire_dateutc: String,
}

// ✅ Login: Crear usuario y sesión en PostgreSQL
async fn login(
    data: web::Json<LoginRequest>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let user = data.into_inner();
    let session_token = Uuid::new_v4().to_string();
    let session_expire_dateutc = Utc::now() + Duration::days(30);

    let account = sqlx::query!(
        "INSERT INTO account (account_name, google_id, google_email, google_access_token,
                              google_access_token_expire_dateutc, google_refresh_token, google_profile_pic, google_last_login_dateutc)
         VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
         ON CONFLICT (google_id) DO UPDATE
         SET account_name = $1, google_email = $3, google_access_token = $4,
             google_access_token_expire_dateutc = $5, google_refresh_token = $6,
             google_profile_pic = $7, google_last_login_dateutc = NOW()
         RETURNING account_id",
        user.account_name,
        user.google_id,
        user.google_email,
        user.google_access_token,
        user.google_access_token_expire_dateutc,
        user.google_refresh_token,
        user.google_profile_pic
    )
        .fetch_one(pool.get_ref())
        .await
        .unwrap();

    let account_id = account.account_id;

    sqlx::query!(
        "INSERT INTO user_session (session_token, session_create_dateutc, session_expire_dateutc, account_id)
         VALUES ($1, NOW(), $2, $3)
         ON CONFLICT (account_id) DO UPDATE
         SET session_token = $1, session_expire_dateutc = $2",
        session_token,
        session_expire_dateutc,
        account_id
    )
        .execute(pool.get_ref())
        .await
        .unwrap();

    HttpResponse::Ok().json(SessionResponse {
        session_token,
        session_expire_dateutc: session_expire_dateutc.format("%Y-%m-%d %H:%M:%S").to_string(),
    })
}

// ✅ Validar sesión
async fn validate_session(
    pool: web::Data<PgPool>,
    headers: web::Header<actix_web::http::header::HeaderValue>,
) -> impl Responder {
    let session_token = headers.get("Authorization").and_then(|h| h.to_str().ok());

    if let Some(token) = session_token {
        let result = sqlx::query!(
            "SELECT account_id FROM user_session WHERE session_token = $1 AND session_expire_dateutc > NOW()",
            token
        )
            .fetch_optional(pool.get_ref())
            .await
            .unwrap();

        if let Some(row) = result {
            return HttpResponse::Ok().json(format!("Sesión válida para account_id: {}", row.account_id));
        }
    }

    HttpResponse::Unauthorized().json("Sesión no válida o expirada")
}

// ✅ Cerrar sesión
async fn logout(
    pool: web::Data<PgPool>,
    headers: web::Header<actix_web::http::header::HeaderValue>,
) -> impl Responder {
    let session_token = headers.get("Authorization").and_then(|h| h.to_str().ok());

    if let Some(token) = session_token {
        sqlx::query!(
            "DELETE FROM user_session WHERE session_token = $1",
            token
        )
            .execute(pool.get_ref())
            .await
            .unwrap();
    }

    HttpResponse::Ok().json("Sesión cerrada")
}

// ✅ Servidor Actix-Web
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no definido");
    let pool = PgPool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/login", web::post().to(login))
            .route("/validate_session", web::get().to(validate_session))
            .route("/logout", web::post().to(logout))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


// use actix_web::{web, App, HttpServer, HttpResponse, Responder};
// use actix_session::{Session, SessionMiddleware, storage::DatabaseSessionStore};
// use actix_identity::{Identity, IdentityMiddleware};
// use serde::{Deserialize, Serialize};
// use sqlx::{PgPool, Row};
// use chrono::{Utc, Duration};
// use uuid::Uuid;
// use dotenv::dotenv;
// use std::env;
//
// #[derive(Debug, Serialize, Deserialize)]
// struct LoginRequest {
//     google_id: String,
//     account_name: String,
//     google_email: String,
//     google_access_token: String,
//     google_access_token_expire_dateutc: String,
//     google_refresh_token: Option<String>,
//     google_profile_pic: String,
// }
//
// #[derive(Debug, Serialize)]
// struct SessionResponse {
//     session_token: String,
//     session_expire_dateutc: String,
// }
//
// async fn login(
//     data: web::Json<LoginRequest>,
//     pool: web::Data<PgPool>,
//     session: Session,
// ) -> impl Responder {
//     let user = data.into_inner();
//     let session_token = Uuid::new_v4().to_string();
//     let session_expire_dateutc = Utc::now() + Duration::days(30);
//
//     // 1️⃣ Insertar o actualizar la cuenta del usuario
//     let account = sqlx::query!(
//         "INSERT INTO account (account_name, google_id, google_email, google_access_token,
//                               google_access_token_expire_dateutc, google_refresh_token, google_profile_pic, google_last_login_dateutc)
//          VALUES ($1, $2, $3, $4, $5, $6, $7, NOW())
//          ON CONFLICT (google_id) DO UPDATE
//          SET account_name = $1, google_email = $3, google_access_token = $4,
//              google_access_token_expire_dateutc = $5, google_refresh_token = $6,
//              google_profile_pic = $7, google_last_login_dateutc = NOW()
//          RETURNING account_id",
//         user.account_name,
//         user.google_id,
//         user.google_email,
//         user.google_access_token,
//         user.google_access_token_expire_dateutc,
//         user.google_refresh_token,
//         user.google_profile_pic
//     )
//         .fetch_one(pool.get_ref())
//         .await
//         .unwrap();
//
//     let account_id = account.account_id;
//
//     // 2️⃣ Crear o actualizar la sesión del usuario
//     sqlx::query!(
//         "INSERT INTO user_session (session_token, session_create_dateutc, session_expire_dateutc, account_id)
//          VALUES ($1, NOW(), $2, $3)
//          ON CONFLICT (account_id) DO UPDATE
//          SET session_token = $1, session_expire_dateutc = $2",
//         session_token,
//         session_expire_dateutc,
//         account_id
//     )
//         .execute(pool.get_ref())
//         .await
//         .unwrap();
//
//     session.insert("session_token", &session_token).unwrap();
//     session.renew();
//
//     HttpResponse::Ok().json(SessionResponse {
//         session_token,
//         session_expire_dateutc: session_expire_dateutc.format("%Y-%m-%d %H:%M:%S").to_string(),
//     })
// }
//
// async fn validate_session(
//     session: Session,
//     pool: web::Data<PgPool>,
// ) -> impl Responder {
//     if let Some(session_token) = session.get::<String>("session_token").unwrap() {
//         let result = sqlx::query!(
//             "SELECT account_id FROM user_session WHERE session_token = $1 AND session_expire_dateutc > NOW()",
//             session_token
//         )
//             .fetch_optional(pool.get_ref())
//             .await
//             .unwrap();
//
//         if let Some(row) = result {
//             return HttpResponse::Ok().json(format!("Sesión válida para account_id: {}", row.account_id));
//         }
//     }
//     HttpResponse::Unauthorized().json("Sesión no válida o expirada")
// }
//
// async fn logout(session: Session, pool: web::Data<PgPool>) -> impl Responder {
//     if let Some(session_token) = session.get::<String>("session_token").unwrap() {
//         sqlx::query!(
//             "DELETE FROM user_session WHERE session_token = $1",
//             session_token
//         )
//             .execute(pool.get_ref())
//             .await
//             .unwrap();
//     }
//     session.clear();
//     HttpResponse::Ok().json("Sesión cerrada")
// }
//
// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL no definido");
//     let pool = PgPool::connect(&database_url).await.unwrap();
//
//     let session_store = DatabaseSessionStore::new(pool.clone()).await.unwrap();
//
//     HttpServer::new(move || {
//         App::new()
//             .app_data(web::Data::new(pool.clone()))
//             .wrap(IdentityMiddleware::default())
//             .wrap(SessionMiddleware::new(session_store.clone()))
//             .route("/login", web::post().to(login))
//             .route("/validate_session", web::get().to(validate_session))
//             .route("/logout", web::post().to(logout))
//     })
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
// }
