use std::{
    collections::HashMap,
    num::{NonZeroU8, NonZeroU32},
};

use mediasoup::prelude::{
    MimeTypeAudio, Producer, Router, RouterOptions, RtcpFeedback, RtpCodecCapability,
    RtpCodecParametersParameters, WorkerManager, WorkerSettings,
};
use tokio::sync::{Mutex, RwLock};

use crate::board::Board;

fn media_codecs() -> Vec<RtpCodecCapability> {
    vec![RtpCodecCapability::Audio {
        mime_type: MimeTypeAudio::Opus,
        preferred_payload_type: None,
        clock_rate: NonZeroU32::new(48000).unwrap(),
        channels: NonZeroU8::new(2).unwrap(),
        parameters: RtpCodecParametersParameters::from([("useinbandfec", 1_u32.into())]),
        rtcp_feedback: vec![RtcpFeedback::TransportCc],
    }]
}

pub struct Room {
    pub board: RwLock<Board>,
    pub router: Router,
    pub clients: Mutex<HashMap<String, Vec<Producer>>>,
}

impl Room {
    pub async fn new(worker_manager: &WorkerManager) -> Self {
        let worker = worker_manager
            .create_worker(WorkerSettings::default())
            .await
            .unwrap();
        let router = worker
            .create_router(RouterOptions::new(media_codecs()))
            .await
            .unwrap();
        Self {
            board: RwLock::new(Board::new()),
            router,
            clients: Mutex::new(HashMap::new()),
        }
    }

    pub async fn add_user(&self, id: String) {
        let mut clients = self.clients.lock().await;
        clients.insert(id, Vec::new());
    }

    pub async fn remove_user(&self, id: String) {
        let mut clients = self.clients.lock().await;
        clients.remove(&id);
    }
}
