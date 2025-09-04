use crate::internal::errors::ParseError;

/// Expects and consumes a token that matches a specific predicate, or returns an error.
/// 
/// This function is the primary way to enforce grammar rules during parsing. It ensures
/// that required tokens are present and advances the position only when the expectation
/// is met. If the expectation fails, it returns a descriptive error.
/// 
/// # Arguments
/// * `tokens` - Reference to the vector of tokens
/// * `pos` - Mutable reference to the current position (will be incremented if match)
/// * `expect_fn` - A function that takes a Token and returns true if it matches expectations
/// * `expected` - A string describing what was expected (used in error messages)
/// 
/// # Returns
/// * `Result<(Token, &'a str), ParseError>` - The consumed token and its slice, or an error
/// 
/// # Example
/// ```
/// use fiasto::internal::expect::expect;
/// use fiasto::internal::lexer::Token;
/// 
/// let tokens = vec![
///     (Token::ColumnName, "y"),
///     (Token::Tilde, "~"),
///     (Token::ColumnName, "x")
/// ];
/// let mut pos = 0;
/// 
/// // Expect a column name
/// let result = expect(&tokens, &mut pos, |t| matches!(t, Token::ColumnName), "ColumnName");
/// assert!(result.is_ok());
/// let (token, slice) = result.unwrap();
/// assert_eq!(token, Token::ColumnName);
/// assert_eq!(slice, "y");
/// assert_eq!(pos, 1); // Position advanced
/// 
/// // Expect a tilde
/// let result = expect(&tokens, &mut pos, |t| matches!(t, Token::Tilde), "~");
/// assert!(result.is_ok());
/// let (token, slice) = result.unwrap();
/// assert_eq!(token, Token::Tilde);
/// assert_eq!(slice, "~");
/// assert_eq!(pos, 2); // Position advanced
/// ```
/// 
/// # How it works
/// 1. Examines the token at the current position
/// 2. If the token matches the predicate, consumes it and advances position
/// 3. If no match, returns a ParseError with details about what was expected
/// 4. If at end of tokens, returns an error indicating unexpected end of input
/// 
/// # Use Cases
/// - Enforcing required grammar elements (e.g., `~` after response variable)
/// - Validating token sequences (e.g., `(` after function names)
/// - Error reporting with specific expectations
/// - Implementing strict parsing rules
/// 
/// # Error Types
/// - `ParseError::Unexpected` - When a token doesn't match expectations
/// - `ParseError::Unexpected` with `found: None` - When at end of input unexpectedly
pub fn expect<'a>(
    tokens: &'a [(crate::internal::lexer::Token, &'a str)],
    pos: &mut usize,
    expect_fn: fn(&crate::internal::lexer::Token) -> bool,
    expected: &'static str,
) -> Result<(crate::internal::lexer::Token, &'a str), ParseError> {
    if let Some((tok, slice)) = tokens.get(*pos).cloned() {
        if expect_fn(&tok) {
            *pos += 1;
            Ok((tok, slice))
        } else {
            Err(ParseError::Unexpected {
                expected,
                found: Some(tok),
            })
        }
    } else {
        Err(ParseError::Unexpected {
            expected,
            found: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::lexer::Token;

    #[test]
    fn test_expect_success() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~")
        ];
        let mut pos = 0;
        
        let result = expect(&tokens, &mut pos, |t| matches!(t, Token::ColumnName), "ColumnName");
        assert!(result.is_ok());
        let (token, slice) = result.unwrap();
        assert_eq!(token, Token::ColumnName);
        assert_eq!(slice, "y");
        assert_eq!(pos, 1); // Position advanced
    }

    #[test]
    fn test_expect_failure_wrong_token() {
        let tokens = vec![
            (Token::Tilde, "~"),
            (Token::ColumnName, "y")
        ];
        let mut pos = 0;
        
        let result = expect(&tokens, &mut pos, |t| matches!(t, Token::ColumnName), "ColumnName");
        assert!(result.is_err());
        
        if let ParseError::Unexpected { expected, found } = result.unwrap_err() {
            assert_eq!(expected, "ColumnName");
            assert_eq!(found, Some(Token::Tilde));
        } else {
            panic!("Expected ParseError::Unexpected");
        }
        
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_expect_failure_end_of_input() {
        let tokens = vec![
            (Token::ColumnName, "y")
        ];
        let mut pos = 1; // At end
        
        let result = expect(&tokens, &mut pos, |t| matches!(t, Token::Tilde), "~");
        assert!(result.is_err());
        
        if let ParseError::Unexpected { expected, found } = result.unwrap_err() {
            assert_eq!(expected, "~");
            assert_eq!(found, None);
        } else {
            panic!("Expected ParseError::Unexpected");
        }
        
        assert_eq!(pos, 1); // Position unchanged
    }

    #[test]
    fn test_expect_with_complex_predicate() {
        let tokens = vec![
            (Token::ColumnName, "x"),
            (Token::Integer, "42"),
            (Token::One, "1")
        ];
        let mut pos = 0;
        
        // Expect any numeric token
        let numeric_predicate = |t: &Token| matches!(t, Token::Integer | Token::One);
        
        let result = expect(&tokens, &mut pos, numeric_predicate, "numeric token");
        assert!(result.is_err()); // ColumnName is not numeric
        assert_eq!(pos, 0);
        
        // Move to integer
        pos = 1;
        let result = expect(&tokens, &mut pos, numeric_predicate, "numeric token");
        assert!(result.is_ok()); // Integer is numeric
        assert_eq!(pos, 2);
    }

    #[test]
    fn test_expect_advances_position_on_success() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x")
        ];
        let mut pos = 0;
        
        // First expectation
        let result = expect(&tokens, &mut pos, |t| matches!(t, Token::ColumnName), "ColumnName");
        assert!(result.is_ok());
        assert_eq!(pos, 1);
        
        // Second expectation
        let result = expect(&tokens, &mut pos, |t| matches!(t, Token::Tilde), "~");
        assert!(result.is_ok());
        assert_eq!(pos, 2);
        
        // Third expectation
        let result = expect(&tokens, &mut pos, |t| matches!(t, Token::ColumnName), "ColumnName");
        assert!(result.is_ok());
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_expect_with_empty_tokens() {
        let tokens: Vec<(Token, &str)> = vec![];
        let mut pos = 0;
        
        let result = expect(&tokens, &mut pos, |t| matches!(t, Token::ColumnName), "ColumnName");
        assert!(result.is_err());
        
        if let ParseError::Unexpected { expected, found } = result.unwrap_err() {
            assert_eq!(expected, "ColumnName");
            assert_eq!(found, None);
        } else {
            panic!("Expected ParseError::Unexpected");
        }
        
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_expect_preserves_string_slice() {
        let tokens = vec![
            (Token::ColumnName, "response_variable"),
            (Token::Integer, "12345")
        ];
        let mut pos = 0;
        
        let result = expect(&tokens, &mut pos, |t| matches!(t, Token::ColumnName), "ColumnName");
        assert!(result.is_ok());
        let (_, slice) = result.unwrap();
        assert_eq!(slice, "response_variable");
    }
}
