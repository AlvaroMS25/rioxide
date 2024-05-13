use std::fmt::{self, Write};

use crate::{display::InterpreterDisplay, interpreter::Interpreter, primitives::composed::{Function, List, Pair}};

impl InterpreterDisplay for List<'_> {
    fn fmt(&self, f: &mut dyn Write, interpreter: &Interpreter<'_>) -> fmt::Result {
        write!(f, "(")?;
        let mut first = true;

        for i in &self.0 {
            if !first {
                write!(f, " ")?;
            } else {
                first = false;
            }

            i.fmt(f, interpreter)?;
        }

        write!(f, ")")?;

        Ok(())
    }
}

impl InterpreterDisplay for Pair<'_> {
    fn fmt(&self, f: &mut dyn Write, interpreter: &Interpreter<'_>) -> fmt::Result {
        write!(f, "(")?;
        self.left.fmt(f, interpreter)?;
        write!(f, " . ")?;
        self.right.fmt(f, interpreter)?;
        write!(f, ")")
    }
}

impl InterpreterDisplay for Function<'_> {
    fn fmt(&self, f: &mut dyn Write, _: &Interpreter<'_>) -> fmt::Result {
        write!(f, "#<procedure:{}>", self.name)
    }
}
