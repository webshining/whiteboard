use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use super::room::Room;

pub struct Manager {
    pub rooms: RwLock<HashMap<String, Arc<Room>>>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            rooms: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_or_create(&self, id: String) -> Arc<Room> {
        {
            let rooms = self.rooms.read().await;
            if let Some(room) = rooms.get(&id) {
                return room.clone();
            }
        }

        let mut rooms = self.rooms.write().await;
        if let Some(room) = rooms.get(&id) {
            return room.clone();
        }

        let room = Arc::new(Room::new());
        rooms.insert(id, room.clone());
        room
    }
}
