use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
};

use crate::{board::BoardData, room::Manager};
use mediasoup::prelude::{
    ListenInfo, Protocol, Transport, WebRtcTransportListenInfos, WebRtcTransportOptions,
    WorkerManager,
};
use socketioxide::{
    SocketIo,
    extract::{AckSender, Data, SocketRef},
    layer::SocketIoLayer,
};
use tracing::info;

pub async fn ws(worker_manager: Arc<WorkerManager>, rooms_manager: Arc<Manager>) -> SocketIoLayer {
    let (layer, io) = SocketIo::new_layer();

    io.ns("/", async move |s: SocketRef| {
        let worker_manager = worker_manager.clone();
        let rooms_manager = rooms_manager.clone();

        let room = rooms_manager
            .get_or_create(&worker_manager, "1".to_string())
            .await;

        // --- CLIENT ACTIONS ---
        // -- connect --
        info!("client {} joined", &s.id);
        room.add_user(s.id.to_string()).await;
        // -- diconnect --
        {
            let room = room.clone();
            s.on_disconnect(async move |s: SocketRef| {
                info!("client {} left", &s.id);
                room.remove_user(s.id.to_string()).await;
            });
        }

        // --- WEBRTC ACTIONS ---
        {
            let room = room.clone();
            s.on("rtpCapabilities", move |ack: AckSender| {
                let rtp_capabilities = room.router.rtp_capabilities();
                ack.send(&rtp_capabilities).unwrap();
            });
        }
        {
            let room = room.clone();
            let transport_options =
                WebRtcTransportOptions::new(WebRtcTransportListenInfos::new(ListenInfo {
                    protocol: Protocol::Udp,
                    ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
                    announced_address: None,
                    expose_internal_ip: false,
                    port: None,
                    port_range: None,
                    flags: None,
                    send_buffer_size: None,
                    recv_buffer_size: None,
                }));
            s.on("createTransport", async move |ack: AckSender| {
                let transport = room
                    .router
                    .create_webrtc_transport(transport_options)
                    .await
                    .unwrap();
                let options = serde_json::json!({
                    "id": transport.id(),
                    "iceParameters": transport.ice_parameters(),
                    "iceCandidates": transport.ice_candidates(),
                    "dtlsParameters": transport.dtls_parameters()
                });
                ack.send(&options).unwrap();
            });
        }
        {
            s.on("connectTransport", async move |ack: AckSender| {
                ack.send(&true).unwrap();
            })
        }

        // --- BOARD ACTIONS ---
        // -- send board state --
        {
            let board = room.board.read().await;
            s.emit("change", &board.data()).unwrap();
        }
        // -- broadcast board change --
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
