use std::fmt::{self, Display};

pub trait InterpreterDisplay {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl<T> InterpreterDisplay for T
where
    T: Display
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, fmt)
    }
}
