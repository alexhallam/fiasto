//! # Fiasto: High-Performance Statistical Formula Parser
//!
//! Fiasto is a modern, high-performance parser for statistical formulas written in Rust.
//! It parses R-style formulas (Wilkinson notation) and returns comprehensive metadata
//! about variables, transformations, interactions, and random effects.
//!
//! ## Features
//!
//! - **Comprehensive Formula Support**: Full R/Wilkinson notation including complex random effects
//! - **Variable-Centric Output**: Variables are first-class citizens with detailed metadata
//! - **Advanced Random Effects**: brms-style syntax with correlation control and grouping options
//! - **High Performance**: Zero-copy processing and efficient tokenization
//! - **Rich Metadata**: Detailed information about transformations, interactions, and model structure
//!
//! ## Quick Start
//!
//! ```rust
//! use fiasto::parse_formula;
//!
//! // Parse a simple linear model
//! let result = parse_formula("y ~ x + z");
//! match result {
//!     Ok(metadata) => println!("{}", serde_json::to_string_pretty(&metadata).unwrap()),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! ## Supported Syntax
//!
//! ### Basic Models
//! - Linear models: `y ~ x + z`
//! - Polynomial terms: `y ~ poly(x, 3)`
//! - Interactions: `y ~ x:z` or `y ~ x*z`
//! - Family specification: `y ~ x, family = gaussian`
//!
//! ### Random Effects
//! - Random intercepts: `(1 | group)`
//! - Random slopes: `(0 + x | group)`
//! - Correlated effects: `(x | group)`
//! - Uncorrelated effects: `(x || group)`
//! - Advanced grouping: `(1 | gr(group, cor = FALSE))`
//!
//! ## Output Format
//!
//! The parser returns a variable-centric JSON structure where each variable
//! is described with its roles, transformations, interactions, and random effects.
//! This makes it easy to understand the complete model structure and generate
//! appropriate design matrices.
//!
//! ## Performance
//!
//! Fiasto is designed for high performance with:
//! - Efficient tokenization using the `logos` crate
//! - Zero-copy string processing where possible
//! - Minimal memory allocations
//! - Fast pattern matching for complex syntax
//!
//! ## Use Cases
//!
//! - **Statistical Software**: Integration into statistical computing environments
//! - **Data Analysis Tools**: Parsing user-specified models
//! - **Model Validation**: Understanding complex model structures
//! - **Code Generation**: Creating design matrices from formulas
//! - **Documentation**: Automatically documenting model specifications

pub mod internal {
    pub mod ast;
    pub mod data_structures;
    pub mod errors;
    pub mod expect;
    pub mod lexer;
    pub mod matches;
    pub mod meta_builder;
    pub mod new;
    pub mod next;
    pub mod parse;
    pub mod parse_arg;
    pub mod parse_arg_list;
    pub mod parse_family;
    pub mod parse_formula;
    pub mod parse_random_effect;
    pub mod parse_response;
    pub mod parse_rhs;
    pub mod parse_term;
    pub mod parser;
    pub mod peek;
}

use internal::parse::{MetaBuilder, Parser, Term};
use serde_json::Value;

