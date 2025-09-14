use std::{collections::HashMap, sync::Arc};

use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef},
    layer::SocketIoLayer,
};
use tokio::sync::RwLock;
use tracing::debug;

use crate::models::{
    board::BoardData,
    room::{Room, Rooms},
};

pub async fn ws() -> SocketIoLayer {
    let (layer, io) = SocketIo::new_layer();
    let rooms: Rooms = Arc::new(RwLock::new(HashMap::new()));

    let room = {
        let mut rooms_guard = rooms.write().await;
        rooms_guard
            .entry("1".to_string())
            .or_insert_with(|| Arc::new(Room::new()))
            .clone()
    };

    io.ns("/board", async move |s: SocketRef| {
        debug!("connected {}", s.id);
        s.on(
            "change",
            async move |s: SocketRef, Data::<BoardData>(data)| {
                let mut board = room.board.write().await;
                board.change(data.clone());
                s.broadcast().emit("change", &data).await.unwrap();
            },
        );
    });

    layer
}
