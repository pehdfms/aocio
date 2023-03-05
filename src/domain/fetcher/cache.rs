use std::{collections::HashMap, convert::Infallible, error, fs, io, path::PathBuf};

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

    fn read(&self, _: AocYear, __: AocDay) -> Option<String> {
        None
    }

    fn write(&mut self, _: AocYear, __: AocDay, ___: &str) -> Result<(), Self::WriteCacheError> {
        Ok(())
    }
}

impl NoCache {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

#[derive(Default)]
pub struct MemoryCache {
    cache_map: HashMap<(AocYear, AocDay), String>,
}

impl MemoryCache {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
        self.cache_map.get(&(year, day)).map(ToString::to_string)
    }

    fn write(
        &mut self,
        year: AocYear,
        day: AocDay,
        input: &str,
    ) -> Result<(), Self::WriteCacheError> {
        self.cache_map.insert((year, day), input.to_string());
        Ok(())
    }
}

pub trait PathFormatter = Fn(AocYear, AocDay) -> PathBuf;

pub struct FileCache<F: PathFormatter> {
    path_formatter: F,
}

impl<F: PathFormatter> FileCache<F> {
    pub const fn new(path_formatter: F) -> Self {
        Self { path_formatter }
    }
}

impl<F: PathFormatter> Cache for FileCache<F> {
    type WriteCacheError = io::Error;

    fn read(&self, year: AocYear, day: AocDay) -> Option<String> {
        let path = (self.path_formatter)(year, day);
        fs::read_to_string(path).ok()
    }

    fn write(
        &mut self,
        year: AocYear,
        day: AocDay,
        input: &str,
    ) -> Result<(), Self::WriteCacheError> {
        let path = (self.path_formatter)(year, day);

        if let Some(directory) = &path.parent() {
            fs::create_dir_all(directory)?;
        }

        fs::write(path, input)
    }
}
