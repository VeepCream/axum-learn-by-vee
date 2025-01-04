use axum_learn_by_vee::*;
use tracing::info;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;


#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let port = &*config::API_POST;
    
    let ip_server = format!("0.0.0.0:{port}");

    let (router, api) = OpenApiRouter::new()
        .nest("/api", routes::router())
        .split_for_parts();

    let app = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));
    info!("Server is running on: {}", &ip_server);

    let listener = tokio::net::TcpListener::bind(&ip_server).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
