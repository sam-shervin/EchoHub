use std::sync::Arc;

use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[cfg(test)]
impl SignupRequest {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            username,
            email,
            password,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct TokenResponse {
    pub user_id: String,
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub scope: String,
}

impl TokenResponse {
    pub fn new(
        user_id: String,
        token_type: String,
        access_token: String,
        refresh_token: String,
        expires_in: u64,
        scope: String,
    ) -> Self {
        Self {
            user_id,
            token_type,
            access_token,
            refresh_token,
            expires_in,
            scope,
        }
    }
}
