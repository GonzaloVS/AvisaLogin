use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserData {
    pub id: i32,
    pub username: String,
    pub email: String,
}
