use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::models::board::Board;

#[derive(Debug)]
pub struct Room {
    pub board: RwLock<Board>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            board: RwLock::new(Board::new()),
        }
    }
}

pub type Rooms = Arc<RwLock<HashMap<String, Arc<Room>>>>;
