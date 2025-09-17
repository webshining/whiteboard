use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

pub struct Board {
    pub elements: HashMap<String, Value>,
    pub files: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BoardData {
    pub elements: Vec<Value>,
    pub files: Map<String, Value>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
            files: HashMap::new(),
        }
    }

    pub fn change(&mut self, data: BoardData) {
        for element in data.elements {
            let id = element.get("id").unwrap().to_string();
            self.elements.insert(id, element);
        }
        for (id, file) in data.files {
            self.files.insert(id, file);
        }
    }

    pub fn data(&self) -> BoardData {
        let elements = self.elements.values().cloned().collect();
        let mut files = Map::new();
        for (k, v) in self.files.clone() {
            files.insert(k, v);
        }

        BoardData {
            elements: elements,
            files: files,
        }
    }
}
