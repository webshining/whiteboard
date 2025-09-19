use std::collections::HashMap;

use mediasoup::prelude::{Producer, Router, RouterOptions, WorkerManager, WorkerSettings};
use tokio::sync::{Mutex, RwLock};

use crate::board::Board;

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
            .create_router(RouterOptions::default())
            .await
            .unwrap();
        Self {
            board: RwLock::new(Board::new()),
            router,
            clients: Mutex::new(HashMap::new()),
        }
    }

    pub async fn add_user(&mut self, id: String) {
        let mut clients = self.clients.lock().await;
        clients.insert(id, Vec::new());
    }

    pub async fn remove_user(&mut self, id: String) {
        let mut clients = self.clients.lock().await;
        clients.remove(&id);
    }
}
