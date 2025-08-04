use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserAuthModel {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserAuthentication {
    pub jwt: String,
    pub name: String,
    pub email: String,
}