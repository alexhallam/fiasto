use thiserror::Error;

// ---------------------------
// ERRORS
// ---------------------------

#[derive(Error, Debug)]
/// This checks for the following
/// - lexing errors
/// - unexpected end of input
/// - unexpected tokens
/// - invalid syntax
pub enum ParseError {
    #[error("lexing error at {0:?}")]
    Lex(String),
    #[error("unexpected end of input")]
    Eoi,
    #[error("unexpected token: expected {expected:?}, found {found:?}")]
    Unexpected {
        expected: &'static str,
        found: Option<super::lexer::Token>,
    },
    #[error("invalid syntax: {0}")]
    Syntax(String),
}
