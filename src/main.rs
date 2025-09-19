use std::sync::Arc;

use axum::{Router, routing::get};
use mediasoup::prelude::WorkerManager;
use tower_http::{cors::CorsLayer, services::ServeDir};

mod board;
mod room;
mod routes;
use crate::{
    room::Manager,
    routes::{root::root, ws::ws},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let worker_manager = Arc::new(WorkerManager::new());
    let rooms_manager = Arc::new(Manager::new());

    let app = Router::new()
        .route("/", get(root))
        .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
        .layer(ws(worker_manager.clone(), rooms_manager.clone()).await)
        .layer(CorsLayer::permissive())
        .with_state(worker_manager.clone())
        .with_state(rooms_manager.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:4000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
