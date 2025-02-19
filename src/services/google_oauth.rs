use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleAuthResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub id_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: String,
}

pub async fn exchange_code_for_token(code: &str, client_id: &str, client_secret: &str, redirect_uri: &str) -> Result<GoogleAuthResponse, reqwest::Error> {
    let client = Client::new();
    let params = [
        ("code", code),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("redirect_uri", redirect_uri),
        ("grant_type", "authorization_code"),
    ];

    let res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await?
        .json::<GoogleAuthResponse>()
        .await?;

    Ok(res)
}

pub async fn get_user_info(access_token: &str) -> Result<GoogleUserInfo, reqwest::Error> {
    let client = Client::new();
    let res = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(access_token)
        .send()
        .await?
        .json::<GoogleUserInfo>()
        .await?;

    Ok(res)
}
