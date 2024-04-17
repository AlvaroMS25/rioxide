use std::{borrow::Cow, str::Chars};

pub enum DataType<'a> {
    String(Cow<'a, str>),
    Integer(i32),
    Rational(Rational),
    Complex(Complex),
    Floating(f32),
    Double(f64),
    Hex(i64),
    Octal(i64),
    Binary(i64),
    Bytes(Cow<'a, [u8]>),
    Boolean(bool),
    Symbol(&'a str)
}

pub struct Rational {
    left: i32,
    right: i32
}

pub struct Complex {
    real: i32,
    imaginary: i32
}

impl<'a> DataType<'a> {
    pub fn parse(item: &'a str) -> Option<Self> {
        let first = item.chars().next()?;

        match first {
            '\'' => Some(DataType::Symbol(item)),
            '#' => Self::parse_prefixed(item),
            '"' => Some(DataType::String(Cow::Borrowed(&item[1..item.len() - 2]))),
            _ => Self::parse_number(item)
        }
    }

    fn parse_prefixed(item: &'a str) -> Option<DataType<'a>> {
        todo!()
    }

    fn parse_number(item: &'a str) -> Option<DataType<'a>> {
        todo!()
    }
}
