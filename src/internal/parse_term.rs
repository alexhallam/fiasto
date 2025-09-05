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
    // Check if this is a random effect (starts with opening parenthesis)
    if crate::internal::peek::peek(tokens, *pos)
        .map(|(t, _)| matches!(t, Token::FunctionStart))
        .unwrap_or(false)
    {
        // Check if the next token after the opening parenthesis suggests a random effect
        let saved_pos = *pos;
        *pos += 1; // Skip the opening parenthesis

        let is_random_effect = crate::internal::peek::peek(tokens, *pos)
            .map(|(t, _)| {
                matches!(
                    t,
                    Token::One | Token::Zero | Token::Minus | Token::ColumnName
                )
            })
            .unwrap_or(false);

        *pos = saved_pos; // Restore position

        if is_random_effect {
            // Parse as random effect
            let random_effect =
                crate::internal::parse_random_effect::parse_random_effect(tokens, pos)?;
            return Ok(Term::RandomEffect(random_effect));
        }
    }

    // Parse the leftmost atomic term (column, function, etc.)
    let atomic_term = {
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
                        | Token::Gr
                        | Token::Mm
                        | Token::Mmc
                        | Token::Cs
                        | Token::FunctionStart
                )
            },
            "Function token or ColumnName",
        )?;
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
            let args = crate::internal::parse_arg_list::parse_arg_list(tokens, pos)?;
            crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::FunctionEnd), ")")?;
            Term::Function { name: fname, args }
        } else {
            match tok {
                Token::ColumnName => {
                    // Return the atomic column name; interactions ('*' or ':') are
                    // handled by the loop after atomic term parsing to support
                    // chained interactions like `a*b*c`.
                    Term::Column(name_slice.to_string())
                }
                Token::Poly => return Err(ParseError::Syntax("expected '(' after 'poly'".into())),
                Token::Log => return Err(ParseError::Syntax("expected '(' after 'log'".into())),
                Token::Offset => return Err(ParseError::Syntax("expected '(' after 'offset'".into())),
                Token::Factor => return Err(ParseError::Syntax("expected '(' after 'factor'".into())),
                Token::Scale => return Err(ParseError::Syntax("expected '(' after 'scale'".into())),
                Token::Standardize => return Err(ParseError::Syntax("expected '(' after 'standardize'".into())),
                Token::Center => return Err(ParseError::Syntax("expected '(' after 'center'".into())),
                Token::BSplines => return Err(ParseError::Syntax("expected '(' after 'bs'".into())),
                Token::GaussianProcess => return Err(ParseError::Syntax("expected '(' after 'gp'".into())),
                Token::Monotonic => return Err(ParseError::Syntax("expected '(' after 'mono'".into())),
                Token::MeasurementError => return Err(ParseError::Syntax("expected '(' after 'me'".into())),
                Token::MissingValues => return Err(ParseError::Syntax("expected '(' after 'mi'".into())),
                Token::ForwardFill => return Err(ParseError::Syntax("expected '(' after 'forward_fill'".into())),
                Token::BackwardFill => return Err(ParseError::Syntax("expected '(' after 'backward_fill'".into())),
                Token::Diff => return Err(ParseError::Syntax("expected '(' after 'diff'".into())),
                Token::Lag => return Err(ParseError::Syntax("expected '(' after 'lag'".into())),
                Token::Lead => return Err(ParseError::Syntax("expected '(' after 'lead'".into())),
                Token::Trunc => return Err(ParseError::Syntax("expected '(' after 'trunc'".into())),
                Token::Weights => return Err(ParseError::Syntax("expected '(' after 'weights'".into())),
                Token::Trials => return Err(ParseError::Syntax("expected '(' after 'trials'".into())),
                Token::Censored => return Err(ParseError::Syntax("expected '(' after 'cens'".into())),
                _ => return Err(ParseError::Unexpected {
                    expected: "term",
                    found: Some(tok),
                }),
            }
        }
    };

    // Now check for multiplication (interaction) tokens and build up the interaction chain
    let mut term = atomic_term;
    loop {
        if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::InteractionAndEffect | Token::InteractionOnly)) {
            // `matches` already consumed the interaction token, so parse the right-hand term now
            let right = parse_term(tokens, pos)?;
            term = Term::Interaction {
                left: Box::new(term),
                right: Box::new(right),
            };
        } else {
            break;
        }
    }
    Ok(term)
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
