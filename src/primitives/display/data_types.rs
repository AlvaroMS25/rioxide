use pcre2::bytes::Regex;

use crate::{display::{InterpreterDisplay, RawDisplay}, interpreter::Interpreter, primitives::{Complex, DataType, LiteralNumber, Rational, Repr}};
use std::{borrow::Cow, fmt::{self, Write}, ops::Deref};

impl InterpreterDisplay for DataType<'_> {
    fn fmt(&self, f: &mut dyn Write, interpreter: &Interpreter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => s.fmt(f, interpreter),
            Self::Character(c) => CharacterDisplay(c).fmt(f, interpreter),
            Self::Regex(r) => r.fmt(f, interpreter),
            Self::Integer(i) => i.fmt(f, interpreter),
            Self::Rational(r) => r.fmt(f, interpreter),
            Self::Complex(c) => c.fmt(f, interpreter),
            Self::Floating(fl) => fl.fmt(f, interpreter),
            Self::Double(d) => d.fmt(f, interpreter),
            Self::Hex(l) | Self::Octal(l) | Self::Binary(l) => l.fmt(f, interpreter),
            Self::Bytes(b) => b.fmt(f, interpreter),
            Self::Boolean(b) => b.fmt(f, interpreter)
        }
    }
}

impl InterpreterDisplay for &str {
    fn fmt(&self, f: &mut dyn Write, _: &Interpreter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

pub struct CharacterDisplay<'a>(pub &'a Cow<'a, str>);

impl InterpreterDisplay for CharacterDisplay<'_> {
    fn fmt(&self, f: &mut dyn Write, _: &Interpreter<'_>) -> fmt::Result {
        write!(f, "#\\{}", self.0)
    }
}

impl InterpreterDisplay for Regex {
    fn fmt(&self, f: &mut dyn Write, _: &Interpreter<'_>) -> fmt::Result {
        write!(f, "#rx\"{}\"", self.as_str())
    }
}

impl InterpreterDisplay for i32 {
    fn fmt(&self, f: &mut dyn Write, _: &Interpreter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl InterpreterDisplay for Rational {
    fn fmt(&self, f: &mut dyn Write, _: &Interpreter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.left, self.right)
    }
}

impl InterpreterDisplay for Complex {
    fn fmt(&self, f: &mut dyn Write, _: &Interpreter<'_>) -> fmt::Result {
        let real_sign = if self.real >= 0 { "+" } else { "-" };
        let prefix = if self.includes_prefix { real_sign } else { "" };
        let im_sign = if self.imaginary >= 0 { "+" } else { "-" };
        write!(f, "{}{}{}{}i", prefix, self.real, im_sign, self.imaginary)
    }
}

impl InterpreterDisplay for f32 {
    fn fmt(&self, f: &mut dyn Write, _: &Interpreter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl InterpreterDisplay for f64 {
    fn fmt(&self, f: &mut dyn Write, _: &Interpreter<'_>) -> fmt::Result {
        write!(f, "{self:e}")
    }
}

impl InterpreterDisplay for LiteralNumber<'_> {
    fn fmt(&self, f: &mut dyn Write, _: &Interpreter<'_>) -> fmt::Result {
        let prefix = match self.repr {
            Repr::Hex => "#x",
            Repr::Octal => "#o",
            Repr::Binary => "#b"
        };

        write!(f, "{}{}", prefix, self.inner)
    }
}

impl InterpreterDisplay for Cow<'_, [u8]> {
    fn fmt(&self, f: &mut dyn Write, _: &Interpreter<'_>) -> fmt::Result {
        let s = std::str::from_utf8(self)
            .expect("Must be utf8 to be printed");

        write!(f, "#\"{s}\"")
    }
}

impl InterpreterDisplay for Cow<'_, str> {
    fn fmt(&self, f: &mut dyn Write, interpreter: &Interpreter<'_>) -> fmt::Result {
        self.deref().fmt(f, interpreter)
    }
}

impl InterpreterDisplay for bool {
    fn fmt(&self, f: &mut dyn Write, _: &Interpreter<'_>) -> fmt::Result {
        write!(f, "#{}", if *self { 't' } else { 'f' })
    }
}

impl RawDisplay for DataType<'_> {
    fn raw_fmt(&self, f: &mut dyn Write, interpreter: &Interpreter<'_>) -> fmt::Result {
        self.fmt(f, interpreter)
    }
}
