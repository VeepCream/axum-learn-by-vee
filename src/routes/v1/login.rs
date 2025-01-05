use anyhow::Result;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use serde::Deserialize;
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use chrono::{Duration, Utc};

use crate::config;
use crate::utils::jwt_authentication;
use crate::utils::security_addon::SecurityAddon;
use crate::utils::jwt_authentication::jwt_model::{Claims, Passport};

const ROUTE_NAME: &str = "v1/login";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = ROUTE_NAME, description = "login"),
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(login))
}

#[utoipa::path(
    post, 
    path = "", responses(
        (status = StatusCode::OK, body = Passport),
        (status = StatusCode::BAD_GATEWAY, body = Passport)
    ),
    tag = ROUTE_NAME)]
async fn login(Json(payload): Json<LoginModel>) -> Result<impl IntoResponse, (StatusCode, String)> {

    let access_token_claims = Claims {
        id: 1,
        exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
    };

    let refresh_token_claims = Claims {
        id: 1,
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
    };

    let access_token =
        jwt_authentication::generate_token(config::SECRET_ENV.to_string(), &access_token_claims)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let refresh_token =
        jwt_authentication::generate_token(config::SECRET_ENV.to_string(), &refresh_token_claims)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;


    let passport = Passport {
        access_token: access_token,
        refresh_token: refresh_token,
    };

    Ok((StatusCode::OK, Json(passport)))
}

#[derive(ToSchema, Deserialize)]
struct LoginModel {
    username: String,
    password: String,
}