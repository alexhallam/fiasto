use crate::internal::{
    ast::{Family, Term},
    errors::ParseError,
    lexer::Token,
};

// ---------------------------
// PARSER
// ---------------------------

/// Parser for the formula
/// This is responsible for parsing the formula string into an AST
/// The <'a> means that the parser will borrow the input string for the duration of its lifetime
/// The `input` field is a reference to the original input string
/// The `tokens` field is a vector of all the tokens found in the input string
/// The `pos` field is the current position in the token stream
pub struct Parser<'a> {
    pub input: &'a str,
    pub tokens: Vec<(Token, &'a str)>,
    pub pos: usize,
}

/// The parser implementation does the actual work of parsing the formula
impl<'a> Parser<'a> {
    /// Creates a new parser instance
    pub fn new(input: &'a str) -> Result<Self, ParseError> {
        crate::internal::new::new(input)
    }

    /// Parses the formula and returns the response, terms, intercept flag, and family
    pub fn parse_formula(
        &mut self,
    ) -> Result<(String, Vec<Term>, bool, Option<Family>), ParseError> {
        crate::internal::parse_formula::parse_formula(&self.tokens, &mut self.pos)
    }
}
