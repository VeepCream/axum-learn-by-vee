use axum::middleware;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::utils::middleware::token_authorization;
use crate::utils::security_addon::SecurityAddon;

const ROUTE_NAME: &str = "v1/test_route/one_test";

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = ROUTE_NAME, description = "one_test"),
    ),
    modifiers(&SecurityAddon)
)]
struct ApiDoc;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .routes(routes!(add, edit))
        .route_layer(middleware::from_fn(token_authorization))
}


#[utoipa::path(
    post, 
    path = "", responses(
        (status = StatusCode::OK, body = CreateUser),
        (status = StatusCode::BAD_GATEWAY, body = CreateUser)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = ROUTE_NAME)]
async fn add(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
#[utoipa::path(
    put, 
    path = "", responses(
        (status = StatusCode::OK, body = CreateUser),
        (status = StatusCode::BAD_GATEWAY, body = CreateUser)
    ),
    security(
        ("bearerAuth" = [])
    ),
    tag = ROUTE_NAME)]
async fn edit(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };
    (StatusCode::OK, Json(user))
}

#[derive(ToSchema, Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(ToSchema, Serialize)]
struct User {
    id: u64,
    username: String,
}
