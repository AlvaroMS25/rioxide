use std::fmt::{self, Write};

use crate::{display::InterpreterDisplay, interpreter::Interpreter, primitives::any::Any};

impl InterpreterDisplay for Any<'_> {
    fn fmt(&self, f: &mut dyn Write, interpreter: &Interpreter<'_>) -> fmt::Result {
        match self {
            Any::Primitive(p) => p.fmt(f, interpreter),
            Any::Composed(c) => c.fmt(f, interpreter),
            _ => Ok(())
        }
    }
}