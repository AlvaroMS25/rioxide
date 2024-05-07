use core::fmt;

pub trait InterpreterDisplay {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result;
}
