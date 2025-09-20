use std::sync::Arc;

use crate::{board::BoardData, room::Manager};
use socketioxide::{
    SocketIo,
    extract::{AckSender, Data, SocketRef},
    layer::SocketIoLayer,
};
use tracing::info;

pub async fn ws(rooms_manager: Arc<Manager>) -> SocketIoLayer {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", async move |s: SocketRef| {
        let rooms_manager = rooms_manager.clone();

        let room = rooms_manager.get_or_create("1".to_string()).await;

        // --- CLIENT ACTIONS ---
        info!("client {} joined", &s.id);
        room.add_user(s.id.to_string()).await;
        {
            let room = room.clone();
            s.on_disconnect(async move |s: SocketRef| {
                info!("client {} left", &s.id);
                room.remove_user(s.id.to_string()).await;
            });
        }

        // --- BOARD ACTIONS ---
        {
            let room = room.clone();
            s.on("boardState", async move |ack: AckSender| {
                let board = room.board.read().await;
                let state = board.data();
                ack.send(&state).unwrap();
            })
        }
        {
            let room = room.clone();
            s.on(
                "change",
                async move |s: SocketRef, Data::<BoardData>(data)| {
                    let mut board = room.board.write().await;
                    board.change(data.clone());
                    s.broadcast().emit("change", &data).await.unwrap();
                },
            );
        }
    });

    layer
}
