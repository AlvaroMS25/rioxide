use std::fmt::{self, Display, Write};

use crate::interpreter::Interpreter;

pub trait InterpreterDisplay {
    fn fmt(&self, f: &mut dyn Write, interpreter: &Interpreter<'_>) -> fmt::Result;
}

pub trait RawDisplay {
    fn raw_fmt(&self, f: &mut dyn Write, interpreter: &Interpreter<'_>) -> fmt::Result;
}
