use crate::internal::{errors::ParseError, lexer::Token};

/// Parses the response variable from the beginning of a formula.
/// 
/// This function extracts the left-hand side (response variable) of a formula.
/// In R-style formulas, the response variable appears before the tilde (`~`) symbol.
/// 
/// # Arguments
/// * `tokens` - Reference to the vector of tokens
/// * `pos` - Mutable reference to the current position (will be incremented)
/// 
/// # Returns
/// * `Result<String, ParseError>` - The response variable name, or an error
/// 
/// # Example
/// ```
/// use fiasto::internal::parse_response::parse_response;
/// use fiasto::internal::lexer::Token;
/// 
/// let tokens = vec![
///     (Token::ColumnName, "y"),
///     (Token::Tilde, "~"),
///     (Token::ColumnName, "x")
/// ];
/// let mut pos = 0;
/// 
/// let response = parse_response(&tokens, &mut pos);
/// assert!(response.is_ok());
/// assert_eq!(response.unwrap(), "y");
/// assert_eq!(pos, 1); // Position advanced past response variable
/// ```
/// 
/// # How it works
/// 1. Expects the first token to be a ColumnName (the response variable)
/// 2. Returns the string representation of the response variable
/// 3. Advances the position to prepare for parsing the tilde and right-hand side
/// 
/// # Grammar Rule
/// ```
/// formula = response "~" rhs ["," family_spec]
/// response = column_name
/// ```
/// 
/// # Use Cases
/// - Extracting the dependent variable from regression formulas
/// - Validating that formulas start with a valid column name
/// - Preparing for parsing the right-hand side of the formula
/// 
/// # Examples of Valid Inputs
/// - `"y ~ x"` → response = "y"
/// - `"response_var ~ predictor"` → response = "response_var"
/// - `"target ~ feature1 + feature2"` → response = "target"
pub fn parse_response<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<String, ParseError> {
    let (_, name) = crate::internal::expect::expect(
        tokens,
        pos,
        |t| matches!(t, Token::ColumnName),
        "ColumnName",
    )?;
    Ok(name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::lexer::Token;

    #[test]
    fn test_parse_response_simple() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x")
        ];
        let mut pos = 0;
        
        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "y");
        assert_eq!(pos, 1); // Position advanced
    }

    #[test]
    fn test_parse_response_with_long_name() {
        let tokens = vec![
            (Token::ColumnName, "response_variable"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x")
        ];
        let mut pos = 0;
        
        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "response_variable");
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_response_failure_wrong_token() {
        let tokens = vec![
            (Token::Tilde, "~"),
            (Token::ColumnName, "y")
        ];
        let mut pos = 0;
        
        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_response_failure_end_of_input() {
        let tokens: Vec<(Token, &str)> = vec![];
        let mut pos = 0;
        
        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_response_with_numeric_name() {
        let tokens = vec![
            (Token::ColumnName, "y1"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x")
        ];
        let mut pos = 0;
        
        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "y1");
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_response_with_underscore_name() {
        let tokens = vec![
            (Token::ColumnName, "target_variable"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "feature")
        ];
        let mut pos = 0;
        
        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "target_variable");
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_response_preserves_position_on_failure() {
        let tokens = vec![
            (Token::Plus, "+"),
            (Token::ColumnName, "y")
        ];
        let mut pos = 0;
        
        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_response_with_single_token() {
        let tokens = vec![
            (Token::ColumnName, "z")
        ];
        let mut pos = 0;
        
        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "z");
        assert_eq!(pos, 1);
    }
}
