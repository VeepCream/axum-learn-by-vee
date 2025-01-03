use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn app_root_route() -> Router {
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(health_check));
    return app;
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}
