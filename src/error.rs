use crate::files::FileSpan;
use std::{io, path::PathBuf};

#[derive(Debug)]
pub struct Error {
    pub location: Option<FileSpan>,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    FileReadError {
        file_name: PathBuf,
        error: io::Error,
    },
    FileReadTwice {
        file_name: PathBuf,
        previous_location: Option<FileSpan>,
    },
    InvalidUtf8 {
        bytes: Vec<u8>,
    },
}
