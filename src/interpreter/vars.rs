use std::{collections::HashMap, ops::Deref};

use parking_lot::RwLock;

use crate::{cell::Cell, primitives::any::Any};

#[derive(Clone)]
pub struct VarsStorage<'a> {
    table: HashMap<String, Cell<Any<'a>>>
}

impl<'a> VarsStorage<'a> {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            table: HashMap::with_capacity(capacity)
        }
    }

    pub fn get(&self, key: &str) -> Option<&Any<'a>> {
        self.table.get(key).map(|c| c.deref())
    }

    pub fn insert(&mut self, key: impl ToString, value: Any<'a>) {
        self.table.insert(key.to_string(), Cell::new(value));
    }
}
