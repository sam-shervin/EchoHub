use serde::Deserialize;

#[cfg(test)]
use serde::Serialize;

#[derive(Clone, Debug, Deserialize)]
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
