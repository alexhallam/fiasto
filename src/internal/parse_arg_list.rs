use crate::internal::{ast::Argument, errors::ParseError, lexer::Token};

/// Parses a list of arguments within a function call.
///
/// This function handles the arguments that appear between parentheses in function calls.
/// It supports both empty argument lists (e.g., `func()`) and lists with multiple
/// comma-separated arguments (e.g., `func(x, 2, y)`).
///
/// # Arguments
/// * `tokens` - Reference to the vector of tokens
/// * `pos` - Mutable reference to the current position (will be advanced)
///
/// # Returns
/// * `Result<Vec<Argument>, ParseError>` - Vector of parsed arguments, or an error
///
/// # Example
/// ```
/// use fiasto::internal::parse_arg_list::parse_arg_list;
/// use fiasto::internal::lexer::Token;
///
/// // Parse arguments for poly(x, 2)
/// let tokens = vec![
///     (Token::ColumnName, "x"),
///     (Token::Comma, ","),
///     (Token::Integer, "2"),
///     (Token::FunctionEnd, ")")
/// ];
/// let mut pos = 0;
///
/// let result = parse_arg_list(&tokens, &mut pos);
/// assert!(result.is_ok());
/// let args = result.unwrap();
/// assert_eq!(args.len(), 2);
/// ```
///
/// # How it works
/// 1. Checks if the next token is a closing parenthesis (empty list)
/// 2. If not empty, parses the first argument
/// 3. Continues parsing additional arguments separated by commas
/// 4. Stops when encountering a closing parenthesis or end of tokens
///
/// # Grammar Rule
/// ```text
/// arg_list = [argument ("," argument)*]
/// argument = column_name | integer | "1"
/// ```
///
/// # Use Cases
/// - Parsing function call arguments
/// - Supporting polynomial degrees and other parameters
/// - Handling user-defined function parameters
/// - Building argument structures for function terms
///
/// # Examples of Valid Inputs
/// - `""` → [] (empty list)
/// - `"x"` → [Argument::Ident("x")]
/// - `"x, 2"` → [Argument::Ident("x"), Argument::Integer(2)]
/// - `"x, y, 10"` → [Argument::Ident("x"), Argument::Ident("y"), Argument::Integer(10)]
pub fn parse_arg_list<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<Vec<Argument>, ParseError> {
    let mut args = Vec::new();
    if let Some((tok, _)) = crate::internal::peek::peek(tokens, *pos).cloned() {
        if matches!(tok, Token::FunctionEnd) {
            return Ok(args);
        }
    }

    args.push(crate::internal::parse_arg::parse_arg(tokens, pos)?);
    while crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Comma)) {
        args.push(crate::internal::parse_arg::parse_arg(tokens, pos)?);
    }
    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::lexer::Token;

    #[test]
    fn test_parse_arg_list_empty() {
        let tokens = vec![(Token::FunctionEnd, ")")];
        let mut pos = 0;

        let result = parse_arg_list(&tokens, &mut pos);
        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.len(), 0);
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_arg_list_single_argument() {
        let tokens = vec![(Token::ColumnName, "x"), (Token::FunctionEnd, ")")];
        let mut pos = 0;

        let result = parse_arg_list(&tokens, &mut pos);
        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.len(), 1);
        assert_eq!(pos, 1); // Position advanced past argument
    }

    #[test]
    fn test_parse_arg_list_two_arguments() {
        let tokens = vec![
            (Token::ColumnName, "x"),
            (Token::Comma, ","),
            (Token::Integer, "2"),
            (Token::FunctionEnd, ")"),
        ];
        let mut pos = 0;

        let result = parse_arg_list(&tokens, &mut pos);
        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.len(), 2);
        assert_eq!(pos, 3); // Position advanced past all arguments
    }

    #[test]
    fn test_parse_arg_list_multiple_arguments() {
        let tokens = vec![
            (Token::ColumnName, "x"),
            (Token::Comma, ","),
            (Token::ColumnName, "y"),
            (Token::Comma, ","),
            (Token::Integer, "10"),
            (Token::FunctionEnd, ")"),
        ];
        let mut pos = 0;

        let result = parse_arg_list(&tokens, &mut pos);
        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.len(), 3);
        assert_eq!(pos, 5); // Position advanced past all arguments
    }

    #[test]
    fn test_parse_arg_list_with_integer_argument() {
        let tokens = vec![(Token::Integer, "42"), (Token::FunctionEnd, ")")];
        let mut pos = 0;

        let result = parse_arg_list(&tokens, &mut pos);
        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.len(), 1);
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_arg_list_with_one_argument() {
        let tokens = vec![(Token::One, "1"), (Token::FunctionEnd, ")")];
        let mut pos = 0;

        let result = parse_arg_list(&tokens, &mut pos);
        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.len(), 1);
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_arg_list_mixed_types() {
        let tokens = vec![
            (Token::ColumnName, "variable"),
            (Token::Comma, ","),
            (Token::Integer, "5"),
            (Token::Comma, ","),
            (Token::One, "1"),
            (Token::FunctionEnd, ")"),
        ];
        let mut pos = 0;

        let result = parse_arg_list(&tokens, &mut pos);
        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.len(), 3);
        assert_eq!(pos, 5);
    }

    #[test]
    fn test_parse_arg_list_no_closing_paren() {
        let tokens = vec![
            (Token::ColumnName, "x"),
            (Token::Comma, ","),
            (Token::ColumnName, "y"),
        ];
        let mut pos = 0;

        let result = parse_arg_list(&tokens, &mut pos);
        assert!(result.is_ok()); // This should succeed as it's not this function's job to check for closing paren
        let args = result.unwrap();
        assert_eq!(args.len(), 2);
        assert_eq!(pos, 3);
    }

    #[test]
    fn test_parse_arg_list_with_whitespace_equivalent() {
        // Test that the function handles tokens correctly regardless of spacing
        let tokens = vec![
            (Token::ColumnName, "x"),
            (Token::Comma, ","),
            (Token::ColumnName, "y"),
            (Token::Comma, ","),
            (Token::ColumnName, "z"),
            (Token::FunctionEnd, ")"),
        ];
        let mut pos = 0;

        let result = parse_arg_list(&tokens, &mut pos);
        assert!(result.is_ok());
        let args = result.unwrap();
        assert_eq!(args.len(), 3);
        assert_eq!(pos, 5);
    }
}
