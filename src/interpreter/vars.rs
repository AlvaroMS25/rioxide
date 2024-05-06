use std::collections::HashMap;

use parking_lot::RwLock;

use crate::primitives::any::Any;

#[derive(Clone)]
pub struct VarsStorage<'a> {
    table: HashMap<String, Any<'a>>
}

impl<'a> VarsStorage<'a> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Any<'a>> {
        self.table.get(key)
    }

    pub fn insert(&mut self, key: impl ToString, value: Any<'a>) {
        self.table.insert(key.to_string(), value);
    }
}
