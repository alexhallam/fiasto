use crate::internal::{ast::Term, errors::ParseError, lexer::Token};

/// Parses the right-hand side of a formula, including terms and intercept specification.
/// 
/// This function handles the part of the formula that comes after the tilde (`~`).
/// It parses a sequence of terms separated by plus signs and optionally handles
/// intercept removal with `- 1`.
/// 
/// # Arguments
/// * `tokens` - Reference to the vector of tokens
/// * `pos` - Mutable reference to the current position (will be advanced)
/// 
/// # Returns
/// * `Result<(Vec<Term>, bool), ParseError>` - A tuple containing:
///   - Vector of parsed terms
///   - Boolean indicating whether intercept is included (true) or removed (false)
/// 
/// # Example
/// ```
/// use fiasto::internal::parse_rhs::parse_rhs;
/// use fiasto::internal::lexer::Token;
/// 
/// let tokens = vec![
///     (Token::ColumnName, "x"),
///     (Token::Plus, "+"),
///     (Token::ColumnName, "z"),
///     (Token::Minus, "-"),
///     (Token::One, "1")
/// ];
/// let mut pos = 0;
/// 
/// let result = parse_rhs(&tokens, &mut pos);
/// assert!(result.is_ok());
/// let (terms, has_intercept) = result.unwrap();
/// assert_eq!(terms.len(), 2); // x and z
/// assert!(!has_intercept); // -1 removes intercept
/// ```
/// 
/// # How it works
/// 1. Parses the first term if it exists (no leading plus)
/// 2. Parses additional terms separated by plus signs
/// 3. Optionally handles intercept removal with `- 1`
/// 4. Returns the collected terms and intercept flag
/// 
/// # Grammar Rule
/// ```text
/// rhs = [term] ("+" term)* ["-" "1"]
/// term = column_name | function_call
/// ```
/// 
/// # Use Cases
/// - Parsing predictor variables in regression formulas
/// - Handling intercept inclusion/exclusion
/// - Building term lists for model specification
/// - Supporting additive model structures
/// 
/// # Examples of Valid Inputs
/// - `"x"` → terms=["x"], intercept=true
/// - `"x + z"` → terms=["x", "z"], intercept=true
/// - `"x + z - 1"` → terms=["x", "z"], intercept=false
/// - `""` → terms=[], intercept=true (empty RHS)
pub fn parse_rhs<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<(Vec<Term>, bool), ParseError> {
    let mut terms = Vec::new();
    let mut has_intercept = true;

    // if the next token is not a comma or plus then it is pushed to the parse_term function
    if crate::internal::peek::peek(tokens, *pos).is_some() && !matches!(crate::internal::peek::peek(tokens, *pos).unwrap().0, Token::Comma | Token::Plus) {
        terms.push(crate::internal::parse_term::parse_term(tokens, pos)?);
    }
    // If the token is a plus then it is pushed to the parse_term function
    while crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Plus)) {
        terms.push(crate::internal::parse_term::parse_term(tokens, pos)?);
    }

    // If the token is a minus and a one then it has no intercept
    if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Minus)) {
        if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::One)) {
            has_intercept = false;
        } else {
            return Err(crate::internal::errors::ParseError::Syntax(
                "expected '1' after '-' to remove intercept".into(),
            ));
        }
    }

    Ok((terms, has_intercept))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::lexer::Token;

    #[test]
    fn test_parse_rhs_single_term() {
        let tokens = vec![
            (Token::ColumnName, "x")
        ];
        let mut pos = 0;
        
        let result = parse_rhs(&tokens, &mut pos);
        assert!(result.is_ok());
        let (terms, has_intercept) = result.unwrap();
        assert_eq!(terms.len(), 1);
        assert!(has_intercept);
    }

    #[test]
    fn test_parse_rhs_multiple_terms() {
        let tokens = vec![
            (Token::ColumnName, "x"),
            (Token::Plus, "+"),
            (Token::ColumnName, "z")
        ];
        let mut pos = 0;
        
        let result = parse_rhs(&tokens, &mut pos);
        assert!(result.is_ok());
        let (terms, has_intercept) = result.unwrap();
        assert_eq!(terms.len(), 2);
        assert!(has_intercept);
    }

    #[test]
    fn test_parse_rhs_without_intercept() {
        let tokens = vec![
            (Token::ColumnName, "x"),
            (Token::Minus, "-"),
            (Token::One, "1")
        ];
        let mut pos = 0;
        
        let result = parse_rhs(&tokens, &mut pos);
        assert!(result.is_ok());
        let (terms, has_intercept) = result.unwrap();
        assert_eq!(terms.len(), 1);
        assert!(!has_intercept);
    }

    #[test]
    fn test_parse_rhs_empty() {
        let tokens: Vec<(Token, &str)> = vec![];
        let mut pos = 0;
        
        let result = parse_rhs(&tokens, &mut pos);
        assert!(result.is_ok());
        let (terms, has_intercept) = result.unwrap();
        assert_eq!(terms.len(), 0);
        assert!(has_intercept);
    }

    #[test]
    fn test_parse_rhs_leading_plus() {
        let tokens = vec![
            (Token::Plus, "+"),
            (Token::ColumnName, "x")
        ];
        let mut pos = 0;
        
        let result = parse_rhs(&tokens, &mut pos);
        assert!(result.is_ok());
        let (terms, has_intercept) = result.unwrap();
        assert_eq!(terms.len(), 1); // Only x, not +x
        assert!(has_intercept);
        assert_eq!(pos, 2); // Position advanced past both + and x
    }

    #[test]
    fn test_parse_rhs_minus_without_one() {
        let tokens = vec![
            (Token::ColumnName, "x"),
            (Token::Minus, "-"),
            (Token::ColumnName, "y")
        ];
        let mut pos = 0;
        
        let result = parse_rhs(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 2); // Position advanced past x and minus
    }

    #[test]
    fn test_parse_rhs_multiple_plus_terms() {
        let tokens = vec![
            (Token::ColumnName, "x"),
            (Token::Plus, "+"),
            (Token::ColumnName, "y"),
            (Token::Plus, "+"),
            (Token::ColumnName, "z")
        ];
        let mut pos = 0;
        
        let result = parse_rhs(&tokens, &mut pos);
        assert!(result.is_ok());
        let (terms, has_intercept) = result.unwrap();
        assert_eq!(terms.len(), 3);
        assert!(has_intercept);
    }

    #[test]
    fn test_parse_rhs_with_function_terms() {
        let tokens = vec![
            (Token::Poly, "poly"),
            (Token::FunctionStart, "("),
            (Token::ColumnName, "x"),
            (Token::Comma, ","),
            (Token::Integer, "2"),
            (Token::FunctionEnd, ")"),
            (Token::Plus, "+"),
            (Token::ColumnName, "z")
        ];
        let mut pos = 0;
        
        let result = parse_rhs(&tokens, &mut pos);
        assert!(result.is_ok());
        let (terms, has_intercept) = result.unwrap();
        assert_eq!(terms.len(), 2);
        assert!(has_intercept);
    }

    #[test]
    fn test_parse_rhs_stops_at_comma() {
        let tokens = vec![
            (Token::ColumnName, "x"),
            (Token::Comma, ","),
            (Token::Family, "family")
        ];
        let mut pos = 0;
        
        let result = parse_rhs(&tokens, &mut pos);
        assert!(result.is_ok());
        let (terms, has_intercept) = result.unwrap();
        assert_eq!(terms.len(), 1);
        assert!(has_intercept);
        assert_eq!(pos, 1); // Position at comma
    }
}
