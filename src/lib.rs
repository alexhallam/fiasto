pub mod internal {
    pub mod parse;
    pub mod ast;
    pub mod data_structures;
    pub mod errors;
    pub mod lexer;
    pub mod meta_builder;
    pub mod parser;
    pub mod new;
    pub mod peek;
    pub mod next;
    pub mod matches;
    pub mod expect;
    pub mod parse_response;
    pub mod parse_formula;
    pub mod parse_rhs;
    pub mod parse_term;
    pub mod parse_arg_list;
    pub mod parse_arg;
    pub mod parse_family;
}

use internal::parse::{MetaBuilder, Parser, Term};
use serde_json::Value;

/// Parse a formula string and return the metadata as JSON
///
/// # Arguments
/// * `formula` - A string containing a formula in the format "y ~ x + poly(x, 2), family = gaussian"
///
/// # Returns
/// * `Result<Value, Box<dyn std::error::Error>>` - The formula metadata as JSON, or an error
///
/// # Example
/// ```
/// use fiasto::parse_formula;
///
/// let result = parse_formula("y ~ x + poly(x, 2), family = gaussian");
/// match result {
///     Ok(metadata) => println!("{}", serde_json::to_string_pretty(&metadata).unwrap()),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn parse_formula(formula: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut p = Parser::new(formula)?;
    let (response, terms, has_intercept, _family_opt) = p.parse_formula()?;

    let mut mb = MetaBuilder::new();
    mb.push_response(&response);
    for t in terms {
        match t {
            Term::Column(name) => mb.push_plain_term(&name),
            Term::Function { name, args } => mb.push_function_term(&name, &args),
        }
    }
    let meta = mb.build(formula, has_intercept);

    Ok(serde_json::to_value(meta)?)
}
