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
