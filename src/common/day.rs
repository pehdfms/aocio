use std::{num::ParseIntError, str::FromStr};

use derive_more::Display;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Display)]
#[display(fmt = "{_0}")]
pub struct AocDay(usize);

impl From<AocDay> for usize {
    fn from(value: AocDay) -> Self {
        value.0
    }
}

#[derive(Error, Debug)]
pub enum ParseAocDayError {
    #[error("Advent of Code Day should be between 1 and 25 inclusive, but got {0}")]
    OutOfRange(String),
    #[error("Input {0} is not a valid day")]
    NotANumber(String),
}

impl FromStr for AocDay {
    type Err = ParseAocDayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day: usize = s.parse().map_err(|e: ParseIntError| {
            use std::num::IntErrorKind::{NegOverflow, PosOverflow, Zero};
            match e.kind() {
                NegOverflow | PosOverflow | Zero => ParseAocDayError::OutOfRange(s.to_string()),
                _ => ParseAocDayError::NotANumber(s.to_string()),
            }
        })?;

        if !(1..=25).contains(&day) {
            return Err(ParseAocDayError::OutOfRange(day.to_string()));
        }

        Ok(Self(day))
    }
}

#[derive(Error, Debug)]
pub enum ParseAocDayFromNumberError {
    #[error("Advent of Code Day should be between 1 and 25 inclusive, but got {0}")]
    OutOfRange(String),
}

impl TryFrom<usize> for AocDay {
    type Error = ParseAocDayFromNumberError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if !(1..=25).contains(&value) {
            return Err(ParseAocDayFromNumberError::OutOfRange(value.to_string()));
        }

        Ok(Self(value))
    }
}
