use derive_getters::Getters;
use std::fmt;
use std::fmt::Display;

use num::PrimInt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Parity {
    Odd,
    Even,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Sign {
    Positive,
    Negative,
}

impl Display for Parity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Self::Odd => "odd",
            Self::Even => "even",
        };
        write!(f, "{}", string)
    }
}

impl Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Self::Positive => "positive",
            Self::Negative => "negative",
        };
        write!(f, "{}", string)
    }
}

#[derive(Debug, PartialEq, Clone, Getters)]
pub struct Number<T: PrimInt> {
    value: T,
    positive: bool,
    sign: Sign,
    parity: Parity,
}

impl<T: PrimInt> Number<T> {
    pub fn new(value: T) -> Self {
        let positive = value >= T::zero();
        let sign = if positive {
            Sign::Positive
        } else {
            Sign::Negative
        };
        let rem = value.rem(T::from(2).unwrap());
        let parity = if rem == T::zero() {
            Parity::Even
        } else {
            Parity::Odd
        };
        Self {
            value,
            positive,
            sign,
            parity,
        }
    }
    pub fn set_value(&mut self, value: T) {
        *self = Self::new(value);
    }

    pub fn prime(&self) -> bool {
        let n = self.value;
        if n <= T::from(1).unwrap() {
            return false;
        } else if n <= T::from(3).unwrap() {
            return true;
        } else if matches!(self.parity(), Parity::Even) || n.rem(T::from(3).unwrap()) == T::zero() {
            return false;
        } else {
            let max = T::from(n.to_f64().unwrap().sqrt().ceil()).unwrap();
            let mut i = T::from(5).unwrap();
            while i <= max {
                if n % i == T::zero() || n.rem(i + T::from(2).unwrap()) == T::zero() {
                    return false;
                }
                i = i + T::from(6).unwrap();
            }
        }
        true
    }
}

impl<T: PrimInt + Display> Display for Number<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Number {} is {} and {}",
            self.value, self.sign, self.parity
        )
    }
}

impl<T: PrimInt> From<T> for Number<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests;
