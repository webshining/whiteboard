use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct Board {
    pub elements: Map<String, Value>,
    pub files: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BoardData {
    pub elements: Vec<Value>,
    pub files: Map<String, Value>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            elements: Map::new(),
            files: Map::new(),
        }
    }

    pub fn change(&mut self, data: BoardData) {
        for element in data.elements {
            let id = element.get("id").unwrap().as_str().unwrap().to_string();
            self.elements.insert(id, element);
        }
        for (id, file) in data.files {
            self.files.insert(id, file);
        }
    }
}