/// Parse a statistical formula string and return comprehensive metadata as JSON
///
/// This function parses R-style statistical formulas (Wilkinson notation) and returns
/// a variable-centric metadata structure that describes all variables, their roles,
/// transformations, interactions, and random effects in the model.
///
/// # Formula Syntax
///
/// The parser supports comprehensive statistical formula syntax including:
///
/// ## Basic Syntax
/// - **Response**: `y ~ x` (y is the response variable)
/// - **Fixed Effects**: `y ~ x + z + w` (multiple predictors)
/// - **Intercept Control**: `y ~ x - 1` (no intercept) or `y ~ x + 0` (explicit intercept)
/// - **Family Specification**: `y ~ x, family = gaussian` (distribution family)
///
/// ## Transformations
/// - **Polynomial**: `poly(x, 3)` (orthogonal polynomials of degree 3)
/// - **Logarithm**: `log(x)` (natural logarithm)
/// - **Custom Functions**: `scale(x)`, `center(x)`, `factor(x)`, etc.
///
/// ## Interactions
/// - **Simple**: `x:z` (interaction between x and z)
/// - **Full**: `x*z` (equivalent to `x + z + x:z`)
///
/// ## Random Effects (brms-style)
/// - **Random Intercepts**: `(1 | group)` (random intercepts by group)
/// - **Random Slopes**: `(0 + x | group)` (random slopes for x by group)
/// - **Correlated Effects**: `(x | group)` (random intercept + slope, correlated)
/// - **Uncorrelated Effects**: `(x || group)` (random intercept + slope, uncorrelated)
/// - **Cross-Parameter**: `(x |ID| group)` (cross-parameter correlations)
/// - **Enhanced Grouping**: `(1 | gr(group, cor = FALSE))` (advanced grouping options)
/// - **Multi-Membership**: `(1 | mm(group1, group2))` (multiple membership)
/// - **Nested**: `(1 | group1/group2)` (nested grouping)
/// - **Interaction Grouping**: `(1 | group1:group2)` (interaction of grouping factors)
///
/// # Arguments
///
/// * `formula` - A string containing a statistical formula in R/Wilkinson notation
///
/// # Returns
///
/// * `Result<Value, Box<dyn std::error::Error>>` - The formula metadata as JSON, or an error
///
/// # Output Structure
///
/// The returned JSON contains a variable-centric metadata structure:
///
/// ```json
/// {
///   "formula": "y ~ x + poly(x, 2) + (1 | group), family = gaussian",
///   "metadata": {
///     "has_intercept": true,
///     "is_random_effects_model": true,
///     "has_uncorrelated_slopes_and_intercepts": false,
///     "family": "gaussian"
///   },
///   "all_generated_columns": ["y", "x", "x_poly_1", "x_poly_2", "group"],
///   "columns": {
///     "y": {
///       "id": 1,
///       "roles": ["Response"],
///       "generated_columns": ["y"],
///       "transformations": [],
///       "interactions": [],
///       "random_effects": []
///     },
///     "x": {
///       "id": 2,
///       "roles": ["FixedEffect"],
///       "generated_columns": ["x_poly_1", "x_poly_2"],
///       "transformations": [
///         {
///           "function": "poly",
///           "parameters": {"degree": 2, "orthogonal": true},
///           "generates_columns": ["x_poly_1", "x_poly_2"]
///         }
///       ],
///       "interactions": [],
///       "random_effects": []
///     },
///     "group": {
///       "id": 3,
///       "roles": ["GroupingVariable"],
///       "generated_columns": ["group"],
///       "transformations": [],
///       "interactions": [],
///       "random_effects": [
///         {
///           "kind": "grouping",
///           "grouping_variable": "group",
///           "has_intercept": true,
///           "correlated": true,
///           "variables": []
///         }
///       ]
///     }
///   }
/// }
/// ```
///
/// # Examples
///
/// ## Basic Linear Model
/// ```
/// use fiasto::parse_formula;
///
/// let result = parse_formula("y ~ x + z");
/// match result {
///     Ok(metadata) => println!("{}", serde_json::to_string_pretty(&metadata).unwrap()),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
/// ## Model with Transformations
/// ```
/// use fiasto::parse_formula;
///
/// let result = parse_formula("y ~ x + poly(x, 3) + log(z), family = gaussian");
/// match result {
///     Ok(metadata) => println!("{}", serde_json::to_string_pretty(&metadata).unwrap()),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
/// ## Mixed Effects Model
/// ```
/// use fiasto::parse_formula;
///
/// let result = parse_formula("y ~ x + (1 | group) + (x || group)");
/// match result {
///     Ok(metadata) => println!("{}", serde_json::to_string_pretty(&metadata).unwrap()),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
/// ## Complex Random Effects
/// ```
/// use fiasto::parse_formula;
///
/// let result = parse_formula("y ~ x + (x*z | gr(group, cor = FALSE)) + (0 + y | site)");
/// match result {
///     Ok(metadata) => println!("{}", serde_json::to_string_pretty(&metadata).unwrap()),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
/// ## Interactions
/// ```
/// use fiasto::parse_formula;
///
/// let result = parse_formula("y ~ x:z + x*z + (x:z | group)");
/// match result {
///     Ok(metadata) => println!("{}", serde_json::to_string_pretty(&metadata).unwrap()),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
/// # Variable Roles
///
/// Variables can have multiple roles in the model:
///
/// - **Response**: The dependent variable (always gets ID 1)
/// - **FixedEffect**: Predictor variables in the fixed effects part
/// - **GroupingVariable**: Variables used for grouping in random effects
/// - **RandomEffect**: Variables that have random effects
///
/// # Generated Columns
///
/// Transformations create new columns:
/// - `poly(x, 2)` generates `x_poly_1`, `x_poly_2`
/// - `log(x)` generates `x_log`
/// - `x:z` interaction generates `x_z`
///
/// The `all_generated_columns` array contains all generated column names ordered by variable ID.
///
/// # Error Handling
///
/// The function returns detailed error messages for common issues:
/// - Invalid syntax
/// - Unrecognized functions
/// - Malformed random effects
/// - Missing required arguments
///
/// # Performance
///
/// This parser is designed for high performance with:
/// - Zero-copy string processing where possible
/// - Efficient tokenization using the `logos` crate
/// - Minimal memory allocations
/// - Fast pattern matching
pub fn parse_formula(formula: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut p = Parser::new(formula)?;
    let (response, terms, has_intercept, family_opt) = p.parse_formula()?;

    let mut mb = MetaBuilder::new();
    mb.push_response(&response);
    for t in terms {
        match t {
            Term::Column(name) => mb.push_plain_term(&name),
            Term::Function { name, args } => mb.push_function_term(&name, &args),
            Term::Interaction { left, right } => mb.push_interaction(&left, &right),
            Term::RandomEffect(random_effect) => mb.push_random_effect(&random_effect),
        }
    }
    let family_name = family_opt.map(|f| format!("{:?}", f).to_lowercase());
    let meta = mb.build(formula, has_intercept, family_name);

    Ok(serde_json::to_value(meta)?)
}

