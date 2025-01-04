pub mod one_test;
pub mod two_test;

use utoipa_axum::router::OpenApiRouter;
lazy_static::lazy_static! {
    #[derive(Debug)]
    static ref ROUTE_NAME: String = format!("{:?}-->>test_route",&*super::ROUTE_NAME);
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/one_test", one_test::router())
        .nest("/two_test", two_test::router())

}