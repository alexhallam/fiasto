/// Looks ahead at the next token without consuming it.
///
/// This function is essential for parsing because it allows the parser to
/// examine upcoming tokens to make decisions about what to parse next,
/// without advancing the position in the token stream.
///
/// # Arguments
/// * `tokens` - Reference to the vector of tokens
/// * `pos` - Current position in the token stream
///
/// # Returns
/// * `Option<&'a (Token, &'a str)>` - The next token and its string slice, or None if at end
///
/// # Example
/// ```
/// use fiasto::internal::peek::peek;
/// use fiasto::internal::lexer::Token;
///
/// let tokens = vec![
///     (Token::ColumnName, "y"),
///     (Token::Tilde, "~"),
///     (Token::ColumnName, "x")
/// ];
///
/// let next_token = peek(&tokens, 0);
/// assert!(next_token.is_some());
/// assert_eq!(next_token.unwrap().0, Token::ColumnName);
/// assert_eq!(next_token.unwrap().1, "y");
///
/// let next_token = peek(&tokens, 2);
/// assert!(next_token.is_some());
/// assert_eq!(next_token.unwrap().0, Token::ColumnName);
/// assert_eq!(next_token.unwrap().1, "x");
///
/// let next_token = peek(&tokens, 3);
/// assert!(next_token.is_none()); // Beyond end of tokens
/// ```
///
/// # How it works
/// 1. Takes a reference to the tokens vector and current position
/// 2. Returns a reference to the token at the specified position
/// 3. Does NOT advance the position (non-consuming)
/// 4. Returns None if position is beyond the end of tokens
///
/// # Use Cases
/// - Checking if the next token is a specific type before consuming
/// - Implementing lookahead parsing strategies
/// - Error recovery by examining upcoming tokens
/// - Conditional parsing based on token sequences
pub fn peek<'a>(
    tokens: &'a [(crate::internal::lexer::Token, &'a str)],
    pos: usize,
) -> Option<&'a (crate::internal::lexer::Token, &'a str)> {
    tokens.get(pos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::lexer::Token;

    #[test]
    fn test_peek_at_start() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x"),
        ];

        let result = peek(&tokens, 0);
        assert!(result.is_some());
        let (token, slice) = result.unwrap();
        assert_eq!(*token, Token::ColumnName);
        assert_eq!(*slice, "y");
    }

    #[test]
    fn test_peek_at_middle() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x"),
        ];

        let result = peek(&tokens, 1);
        assert!(result.is_some());
        let (token, slice) = result.unwrap();
        assert_eq!(*token, Token::Tilde);
        assert_eq!(*slice, "~");
    }

    #[test]
    fn test_peek_at_end() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x"),
        ];

        let result = peek(&tokens, 2);
        assert!(result.is_some());
        let (token, slice) = result.unwrap();
        assert_eq!(*token, Token::ColumnName);
        assert_eq!(*slice, "x");
    }

    #[test]
    fn test_peek_beyond_end() {
        let tokens = vec![(Token::ColumnName, "y"), (Token::Tilde, "~")];

        let result = peek(&tokens, 2);
        assert!(result.is_none());
    }

    #[test]
    fn test_peek_empty_tokens() {
        let tokens: Vec<(Token, &str)> = vec![];

        let result = peek(&tokens, 0);
        assert!(result.is_none());
    }

    #[test]
    fn test_peek_with_function_tokens() {
        let tokens = vec![
            (Token::Poly, "poly"),
            (Token::FunctionStart, "("),
            (Token::ColumnName, "x"),
            (Token::Comma, ","),
            (Token::Integer, "2"),
            (Token::FunctionEnd, ")"),
        ];

        let result = peek(&tokens, 3);
        assert!(result.is_some());
        let (token, slice) = result.unwrap();
        assert_eq!(*token, Token::Comma);
        assert_eq!(*slice, ",");
    }
}
