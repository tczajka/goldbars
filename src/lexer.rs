use crate::files::{FileSpan, Span};
use std::str::CharIndices;

/// Compute the beginnings of lines.
pub fn compute_line_starts(text: &str) -> Vec<usize> {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum State {
        FileStart,
        LineStart,
        AfterCr,
        Midline,
    }
    use State::*;

    let mut line_starts = Vec::new();
    let mut state = FileStart;
    for (index, c) in text.char_indices() {
        match state {
            State::FileStart => {
                state = LineStart;
                if c == BYTE_ORDER_MARK {
                    continue;
                }
            }
            State::AfterCr => {
                state = LineStart;
                if c == '\n' {
                    continue;
                }
            }
            _ => {}
        }
        if state == LineStart {
            line_starts.push(index);
        }
        state = if c == '\r' {
            AfterCr
        } else if is_newline(c) {
            LineStart
        } else {
            Midline
        };
    }
    line_starts
}

fn is_newline(c: char) -> bool {
    [
        '\n',       // Line feed
        '\u{B}',    // Vertical tab
        '\u{C}',    // Form feed
        '\r',       // Carriage return
        '\u{85}',   // Next line
        '\u{2028}', // Line separator
        '\u{2029}', // Paragraph separator
    ]
    .contains(&c)
}

const BYTE_ORDER_MARK: char = '\u{FEFF}';

pub struct Lexer<'a> {
    /// File number.
    file_number: usize,
    /// The text.
    text: &'a str,
    /// Current character. None if end of file.
    current_char: Option<char>,
    /// Index of the current char.
    current_index: usize,
    /// Remaining characters.
    char_indices: CharIndices<'a>,
    /// Current token.
    current_token: Token,
    /// Stack of indentation levels.
    indentation_levels: Vec<IndentationLevel>,
    /// Most recently seen indentation.
    last_indentation: Span,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from a string.
    pub fn new(file_number: usize, text: &'a str, mode: LexerMode) -> Self {
        let mut char_indices = text.char_indices();
        let (current_index, current_char) = match char_indices.next() {
            Some((current_index, current_char)) => (current_index, Some(current_char)),
            None => (text.len(), None),
        };
        let bogus_token = Token {
            token: TokenKind::EndOfFile,
            file_span: FileSpan {
                file_index: file_number,
                span: Span {
                    start: current_index,
                    end: current_index,
                },
            },
        };
        let mut lexer = Self {
            file_number,
            text,
            current_char,
            current_index,
            char_indices,
            current_token: bogus_token,
            indentation_levels: vec![IndentationLevel {
                bytes: 0,
                column: 1,
            }],
            last_indentation: Span {
                start: current_index,
                end: current_index,
            },
        };
        lexer.read_indentation();
        lexer.advance(mode);
        lexer
    }

    /// Returns the current token.
    pub fn current_token(&self) -> Token {
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
pub enum TokenKind {
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
pub struct Token {
    token: TokenKind,
    file_span: FileSpan,
}

#[derive(Clone, Copy, Debug)]
struct IndentationLevel {
    /// Number of bytes.
    bytes: usize,
    /// Column number (starting at 1).
    column: usize,
}
