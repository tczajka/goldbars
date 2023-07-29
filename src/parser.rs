use crate::Error;
use std::{fs, path::Path};

pub fn read_journal(file_name: &Path) -> Result<(), Error> {
    // Open file.
    let text_bytes = fs::read(file_name).map_err(|e| Error::FileReadError {
        file_name: file_name.to_path_buf(),
        error: e,
    })?;
    // Decode UTF-8.
    // Parse file.
    // Return result.
    todo!()
}
