use std::{borrow::Cow, str::Chars};
use lazy_static::lazy_static;
use crate::{lexer::Token, macros::*};

use pcre2::bytes::Regex;

lazy_static! {
    static ref COMPLEX_REGEX: Regex = Regex::new(r"[+-]?(((\d+\.\d*|\d*\.\d+|\d+)[+-])?((\d+\.\d*|\d*\.\d+|\d+)i|i(\d+\.\d*|\d*\.\d+|\d+)|i)|(\d+\.\d*|\d*\.\d+|\d+)?e\^(\([+-]?|[+-]?\()((\d+\.\d*|\d*\.\d+|\d+)i|i(\d+\.\d*|\d*\.\d+|\d+)|i)\))").unwrap();
}

get_enum! {
    #[derive(Debug, Clone)]
    pub enum DataType<'a> {
        String(Cow<'a, str>),
        Character(Cow<'a, str>),
        #[allow(unused)]
        Regex(Regex), // not parsed right now
        Integer(i32),
        Rational(Rational),
        Complex(Complex),
        Floating(f32),
        Double(f64),
        Hex(LiteralNumber<'a>),
        Octal(LiteralNumber<'a>),
        Binary(LiteralNumber<'a>),
        Bytes(Cow<'a, [u8]>),
        Boolean(bool),
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rational {
    left: i32,
    right: i32
}

#[derive(Debug, Copy, Clone)]
pub struct Complex {
    includes_prefix: bool,
    real: i32,
    imaginary: i32
}

#[derive(Debug, Copy, Clone)]
pub enum Repr {
    Hex,
    Octal,
    Binary
}

#[derive(Debug, Clone)]
pub struct LiteralNumber<'a> {
    inner: Cow<'a, str>,
    #[allow(unused)]
    repr: Repr
} 

impl<'a> DataType<'a> {
    pub fn len(&self) -> usize {
        use DataType::*;

        match self {
            String(item) | Character(item) => len_u8buf(&item.as_ref()),
            Regex(r) => r.as_str().len(),
            Integer(i) => len_num(*i),
            Rational(r) => len_num(r.left) + len_num(r.right) + 1, // +1 for "/" character
            Complex(c) => len_num(c.real) 
                + len_num(c.imaginary) + 1 // + 1 for i
                + if c.includes_prefix { 1 } else { 0 } // +1 if the number is prefixed with "+"
                + if c.imaginary > 0 { 1 } else { 0 }, // +1 if the imaginary part is positive, because len_num does not detect it
            Floating(f) => {
                let l = len_num(*f);
                //println!("floating with value: {f}, len: {l}");
                l
            },
            Double(d) => len_num(*d),
            Hex(n) | Octal(n) | Binary(n) => n.inner.len(),
            Bytes(b) => len_u8buf(&b.as_ref()),
            Boolean(_) => 2, // #t or #f
        }
    }

    pub fn parse(item: &'a str) -> Option<Self> {
        let first = item.chars().next()?;

        match first {
            '#' => Self::parse_prefixed(item),
            '"' => Self::parse_str(item).map(DataType::String),
            _ if item.chars().filter(|c| c.is_numeric()).next().is_some() => Self::parse_number(item),
            _ => None
        }
    }

    fn parse_str(item: &'a str) -> Option<Cow<'a, str>> {
        let mut characters = item.char_indices();
        let mut until = 0;

        while let Some((idx, next_char)) = characters.next() {
            if next_char == '\\' {
                characters.next().unwrap(); // skip the escaped character
                continue;
            }

            until = idx;

            if next_char == '"' && idx > 0 {
                break;
            }
        }

        Some(Cow::Borrowed(&item[..=until]))
    }

    fn parse_prefixed(item: &'a str) -> Option<DataType<'a>> {
        sw!(item, "#t", || Some(DataType::Boolean(true)));
        sw!(item, "#f", || Some(DataType::Boolean(false)));
        sw!(item, "#\"", || Some(DataType::Bytes(Cow::Borrowed(&item[1..].as_bytes()))));
        sw!(item, "#\\", || Some(DataType::Character(Cow::Borrowed(&item[2..]))));
        sw!(item, "#x", || Some(DataType::Hex(Self::parse_literal_number(Repr::Hex, item))));
        sw!(item, "#o", || Some(DataType::Octal(Self::parse_literal_number(Repr::Octal, item))));
        sw!(item, "#b", || Some(DataType::Binary(Self::parse_literal_number(Repr::Binary, item))));

        None
    }

    fn parse_literal_number(repr: Repr, item: &'a str) -> LiteralNumber<'a> {
        let num = Self::take_numbers(&item[2..]);
        LiteralNumber {
            repr,
            inner: Cow::Borrowed(&item[0.. num.len() + 2])
        }
    }

    fn parse_number(item: &'a str) -> Option<DataType<'a>> {
        c!(item, "/", || Self::parse_rational(item));
        c!(item, "i", || Self::parse_complex(item));
        c!(item, "e", || Some(DataType::Double(item.parse::<f64>().ok()?)));
        c!(item, ".", || Some(DataType::Floating(item.parse::<f32>().ok()?)));

        Some(DataType::Integer(Self::take_numbers(item).parse::<i32>().ok()?))
    }

    fn take_numbers(item: &'a str) -> &'a str {
        let mut buf = &item[0..1];
        let mut chars = item.char_indices().skip(1);

        while let Some((idx, curr)) = chars.next() {
            if curr.is_numeric() {
                buf = &item[0..=idx];
            } else {
                break;
            }
        }

        buf
    }

    fn parse_complex(item: &'a str) -> Option<DataType<'a>> {
        use std::str::from_utf8;
        
        let captures = COMPLEX_REGEX.captures(item.as_bytes()).ok()??;

        if captures.len() == 0 {
            return None;
        }

        let minus_real = captures.get(0).map(|c| {
            let content = from_utf8(c.as_bytes()).unwrap();
            &content[0..1] == "-"
        }).unwrap_or(false);

        let minus_im = captures.get(2).map(|c| {
            let content = from_utf8(c.as_bytes()).unwrap();
            &content[content.len()-1 ..] == "-"
        }).unwrap_or(false);

        Some(DataType::Complex(Complex {
            includes_prefix: minus_real || captures.get(0).map(|c| {
                let parsed = from_utf8(c.as_bytes()).unwrap();
                &parsed[0..1] == "-" || &parsed[0..1] == "+"
            }).unwrap_or(false),
            real: {
                let parsed = captures.get(3).map(|m| from_utf8(m.as_bytes()).unwrap())?
                    .parse::<i32>().ok()?;

                if minus_real {
                    -parsed
                } else {
                    parsed
                }
            },
            imaginary: {
                let parsed = captures.get(5).map(|m| from_utf8(m.as_bytes()).unwrap())?
                    .parse::<i32>().ok()?;

                if minus_im {
                    -parsed
                } else {
                    parsed
                }
            }
        }))
    }

    fn parse_rational(item: &'a str) -> Option<DataType<'a>> {
        let mut split = item.split("/");
        let (first, second) = (split.next()?, split.next()?);

        Some(DataType::Rational(Rational {
            left: first.parse().ok()?,
            right: Self::take_numbers(second).parse().ok()?,
        }))
    }

    fn parse_num(item: &'a str) -> Option<i64> {
        item.parse().ok()
    }
}

fn len_u8buf<B: AsRef<[u8]>>(item: &B) -> usize {
    item.as_ref().len()
}

fn len_num<T>(item: T) -> usize
where
    T: ToString
{
    item.to_string().len()
}

#[test]
fn test_len() {
    println!("{}", (3.2f64).to_string().len());
}
