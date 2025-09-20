use std::sync::Arc;

use axum::{Router, routing::get};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;

mod config;

mod board;
mod room;
mod routes;

use crate::{
    config::Config,
    room::Manager,
    routes::{root::root, ws::ws},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = Config::new();
    let rooms_manager = Arc::new(Manager::new());

    let app = Router::new()
        .route("/", get(root))
        .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
        .layer(ws(rooms_manager.clone()).await)
        .layer(CorsLayer::permissive())
        .with_state(rooms_manager.clone());

    info!("server started at http://{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
