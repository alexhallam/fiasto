use crate::internal::{ast::{Family, Term}, errors::ParseError, lexer::Token};

/// Parses a complete formula and returns its components.
/// 
/// This is the main entry point for parsing R-style formulas. It orchestrates
/// the parsing of all formula components: response variable, right-hand side terms,
/// intercept flag, and optional family specification.
/// 
/// # Arguments
/// * `tokens` - Reference to the vector of tokens
/// * `pos` - Mutable reference to the current position (will be advanced)
/// 
/// # Returns
/// * `Result<(String, Vec<Term>, bool, Option<Family>), ParseError>` - A tuple containing:
///   - Response variable name
///   - Vector of terms from the right-hand side
///   - Boolean indicating whether intercept is included
///   - Optional family specification
/// 
/// # Example
/// ```
/// use fiasto::internal::parse_formula::parse_formula;
/// use fiasto::internal::lexer::Token;
/// 
/// let tokens = vec![
///     (Token::ColumnName, "y"),
///     (Token::Tilde, "~"),
///     (Token::ColumnName, "x"),
///     (Token::Plus, "+"),
///     (Token::ColumnName, "z"),
///     (Token::Comma, ","),
///     (Token::Family, "family"),
///     (Token::Equal, "="),
///     (Token::Gaussian, "gaussian")
/// ];
/// let mut pos = 0;
/// 
/// let result = parse_formula(&tokens, &mut pos);
/// assert!(result.is_ok());
/// let (response, terms, has_intercept, family) = result.unwrap();
/// assert_eq!(response, "y");
/// assert_eq!(terms.len(), 2);
/// assert!(has_intercept);
/// assert!(family.is_some());
/// ```
/// 
/// # How it works
/// 1. Parses the response variable using `parse_response`
/// 2. Expects and consumes a tilde (`~`) symbol
/// 3. Parses the right-hand side using `parse_rhs`
/// 4. Optionally parses family specification if comma is present
/// 
/// # Grammar Rule
/// ```text
/// formula = response "~" rhs ["," family_spec]
/// response = column_name
/// rhs = term_list [intercept_spec]
/// family_spec = "family" "=" family_name
/// ```
/// 
/// # Use Cases
/// - Parsing complete regression formulas
/// - Extracting all components of a statistical model specification
/// - Validating formula syntax and structure
/// - Preparing for model building and metadata generation
/// 
/// # Examples of Valid Inputs
/// - `"y ~ x"` → response="y", terms=["x"], intercept=true, family=None
/// - `"y ~ x + z - 1"` → response="y", terms=["x", "z"], intercept=false, family=None
/// - `"y ~ x, family=gaussian"` → response="y", terms=["x"], intercept=true, family=Gaussian
pub fn parse_formula<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<(String, Vec<Term>, bool, Option<Family>), ParseError> {
    let response = crate::internal::parse_response::parse_response(tokens, pos)?;
    crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::Tilde), "~")?;
    let (terms, has_intercept) = crate::internal::parse_rhs::parse_rhs(tokens, pos)?;

    let mut family = None;
    if crate::internal::matches::matches(tokens, pos, |t| matches!(t, Token::Comma)) {
        crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::Family), "family")?;
        crate::internal::expect::expect(tokens, pos, |t| matches!(t, Token::Equal), "=")?;
        family = Some(crate::internal::parse_family::parse_family(tokens, pos)?);
    }

    Ok((response, terms, has_intercept, family))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::lexer::Token;

    #[test]
    fn test_parse_formula_simple() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x")
        ];
        let mut pos = 0;
        
        let result = parse_formula(&tokens, &mut pos);
        assert!(result.is_ok());
        let (response, terms, has_intercept, family) = result.unwrap();
        assert_eq!(response, "y");
        assert_eq!(terms.len(), 1);
        assert!(has_intercept);
        assert!(family.is_none());
    }

    #[test]
    fn test_parse_formula_with_multiple_terms() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x"),
            (Token::Plus, "+"),
            (Token::ColumnName, "z")
        ];
        let mut pos = 0;
        
        let result = parse_formula(&tokens, &mut pos);
        assert!(result.is_ok());
        let (response, terms, has_intercept, family) = result.unwrap();
        assert_eq!(response, "y");
        assert_eq!(terms.len(), 2);
        assert!(has_intercept);
        assert!(family.is_none());
    }

    #[test]
    fn test_parse_formula_without_intercept() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x"),
            (Token::Minus, "-"),
            (Token::One, "1")
        ];
        let mut pos = 0;
        
        let result = parse_formula(&tokens, &mut pos);
        assert!(result.is_ok());
        let (response, terms, has_intercept, family) = result.unwrap();
        assert_eq!(response, "y");
        assert_eq!(terms.len(), 1);
        assert!(!has_intercept);
        assert!(family.is_none());
    }

    #[test]
    fn test_parse_formula_with_family() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x"),
            (Token::Comma, ","),
            (Token::Family, "family"),
            (Token::Equal, "="),
            (Token::Gaussian, "gaussian")
        ];
        let mut pos = 0;
        
        let result = parse_formula(&tokens, &mut pos);
        assert!(result.is_ok());
        let (response, terms, has_intercept, family) = result.unwrap();
        assert_eq!(response, "y");
        assert_eq!(terms.len(), 1);
        assert!(has_intercept);
        assert!(family.is_some());
        assert_eq!(family.unwrap(), Family::Gaussian);
    }

    #[test]
    fn test_parse_formula_failure_missing_tilde() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::ColumnName, "x")
        ];
        let mut pos = 0;
        
        let result = parse_formula(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 1); // Position advanced past response
    }

    #[test]
    fn test_parse_formula_failure_missing_family_after_comma() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::ColumnName, "x"),
            (Token::Comma, ",")
        ];
        let mut pos = 0;
        
        let result = parse_formula(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 4); // Position advanced to comma
    }

    #[test]
    fn test_parse_formula_with_function_terms() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~"),
            (Token::Poly, "poly"),
            (Token::FunctionStart, "("),
            (Token::ColumnName, "x"),
            (Token::Comma, ","),
            (Token::Integer, "2"),
            (Token::FunctionEnd, ")")
        ];
        let mut pos = 0;
        
        let result = parse_formula(&tokens, &mut pos);
        assert!(result.is_ok());
        let (response, terms, has_intercept, family) = result.unwrap();
        assert_eq!(response, "y");
        assert_eq!(terms.len(), 1);
        assert!(has_intercept);
        assert!(family.is_none());
    }

    #[test]
    fn test_parse_formula_empty_rhs() {
        let tokens = vec![
            (Token::ColumnName, "y"),
            (Token::Tilde, "~")
        ];
        let mut pos = 0;
        
        let result = parse_formula(&tokens, &mut pos);
        assert!(result.is_ok());
        let (response, terms, has_intercept, family) = result.unwrap();
        assert_eq!(response, "y");
        assert_eq!(terms.len(), 0);
        assert!(has_intercept);
        assert!(family.is_none());
    }
}
