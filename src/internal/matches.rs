/// Conditionally consumes a token if it matches a predicate, advancing the position.
///
/// This function is a powerful utility for implementing conditional parsing logic.
/// It allows the parser to optionally consume tokens based on whether they meet
/// certain criteria, which is essential for handling optional elements in formulas.
///
/// # Arguments
/// * `tokens` - Reference to the vector of tokens
/// * `pos` - Mutable reference to the current position (will be incremented if match)
/// * `pred` - A predicate function that takes a Token and returns a boolean
///
/// # Returns
/// * `bool` - True if the token matched and was consumed, false otherwise
///
/// # Example
/// ```
/// use fiasto::internal::matches::matches;
/// use fiasto::internal::lexer::Token;
///
/// let tokens = vec![
///     (Token::Plus, "+"),
///     (Token::ColumnName, "x"),
///     (Token::Minus, "-"),
///     (Token::ColumnName, "y")
/// ];
/// let mut pos = 0;
///
/// // Try to match a plus sign
/// let matched_plus = matches(&tokens, &mut pos, |t| matches!(t, Token::Plus));
/// assert!(matched_plus); // Plus was found and consumed
/// assert_eq!(pos, 1); // Position advanced
///
/// // Try to match another plus sign (should fail)
/// let matched_plus_again = matches(&tokens, &mut pos, |t| matches!(t, Token::Plus));
/// assert!(!matched_plus_again); // No plus found
/// assert_eq!(pos, 1); // Position unchanged
///
/// // Skip the column name at position 1
/// pos += 1; // Now at position 2 (the minus sign)
/// 
/// // Try to match a minus sign
/// let matched_minus = matches(&tokens, &mut pos, |t| matches!(t, Token::Minus));
/// assert!(matched_minus); // Minus was found and consumed
/// assert_eq!(pos, 3); // Position advanced
/// ```
///
/// # How it works
/// 1. Examines the token at the current position using `peek`
/// 2. If the token matches the predicate, consumes it and advances position
/// 3. If no match, leaves position unchanged and returns false
/// 4. Returns true if a token was consumed, false otherwise
///
/// # Use Cases
/// - Parsing optional operators (e.g., `+` in `x + y`)
/// - Handling optional punctuation (e.g., commas in lists)
/// - Implementing conditional parsing logic
/// - Error recovery by consuming expected tokens
///
/// # Common Patterns
/// - `matches(tokens, pos, |t| matches!(t, Token::Plus))` - Match plus signs
/// - `matches(tokens, pos, |t| matches!(t, Token::Comma))` - Match commas
/// - `matches(tokens, pos, |t| matches!(t, Token::Minus))` - Match minus signs
pub fn matches<'a, F>(
    tokens: &'a [(crate::internal::lexer::Token, &'a str)],
    pos: &mut usize,
    pred: F,
) -> bool
where
    F: Fn(&crate::internal::lexer::Token) -> bool,
{
    if let Some((tok, _)) = tokens.get(*pos) {
        if pred(tok) {
            *pos += 1;
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::lexer::Token;

    #[test]
    fn test_matches_consumes_when_predicate_true() {
        let tokens = vec![(Token::Plus, "+"), (Token::ColumnName, "x")];
        let mut pos = 0;

        let result = matches(&tokens, &mut pos, |t| matches!(t, Token::Plus));
        assert!(result);
        assert_eq!(pos, 1); // Position advanced
    }

    #[test]
    fn test_matches_does_not_consume_when_predicate_false() {
        let tokens = vec![(Token::Minus, "-"), (Token::ColumnName, "x")];
        let mut pos = 0;

        let result = matches(&tokens, &mut pos, |t| matches!(t, Token::Plus));
        assert!(!result);
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_matches_with_multiple_plus_signs() {
        let tokens = vec![
            (Token::Plus, "+"),
            (Token::Plus, "+"),
            (Token::ColumnName, "x"),
        ];
        let mut pos = 0;

        // First plus
        let first = matches(&tokens, &mut pos, |t| matches!(t, Token::Plus));
        assert!(first);
        assert_eq!(pos, 1);

        // Second plus
        let second = matches(&tokens, &mut pos, |t| matches!(t, Token::Plus));
        assert!(second);
        assert_eq!(pos, 2);

        // No more plus signs
        let third = matches(&tokens, &mut pos, |t| matches!(t, Token::Plus));
        assert!(!third);
        assert_eq!(pos, 2); // Position unchanged
    }

    #[test]
    fn test_matches_at_end_of_tokens() {
        let tokens = vec![(Token::ColumnName, "x")];
        let mut pos = 1; // At end

        let result = matches(&tokens, &mut pos, |t| matches!(t, Token::Plus));
        assert!(!result);
        assert_eq!(pos, 1); // Position unchanged
    }

    #[test]
    fn test_matches_with_complex_predicate() {
        let tokens = vec![
            (Token::ColumnName, "x"),
            (Token::Integer, "42"),
            (Token::One, "1"),
        ];
        let mut pos = 0;

        // Match any numeric token
        let numeric_predicate = |t: &Token| matches!(t, Token::Integer | Token::One);

        let first = matches(&tokens, &mut pos, numeric_predicate);
        assert!(!first); // ColumnName is not numeric
        assert_eq!(pos, 0);

        // Move to integer
        pos = 1;
        let second = matches(&tokens, &mut pos, numeric_predicate);
        assert!(second); // Integer is numeric
        assert_eq!(pos, 2);

        let third = matches(&tokens, &mut pos, numeric_predicate);
        assert!(third); // One is numeric
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_matches_with_empty_tokens() {
        let tokens: Vec<(Token, &str)> = vec![];
        let mut pos = 0;

        let result = matches(&tokens, &mut pos, |t| matches!(t, Token::Plus));
        assert!(!result);
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_matches_preserves_position_when_no_match() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x"),
        ];
        let mut pos = 1; // At tilde

        let result = matches(&tokens, &mut pos, |t| matches!(t, Token::Plus));
        assert!(!result);
        assert_eq!(pos, 1); // Still at tilde
    }
}
