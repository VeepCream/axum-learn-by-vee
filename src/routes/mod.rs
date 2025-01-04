pub mod v1;

use utoipa_axum::router::OpenApiRouter;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/v1", v1::test_route::router())
}