/// Consumes and returns the next token, advancing the position in the token stream.
/// 
/// This function is the primary way to consume tokens during parsing. Unlike `peek`,
/// it advances the position counter, effectively "consuming" the token so it won't
/// be seen again in subsequent calls.
/// 
/// # Arguments
/// * `tokens` - Reference to the vector of tokens
/// * `pos` - Mutable reference to the current position (will be incremented)
/// 
/// # Returns
/// * `Option<(Token, &'a str)>` - The consumed token and its string slice, or None if at end
/// 
/// # Example
/// ```
/// use fiasto::internal::next::next;
/// use fiasto::internal::lexer::Token;
/// 
/// let tokens = vec![
///     (Token::ColumnName, "y"),
///     (Token::Tilde, "~"),
///     (Token::ColumnName, "x")
/// ];
/// let mut pos = 0;
/// 
/// let first_token = next(&tokens, &mut pos);
/// assert!(first_token.is_some());
/// let (token, slice) = first_token.unwrap();
/// assert_eq!(token, Token::ColumnName);
/// assert_eq!(slice, "y");
/// assert_eq!(pos, 1); // Position advanced
/// 
/// let second_token = next(&tokens, &mut pos);
/// assert!(second_token.is_some());
/// let (token, slice) = second_token.unwrap();
/// assert_eq!(token, Token::Tilde);
/// assert_eq!(slice, "~");
/// assert_eq!(pos, 2); // Position advanced again
/// ```
/// 
/// # How it works
/// 1. Takes a reference to the tokens vector and mutable reference to position
/// 2. Returns the token at the current position
/// 3. Increments the position counter (consuming the token)
/// 4. Returns None if position is beyond the end of tokens
/// 
/// # Use Cases
/// - Consuming expected tokens in sequence
/// - Moving through the token stream during parsing
/// - Implementing recursive descent parsing
/// - Error recovery by consuming tokens
/// 
/// # Important Notes
/// - This function modifies the position parameter
/// - Once a token is consumed, it cannot be "unconsumed"
/// - Always call `peek` first if you need to examine a token before consuming it
pub fn next<'a>(
    tokens: &'a [(crate::internal::lexer::Token, &'a str)],
    pos: &mut usize,
) -> Option<(crate::internal::lexer::Token, &'a str)> {
    let t = tokens.get(*pos).cloned();
    if t.is_some() {
        *pos += 1;
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::lexer::Token;

    #[test]
    fn test_next_consumes_tokens() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x")
        ];
        let mut pos = 0;
        
        let first = next(&tokens, &mut pos);
        assert_eq!(pos, 1);
        assert!(first.is_some());
        assert_eq!(first.unwrap().0, Token::ColumnName);
        
        let second = next(&tokens, &mut pos);
        assert_eq!(pos, 2);
        assert!(second.is_some());
        assert_eq!(second.unwrap().0, Token::Tilde);
        
        let third = next(&tokens, &mut pos);
        assert_eq!(pos, 3);
        assert!(third.is_some());
        assert_eq!(third.unwrap().0, Token::ColumnName);
    }

    #[test]
    fn test_next_at_end_returns_none() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~")
        ];
        let mut pos = 2; // At end
        
        let result = next(&tokens, &mut pos);
        assert!(result.is_none());
        assert_eq!(pos, 2); // Position unchanged
    }

    #[test]
    fn test_next_beyond_end_returns_none() {
        let tokens = vec![
            (Token::ColumnName, "y")
        ];
        let mut pos = 5; // Beyond end
        
        let result = next(&tokens, &mut pos);
        assert!(result.is_none());
        assert_eq!(pos, 5); // Position unchanged
    }

    #[test]
    fn test_next_with_empty_tokens() {
        let tokens: Vec<(Token, &str)> = vec![];
        let mut pos = 0;
        
        let result = next(&tokens, &mut pos);
        assert!(result.is_none());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_next_preserves_string_slices() {
        let tokens = vec![
            (Token::ColumnName, "response_var"),
            (Token::Integer, "42")
        ];
        let mut pos = 0;
        
        let first = next(&tokens, &mut pos);
        assert_eq!(first.unwrap().1, "response_var");
        
        let second = next(&tokens, &mut pos);
        assert_eq!(second.unwrap().1, "42");
    }

    #[test]
    fn test_next_with_function_tokens() {
        let tokens = vec![
            (Token::Poly, "poly"),
            (Token::FunctionStart, "("),
            (Token::ColumnName, "x"),
            (Token::Comma, ","),
            (Token::Integer, "3"),
            (Token::FunctionEnd, ")")
        ];
        let mut pos = 0;
        
        // Consume function name
        let func_name = next(&tokens, &mut pos);
        assert_eq!(func_name.unwrap().0, Token::Poly);
        assert_eq!(pos, 1);
        
        // Consume opening parenthesis
        let open_paren = next(&tokens, &mut pos);
        assert_eq!(open_paren.unwrap().0, Token::FunctionStart);
        assert_eq!(pos, 2);
    }
}
