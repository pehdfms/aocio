use std::path::{Path, PathBuf};

use thiserror::Error;

pub struct FilePath {
    directory: PathBuf,
    filename: String,
}

impl FilePath {
    pub fn as_path(&self) -> PathBuf {
        self.directory.join(&self.filename)
    }

    pub fn directory(&self) -> &Path {
        self.directory.as_path()
    }

    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn exists(&self) -> bool {
        self.as_path().exists()
    }
}

#[derive(Debug, Error)]
pub enum InvalidFilePathError {
    #[error("Could not get a filename from Path {0}")]
    NoFilename(PathBuf),
    #[error("Filename {0} is invalid")]
    InvalidFilename(PathBuf),
}

impl TryFrom<PathBuf> for FilePath {
    type Error = InvalidFilePathError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let directory = value
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf();

        let filename = value
            .file_name()
            .ok_or_else(|| InvalidFilePathError::NoFilename(value.clone()))?
            .to_str()
            .ok_or_else(|| InvalidFilePathError::InvalidFilename(value.clone()))?
            .to_string();

        Ok(Self {
            directory,
            filename,
        })
    }
}
