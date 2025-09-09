//! # Fiasto: High-Performance Statistical Formula Parser
//! Pronouned like **fiasco**, but with a **t** instead of an **c**
//!
//! ## (F)ormulas (I)n (AST) (O)ut
//!
//! A Language-Agnostic modern Wilkinson's formula parser and lexer.
//!
//! ## Motivation
//!
//! Formula parsing and materialization is normally done in a single
//! library. Python, for example, has `patsy`/`formulaic`/`formulae` which all do parsing & materialization.
//! R's `model.matrix` also handles formula parsing and design matrix creation.
//!
//! There is nothing wrong with this coupling. I wanted to try decoupling the parsing and materialization.
//! I thought this would allow a focused library that could be used in multiple languages or dataframe libraries.
//! This package has a clear path, to parse and/or lex formulas and return structured JSON metadata.
//!
//! Note: Technically an AST is not returned. A simplified/structured intermediate
//! representation (IR) in the form of json is returned. This json IR ought to be easy for many language bindings to use.
//!
//! ## 🎯 Simple API
//!
//! The library exposes a clean, focused API:
//!
//! - `parse_formula()` - Takes a Wilkinson's formula string and returns structured JSON metadata
//! - `lex_formula()` - Tokenizes a formula string and returns JSON describing each token
//!
//! "Only two functions?! What kind of library is this?!"
//!
//! An easy to maintain library with a small surface area. The best kind.
//!
//! ## Output Format
//!
//! The parser returns a variable-centric JSON structure where each variable
//! is described with its roles, transformations, interactions, and random effects.
//! This makes it easy to understand the complete model structure and generate
//! appropriate design matrices. [wayne](https://github.com/alexhallam/wayne) is a python package
//! that can take this JSON and generates design matrices for use in statistical modeling.
//!
//! ## Features
//!
//! - **Comprehensive Formula Support**: Full R/Wilkinson notation including complex random effects
//! - **Variable-Centric Output**: Variables are first-class citizens with detailed metadata
//! - **Advanced Random Effects**: brms-style syntax with correlation control and grouping options
//! - **High Performance**: Zero-copy processing and efficient tokenization
//! - **Pretty Error Messages**: Colored, contextual error reporting with syntax highlighting
//! - **Robust Error Recovery**: Graceful handling of malformed formulas with specific error types
//! - **Language Agnostic Output**: JSON format for easy integration with various programming languages
//! - **Comprehensive Documentation**: Detailed usage examples and grammar rules
//! - **Comprehensive Metadata**: Variable roles, transformations, interactions, and relationships
//! - **Automatic Naming For Generated Columns**: Consistent, descriptive names for transformed and interaction terms
//! - **Dual API**: Both parsing and lexing functions for flexibility
//! - **Efficient tokenization**: using one of the fastest lexer generators for Rust ([logos](https://docs.rs/logos/0.15.1/logos/index.html) crate)
//! - **Fast pattern matching**: using match statements and enum-based token handling. Rust match statements are zero-cost abstractions.
//! - **Minimal string copying**: with extensive use of string slices (`&str`) where possible
//!
//! ## Use Cases:
//!
//! - **Formula Validation**: Check if formulas are valid against datasets before expensive computation
//! - **Cross-Platform Model Specs**: Define models once, implement in multiple statistical frameworks
//!
//! ## Quick Start `parse_formula`
//!
//! To parse a formula and get JSON metadata:
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
//! ### Intercept-Only, No-Intercept, and Multivariate Models
//!
//! All model types are fully supported:
//! ```rust
//! use fiasto::parse_formula;
//!
//! // Parse an intercept-only model
//! let result = parse_formula("y ~ 1");
//! match result {
//!     Ok(metadata) => {
//!         // The metadata will include an "intercept" column
//!         // and has_intercept will be true
//!         println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
//!     }
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//!
//! // Parse a no-intercept model
//! let result = parse_formula("y ~ 0");
//! match result {
//!     Ok(metadata) => {
//!         // The metadata will NOT include an "intercept" column
//!         // and has_intercept will be false
//!         println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
//!     }
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//!
//! // Parse a multivariate model
//! let result = parse_formula("bind(y1, y2) ~ x + z");
//! match result {
//!     Ok(metadata) => {
//!         // The metadata will include both y1 and y2 as response variables
//!         // with ID 1, and x, z as predictors with IDs 2, 3
//!         println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
//!     }
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//! This prints a JSON object like:
//!
//! ```json
//! {
//!     "all_generated_columns": [
//!     "y",
//!     "x",
//!     "z"
//!   ],
//!   "columns": {
//!     "x": {
//!       "generated_columns": [
//!         "x"
//!       ],
//!       "id": 2,
//!       "interactions": [],
//!       "random_effects": [],
//!       "roles": [
//!         "FixedEffect"
//!       ],
//!       "transformations": []
//!     },
//!     "y": {
//!       "generated_columns": [
//!         "y"
//!       ],
//!       "id": 1,
//!       "interactions": [],
//!       "random_effects": [],
//!       "roles": [
//!         "Response"
//!       ],
//!       "transformations": []
//!     },
//!     "z": {
//!       "generated_columns": [
//!         "z"
//!       ],
//!       "id": 3,
//!       "interactions": [],
//!       "random_effects": [],
//!       "roles": [
//!         "FixedEffect"
//!       ],
//!       "transformations": []
//!     }
//!   },
//!   "formula": "y ~ x + z",
//!   "metadata": {
//!     "family": null,
//!     "has_intercept": true,
//!     "has_uncorrelated_slopes_and_intercepts": false,
//!     "is_random_effects_model": false
//!   }
//! }
//! ```
//! ## Quick Start `lex_formula`
//!
//! To lex a formula and get token information:
//! ```rust
//! use fiasto::lex_formula;
//!
//! // Lex a simple linear model
//! let result = lex_formula("y ~ x + z");
//! match result {
//!     Ok(tokens) => println!("{}", serde_json::to_string_pretty(&tokens).unwrap()),
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//! This prints objects like:
//!
//! ```json
//! { "token": "ColumnName", "lexeme": "mpg" }
//! { "token": "Tilde", "lexeme": "~" }
//! { "token": "Plus", "lexeme": "+" }
//! ```
//!
//! ## Run Examples
//! You can run the examples in the `examples/` directory with the command: `cargo run --example <example_name>`
//! For example:
//! - `cargo run --example intercept_only` - Demonstrates intercept-only model parsing
//! - `cargo run --example 03` - Demonstrates parsing a complex formula shown below
//! ```rust
//! use fiasto::parse_formula;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let input = "y ~ x + poly(x, 2) + poly(x1, 4) + log(x1) - 1, family = gaussian";
//!
//!     println!("Testing public parse_formula function:");
//!     println!("Input: {}", input);
//!
//!     let result = parse_formula(input)?;
//!
//!     println!("FORMULA METADATA (as JSON):");
//!     println!("{}", result);
//!     println!("{}", serde_json::to_string_pretty(&result)?);
//!
//!     println!("\n\n");
//!
//!     Ok(())
//! }
//! ```
//! ## Supported Syntax
//!
//! ### Basic Models
//! - Linear models: `y ~ x + z`
//! - Intercept-only models: `y ~ 1`
//! - No-intercept models: `y ~ 0`
//! - Multivariate models: `bind(y1, y2) ~ x + z`
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
    let (response, terms, mut has_intercept, family_opt) = match p.parse_formula() {
        Ok(v) => v,
        Err(e) => {
            // Print pretty, colored error by default for CLI users
            eprintln!("{}", p.pretty_error(&e));
            return Err(Box::new(e));
        }
    };

    let mut mb = MetaBuilder::new();
    mb.push_response(&response);

    // Check if we have a zero term, which means no intercept
    let has_zero_term = terms.iter().any(|t| matches!(t, Term::Zero));
    if has_zero_term {
        has_intercept = false;
    }

    for t in terms {
        match t {
            Term::Column(name) => mb.push_plain_term(&name),
            Term::Function { name, args } => mb.push_function_term(&name, &args),
            Term::Interaction { left, right } => mb.push_interaction(&left, &right),
            Term::RandomEffect(random_effect) => mb.push_random_effect(&random_effect),
            Term::Intercept => {
                // Intercept terms are handled by the has_intercept flag in the build method
                // No additional processing needed here
            }
            Term::Zero => {
                // Zero terms indicate no intercept - this is handled by the has_intercept flag
                // No additional processing needed here
            }
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
    use crate::internal::lexer::Token;
    use logos::Logos;

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
                return Err(Box::new(crate::internal::errors::ParseError::Lex(
                    lex.slice().to_string(),
                )));
            }
        }
    }
    Ok(serde_json::Value::Array(tokens))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intercept_and_formula_order_with_intercept() {
        // Test the exact example from issue #6: y ~ x + poly(x, 2) + log(z)
        let formula = "y ~ x + poly(x, 2) + log(z)";
        let result = parse_formula(formula).expect("Should parse successfully");

        // Check that intercept is present in all_generated_columns
        let all_columns = result
            .get("all_generated_columns")
            .expect("Should have all_generated_columns")
            .as_array()
            .expect("Should be an array");

        assert!(
            all_columns
                .iter()
                .any(|col| col.as_str() == Some("intercept")),
            "Intercept should be present in all_generated_columns"
        );

        // Check the specific order: y, intercept, x, x_poly_1, x_poly_2, z_log
        let expected_columns = vec!["y", "intercept", "x", "x_poly_1", "x_poly_2", "z_log"];
        let actual_columns: Vec<&str> = all_columns
            .iter()
            .map(|col| col.as_str().unwrap())
            .collect();

        assert_eq!(
            actual_columns, expected_columns,
            "all_generated_columns should have the correct order"
        );

        // Check the formula order mapping
        let formula_order = result
            .get("all_generated_columns_formula_order")
            .expect("Should have all_generated_columns_formula_order")
            .as_object()
            .expect("Should be an object");

        assert_eq!(formula_order.get("1").unwrap().as_str(), Some("y"));
        assert_eq!(formula_order.get("2").unwrap().as_str(), Some("intercept"));
        assert_eq!(formula_order.get("3").unwrap().as_str(), Some("x"));
        assert_eq!(formula_order.get("4").unwrap().as_str(), Some("x_poly_1"));
        assert_eq!(formula_order.get("5").unwrap().as_str(), Some("x_poly_2"));
        assert_eq!(formula_order.get("6").unwrap().as_str(), Some("z_log"));

        // Check that has_intercept is true
        let metadata = result.get("metadata").expect("Should have metadata");
        assert_eq!(metadata.get("has_intercept").unwrap().as_bool(), Some(true));
    }

    #[test]
    fn test_intercept_and_formula_order_without_intercept() {
        // Test without intercept: y ~ x + poly(x, 2) + log(z) - 1
        let formula = "y ~ x + poly(x, 2) + log(z) - 1";
        let result = parse_formula(formula).expect("Should parse successfully");

        // Check that intercept is NOT present in all_generated_columns
        let all_columns = result
            .get("all_generated_columns")
            .expect("Should have all_generated_columns")
            .as_array()
            .expect("Should be an array");

        assert!(
            !all_columns
                .iter()
                .any(|col| col.as_str() == Some("intercept")),
            "Intercept should NOT be present when has_intercept is false"
        );

        // Check the specific order: y, x, x_poly_1, x_poly_2, z_log
        let expected_columns = vec!["y", "x", "x_poly_1", "x_poly_2", "z_log"];
        let actual_columns: Vec<&str> = all_columns
            .iter()
            .map(|col| col.as_str().unwrap())
            .collect();

        assert_eq!(
            actual_columns, expected_columns,
            "all_generated_columns should have the correct order without intercept"
        );

        // Check the formula order mapping (should not have intercept)
        let formula_order = result
            .get("all_generated_columns_formula_order")
            .expect("Should have all_generated_columns_formula_order")
            .as_object()
            .expect("Should be an object");

        assert_eq!(formula_order.get("1").unwrap().as_str(), Some("y"));
        assert_eq!(formula_order.get("2").unwrap().as_str(), Some("x"));
        assert_eq!(formula_order.get("3").unwrap().as_str(), Some("x_poly_1"));
        assert_eq!(formula_order.get("4").unwrap().as_str(), Some("x_poly_2"));
        assert_eq!(formula_order.get("5").unwrap().as_str(), Some("z_log"));

        // Check that has_intercept is false
        let metadata = result.get("metadata").expect("Should have metadata");
        assert_eq!(
            metadata.get("has_intercept").unwrap().as_bool(),
            Some(false)
        );
    }

    #[test]
    fn test_simple_formula_with_intercept() {
        // Test simple formula: y ~ x
        let formula = "y ~ x";
        let result = parse_formula(formula).expect("Should parse successfully");

        // Check that intercept is present
        let all_columns = result
            .get("all_generated_columns")
            .expect("Should have all_generated_columns")
            .as_array()
            .expect("Should be an array");

        assert!(
            all_columns
                .iter()
                .any(|col| col.as_str() == Some("intercept")),
            "Intercept should be present in simple formula"
        );

        // Check the order: y, intercept, x
        let expected_columns = vec!["y", "intercept", "x"];
        let actual_columns: Vec<&str> = all_columns
            .iter()
            .map(|col| col.as_str().unwrap())
            .collect();

        assert_eq!(actual_columns, expected_columns);

        // Check formula order mapping
        let formula_order = result
            .get("all_generated_columns_formula_order")
            .expect("Should have all_generated_columns_formula_order")
            .as_object()
            .expect("Should be an object");

        assert_eq!(formula_order.get("1").unwrap().as_str(), Some("y"));
        assert_eq!(formula_order.get("2").unwrap().as_str(), Some("intercept"));
        assert_eq!(formula_order.get("3").unwrap().as_str(), Some("x"));
    }

    #[test]
    fn test_complex_formula_with_intercept() {
        // Test complex formula with multiple variables and transformations
        let formula = "y ~ x1 + x2*x3 + poly(x1, 2) + log(z)";
        let result = parse_formula(formula).expect("Should parse successfully");

        // Check that intercept is present
        let all_columns = result
            .get("all_generated_columns")
            .expect("Should have all_generated_columns")
            .as_array()
            .expect("Should be an array");

        assert!(
            all_columns
                .iter()
                .any(|col| col.as_str() == Some("intercept")),
            "Intercept should be present in complex formula"
        );

        // Check that intercept is at index 1 (after response)
        assert_eq!(all_columns[1].as_str(), Some("intercept"));

        // Check formula order mapping starts correctly
        let formula_order = result
            .get("all_generated_columns_formula_order")
            .expect("Should have all_generated_columns_formula_order")
            .as_object()
            .expect("Should be an object");

        assert_eq!(formula_order.get("1").unwrap().as_str(), Some("y"));
        assert_eq!(formula_order.get("2").unwrap().as_str(), Some("intercept"));

        // Check that has_intercept is true
        let metadata = result.get("metadata").expect("Should have metadata");
        assert_eq!(metadata.get("has_intercept").unwrap().as_bool(), Some(true));
    }

    #[test]
    fn test_intercept_only_model() {
        // Test the basic intercept-only model: y ~ 1
        let formula = "y ~ 1";
        let result = parse_formula(formula).expect("Should parse successfully");

        // Check that intercept is present
        let all_columns = result
            .get("all_generated_columns")
            .expect("Should have all_generated_columns")
            .as_array()
            .expect("Should be an array");

        assert!(
            all_columns
                .iter()
                .any(|col| col.as_str() == Some("intercept")),
            "Intercept should be present in intercept-only model"
        );

        // Check the order: y, intercept
        let expected_columns = vec!["y", "intercept"];
        let actual_columns: Vec<&str> = all_columns
            .iter()
            .map(|col| col.as_str().unwrap())
            .collect();

        assert_eq!(actual_columns, expected_columns);

        // Check formula order mapping
        let formula_order = result
            .get("all_generated_columns_formula_order")
            .expect("Should have all_generated_columns_formula_order")
            .as_object()
            .expect("Should be an object");

        assert_eq!(formula_order.get("1").unwrap().as_str(), Some("y"));
        assert_eq!(formula_order.get("2").unwrap().as_str(), Some("intercept"));

        // Check that has_intercept is true
        let metadata = result.get("metadata").expect("Should have metadata");
        assert_eq!(metadata.get("has_intercept").unwrap().as_bool(), Some(true));

        // Check that only response variable is in columns (no other variables)
        let columns = result
            .get("columns")
            .expect("Should have columns")
            .as_object()
            .expect("Should be an object");
        assert_eq!(
            columns.len(),
            1,
            "Should only have response variable in columns"
        );
        assert!(
            columns.contains_key("y"),
            "Should have response variable 'y'"
        );
    }

    #[test]
    fn test_intercept_only_model_with_family() {
        // Test intercept-only model with family specification: y ~ 1, family = gaussian
        let formula = "y ~ 1, family = gaussian";
        let result = parse_formula(formula).expect("Should parse successfully");

        // Check that intercept is present
        let all_columns = result
            .get("all_generated_columns")
            .expect("Should have all_generated_columns")
            .as_array()
            .expect("Should be an array");

        assert!(
            all_columns
                .iter()
                .any(|col| col.as_str() == Some("intercept")),
            "Intercept should be present in intercept-only model with family"
        );

        // Check family is set correctly
        let metadata = result.get("metadata").expect("Should have metadata");
        assert_eq!(metadata.get("family").unwrap().as_str(), Some("gaussian"));

        // Check that has_intercept is true
        assert_eq!(metadata.get("has_intercept").unwrap().as_bool(), Some(true));
    }

    #[test]
    fn test_no_intercept_model() {
        // Test no-intercept model: y ~ 0
        let formula = "y ~ 0";
        let result = parse_formula(formula).expect("Should parse successfully");

        // Check that intercept is NOT present
        let all_columns = result
            .get("all_generated_columns")
            .expect("Should have all_generated_columns")
            .as_array()
            .expect("Should be an array");

        assert!(
            !all_columns
                .iter()
                .any(|col| col.as_str() == Some("intercept")),
            "Intercept should NOT be present in y ~ 0 model"
        );

        // Check the order: just y
        let expected_columns = vec!["y"];
        let actual_columns: Vec<&str> = all_columns
            .iter()
            .map(|col| col.as_str().unwrap())
            .collect();

        assert_eq!(actual_columns, expected_columns);

        // Check formula order mapping (should not have intercept)
        let formula_order = result
            .get("all_generated_columns_formula_order")
            .expect("Should have all_generated_columns_formula_order")
            .as_object()
            .expect("Should be an object");

        assert_eq!(formula_order.get("1").unwrap().as_str(), Some("y"));
        assert_eq!(
            formula_order.len(),
            1,
            "Should only have response variable in formula order"
        );

        // Check that has_intercept is false
        let metadata = result.get("metadata").expect("Should have metadata");
        assert_eq!(
            metadata.get("has_intercept").unwrap().as_bool(),
            Some(false)
        );
    }

    #[test]
    fn test_invalid_intercept_syntax() {
        // Test that y ~ 1 - 1 fails (contradictory syntax)
        let formula = "y ~ 1 - 1";
        let result = parse_formula(formula);

        assert!(
            result.is_err(),
            "y ~ 1 - 1 should fail because it's contradictory syntax"
        );

        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(
                error_msg.contains("cannot have both intercept term and intercept removal"),
                "Error should mention contradictory syntax"
            );
        }
    }

    #[test]
    fn test_invalid_zero_combination() {
        // Test that y ~ 0 + 1 fails (0 cannot be combined with other terms)
        let formula = "y ~ 0 + 1";
        let result = parse_formula(formula);

        assert!(
            result.is_err(),
            "y ~ 0 + 1 should fail because 0 cannot be combined with other terms"
        );

        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(
                error_msg.contains("zero term (0) cannot be combined with other terms"),
                "Error should mention zero term combination restriction"
            );
        }
    }

    #[test]
    fn test_multivariate_response_basic() {
        // Test basic multivariate response: bind(y1, y2) ~ x
        let formula = "bind(y1, y2) ~ x";
        let result = parse_formula(formula).expect("Should parse successfully");

        // Check that both response variables are present
        let columns = result
            .get("columns")
            .expect("Should have columns")
            .as_object()
            .expect("Should be an object");

        assert!(columns.contains_key("y1"), "Should contain y1 response variable");
        assert!(columns.contains_key("y2"), "Should contain y2 response variable");

        // Check that both have Response role
        let y1_info = columns.get("y1").expect("Should have y1");
        let y1_roles = y1_info.get("roles").expect("Should have roles").as_array().expect("Should be array");
        assert!(y1_roles.iter().any(|r| r.as_str() == Some("Response")), "y1 should have Response role");

        let y2_info = columns.get("y2").expect("Should have y2");
        let y2_roles = y2_info.get("roles").expect("Should have roles").as_array().expect("Should be array");
        assert!(y2_roles.iter().any(|r| r.as_str() == Some("Response")), "y2 should have Response role");

        // Check that both have ID 1 (response variables)
        assert_eq!(y1_info.get("id").expect("Should have id").as_u64(), Some(1));
        assert_eq!(y2_info.get("id").expect("Should have id").as_u64(), Some(1));

        // Check generated columns include both response variables
        let all_columns = result
            .get("all_generated_columns")
            .expect("Should have all_generated_columns")
            .as_array()
            .expect("Should be an array");

        let column_names: Vec<&str> = all_columns
            .iter()
            .map(|col| col.as_str().unwrap())
            .collect();

        assert!(column_names.contains(&"y1"), "Should contain y1 in generated columns");
        assert!(column_names.contains(&"y2"), "Should contain y2 in generated columns");
        assert!(column_names.contains(&"x"), "Should contain x in generated columns");
        assert!(column_names.contains(&"intercept"), "Should contain intercept in generated columns");
    }

    #[test]
    fn test_multivariate_response_three_variables() {
        // Test multivariate response with 3 variables: bind(y1, y2, y3) ~ x + z
        let formula = "bind(y1, y2, y3) ~ x + z";
        let result = parse_formula(formula).expect("Should parse successfully");

        // Check that all three response variables are present
        let columns = result
            .get("columns")
            .expect("Should have columns")
            .as_object()
            .expect("Should be an object");

        for var_name in &["y1", "y2", "y3"] {
            assert!(columns.contains_key(*var_name), "Should contain {} response variable", var_name);
            
            let var_info = columns.get(*var_name).expect(&format!("Should have {}", var_name));
            let roles = var_info.get("roles").expect("Should have roles").as_array().expect("Should be array");
            assert!(roles.iter().any(|r| r.as_str() == Some("Response")), "{} should have Response role", var_name);
            assert_eq!(var_info.get("id").expect("Should have id").as_u64(), Some(1));
        }

        // Check that predictor variables have correct IDs (starting from 2)
        let x_info = columns.get("x").expect("Should have x");
        let z_info = columns.get("z").expect("Should have z");
        assert_eq!(x_info.get("id").expect("Should have id").as_u64(), Some(2));
        assert_eq!(z_info.get("id").expect("Should have id").as_u64(), Some(3));
    }

    #[test]
    fn test_multivariate_response_with_family() {
        // Test multivariate response with family: bind(y1, y2) ~ x, family = gaussian
        let formula = "bind(y1, y2) ~ x, family = gaussian";
        let result = parse_formula(formula).expect("Should parse successfully");

        // Check family is set correctly
        let metadata = result.get("metadata").expect("Should have metadata");
        assert_eq!(metadata.get("family").expect("Should have family").as_str(), Some("gaussian"));

        // Check that both response variables are present
        let columns = result
            .get("columns")
            .expect("Should have columns")
            .as_object()
            .expect("Should be an object");

        assert!(columns.contains_key("y1"), "Should contain y1 response variable");
        assert!(columns.contains_key("y2"), "Should contain y2 response variable");
        assert!(columns.contains_key("x"), "Should contain x predictor variable");
    }

    #[test]
    fn test_multivariate_response_invalid_single_variable() {
        // Test that bind() with only one variable fails
        let formula = "bind(y1) ~ x";
        let result = parse_formula(formula);

        assert!(result.is_err(), "bind() with single variable should fail");

        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(
                error_msg.contains("bind() requires at least 2 variables"),
                "Error should mention bind() requires at least 2 variables"
            );
        }
    }
}
