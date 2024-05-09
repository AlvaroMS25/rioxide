use std::{collections::HashMap, fmt, ops::Deref};

use parking_lot::RwLock;

use crate::{cell::Cell, container::VarsContainer, primitives::any::Any};

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

pub struct OwnedStorage {
    table: HashMap<String, Cell<Any<'static>>>
}

impl OwnedStorage {
    pub fn new() -> Self {
        Self {
            table: HashMap::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            table: HashMap::with_capacity(capacity)
        }
    }

    pub fn get(&self, key: &str) -> Option<&Any<'static>> {
        self.table.get(key).map(|c| c.deref())
    }

    pub fn insert(&mut self, key: &str, value: Any<'_>) {
        self.table.insert(key.to_string(), Cell::new(value.make_static()));
    }
}

impl<'a> VarsContainer<'a> for VarsStorage<'a> {
    fn get(&self, key: &str) -> Option<&Any> {
        self.get(key)
    }

    fn insert(&mut self, key: &str, value: Any<'a>) {
        self.insert(key, value)
    }
}

impl<'b> VarsContainer<'b> for OwnedStorage {
    fn get(&self, key: &str) -> Option<&Any> {
        self.get(key)
    }

    fn insert(&mut self, key: &str, value: Any<'b>) {
        self.insert(key, value)
    }
}

impl fmt::Debug for VarsStorage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <HashMap<String, Cell<Any<'_>>> as fmt::Debug>::fmt(&self.table, f)
    }
}

impl fmt::Debug for OwnedStorage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <HashMap<String, Cell<Any<'static>>> as fmt::Debug>::fmt(&self.table, f)
    }
}
