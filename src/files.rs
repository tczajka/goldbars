use crate::{
    error::{Error, ErrorKind},
    lexer::compute_line_starts,
};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

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
    fn read(&mut self, name: &Path, import_statement: Option<FileSpan>) -> Result<usize, Error> {
        // Check if file already read.
        if let Some(&index) = self.names.get(name) {
            return Err(Error {
                location: import_statement,
                kind: ErrorKind::FileReadTwice {
                    file_name: name.to_path_buf(),
                    previous_location: self.files[index].import_statement,
                },
            });
        }

        // Read file.
        let bytes = fs::read(name).map_err(|e| Error {
            location: import_statement,
            kind: ErrorKind::FileReadError {
                file_name: name.to_path_buf(),
                error: e,
            },
        })?;

        let file_index = self.files.len();

        // Parse UTF-8.
        // If there is an error, make a lossy conversion and keep the error.
        let (text, utf8_error) = match String::from_utf8(bytes) {
            Ok(text) => (text, None),
            Err(from_utf8_error) => {
                let bytes = from_utf8_error.as_bytes();
                let utf8_error = from_utf8_error.utf8_error();
                let index = utf8_error.valid_up_to();
                let text = String::from_utf8_lossy(bytes).into_owned();

                let mut invalid_bytes = &bytes[index..];
                if let Some(n) = utf8_error.error_len() {
                    invalid_bytes = &invalid_bytes[..n];
                }

                assert_eq!(
                    text[index..].chars().next(),
                    Some(char::REPLACEMENT_CHARACTER)
                );

                let error = Error {
                    location: Some(FileSpan {
                        file_index,
                        span: Span {
                            start: index,
                            end: index + char::REPLACEMENT_CHARACTER.len_utf8(),
                        },
                    }),
                    kind: ErrorKind::InvalidUtf8 {
                        bytes: invalid_bytes.to_vec(),
                    },
                };

                (text, Some(error))
            }
        };

        let line_starts = compute_line_starts(&text);

        self.files.push(File {
            name: name.to_path_buf(),
            import_statement,
            text,
            line_starts,
        });
        self.names.insert(name.to_path_buf(), file_index);

        // Report UTF-8 error.
        if let Some(utf8_error) = utf8_error {
            return Err(utf8_error);
        }

        Ok(file_index)
    }
}

/// Single file contents.
struct File {
    /// File name.
    name: PathBuf,
    /// Import statement location.
    import_statement: Option<FileSpan>,
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
