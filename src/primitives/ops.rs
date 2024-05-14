use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

use crate::native::error::NativeFnError;

use super::{Complex, DataType};

pub enum ComparisonOperator {
    Simple(NonImaginary),
    #[allow(unused)]
    Complex(RationalOrComplex)
}

impl ComparisonOperator {
    pub fn from_primitive(prim: &DataType<'_>) -> Option<Self> {
        Some(match prim {
            DataType::Integer(i) => Self::Simple(NonImaginary(*i as _)),
            DataType::Floating(f) => Self::Simple(NonImaginary(*f as _)),
            DataType::Double(d) => Self::Simple(NonImaginary(*d)),
            DataType::Rational(r) => Self::Complex(RationalOrComplex {
                left: r.left,
                right: r.right,
            }),
            DataType::Complex(c) => Self::Complex(RationalOrComplex {
                left: c.real,
                right: c.imaginary
            }),
            _ => return None
        })
    }
}

impl Add for ComparisonOperator {
    type Output = Result<Self, NativeFnError>;

    fn add(self, rhs: Self) -> Self::Output {
        Ok(match (self, rhs) {
            (Self::Simple(s), Self::Simple(r)) => Self::Simple(s+r),
            (Self::Complex(_), Self::Complex(_)) 
                => return Err(NativeFnError::NotYetImplemented("Complex/rational addition")),
            _ => return Err(NativeFnError::InvalidOperands { expected: "Simple or complex numbers" })
        })
    }
}

impl Sub for ComparisonOperator {
    type Output = Result<Self, NativeFnError>;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Ok(match (self, rhs) {
            (Self::Simple(s), Self::Simple(r)) => Self::Simple(s-r),
            (Self::Complex(_), Self::Complex(_)) 
                => return Err(NativeFnError::NotYetImplemented("Complex/rational substraction")),
            _ => return Err(NativeFnError::InvalidOperands { expected: "Simple or complex numbers" })
        })
    }
}

impl Mul for ComparisonOperator {
    type Output = Result<Self, NativeFnError>;

    fn mul(self, rhs: Self) -> Self::Output {
        Ok(match (self, rhs) {
            (Self::Simple(s), Self::Simple(r)) => Self::Simple(s*r),
            (Self::Complex(_), Self::Complex(_)) 
                => return Err(NativeFnError::NotYetImplemented("Complex/rational multiplication")),
            _ => return Err(NativeFnError::InvalidOperands { expected: "Simple or complex numbers" })
        })
    }
}

impl Div for ComparisonOperator {
    type Output = Result<Self, NativeFnError>;

    fn div(self, rhs: Self) -> Self::Output {
        Ok(match (self, rhs) {
            (Self::Simple(s), Self::Simple(r)) => Self::Simple(s/r),
            (Self::Complex(_), Self::Complex(_)) 
                => return Err(NativeFnError::NotYetImplemented("Complex/rational division")),
            _ => return Err(NativeFnError::InvalidOperands { expected: "Simple or complex numbers" })
        })
    }
}

impl PartialEq for ComparisonOperator {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Simple(l0), Self::Simple(r0)) => l0 == r0,
            (Self::Complex(_), Self::Complex(_)) => false,
            _ => false,
        }
    }
}

impl Eq for ComparisonOperator {}

impl PartialOrd for ComparisonOperator {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Simple(l0), Self::Simple(r0)) => l0.partial_cmp(r0),
            (Self::Complex(_), Self::Complex(_)) => None,
            _ => None
        }
    }
}

pub struct RationalOrComplex {
    left: i32,
    right: i32,
}

pub struct NonImaginary(f64);

impl Add for NonImaginary {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for NonImaginary {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Mul for NonImaginary {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Div for NonImaginary {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl PartialEq for NonImaginary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for NonImaginary {}

impl PartialOrd for NonImaginary {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}


