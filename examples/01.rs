use logos::Logos;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

// ---------------------------
// LEXER
// ---------------------------

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
    #[token("-")]
    Minus,
    #[token("1")]
    One,
    #[regex(r"[2-9]\d*")]
    Integer,

    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*")]
    ColumnName,

    #[token("~")]
    Tilde,
    #[token("+")]
    Plus,

    #[token("(")]
    FunctionStart,
    #[token(")")]
    FunctionEnd,

    #[token("poly")]
    Poly,

    #[token(",")]
    Comma,
    #[token("=")]
    Equal,

    #[token("family")]
    Family,

    #[token("gaussian")]
    Gaussian,
    #[token("binomial")]
    Binomial,
    #[token("poisson")]
    Poisson,
}

// ---------------------------
// DATA STRUCTURES
// ---------------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Represents the distinct column names as they were input by the user
/// Example:
/// "formula": "y ~ x + poly(x, 2) + poly(x1, 4) + log(x1) - 1, family = gaussian"
///     {
///       "id": 1,
///       "name": "y"
///     },
///     {
///       "id": 2,
///       "name": "x"
///     },
///     {
///       "id": 3,
///       "name": "x1"
///     }
struct ColumnNameStruct {
    id: u32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Represents transformations applied to a column
/// Example:
///   "formula": "y ~ x + poly(x, 2) + poly(x1, 4) + log(x1) - 1, family = gaussian"
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
struct TransformationStruct {
    column_name_struct_id: u32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ColumnSuggestedNameStruct {
    column_name_struct_id: u32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FormulaMetaData {
    transformations: Vec<TransformationStruct>,
    column_names: Vec<ColumnNameStruct>,
    has_intercept: bool,
    formula: String,
    response_columns: Vec<ColumnSuggestedNameStruct>,
    fix_effects_columns: Vec<ColumnSuggestedNameStruct>,
    random_effects_columns: Vec<ColumnSuggestedNameStruct>,
}

// ---------------------------
// SIMPLE AST
// ---------------------------

#[derive(Debug, Clone)]
enum Family {
    Gaussian,
    Binomial,
    Poisson,
}

#[derive(Debug, Clone)]
enum Term {
    Column(String),
    Function { name: String, args: Vec<Argument> },
}

#[derive(Debug, Clone)]
enum Argument {
    Ident(String),
    Integer(u32),
}

// ---------------------------
// PARSER
// ---------------------------

#[derive(Error, Debug)]
/// This checks for the following
/// - lexing errors
/// - unexpected end of input
/// - unexpected tokens
/// - invalid syntax
enum ParseError {
    #[error("lexing error at {0:?}")]
    Lex(String),
    #[error("unexpected end of input")]
    Eoi,
    #[error("unexpected token: expected {expected:?}, found {found:?}")]
    Unexpected {
        expected: &'static str,
        found: Option<Token>,
    },
    #[error("invalid syntax: {0}")]
    Syntax(String),
}

/// Parser for the formula
/// This is responsible for parsing the formula string into an AST
/// The <'a> means that the parser will borrow the input string for the duration of its lifetime
/// The `input` field is a reference to the original input string
/// The `tokens` field is a vector of all the tokens found in the input string
/// The `pos` field is the current position in the token stream
struct Parser<'a> {
    input: &'a str,
    tokens: Vec<(Token, &'a str)>,
    pos: usize,
}

/// The parser implementation does the actual work of parsing the formula
impl<'a> Parser<'a> {
    // `new` creates a new parser instance. This function initializes the lexer and token vector.
    // lex.next() iterates through the tokens
    // lex.slice() shows the current token as a string
    // An example of the input to `new()` is "y ~ x + poly(x, 2) + poly(x1, 4) + log(x1) - 1, family = gaussian"
    // the `new()` function would return a Parser instance which has the following data:
    //   - input: a reference to the original input string
    //   - tokens: a vector of all the tokens found in the input string
    //   - pos: the current position in the token stream
    fn new(input: &'a str) -> Result<Self, ParseError> {
        let mut lex = Token::lexer(input);
        let mut tokens = Vec::new();

        while let Some(item) = lex.next() {
            match item {
                Ok(tok) => {
                    let slice = lex.slice();
                    tokens.push((tok, slice));
                }
                Err(()) => {
                    return Err(ParseError::Lex(lex.slice().to_string()));
                }
            }
        }

        Ok(Self {
            input,
            tokens,
            pos: 0,
        })
    }
    // The `peek` function returns the next token without consuming it
    // It is important to look ahead without consuming because we may need to check the next token multiple times
    // Without `peek`, we would have to call `next` to see the next token, which would consume it.
    fn peek(&self) -> Option<&(Token, &'a str)> {
        self.tokens.get(self.pos)
    }

    // The `next` function returns the next token and consumes it
    // The consuming action is done by incrementing the `pos` field
    fn next(&mut self) -> Option<(Token, &'a str)> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    // I don't get this generic thing
    fn matches<F>(&mut self, pred: F) -> bool
    where
        F: Fn(&Token) -> bool,
    {
        if let Some((tok, _)) = self.peek() {
            if pred(tok) {
                self.pos += 1;
                return true;
            }
        }
        false
    }

    // The `expect` function checks if the next token matches the given pattern
    // The `expect_fn` is a function that takes a reference to a Token and returns a boolean
    // It is true if the token matches the expected pattern
    fn expect(
        &mut self,
        expect_fn: fn(&Token) -> bool,
        expected: &'static str,
    ) -> Result<(Token, &'a str), ParseError> {
        if let Some((tok, slice)) = self.peek().cloned() {
            if expect_fn(&tok) {
                self.pos += 1;
                Ok((tok, slice))
            } else {
                Err(ParseError::Unexpected {
                    expected,
                    found: Some(tok),
                })
            }
        } else {
            Err(ParseError::Unexpected {
                expected,
                found: None,
            })
        }
    }

    /// The `parse_response` function parses the response variable
    /// If the token is a column name then it is a response variable
    /// In a future step in parse_formula, the response is expected to be followed by a tilde
    fn parse_response(&mut self) -> Result<String, ParseError> {
        let (_, name) = self.expect(|t| matches!(t, Token::ColumnName), "ColumnName")?;
        Ok(name.to_string())
    }

    // The `parse_formula` function is the main entry point for parsing the formula
    // This will get the response variable, the terms, flag for the intercept, and get the family
    fn parse_formula(&mut self) -> Result<(String, Vec<Term>, bool, Option<Family>), ParseError> {
        let response = self.parse_response()?;
        self.expect(|t| matches!(t, Token::Tilde), "~")?;
        let (terms, has_intercept) = self.parse_rhs()?;

        let mut family = None;
        if self.matches(|t| matches!(t, Token::Comma)) {
            self.expect(|t| matches!(t, Token::Family), "family")?;
            self.expect(|t| matches!(t, Token::Equal), "=")?;
            family = Some(self.parse_family()?);
        }

        Ok((response, terms, has_intercept, family))
    }

    // The `parse_rhs` function parses the right-hand side of the formula
    // It will parse the terms and the intercept
    // If the token is a plus or minus then it is a term
    fn parse_rhs(&mut self) -> Result<(Vec<Term>, bool), ParseError> {
        let mut terms = Vec::new();
        let mut has_intercept = true;

        // if the next token is not a comma then it is pushed to the parse_term function
        if self.peek().is_some() && !matches!(self.peek().unwrap().0, Token::Comma) {
            terms.push(self.parse_term()?);
        }
        // If the token is a plus then it is pushed to the parse_term function
        while self.matches(|t| matches!(t, Token::Plus)) {
            terms.push(self.parse_term()?);
        }

        // If the token is a minus and a one then it has no intercept
        if self.matches(|t| matches!(t, Token::Minus)) {
            if self.matches(|t| matches!(t, Token::One)) {
                has_intercept = false;
            } else {
                return Err(ParseError::Syntax(
                    "expected '1' after '-' to remove intercept".into(),
                ));
            }
        }

        Ok((terms, has_intercept))
    }

    // The `parse_term` function parses a term
    // It will parse a column name or a function
    // If the token is a function start then it will parse the function and argument
    // If the token is a column name then it will parse the column name
    // If the token is a poly then it will parse the poly function
    fn parse_term(&mut self) -> Result<Term, ParseError> {
        // If the token is a poly or column name then it will parse with `tok`
        let (tok, name_slice) = self.expect(
            |t| matches!(t, Token::Poly | Token::ColumnName),
            "Poly or ColumnName",
        )?;
        // `tok` is matched to see if it is a function start
        // if it is a function start then it will check to see if the token is poly or a column name
        // if it is a poly then it will return "poly" else it will return the column name
        if self.matches(|t| matches!(t, Token::FunctionStart)) {
            let fname = match tok {
                Token::Poly => "poly".to_string(),
                Token::ColumnName => name_slice.to_string(),
                _ => unreachable!(),
            };
            // `parse_arg_list` is defined below
            // it returns the argument if followed by a function_end.
            // for example if poly(x, 3) is the input then we look for ")" and say that 3 is the argument
            let args = self.parse_arg_list()?;
            self.expect(|t| matches!(t, Token::FunctionEnd), ")")?;
            Ok(Term::Function { name: fname, args })
        } else {
            // If the token is a column name then it will parse the column name
            // If the token is a poly then it will return an error
            match tok {
                Token::ColumnName => Ok(Term::Column(name_slice.to_string())),
                Token::Poly => Err(ParseError::Syntax("expected '(' after 'poly'".into())),
                _ => Err(ParseError::Unexpected {
                    expected: "term",
                    found: Some(tok),
                }),
            }
        }
    }

    // The `parse_arg_list` function parses the argument list
    // the arguments are the things that are followd by a ")" function_end
    fn parse_arg_list(&mut self) -> Result<Vec<Argument>, ParseError> {
        let mut args = Vec::new();
        if let Some((tok, _)) = self.peek().cloned() {
            if matches!(tok, Token::FunctionEnd) {
                return Ok(args);
            }
        }

        args.push(self.parse_arg()?);
        while self.matches(|t| matches!(t, Token::Comma)) {
            args.push(self.parse_arg()?);
        }
        Ok(args)
    }

    // `parse_arg()` returns a string if it is like a column name
    // returns a string if like an integer
    // returns a 1 if like a 1
    // errors else
    fn parse_arg(&mut self) -> Result<Argument, ParseError> {
        if let Some((tok, slice)) = self.peek().cloned() {
            match tok {
                Token::ColumnName => {
                    self.next();
                    Ok(Argument::Ident(slice.to_string()))
                }
                Token::Integer => {
                    self.next();
                    Ok(Argument::Integer(slice.parse().unwrap()))
                }
                Token::One => {
                    self.next();
                    Ok(Argument::Integer(1))
                }
                _ => Err(ParseError::Unexpected {
                    expected: "argument",
                    found: Some(tok),
                }),
            }
        } else {
            // ParseError::Eoi is... idk
            Err(ParseError::Eoi)
        }
    }

    // this expects a Family to be a valid family
    fn parse_family(&mut self) -> Result<Family, ParseError> {
        let (tok, _) = self.expect(
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
}

// ---------------------------
// META BUILDER
// ---------------------------

#[derive(Default)]
/// The MetaBuilder is responsible for building the formula metadata
/// The name_to_id is a map of the column names to their id. Useful for joins
/// The columns is a vector of the column names
/// The transformations is a vector of the transformations
/// The response_cols is a vector of the response columns
/// The fixed_cols is a vector of the fixed columns
/// The random_cols is a vector of the random columns
struct MetaBuilder {
    name_to_id: HashMap<String, u32>,
    columns: Vec<ColumnNameStruct>,
    transformations: Vec<TransformationStruct>,
    response_cols: Vec<ColumnSuggestedNameStruct>,
    fixed_cols: Vec<ColumnSuggestedNameStruct>,
    random_cols: Vec<ColumnSuggestedNameStruct>,
}

/// MetaBuilder does the following
/// `new()` instantance
/// ensure_col() - this function
impl MetaBuilder {
    fn new() -> Self {
        Self::default()
    }

    // ensure_col(), this function will ...
    fn ensure_col(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.name_to_id.get(name) {
            return id;
        }
        let id = self.columns.len() as u32 + 1;
        self.columns.push(ColumnNameStruct {
            id,
            name: name.to_string(),
        });
        self.name_to_id.insert(name.to_string(), id);
        id
    }

    // This function will ...
    fn push_response(&mut self, name: &str) {
        let id = self.ensure_col(name);
        self.response_cols.push(ColumnSuggestedNameStruct {
            column_name_struct_id: id,
            name: name.to_string(),
        });
    }

    // this function pushes fixed cols to terms
    fn push_plain_term(&mut self, name: &str) {
        let id = self.ensure_col(name);
        self.fixed_cols.push(ColumnSuggestedNameStruct {
            column_name_struct_id: id,
            name: name.to_string(),
        });
    }

    // This function returns the transformation associated with a column name
    fn push_function_term(&mut self, fname: &str, args: &[Argument]) {
        let base_ident = args.iter().find_map(|a| match a {
            Argument::Ident(s) => Some(s.as_str()),
            _ => None,
        });

        let base_id = base_ident.map(|col| self.ensure_col(col)).unwrap_or(0);

        let arg_str = args
            .iter()
            .map(|a| match a {
                Argument::Ident(s) => s.clone(),
                Argument::Integer(n) => n.to_string(),
            })
            .collect::<Vec<_>>()
            .join(", ");
        let suggested = format!("{fname}({arg_str})");

        if base_id != 0 {
            self.transformations.push(TransformationStruct {
                column_name_struct_id: base_id,
                name: fname.to_string(),
            });
            self.fixed_cols.push(ColumnSuggestedNameStruct {
                column_name_struct_id: base_id,
                name: suggested,
            });
        } else {
            self.fixed_cols.push(ColumnSuggestedNameStruct {
                column_name_struct_id: 0,
                name: suggested,
            });
        }
    }

    fn build(self, input: &str, has_intercept: bool) -> FormulaMetaData {
        FormulaMetaData {
            transformations: self.transformations,
            column_names: self.columns,
            has_intercept,
            formula: input.to_string(),
            response_columns: self.response_cols,
            fix_effects_columns: self.fixed_cols,
            random_effects_columns: self.random_cols,
        }
    }
}

// ---------------------------
// DEMO MAIN
// ---------------------------

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "y ~ x + poly(x, 2) + poly(x1, 4) + log(x1) - 1, family = gaussian";

    println!("TOKENS:");
    let mut lex = Token::lexer(input);
    while let Some(item) = lex.next() {
        match item {
            Ok(tok) => println!("{:?}: {}", tok, lex.slice()),
            Err(()) => println!("LEX ERROR at {:?}", lex.slice()),
        }
    }
    println!();

    let mut p = Parser::new(input)?;
    let (response, terms, has_intercept, family_opt) = p.parse_formula()?;

    let mut mb = MetaBuilder::new();
    mb.push_response(&response);
    for t in terms {
        match t {
            Term::Column(name) => mb.push_plain_term(&name),
            Term::Function { name, args } => mb.push_function_term(&name, &args),
        }
    }
    let meta = mb.build(input, has_intercept);

    println!("FAMILY (parsed, not stored): {:?}", family_opt);
    println!("FORMULA METADATA:");
    println!("{}", serde_json::to_string_pretty(&meta)?);

    Ok(())
}
