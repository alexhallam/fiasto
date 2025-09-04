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

/// Parse a formula string and return the metadata as JSON
///
/// # Arguments
/// * `formula` - A string containing a formula in the format "y ~ x + poly(x, 2), family = gaussian"
/// `formula = "y ~ x + poly(x, 2) + poly(x1, 4) + log(x1) - 1, family = gaussian"`
/// ```text
/// {
///  "column_names": [
///    {
///      "id": 1,
///      "name": "y"
///    },
///    {
///      "id": 2,
///      "name": "x"
///    },
///    {
///      "id": 3,
///      "name": "x1"
///    }
///  ],
///  "fix_effects_columns": [
///    {
///      "column_name_struct_id": 2,
///      "name": "x"
///    },
///    {
///      "column_name_struct_id": 2,
///      "name": "poly(x, 2)"
///    },
///    {
///      "column_name_struct_id": 3,
///      "name": "poly(x1, 4)"
///    },
///    {
///      "column_name_struct_id": 3,
///      "name": "log(x1)"
///    }
///  ],
///  "formula": "y ~ x + poly(x, 2) + poly(x1, 4) + log(x1) - 1, family = gaussian",
///  "has_intercept": false,
///  "random_effects_columns": [],
///  "response_columns": [
///    {
///      "column_name_struct_id": 1,
///      "name": "y"
///    }
///  ],
///  "transformations": [
///    {
///      "column_name_struct_id": 2,
///      "name": "poly"
///    },
///    {
///      "column_name_struct_id": 3,
///      "name": "poly"
///    },
///    {
///      "column_name_struct_id": 3,
///      "name": "log"
///    }
///  ]
/// }
/// ```
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
