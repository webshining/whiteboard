use axum::{Router, routing::get};
use tower_http::{cors::CorsLayer, services::ServeDir};

mod models;
mod routes;
use crate::routes::{root::root, ws::ws};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/", get(root))
        .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
        .layer(ws().await)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
