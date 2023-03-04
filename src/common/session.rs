use std::str::FromStr;

use derive_more::Display;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Display)]
#[display(fmt = "{_0}")]
pub struct Session(String);

#[derive(Error, Debug)]
pub enum ParseSessionError {
    #[error("Session token can not be empty")]
    Empty,
}

impl FromStr for Session {
    type Err = ParseSessionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseSessionError::Empty);
        }

        Ok(Self(s.to_string()))
    }
}
