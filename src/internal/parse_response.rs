use crate::internal::{ast::Response, errors::ParseError, lexer::Token};

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
/// * `Result<Response, ParseError>` - The response variable(s), or an error
///
/// # Example
/// ```
/// use fiasto::internal::parse_response::parse_response;
/// use fiasto::internal::lexer::Token;
/// use fiasto::internal::ast::Response;
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
/// match response.unwrap() {
///     Response::Single(name) => assert_eq!(name, "y"),
///     _ => panic!("Expected single response")
/// }
/// assert_eq!(pos, 1); // Position advanced past response variable
/// ```
///
/// # How it works
/// 1. Expects either a ColumnName (single response) or Bind token (multivariate response)
/// 2. For single responses, returns the variable name
/// 3. For multivariate responses, parses the bind() function call
/// 4. Advances the position to prepare for parsing the tilde and right-hand side
///
/// # Grammar Rule
/// ```text
/// formula = response "~" rhs ["," family_spec]
/// response = column_name | bind(column_name, ...)
/// ```
///
/// # Use Cases
/// - Extracting the dependent variable(s) from regression formulas
/// - Supporting both univariate and multivariate response models
/// - Validating that formulas start with a valid response specification
/// - Preparing for parsing the right-hand side of the formula
///
/// # Examples of Valid Inputs
/// - `"y ~ x"` → response = Response::Single("y")
/// - `"bind(y1, y2) ~ x"` → response = Response::Multivariate(vec!["y1", "y2"])
/// - `"response_var ~ predictor"` → response = Response::Single("response_var")
pub fn parse_response<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<Response, ParseError> {
    let (token, name) = crate::internal::expect::expect(
        tokens,
        pos,
        |t| matches!(t, Token::ColumnName | Token::Bind),
        "ColumnName or Bind",
    )?;

    match token {
        Token::ColumnName => {
            // Single response variable
            Ok(Response::Single(name.to_string()))
        }
        Token::Bind => {
            // Multivariate response: bind(y1, y2, ...)
            crate::internal::expect::expect(
                tokens,
                pos,
                |t| matches!(t, Token::FunctionStart),
                "(",
            )?;

            let mut variables = Vec::new();

            // Parse first variable
            let (_, first_var) = crate::internal::expect::expect(
                tokens,
                pos,
                |t| matches!(t, Token::ColumnName),
                "ColumnName",
            )?;
            variables.push(first_var.to_string());

            // Parse additional variables separated by commas
            while crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Comma)) {
                let (_, var_name) = crate::internal::expect::expect(
                    tokens,
                    pos,
                    |t| matches!(t, Token::ColumnName),
                    "ColumnName",
                )?;
                variables.push(var_name.to_string());
            }

            crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::FunctionEnd), ")")?;

            if variables.len() < 2 {
                return Err(ParseError::Syntax(
                    "bind() requires at least 2 variables".into(),
                ));
            }

            Ok(Response::Multivariate(variables))
        }
        _ => unreachable!(),
    }
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
            (Token::ColumnName, "x"),
        ];
        let mut pos = 0;

        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Response::Single(name) => assert_eq!(name, "y"),
            _ => panic!("Expected single response"),
        }
        assert_eq!(pos, 1); // Position advanced
    }

    #[test]
    fn test_parse_response_with_long_name() {
        let tokens = vec![
            (Token::ColumnName, "response_variable"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x"),
        ];
        let mut pos = 0;

        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Response::Single(name) => assert_eq!(name, "response_variable"),
            _ => panic!("Expected single response"),
        }
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_response_failure_wrong_token() {
        let tokens = vec![(Token::Tilde, "~"), (Token::ColumnName, "y")];
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
            (Token::ColumnName, "x"),
        ];
        let mut pos = 0;

        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Response::Single(name) => assert_eq!(name, "y1"),
            _ => panic!("Expected single response"),
        }
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_response_with_underscore_name() {
        let tokens = vec![
            (Token::ColumnName, "target_variable"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "feature"),
        ];
        let mut pos = 0;

        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Response::Single(name) => assert_eq!(name, "target_variable"),
            _ => panic!("Expected single response"),
        }
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_response_preserves_position_on_failure() {
        let tokens = vec![(Token::Plus, "+"), (Token::ColumnName, "y")];
        let mut pos = 0;

        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_response_with_single_token() {
        let tokens = vec![(Token::ColumnName, "z")];
        let mut pos = 0;

        let result = parse_response(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Response::Single(name) => assert_eq!(name, "z"),
            _ => panic!("Expected single response"),
        }
        assert_eq!(pos, 1);
    }
}
