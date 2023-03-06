use std::{num::ParseIntError, str::FromStr};

use derive_more::Display;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Display)]
pub enum AocPart {
    #[display(fmt = "1")]
    Part1 = 1,
    #[display(fmt = "2")]
    Part2 = 2,
}

#[derive(Error, Debug)]
pub enum ParseAocPartError {
    #[error("Advent of Code Day should be between 1 and 25 inclusive, but got {0}")]
    NotAPart(String),
    #[error("Input {0} is not a valid day")]
    NotANumber(String),
}

impl FromStr for AocPart {
    type Err = ParseAocPartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day: usize = s.parse().map_err(|e: ParseIntError| {
            use std::num::IntErrorKind::{NegOverflow, PosOverflow, Zero};
            match e.kind() {
                NegOverflow | PosOverflow | Zero => ParseAocPartError::NotAPart(s.to_string()),
                _ => ParseAocPartError::NotANumber(s.to_string()),
            }
        })?;

        match day {
            1 => Ok(AocPart::Part1),
            2 => Ok(AocPart::Part2),
            _ => Err(ParseAocPartError::NotAPart(s.to_string())),
        }
    }
}
