use axum_learn_by_vee::{config::*, routes::v1::app_root_route};
use tracing::info;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let port = &*API_POST;
    let app = app_root_route();
    let ip_server = format!("0.0.0.0:{port}");

    info!("Server is running on: {}", &ip_server);

    let listener = tokio::net::TcpListener::bind(&ip_server).await.unwrap();
    axum::serve(listener, app).await.unwrap();

}
