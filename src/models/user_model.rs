use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserModel {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserModelCreate {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserModelUpdate {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
}