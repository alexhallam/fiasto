use crate::internal::{errors::ParseError, lexer::Token};
use logos::Logos;

/// Creates a new Parser instance by tokenizing the input formula string.
///
/// This function is the entry point for parsing formulas. It takes a formula string
/// and converts it into a sequence of tokens that can be parsed by the other
/// parsing functions.
///
/// # Arguments
/// * `input` - A string containing a formula (e.g., "y ~ x + poly(x, 2)")
///
/// # Returns
/// * `Result<Parser<'a>, ParseError>` - A Parser instance or an error if tokenization fails
///
/// # Example
/// ```
/// use fiasto::internal::new::new;
///
/// let result = new("y ~ x + z");
/// assert!(result.is_ok());
///
/// let parser = result.unwrap();
/// assert_eq!(parser.input, "y ~ x + z");
/// assert_eq!(parser.pos, 0);
/// ```
///
/// # How it works
/// 1. Creates a Logos lexer from the input string
/// 2. Iterates through all tokens, collecting them into a vector
/// 3. Returns a Parser struct with the tokens and initial position
///
/// # Tokenization Process
/// The input "y ~ x + z" would be tokenized as:
/// - ColumnName: "y"
/// - Tilde: "~"  
/// - ColumnName: "x"
/// - Plus: "+"
/// - ColumnName: "z"
pub fn new<'a>(input: &'a str) -> Result<crate::internal::parser::Parser<'a>, ParseError> {
    let mut lex = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(item) = lex.next() {
        match item {
            Ok(tok) => {
                let slice = lex.slice();
                tokens.push((tok, slice));
            }
            Err(()) => {
                return Err(ParseError::Lex(lex.slice().to_string()));
            }
        }
    }

    Ok(crate::internal::parser::Parser {
        input,
        tokens,
        pos: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_simple_formula() {
        let result = new("y ~ x");
        assert!(result.is_ok());

        let parser = result.unwrap();
        assert_eq!(parser.input, "y ~ x");
        assert_eq!(parser.pos, 0);
        assert_eq!(parser.tokens.len(), 3); // y, ~, x
    }

    #[test]
    fn test_new_complex_formula() {
        let result = new("y ~ x + poly(x, 2)");
        assert!(result.is_ok());

        let parser = result.unwrap();
        assert_eq!(parser.input, "y ~ x + poly(x, 2)");
        assert_eq!(parser.pos, 0);
        assert!(parser.tokens.len() > 5); // Multiple tokens including function
    }

    #[test]
    fn test_new_with_whitespace() {
        let result = new("  y   ~   x  ");
        assert!(result.is_ok());

        let parser = result.unwrap();
        assert_eq!(parser.input, "  y   ~   x  ");
        assert_eq!(parser.pos, 0);
        // Whitespace should be skipped, so we get the same tokens
        assert_eq!(parser.tokens.len(), 3);
    }

    #[test]
    fn test_new_empty_string() {
        let result = new("");
        assert!(result.is_ok());

        let parser = result.unwrap();
        assert_eq!(parser.input, "");
        assert_eq!(parser.pos, 0);
        assert_eq!(parser.tokens.len(), 0);
    }
}
