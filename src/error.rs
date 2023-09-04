use crate::files::{FileSpan, IncludeFileLocation};
use std::{io, path::PathBuf};

#[derive(Debug)]
pub struct Error {
    pub location: Option<FileSpan>,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    FileReadError {
        file_index: usize,
        error: io::Error,
    },
    FileIncludedTwice {
        file_name: PathBuf,
        previous_location: IncludeFileLocation,
    },
    InvalidUtf8 {
        bytes: Vec<u8>,
    },
}
