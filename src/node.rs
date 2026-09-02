use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolNode {
    pub id: String,
    pub connections: HashMap<String, f64>,
}

impl SymbolNode {
    pub fn new(id: &str) -> Self {
        SymbolNode {
            id: id.to_string(),
            connections: HashMap::new(),
        }
    }

    pub fn link(&mut self, target_id: &str, strength: f64) {
        self.connections.insert(target_id.to_string(), strength);
    }
}
