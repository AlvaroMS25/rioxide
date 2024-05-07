use std::{cell::UnsafeCell, ops::Deref};

pub struct Cell<T> {
    inner: UnsafeCell<T>
}

impl<T> Cell<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: UnsafeCell::new(inner)
        }
    }

    pub unsafe fn get_mut_unchecked(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }
}

impl<T> Deref for Cell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.inner.get() }
    }
}

impl<T: Clone> Clone for Cell<T> {
    fn clone(&self) -> Self {
        Self {
            inner: UnsafeCell::new(self.deref().clone())
        }
    }
}
