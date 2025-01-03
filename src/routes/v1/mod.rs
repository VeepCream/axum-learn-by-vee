pub mod test_route;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use test_route::get_test_route;

pub fn app_root_route() -> Router {
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(health_check))
        .nest("/test_route", get_test_route());
    return app;
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}
