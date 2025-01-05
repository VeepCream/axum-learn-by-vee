use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct Passport {
    pub refresh_token: String,
    pub access_token: String,
}

#[derive(ToSchema, Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: usize,
    pub iat: usize,
}