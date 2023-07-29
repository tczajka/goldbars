use std::{
    error,
    fmt::{self, Display, Formatter},
    io,
    path::PathBuf,
};

#[derive(Debug)]
pub enum Error {
    FileReadError {
        file_name: PathBuf,
        error: io::Error,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::FileReadError { file_name, error } => {
                write!(f, "Error reading file {}: {error}", file_name.display())
            }
        }
    }
}

impl error::Error for Error {}
