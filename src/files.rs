use crate::{
    error::{Error, ErrorKind},
    lexer::compute_line_starts,
};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    str,
};

#[derive(Debug)]
/// Set of files.
struct Files {
    /// Files.
    files: Vec<File>,
    /// Name lookup.
    names: HashMap<PathBuf, usize>,
}

impl Files {
    /// Create a new set of files.
    fn new() -> Self {
        Self {
            files: Vec::new(),
            names: HashMap::new(),
        }
    }

    /// Read a file.
    ///
    /// Returns file index.
    fn read(
        &mut self,
        name: &Path,
        include_location: IncludeFileLocation,
        errors: &mut Vec<Error>,
    ) -> Result<usize, ()> {
        let include_error_location = match include_location {
            IncludeFileLocation::CommandLine => None,
            IncludeFileLocation::Include(file_span) => Some(file_span),
        };

        // Check if file already read.
        if let Some(&index) = self.names.get(name) {
            errors.push(Error {
                location: include_error_location,
                kind: ErrorKind::FileReadTwice {
                    file_name: name.to_path_buf(),
                    previous_location: self.files[index].include_location,
                },
            });
            return Err(());
        }

        // Read file.
        let bytes = match fs::read(name) {
            Ok(bytes) => bytes,
            Err(error) => {
                errors.push(Error {
                    location: include_error_location,
                    kind: ErrorKind::FileReadError {
                        file_name: name.to_path_buf(),
                        error,
                    },
                });
                return Err(());
            }
        };

        let file_index = self.files.len();
        let text = parse_utf8(bytes, file_index, errors);
        let line_starts = compute_line_starts(&text);
        self.files.push(File {
            name: name.to_path_buf(),
            include_location,
            text,
            line_starts,
        });
        self.names.insert(name.to_path_buf(), file_index);

        Ok(file_index)
    }
}

// Parse UTF-8.
// If there are any errors, make a lossy conversion and keep track of the errors.
fn parse_utf8(bytes: Vec<u8>, file_index: usize, errors: &mut Vec<Error>) -> String {
    String::from_utf8(bytes).unwrap_or_else(|from_utf8_error| {
        let mut bytes = from_utf8_error.as_bytes();
        let mut utf8_error = from_utf8_error.utf8_error();
        let mut text = String::new();

        loop {
            let (valid_bytes, remainder) = bytes.split_at(utf8_error.valid_up_to());
            // SAFETY: `valid_bytes` is known to be valid UTF-8.
            text.push_str(unsafe { str::from_utf8_unchecked(valid_bytes) });
            let invalid_len = utf8_error.error_len().unwrap_or(remainder.len());
            let (invalid_bytes, remainder) = remainder.split_at(invalid_len);
            bytes = remainder;

            let start = text.len();
            text.push(char::REPLACEMENT_CHARACTER);
            let end = text.len();
            errors.push(Error {
                location: Some(FileSpan {
                    file_index,
                    span: Span { start, end },
                }),
                kind: ErrorKind::InvalidUtf8 {
                    bytes: invalid_bytes.to_vec(),
                },
            });
            match str::from_utf8(bytes) {
                Ok(valid_text) => {
                    text.push_str(valid_text);
                    break;
                }
                Err(e) => {
                    bytes = remainder;
                    utf8_error = e;
                }
            }
        }
        text
    })
}

#[derive(Debug)]
/// Single file contents.
struct File {
    /// File name.
    name: PathBuf,
    /// Include statement location.
    include_location: IncludeFileLocation,
    /// File contents.
    text: String,
    /// Indexes where lines start.
    /// The first index is always 0.
    /// If there is a final newline, the last index is the length of the file.
    line_starts: Vec<usize>,
}

/// Substring of a string.
#[derive(Copy, Clone, Debug)]
pub struct Span {
    /// First byte index of the span.
    pub start: usize,
    /// One past the last byte index of the span.
    pub end: usize,
}

/// Substring of a file.
#[derive(Copy, Clone, Debug)]
pub struct FileSpan {
    /// File number.
    pub file_index: usize,
    /// The span.
    pub span: Span,
}

/// How was this file included?
#[derive(Copy, Clone, Debug)]
pub enum IncludeFileLocation {
    /// Command line.
    CommandLine,
    /// Include statement.
    Include(FileSpan),
}
