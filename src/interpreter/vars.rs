use std::collections::HashMap;

use parking_lot::RwLock;

use crate::interpreter::any::Any;

pub struct VarsStorage<'a> {
    table: HashMap<String, RwLock<Any<'a>>>
}
