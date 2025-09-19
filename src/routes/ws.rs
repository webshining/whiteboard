use std::sync::Arc;

use crate::{board::BoardData, room::Manager};
use mediasoup::prelude::WorkerManager;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef},
    layer::SocketIoLayer,
};
use tracing::debug;

pub async fn ws(worker_manager: Arc<WorkerManager>, rooms_manager: Arc<Manager>) -> SocketIoLayer {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/board", async move |s: SocketRef| {
        let worker_manager = worker_manager.clone();
        let rooms_manager = rooms_manager.clone();

        let room = rooms_manager
            .get_or_create(&worker_manager, "1".to_string())
            .await;

        // --- CLIENT ACTIONS ---
        // on connect
        s.on("connect", async move |s: SocketRef| {
            debug!("client {} joined", &s.id);
            room.add_user(s.id.to_string()).await;
        });
        // on disconnect
        s.on("disconnect", async move |s: SocketRef| {
            debug!("client {} left", &s.id);
            room.remove_user(s.id.to_string()).await;
        });

        // --- BOARD ACTIONS ---
        // send board state
        let board = room.board.read().await;
        s.emit("change", &board.data()).unwrap();

        // broadcast board change
        let room_clone = room.clone();
        s.on(
            "change",
            async move |s: SocketRef, Data::<BoardData>(data)| {
                let mut board = room_clone.board.write().await;
                board.change(data.clone());
                s.broadcast().emit("change", &data).await.unwrap();
            },
        );

        s.on()
    });

    layer
}
