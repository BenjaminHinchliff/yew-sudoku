use core::fmt;
use std::convert::TryFrom;

use enum_iterator::IntoEnumIterator;

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("unable to convert {0} into digit")]
    ConversionOutOfRange(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoEnumIterator)]
pub enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl TryFrom<u8> for Digit {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Digit::One,
            2 => Digit::Two,
            3 => Digit::Three,
            4 => Digit::Four,
            5 => Digit::Five,
            6 => Digit::Six,
            7 => Digit::Seven,
            8 => Digit::Eight,
            9 => Digit::Nine,
            _ => return Err(Error::ConversionOutOfRange(value)),
        })
    }
}

impl From<Digit> for u8 {
    fn from(digit: Digit) -> Self {
        match digit {
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
        }
    }
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}
