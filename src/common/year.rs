use std::{num::ParseIntError, str::FromStr};

use derive_more::Display;
use thiserror::Error;

use super::day::AocDay;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Display)]
#[display(fmt = "{_0}")]
pub struct AocYear(u32);

impl AocYear {
    #[must_use]
    pub fn get_days() -> Vec<AocDay> {
        (1..=25)
            .map(|day| {
                day.try_into()
                    .expect("Hardcoded day range is valid for AocDays")
            })
            .collect()
    }
}

#[derive(Error, Debug)]
pub enum ParseAocYearError {
    #[error("Advent of Code Year should be at least 2015, but got {0}")]
    OutOfRange(String),
    #[error("Input {0} is not a valid year")]
    NotANumber(String),
}

impl FromStr for AocYear {
    type Err = ParseAocYearError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let year: u32 = s.parse().map_err(|e: ParseIntError| {
            use std::num::IntErrorKind::{NegOverflow, PosOverflow, Zero};
            match e.kind() {
                NegOverflow | PosOverflow | Zero => ParseAocYearError::OutOfRange(s.to_string()),
                _ => ParseAocYearError::NotANumber(s.to_string()),
            }
        })?;

        if year < 2015 {
            return Err(ParseAocYearError::OutOfRange(year.to_string()));
        }

        Ok(Self(year))
    }
}
