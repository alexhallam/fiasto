use crate::internal::{ast::Term, errors::ParseError, lexer::Token};

/// Parses a single term in a formula, which can be either a column name or a function call.
///
/// This function handles the core building blocks of formula terms. A term can be:
/// - A simple column name (e.g., "x", "age", "income")
/// - A function call with arguments (e.g., "poly(x, 2)", "log(price)")
///
/// # Arguments
/// * `tokens` - Reference to the vector of tokens
/// * `pos` - Mutable reference to the current position (will be advanced)
///
/// # Returns
/// * `Result<Term, ParseError>` - The parsed term, or an error
///
/// # Example
/// ```
/// use fiasto::internal::parse_term::parse_term;
/// use fiasto::internal::lexer::Token;
/// use fiasto::internal::ast::Term;
///
/// // Parse a simple column term
/// let tokens = vec![
///     (Token::ColumnName, "x")
/// ];
/// let mut pos = 0;
///
/// let result = parse_term(&tokens, &mut pos);
/// assert!(result.is_ok());
/// match result.unwrap() {
///     Term::Column(name) => assert_eq!(name, "x"),
///     _ => panic!("Expected column term")
/// }
///
/// // Parse a function term
/// let tokens = vec![
///     (Token::Poly, "poly"),
///     (Token::FunctionStart, "("),
///     (Token::ColumnName, "x"),
///     (Token::Comma, ","),
///     (Token::Integer, "2"),
///     (Token::FunctionEnd, ")")
/// ];
/// let mut pos = 0;
///
/// let result = parse_term(&tokens, &mut pos);
/// assert!(result.is_ok());
/// match result.unwrap() {
///     Term::Function { name, args } => {
///         assert_eq!(name, "poly");
///         assert_eq!(args.len(), 2);
///     },
///     _ => panic!("Expected function term")
/// }
/// ```
///
/// # How it works
/// 1. Expects either a Poly token or ColumnName token
/// 2. If followed by FunctionStart, parses as a function call
/// 3. If not followed by FunctionStart, returns as a column term
/// 4. For functions, parses argument list and expects closing parenthesis
///
/// # Grammar Rule
/// ```text
/// term = column_name | function_call
/// function_call = (poly | column_name) "(" arg_list ")"
/// arg_list = [argument ("," argument)*]
/// ```
///
/// # Use Cases
/// - Parsing individual predictor variables
/// - Handling polynomial and other transformations
/// - Supporting user-defined function calls
/// - Building the term structure for models
///
/// # Examples of Valid Inputs
/// - `"x"` → Term::Column("x")
/// - `"poly(x, 2)"` → Term::Function { name: "poly", args: [x, 2] }
/// - `"log(price)"` → Term::Function { name: "log", args: [price] }
pub fn parse_term<'a>(tokens: &'a [(Token, &'a str)], pos: &mut usize) -> Result<Term, ParseError> {
    // If the token is a function token or column name then it will parse with `tok`
    let (tok, name_slice) = crate::internal::expect::expect(
        tokens,
        pos,
        |t| {
            matches!(
                t,
                Token::Poly
                    | Token::ColumnName
                    | Token::Log
                    | Token::Offset
                    | Token::Factor
                    | Token::Scale
                    | Token::Standardize
                    | Token::Center
                    | Token::BSplines
                    | Token::GaussianProcess
                    | Token::Monotonic
                    | Token::MeasurementError
                    | Token::MissingValues
                    | Token::ForwardFill
                    | Token::BackwardFill
                    | Token::Diff
                    | Token::Lag
                    | Token::Lead
                    | Token::Trunc
                    | Token::Weights
                    | Token::Trials
                    | Token::Censored
            )
        },
        "Function token or ColumnName",
    )?;
    // `tok` is matched to see if it is a function start
    // if it is a function start then it will check to see if the token is poly or a column name
    // if it is a poly then it will return "poly" else it will return the column name
    if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::FunctionStart)) {
        let fname = match tok {
            Token::Poly => "poly".to_string(),
            Token::Log => "log".to_string(),
            Token::Offset => "offset".to_string(),
            Token::Factor => "factor".to_string(),
            Token::Scale => "scale".to_string(),
            Token::Standardize => "standardize".to_string(),
            Token::Center => "center".to_string(),
            Token::BSplines => "bs".to_string(),
            Token::GaussianProcess => "gp".to_string(),
            Token::Monotonic => "mono".to_string(),
            Token::MeasurementError => "me".to_string(),
            Token::MissingValues => "mi".to_string(),
            Token::ForwardFill => "forward_fill".to_string(),
            Token::BackwardFill => "backward_fill".to_string(),
            Token::Diff => "diff".to_string(),
            Token::Lag => "lag".to_string(),
            Token::Lead => "lead".to_string(),
            Token::Trunc => "trunc".to_string(),
            Token::Weights => "weights".to_string(),
            Token::Trials => "trials".to_string(),
            Token::Censored => "cens".to_string(),
            Token::ColumnName => name_slice.to_string(),
            _ => unreachable!(),
        };
        // `parse_arg_list` is defined below
        // it returns the argument if followed by a function_end.
        // for example if poly(x, 3) is the input then we look for ")" and say that 3 is the argument
        let args = crate::internal::parse_arg_list::parse_arg_list(tokens, pos)?;
        crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::FunctionEnd), ")")?;
        Ok(Term::Function { name: fname, args })
    } else {
        // If the token is a column name then it will parse the column name
        // If the token is a function token then it will return an error (functions require parentheses)
        match tok {
            Token::ColumnName => Ok(Term::Column(name_slice.to_string())),
            Token::Poly => Err(ParseError::Syntax("expected '(' after 'poly'".into())),
            Token::Log => Err(ParseError::Syntax("expected '(' after 'log'".into())),
            Token::Offset => Err(ParseError::Syntax("expected '(' after 'offset'".into())),
            Token::Factor => Err(ParseError::Syntax("expected '(' after 'factor'".into())),
            Token::Scale => Err(ParseError::Syntax("expected '(' after 'scale'".into())),
            Token::Standardize => Err(ParseError::Syntax(
                "expected '(' after 'standardize'".into(),
            )),
            Token::Center => Err(ParseError::Syntax("expected '(' after 'center'".into())),
            Token::BSplines => Err(ParseError::Syntax("expected '(' after 'bs'".into())),
            Token::GaussianProcess => Err(ParseError::Syntax("expected '(' after 'gp'".into())),
            Token::Monotonic => Err(ParseError::Syntax("expected '(' after 'mono'".into())),
            Token::MeasurementError => Err(ParseError::Syntax("expected '(' after 'me'".into())),
            Token::MissingValues => Err(ParseError::Syntax("expected '(' after 'mi'".into())),
            Token::ForwardFill => Err(ParseError::Syntax(
                "expected '(' after 'forward_fill'".into(),
            )),
            Token::BackwardFill => Err(ParseError::Syntax(
                "expected '(' after 'backward_fill'".into(),
            )),
            Token::Diff => Err(ParseError::Syntax("expected '(' after 'diff'".into())),
            Token::Lag => Err(ParseError::Syntax("expected '(' after 'lag'".into())),
            Token::Lead => Err(ParseError::Syntax("expected '(' after 'lead'".into())),
            Token::Trunc => Err(ParseError::Syntax("expected '(' after 'trunc'".into())),
            Token::Weights => Err(ParseError::Syntax("expected '(' after 'weights'".into())),
            Token::Trials => Err(ParseError::Syntax("expected '(' after 'trials'".into())),
            Token::Censored => Err(ParseError::Syntax("expected '(' after 'cens'".into())),
            _ => Err(ParseError::Unexpected {
                expected: "term",
                found: Some(tok),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::lexer::Token;

    #[test]
    fn test_parse_term_simple_column() {
        let tokens = vec![(Token::ColumnName, "x")];
        let mut pos = 0;

        let result = parse_term(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Term::Column(name) => assert_eq!(name, "x"),
            _ => panic!("Expected column term"),
        }
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_term_poly_function() {
        let tokens = vec![
            (Token::Poly, "poly"),
            (Token::FunctionStart, "("),
            (Token::ColumnName, "x"),
            (Token::Comma, ","),
            (Token::Integer, "2"),
            (Token::FunctionEnd, ")"),
        ];
        let mut pos = 0;

        let result = parse_term(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Term::Function { name, args } => {
                assert_eq!(name, "poly");
                assert_eq!(args.len(), 2);
            }
            _ => panic!("Expected function term"),
        }
        assert_eq!(pos, 6);
    }

    #[test]
    fn test_parse_term_custom_function() {
        let tokens = vec![
            (Token::ColumnName, "log"),
            (Token::FunctionStart, "("),
            (Token::ColumnName, "price"),
            (Token::FunctionEnd, ")"),
        ];
        let mut pos = 0;

        let result = parse_term(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Term::Function { name, args } => {
                assert_eq!(name, "log");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected function term"),
        }
        assert_eq!(pos, 4);
    }

    #[test]
    fn test_parse_term_poly_without_parentheses() {
        let tokens = vec![(Token::Poly, "poly")];
        let mut pos = 0;

        let result = parse_term(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 1); // Position advanced past poly
    }

    #[test]
    fn test_parse_term_function_with_multiple_args() {
        let tokens = vec![
            (Token::ColumnName, "custom_func"),
            (Token::FunctionStart, "("),
            (Token::ColumnName, "x"),
            (Token::Comma, ","),
            (Token::ColumnName, "y"),
            (Token::Comma, ","),
            (Token::Integer, "10"),
            (Token::FunctionEnd, ")"),
        ];
        let mut pos = 0;

        let result = parse_term(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Term::Function { name, args } => {
                assert_eq!(name, "custom_func");
                assert_eq!(args.len(), 3);
            }
            _ => panic!("Expected function term"),
        }
        assert_eq!(pos, 8);
    }

    #[test]
    fn test_parse_term_function_without_closing_paren() {
        let tokens = vec![
            (Token::ColumnName, "func"),
            (Token::FunctionStart, "("),
            (Token::ColumnName, "x"),
        ];
        let mut pos = 0;

        let result = parse_term(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 3); // Position at end
    }

    #[test]
    fn test_parse_term_long_column_name() {
        let tokens = vec![(Token::ColumnName, "very_long_column_name_with_underscores")];
        let mut pos = 0;

        let result = parse_term(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Term::Column(name) => assert_eq!(name, "very_long_column_name_with_underscores"),
            _ => panic!("Expected column term"),
        }
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_term_numeric_column_name() {
        let tokens = vec![(Token::ColumnName, "x1")];
        let mut pos = 0;

        let result = parse_term(&tokens, &mut pos);
        assert!(result.is_ok());
        match result.unwrap() {
            Term::Column(name) => assert_eq!(name, "x1"),
            _ => panic!("Expected column term"),
        }
        assert_eq!(pos, 1);
    }
}
