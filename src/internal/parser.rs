//! # Parser for Statistical Formulas
//!
//! This module provides the main parser for statistical formulas. The parser takes
//! a formula string, tokenizes it, and converts it into an Abstract Syntax Tree (AST)
//! that can be processed to generate metadata.
//!
//! ## Overview
//!
//! The parser is designed to handle the complete range of R-style statistical formula
//! syntax, including:
//!
//! - Basic linear models: `y ~ x + z`
//! - Transformations: `y ~ poly(x, 3) + log(z)`
//! - Interactions: `y ~ x:z + x*z`
//! - Random effects: `y ~ x + (1 | group) + (x || group)`
//! - Complex grouping: `y ~ x + (x | gr(group, cor=FALSE))`
//! - Family specification: `y ~ x, family = gaussian`
//!
//! ## Architecture
//!
//! The parser uses a two-phase approach:
//! 1. **Lexical Analysis**: Converts the input string into tokens
//! 2. **Syntactic Analysis**: Converts tokens into an AST
//!
//! ## Example Usage
//!
//! ```rust
//! use fiasto::internal::parser::Parser;
//!
//! let formula = "y ~ x + poly(x, 2) + (1 | group), family = gaussian";
//! let mut parser = Parser::new(formula).unwrap();
//! let (response, terms, has_intercept, family) = parser.parse_formula().unwrap();
//! 
//! // response = "y"
//! // terms = [Term::Column("x"), Term::Function{...}, Term::RandomEffect{...}]
//! // has_intercept = true
//! // family = Some(Family::Gaussian)
//! ```
//!
//! ## Error Handling
//!
//! The parser provides detailed error messages for common issues:
//! - Invalid syntax
//! - Unrecognized functions
//! - Malformed random effects
//! - Missing required arguments

use crate::internal::{
    ast::{Family, Term},
    errors::ParseError,
    lexer::Token,
};

use owo_colors::OwoColorize;

/// Parser for statistical formulas
///
/// The parser converts formula strings into Abstract Syntax Trees (ASTs).
/// It uses a lifetime parameter `'a` to borrow the input string, ensuring
/// that the parser doesn't outlive the input data.
///
/// # Fields
///
/// * `input` - Reference to the original formula string
/// * `tokens` - Vector of tokens with their string slices
/// * `pos` - Current position in the token stream
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::parser::Parser;
///
/// let formula = "y ~ x + z";
/// let parser = Parser::new(formula).unwrap();
/// // parser.input = "y ~ x + z"
/// // parser.tokens = [(ColumnName, "y"), (Tilde, "~"), ...]
/// // parser.pos = 0
/// ```
pub struct Parser<'a> {
    /// Reference to the original input string
    pub input: &'a str,
    
    /// Vector of tokens with their string slices from the input
    pub tokens: Vec<(Token, &'a str)>,
    
    /// Current position in the token stream
    pub pos: usize,
}

/// Implementation of the parser functionality
impl<'a> Parser<'a> {
    /// Creates a new parser instance from a formula string
    ///
    /// This method tokenizes the input string and creates a parser ready to
    /// parse the formula into an AST.
    ///
    /// # Arguments
    ///
    /// * `input` - The formula string to parse
    ///
    /// # Returns
    ///
    /// * `Ok(Parser)` - Successfully created parser
    /// * `Err(ParseError)` - Lexical analysis failed
    ///
    /// # Examples
///
/// ```rust
/// use fiasto::internal::parser::Parser;
///
/// let formula = "y ~ x + z";
/// let parser = Parser::new(formula).unwrap();
/// ```
    pub fn new(input: &'a str) -> Result<Self, ParseError> {
        crate::internal::new::new(input)
    }

    /// Pretty-print a parse error with context (tokens, last-consumed lexeme, expected/found)
    ///
    /// This produces a colored, human-friendly message useful for CLI output.
    pub fn pretty_error(&self, err: &ParseError) -> String {
        match err {
            ParseError::Lex(s) => {
                format!("{}\n\n{}\n", "Lexing error".red().bold(), s)
            }
            ParseError::Eoi => {
                format!("{}\n\n{}\n", "Unexpected end of input".red().bold(), "the formula ended earlier than expected")
            }
            ParseError::Unexpected { expected, found: _ } => {
                let mut out = String::new();
                
                // Header
                out.push_str(&format!("{}\n", "Syntax error- Unexpected Token".red().bold()));
                
                // Formula: just print the original formula uncolored
                out.push_str(&format!("Formula: {}\n", self.input));
                
                // Show: previous successful lexemes in green then failed lexeme in red
                out.push_str("Show: ");
                for i in 0..self.pos {
                    if let Some((_, lex)) = self.tokens.get(i) {
                        out.push_str(&format!("{} ", lex.green()));
                    }
                }
                let failed = self.tokens.get(self.pos).map(|(_, l)| *l).unwrap_or("<eoi>");
                out.push_str(&format!("{}\n", failed.red()));
                
                // Expected Token: list expected tokens
                out.push_str(&format!("Expected Token: {}\n", expected.to_string()));
                
                out
            }
            ParseError::Syntax(s) => {
                format!("{}\n\n{}\n", "Syntax error".red().bold(), s)
            }
        }
    }

    /// Parses the formula and returns the complete AST information
    ///
    /// This method performs the syntactic analysis of the tokenized formula
    /// and returns the structured representation needed for statistical modeling.
    ///
    /// # Returns
    /// 
    /// A tuple containing:
    /// * `String` - The response variable (left side of ~)
    /// * `Vec<Term>` - All terms in the formula (fixed effects, random effects, etc.)
    /// * `bool` - Whether the model includes an intercept
    /// * `Option<Family>` - The distribution family (if specified)
    ///
    /// # Examples
    ///
    /// ```rust
/// use fiasto::internal::parser::Parser;
///
/// let formula = "y ~ x + (1 | group), family = gaussian";
/// let mut parser = Parser::new(formula).unwrap();
/// let (response, terms, has_intercept, family) = parser.parse_formula().unwrap();
/// 
/// assert_eq!(response, "y");
/// assert!(has_intercept);
/// assert!(family.is_some());
/// ```
    pub fn parse_formula(
        &mut self,
    ) -> Result<(String, Vec<Term>, bool, Option<Family>), ParseError> {
        match crate::internal::parse_formula::parse_formula(&self.tokens, &mut self.pos) {
            Ok(v) => Ok(v),
            Err(e) => {
                // Return the original error unchanged so pretty_error can handle it properly
                Err(e)
            }
        }
    }
}
