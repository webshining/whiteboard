use tokio::sync::RwLock;

use crate::board::Board;

pub struct Room {
    pub board: RwLock<Board>,
}

impl Room {
    pub async fn new() -> Self {
        Self {
            board: RwLock::new(Board::new()),
        }
    }

    pub async fn add_user(&self, id: String) {}

    pub async fn remove_user(&self, id: String) {}
}
