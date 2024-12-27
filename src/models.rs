use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;


#[derive(Debug, Clone, Serialize, Deserialize,FromRow, ToSchema)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub password: String
}
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub email: String,
    pub name: String,
    pub role: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub status: String,
    pub status_code: String,
    pub data: Option<T>,
}