/// Lex a formula and return JSON describing each token.
///
/// The output is an array of objects with fields:
/// - `token`: token name (enum debug)
/// - `lexeme`: the original slice from the input
///
/// # Example
///
/// ```rust
/// use fiasto::lex_formula;
///
/// let formula = "mpg ~ cyl + wt*hp + poly(disp, 4) - 1";
/// let tokens = lex_formula(formula).unwrap();
/// // tokens is a serde_json::Value::Array of objects like:
/// // { "token": "ColumnName", "lexeme": "mpg" }
/// // { "token": "Tilde", "lexeme": "~" }
/// // { "token": "Plus", "lexeme": "+" }
/// println!("{}", serde_json::to_string_pretty(&tokens).unwrap());
/// ```
pub fn lex_formula(formula: &str) -> Result<Value, Box<dyn std::error::Error>> {
    use logos::Logos;
    use crate::internal::lexer::Token;

    let mut lex = Token::lexer(formula);
    let mut tokens = Vec::new();
    while let Some(item) = lex.next() {
        match item {
            Ok(tok) => {
                let slice = lex.slice();
                let obj = serde_json::json!({
                    "token": format!("{:?}", tok),
                    "lexeme": slice,
                });
                tokens.push(obj);
            }
            Err(()) => {
                return Err(Box::new(crate::internal::errors::ParseError::Lex(lex.slice().to_string())));
            }
        }
    }
    Ok(serde_json::Value::Array(tokens))
}
