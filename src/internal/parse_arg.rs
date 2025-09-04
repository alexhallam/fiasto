use crate::internal::{ast::Argument, errors::ParseError, lexer::Token};

/// Parses a single argument within a function call.

/// Parses a single argument within a function call.
/// 
/// This function handles individual arguments that can appear in function calls.
/// Arguments can be column names (identifiers), integers, or the literal "1".
/// 
/// # Arguments
/// * `tokens` - Reference to the vector of tokens
/// * `pos` - Mutable reference to the current position (will be advanced)
/// 
/// # Returns
/// * `Result<Argument, ParseError>` - The parsed argument, or an error
/// 
/// # Example
/// ```
/// use fiasto::internal::parse_arg::parse_arg;
/// use fiasto::internal::lexer::Token;
/// use fiasto::internal::ast::Argument;
/// 
/// // Parse a column name argument
/// let tokens = vec![
///     (Token::ColumnName, "x")
/// ];
/// let mut pos = 0;
/// 
/// let result = parse_arg(&tokens, &mut pos);
/// assert!(result.is_ok());
/// match result.unwrap() {
///     Argument::Ident(name) => assert_eq!(name, "x"),
///     _ => panic!("Expected identifier argument")
/// }
/// 
/// // Parse an integer argument
/// let tokens = vec![
///     (Token::Integer, "42")
/// ];
/// let mut pos = 0;
/// 
/// let result = parse_arg(&tokens, &mut pos);
/// assert!(result.is_ok());
/// match result.unwrap() {
///     Argument::Integer(value) => assert_eq!(value, 42),
///     _ => panic!("Expected integer argument")
/// }
/// ```
/// 
/// # How it works
/// 1. Examines the next token without consuming it
/// 2. Based on token type, creates appropriate Argument variant
/// 3. Advances position and returns the parsed argument
/// 4. Returns error for unexpected token types
/// 
/// # Grammar Rule
/// ```
/// argument = column_name | integer | "1"
/// column_name = identifier
/// integer = [0-9]+
/// ```
/// 
/// # Use Cases
/// - Parsing function call parameters
/// - Supporting polynomial degrees and other numeric parameters
/// - Handling column references in transformations
/// - Building argument structures for function terms
/// 
/// # Examples of Valid Inputs
/// - `"x"` → Argument::Ident("x")
/// - `"42"` → Argument::Integer(42)
/// - `"1"` → Argument::Integer(1)
/// - `"variable_name"` → Argument::Ident("variable_name")
pub fn parse_arg<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<Argument, ParseError> {
    if let Some((tok, slice)) = crate::internal::peek::peek(tokens, *pos).cloned() {
        match tok {
            Token::ColumnName => {
                crate::internal::next::next(tokens, pos);
                Ok(Argument::Ident(slice.to_string()))
            }
            Token::Integer => {
                crate::internal::next::next(tokens, pos);
                Ok(Argument::Integer(slice.parse().unwrap()))
            }
            Token::One => {
                crate::internal::next::next(tokens, pos);
                Ok(Argument::Integer(1))
            }
            _ => Err(ParseError::Unexpected {
                expected: "argument",
                found: Some(tok),
            }),
        }
    } else {
        // ParseError::Eoi is... idk
        Err(ParseError::Eoi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::lexer::Token;

    #[test]
    fn test_parse_arg_column_name() {
        let tokens = vec![
            (Token::ColumnName, "x")
        ];
        let mut pos = 0;
        
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Argument::Ident(name) => assert_eq!(name, "x"),
            _ => panic!("Expected identifier argument")
        }
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_arg_integer() {
        let tokens = vec![
            (Token::Integer, "42")
        ];
        let mut pos = 0;
        
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Argument::Integer(value) => assert_eq!(value, 42),
            _ => panic!("Expected integer argument")
        }
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_arg_one() {
        let tokens = vec![
            (Token::One, "1")
        ];
        let mut pos = 0;
        
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Argument::Integer(value) => assert_eq!(value, 1),
            _ => panic!("Expected integer argument")
        }
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_arg_long_column_name() {
        let tokens = vec![
            (Token::ColumnName, "very_long_variable_name")
        ];
        let mut pos = 0;
        
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Argument::Ident(name) => assert_eq!(name, "very_long_variable_name"),
            _ => panic!("Expected identifier argument")
        }
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_arg_numeric_column_name() {
        let tokens = vec![
            (Token::ColumnName, "x1")
        ];
        let mut pos = 0;
        
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Argument::Ident(name) => assert_eq!(name, "x1"),
            _ => panic!("Expected identifier argument")
        }
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_arg_large_integer() {
        let tokens = vec![
            (Token::Integer, "1000")
        ];
        let mut pos = 0;
        
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Argument::Integer(value) => assert_eq!(value, 1000),
            _ => panic!("Expected integer argument")
        }
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_arg_zero() {
        let tokens = vec![
            (Token::Integer, "0")
        ];
        let mut pos = 0;
        
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Argument::Integer(value) => assert_eq!(value, 0),
            _ => panic!("Expected integer argument")
        }
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_arg_invalid_token() {
        let tokens = vec![
            (Token::Plus, "+")
        ];
        let mut pos = 0;
        
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_arg_end_of_input() {
        let tokens: Vec<(Token, &str)> = vec![];
        let mut pos = 0;
        
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_arg_advances_position_on_success() {
        let tokens = vec![
            (Token::ColumnName, "x"),
            (Token::Integer, "5")
        ];
        let mut pos = 0;
        
        // First argument
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_ok());
        assert_eq!(pos, 1);
        
        // Second argument
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_ok());
        assert_eq!(pos, 2);
    }

    #[test]
    fn test_parse_arg_preserves_string_slice() {
        let tokens = vec![
            (Token::ColumnName, "response_variable"),
            (Token::Integer, "12345")
        ];
        let mut pos = 0;
        
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Argument::Ident(name) => assert_eq!(name, "response_variable"),
            _ => panic!("Expected identifier argument")
        }
        
        pos = 1;
        let result = parse_arg(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Argument::Integer(value) => assert_eq!(value, 12345),
            _ => panic!("Expected integer argument")
        }
    }
}
