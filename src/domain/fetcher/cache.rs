use std::{collections::HashMap, convert::Infallible, error};

use thiserror::Error;

use crate::common::{day::AocDay, year::AocYear};

pub trait Cache {
    type WriteCacheError: error::Error;

    fn read(&self, year: AocYear, day: AocDay) -> Option<String>;
    fn write(
        &mut self,
        year: AocYear,
        day: AocDay,
        input: &str,
    ) -> Result<(), Self::WriteCacheError>;
}

pub struct NoCache;

impl Cache for NoCache {
    type WriteCacheError = Infallible;

    fn read(&self, year: AocYear, day: AocDay) -> Option<String> {
        None
    }

    fn write(
        &mut self,
        year: AocYear,
        day: AocDay,
        input: &str,
    ) -> Result<(), Self::WriteCacheError> {
        Ok(())
    }
}

impl NoCache {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum WriteConflictStrategy {
    Overwrite,
    Error,
    Skip,
}

pub struct MemoryCache {
    handle_conflict: WriteConflictStrategy,
    cache_map: HashMap<(AocYear, AocDay), String>,
}

impl MemoryCache {
    pub fn new(handle_conflict: WriteConflictStrategy) -> Self {
        Self {
            handle_conflict,
            cache_map: HashMap::default(),
        }
    }
}

#[derive(Debug, Error)]
pub enum MemoryWriteCacheError {
    #[error("Found cache conflict with handle conflict strategy set to Error")]
    AlreadyExists(AocYear, AocDay),
}

impl Cache for MemoryCache {
    type WriteCacheError = MemoryWriteCacheError;

    fn read(&self, year: AocYear, day: AocDay) -> Option<String> {
        self.cache_map.get(&(year, day)).map(|s| s.to_string())
    }

    fn write(
        &mut self,
        year: AocYear,
        day: AocDay,
        input: &str,
    ) -> Result<(), Self::WriteCacheError> {
        if self.cache_map.contains_key(&(year, day)) {
            match self.handle_conflict {
                WriteConflictStrategy::Overwrite => (),
                WriteConflictStrategy::Error => {
                    return Err(MemoryWriteCacheError::AlreadyExists(year, day))
                }
                WriteConflictStrategy::Skip => return Ok(()),
            }
        }

        self.cache_map.insert((year, day), input.to_string());
        Ok(())
    }
}
