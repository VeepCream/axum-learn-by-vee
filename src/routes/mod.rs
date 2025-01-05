pub mod v1;

use utoipa_axum::router::OpenApiRouter;
use axum::http::Method;
use std::time::Duration;
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/v1", v1::router())
        .layer(TimeoutLayer::new(Duration::from_secs(5)))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::DELETE,
                    Method::PUT,
                    Method::PATCH,
                ])
                .allow_origin(Any),
        )
        .layer(TraceLayer::new_for_http())
}