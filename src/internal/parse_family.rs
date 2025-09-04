use crate::internal::{ast::Family, errors::ParseError, lexer::Token};

/// Parses a family specification for statistical models.
/// 
/// This function handles the family parameter that specifies the distribution
/// family for generalized linear models. It supports the three standard families:
/// Gaussian (normal), Binomial, and Poisson.
/// 
/// # Arguments
/// * `tokens` - Reference to the vector of tokens
/// * `pos` - Mutable reference to the current position (will be advanced)
/// 
/// # Returns
/// * `Result<Family, ParseError>` - The parsed family, or an error
/// 
/// # Example
/// ```
/// use fiasto::internal::parse_family::parse_family;
/// use fiasto::internal::lexer::Token;
/// use fiasto::internal::ast::Family;
/// 
/// // Parse Gaussian family
/// let tokens = vec![
///     (Token::Gaussian, "gaussian")
/// ];
/// let mut pos = 0;
/// 
/// let result = parse_family(&tokens, &mut pos);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), Family::Gaussian);
/// assert_eq!(pos, 1);
/// ```
/// 
/// # How it works
/// 1. Expects one of the valid family tokens: Gaussian, Binomial, or Poisson
/// 2. Maps the token to the corresponding Family enum variant
/// 3. Advances position and returns the parsed family
/// 4. Returns error for invalid family specifications
/// 
/// # Grammar Rule
/// ```text
/// family = "gaussian" | "binomial" | "poisson"
/// ```
/// 
/// # Use Cases
/// - Specifying distribution families for GLMs
/// - Supporting different model types (linear, logistic, count)
/// - Validating family specifications in formulas
/// - Building complete model specifications
/// 
/// # Examples of Valid Inputs
/// - `"gaussian"` → Family::Gaussian
/// - `"binomial"` → Family::Binomial
/// - `"poisson"` → Family::Poisson
/// 
/// # Statistical Context
/// - **Gaussian**: Normal distribution, used for continuous response variables
/// - **Binomial**: Used for binary/categorical response variables
/// - **Poisson**: Used for count response variables
pub fn parse_family<'a>(
    tokens: &'a [(Token, &'a str)],
    pos: &mut usize,
) -> Result<Family, ParseError> {
    let (tok, _) = crate::internal::expect::expect(
        tokens,
        pos,
        |t| matches!(t, Token::Gaussian | Token::Binomial | Token::Poisson),
        "gaussian | binomial | poisson",
    )?;
    let fam = match tok {
        Token::Gaussian => Family::Gaussian,
        Token::Binomial => Family::Binomial,
        Token::Poisson => Family::Poisson,
        _ => unreachable!(),
    };
    Ok(fam)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal::lexer::Token;

    #[test]
    fn test_parse_family_gaussian() {
        let tokens = vec![
            (Token::Gaussian, "gaussian")
        ];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Family::Gaussian);
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_family_binomial() {
        let tokens = vec![
            (Token::Binomial, "binomial")
        ];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Family::Binomial);
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_family_poisson() {
        let tokens = vec![
            (Token::Poisson, "poisson")
        ];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Family::Poisson);
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_parse_family_invalid_token() {
        let tokens = vec![
            (Token::ColumnName, "x")
        ];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_family_end_of_input() {
        let tokens: Vec<(Token, &str)> = vec![];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_family_with_plus_token() {
        let tokens = vec![
            (Token::Plus, "+")
        ];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_family_with_tilde_token() {
        let tokens = vec![
            (Token::Tilde, "~")
        ];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_family_with_comma_token() {
        let tokens = vec![
            (Token::Comma, ",")
        ];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_family_with_equal_token() {
        let tokens = vec![
            (Token::Equal, "=")
        ];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_family_with_function_tokens() {
        let tokens = vec![
            (Token::Poly, "poly")
        ];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_family_with_numeric_tokens() {
        let tokens = vec![
            (Token::Integer, "42")
        ];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_family_with_one_token() {
        let tokens = vec![
            (Token::One, "1")
        ];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_err());
        assert_eq!(pos, 0); // Position unchanged
    }

    #[test]
    fn test_parse_family_advances_position_on_success() {
        let tokens = vec![
            (Token::Gaussian, "gaussian"),
            (Token::Comma, ",")
        ];
        let mut pos = 0;
        
        let result = parse_family(&tokens, &mut pos);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Family::Gaussian);
        assert_eq!(pos, 1); // Position advanced past family
    }

    #[test]
    fn test_parse_family_all_variants() {
        let families = vec![
            (Token::Gaussian, Family::Gaussian),
            (Token::Binomial, Family::Binomial),
            (Token::Poisson, Family::Poisson),
        ];
        
        for (token, expected_family) in families {
            let tokens = vec![(token, "dummy")];
            let mut pos = 0;
            
            let result = parse_family(&tokens, &mut pos);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), expected_family);
            assert_eq!(pos, 1);
        }
    }
}
