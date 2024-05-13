use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

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

    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }

    fn le(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }

    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }

    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }
}


