use std::sync::Arc;

use crate::{board::BoardData, room::Manager};
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef},
    layer::SocketIoLayer,
};

pub async fn ws(rooms_manager: Arc<Manager>) -> SocketIoLayer {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/board", async move |s: SocketRef| {
        let rooms_manager = rooms_manager.clone();
        let room = rooms_manager.get_or_create("1".to_string()).await;

        let board = room.board.read().await;
        s.emit("change", &board.data()).unwrap();

        let room_clone = room.clone();
        s.on(
            "change",
            async move |s: SocketRef, Data::<BoardData>(data)| {
                let mut board = room_clone.board.write().await;
                board.change(data.clone());
                s.broadcast().emit("change", &data).await.unwrap();
            },
        );
    });

    layer
}
