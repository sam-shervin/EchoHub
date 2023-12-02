use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}
