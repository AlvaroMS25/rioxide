use std::fmt::Debug;

use crate::primitives::any::Any;

pub trait VarsContainer<'a>: Debug {
    fn get(&self, key: &str) -> Option<&Any>;
    fn insert(&mut self, key: &str, value: Any<'a>);
}
