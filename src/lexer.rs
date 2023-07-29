use std::str::CharIndices;

pub struct Lexer<'a> {
    /// File number.
    file_number: usize,
    /// The text.
    text: &'a str,
    /// Current character. None if end of file.
    current_char: Option<char>,
    /// File location of the current char.
    file_location: FileLocation,
    /// Remaining characters.
    char_indices: CharIndices<'a>,
    /// Current token.
    current_token: TokenInstance,
    /// Stack of indentation levels.
    indentation_levels: Vec<IndentationLevel>,
    /// Most recently seen indentation.
    last_indentation: FileRange,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from a string.
    pub fn new(file_number: usize, text: &'a str, mode: LexerMode) -> Self {
        let mut char_indices = text.char_indices();
        let (index, current_char) = match char_indices.next() {
            Some((index, current_char)) => (index, Some(current_char)),
            None => (text.len(), None),
        };
        let file_location = FileLocation {
            index,
            line: 1,
            column: 1,
        };
        let file_range = FileRange {
            file_number,
            start: file_location,
            end: file_location,
        };
        let bogus_token = TokenInstance {
            token: Token::EndOfFile,
            file_range,
        };
        let mut lexer = Self {
            file_number,
            text,
            current_char,
            file_location,
            char_indices,
            current_token: bogus_token,
            indentation_levels: vec![IndentationLevel {
                bytes: 0,
                column: 1,
            }],
            last_indentation: file_range,
        };
        lexer.read_indentation();
        lexer.advance(mode);
        lexer
    }

    /// Returns the current token.
    pub fn current_token(&self) -> TokenInstance {
        self.current_token
    }

    pub fn advance(&mut self, mode: LexerMode) {
        // TODO: Handle indentation.
        match mode {
            LexerMode::Normal => self.advance_normal(),
        }
    }

    fn read_indentation(&mut self) {
        todo!()
    }

    fn advance_normal(&mut self) {
        todo!()
    }
}

/// Mode of the lexer for parsing the next token.
pub enum LexerMode {
    /// Regular mode.
    Normal,
}

/// A token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Token {
    // End of file.
    EndOfFile,
    // End of line.
    EndOfLine,
    // Increase indentation.
    Indent,
    // Decrease indentation.
    Dedent,
}

/// Instance of a token.
#[derive(Clone, Copy, Debug)]
pub struct TokenInstance {
    token: Token,
    file_range: FileRange,
}

/// Range of characters in a file.
#[derive(Clone, Copy, Debug)]
pub struct FileRange {
    /// Number of the file.
    file_number: usize,
    /// Start of the range.
    start: FileLocation,
    /// End of the range, exclusive.
    end: FileLocation,
}

/// Location of a character in a file.
#[derive(Clone, Copy, Debug)]
pub struct FileLocation {
    /// Byte index.
    index: usize,
    /// Line number (starting at 1).
    line: usize,
    /// Column number (starting at 1).
    column: usize,
}

#[derive(Clone, Copy, Debug)]
struct IndentationLevel {
    /// Number of bytes.
    bytes: usize,
    /// Column number (starting at 1).
    column: usize,
}
