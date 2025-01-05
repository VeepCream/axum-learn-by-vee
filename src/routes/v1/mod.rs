pub mod test_route;
pub mod login;

use utoipa_axum::router::OpenApiRouter;

lazy_static::lazy_static! {
    #[derive(Debug)]
    static ref VERSION: String = "V1".to_string();
    #[derive(Debug)]
    static ref ROUTE_NAME: String = format!("{:?}", &*VERSION);
}

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .nest("/test_route", test_route::router())
        .nest("/login", login::router())
}
