use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub username: String,
    pub email: String,
}
