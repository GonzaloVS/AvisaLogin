use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{NaiveDateTime};
use base64::engine::general_purpose;
use base64::Engine;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserData {
    pub account_id: i64,
    pub account_name: String,
    #[sqlx(default, try_from = "NaiveDateTime")]
    pub account_create_dateutc: NaiveDateTime,
    pub google_id: String,
    pub google_email: String,
    pub google_refresh_token: Option<String>,
    pub google_profile_pic_url: Option<String>,
    #[sqlx(try_from = "NaiveDateTime")]
    pub google_last_login_dateutc: Option<NaiveDateTime>,
    #[sqlx(try_from = "NaiveDateTime")]
    pub last_activity_dateutc: Option<NaiveDateTime>,
    pub status: String,
    pub suspension_reason: Option<String>,
    #[sqlx(default, try_from = "NaiveDateTime")]
    pub deactivation_date: NaiveDateTime,
    pub account_password_hash: String,
}


#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub account_id: i64,
    pub account_name: String,
    pub google_email: String,
    pub google_profile_pic: Option<String>,
}

impl UserData {
    /// Convierte `UserData` en `UserResponse`, codificando la imagen en Base64
    pub fn to_response(&self) -> UserResponse {
        UserResponse {
            account_id: self.account_id,
            account_name: self.account_name.clone(),
            google_email: self.google_email.clone(),
            google_profile_pic: self.google_profile_pic_url.as_ref().map(|pic| general_purpose::STANDARD.encode(pic)),  // Convierte a Base64
        }
    }
}
