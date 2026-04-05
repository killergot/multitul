use std::fmt;
use std::io;

#[derive(Debug)]
pub enum GitError {
    Io(io::Error),
    InvalidHeadFormat(String),
    InvalidObject(String),
    DecompressionError(String),
    NotFound(String),
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::Io(e) => write!(f, "IO error: {}", e),
            GitError::InvalidHeadFormat(s) => write!(f, "Invalid HEAD format: {}", s),
            GitError::InvalidObject(s) => write!(f, "Invalid git object: {}", s),
            GitError::DecompressionError(s) => write!(f, "Decompression failed: {}", s),
            GitError::NotFound(s) => write!(f, "Not found: {}", s),
        }
    }
}