use std::{cell::UnsafeCell, fmt, ops::Deref};

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

impl<T: fmt::Debug> fmt::Debug for Cell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <T as fmt::Debug>::fmt(self, f)
    }
}
