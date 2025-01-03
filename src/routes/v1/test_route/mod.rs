use axum::Router;
pub mod one_test;
pub mod two_test;

pub fn get_test_route() -> Router {
    let app = Router::new()
        .nest("/one_test", one_test::routes())
        .nest("/two_test", two_test::routes());
    return app;
}