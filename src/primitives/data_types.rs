use std::{borrow::Cow, str::Chars};
use lazy_static::lazy_static;

use pcre2::bytes::Regex;

lazy_static! {
    static ref COMPLEX_REGEX: Regex = Regex::new(r"(?m)^(?=[iI.\d+-])([+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?(?![iI.\d]))?([+-]?(?:(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?)?[iI])?$").unwrap();
}

macro_rules! sw {
    ($it: ident, $pat: literal, || $($tree:tt)*) => {
        if $it.starts_with($pat) {
            return { $($tree)* };
        }
    };
}

macro_rules! c {
    ($it: ident, $pat: literal, || $($tree:tt)*) => {
        if $it.contains($pat) {
            return { $($tree)* };
        }
    };
}

#[derive(Debug)]
pub enum DataType<'a> {
    String(Cow<'a, str>),
    Character(Cow<'a, str>),
    Regex(Regex),
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

#[derive(Debug)]
pub struct Rational {
    left: i32,
    right: i32
}

#[derive(Debug)]
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

    fn parse_string(item: &'a str, idx: &mut usize) -> Option<DataType<'a>> {
        todo!()
    }

    fn parse_str(item: &'a str) -> Option<Cow<'a, str>> {
        let quotes = item.chars().filter(|c| *c == '"').count();
        if !item.starts_with("\"") || !item.ends_with("\"") {
            return None;
        }
    }

    fn parse_prefixed(item: &'a str) -> Option<DataType<'a>> {
        sw!(item, "#t", || Some(DataType::Boolean(true)));
        sw!(item, "#f", || Some(DataType::Boolean(false)));
        sw!(item, "#\"", || Some(DataType::Bytes(Cow::Borrowed(&item[1..].as_bytes()))));
        sw!(item, "#\\", || Some(DataType::Character(Cow::Borrowed(&item[2..]))));
        sw!(item, "#x", || Some(DataType::Hex(item.parse::<i64>().ok()?)));
        sw!(item, "#o", || Some(DataType::Octal(item.parse::<i64>().ok()?)));
        sw!(item, "#b", || Some(DataType::Binary(item.parse::<i64>().ok()?)));

        None
    }

    fn parse_number(item: &'a str) -> Option<DataType<'a>> {
        c!(item, "/", || Self::parse_complex(item));
        c!(item, "i", || Self::parse_complex(item));
        c!(item, "e", || Some(DataType::Double(item.parse::<f64>().ok()?)));
        c!(item, ".", || Some(DataType::Floating(item.parse::<f32>().ok()?)));

        Some(DataType::Integer(item.parse::<i32>().ok()?))
    }

    fn parse_complex(item: &'a str) -> Option<DataType<'a>> {
        use std::str::from_utf8;

        let captures = COMPLEX_REGEX.captures(item.as_bytes()).ok()??;

        if captures.len() == 0 {
            return None;
        }

        Some(DataType::Complex(Complex {
            real: captures.get(1).map(|m| from_utf8(m.as_bytes()).unwrap()).unwrap_or("0").parse().ok()?,
            imaginary: captures.get(2).map(|m| from_utf8(m.as_bytes()).unwrap()).unwrap_or("0").parse().ok()?
        }))
    }

    fn parse_rational(item: &'a str) -> Option<DataType<'a>> {
        let mut split = item.split("/");
        let (first, second) = (split.next()?, split.next()?);

        Some(DataType::Rational(Rational {
            left: first.parse().ok()?,
            right: second.parse().ok()?,
        }))
    }

    fn parse_num(item: &'a str) -> Option<i64> {
        item.parse().ok()
    }
}
