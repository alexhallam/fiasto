//! # Lexer for Statistical Formula Parsing
//!
//! This module defines the tokenizer for statistical formulas using the `logos` crate.
//! The lexer converts formula strings into a stream of tokens that can be parsed
//! into an Abstract Syntax Tree (AST).
//!
//! ## Overview
//!
//! The lexer supports comprehensive statistical formula syntax including:
//! - Basic mathematical operators and symbols
//! - Variable names and identifiers
//! - Function calls and transformations
//! - Random effects syntax with grouping structures
//! - Distribution families
//! - Boolean and string literals
//!
//! ## Token Categories
//!
//! ### Mathematical Operators
//! - `+`, `-`, `*`, `/` for arithmetic operations
//! - `~` for formula separation (response ~ predictors)
//! - `|`, `||` for random effects grouping
//! - `:` for interactions
//!
//! ### Identifiers and Literals
//! - Variable names: `[a-zA-Z][a-zA-Z0-9_]*`
//! - Integers: `0`, `1`, `[2-9]\d*`
//! - Strings: `"[^"]*"`
//! - Booleans: `true`, `false`, `TRUE`, `FALSE`
//! - Null values: `null`, `NULL`
//!
//! ### Function Tokens
//! - Transformations: `poly`, `log`, `scale`, `center`, etc.
//! - Random effects: `gr`, `mm`, `mmc`, `cs`
//! - Statistical functions: `offset`, `factor`, `bs`, `gp`, etc.
//!
//! ### Special Syntax
//! - Parentheses: `(`, `)`
//! - Comma: `,`
//! - Equals: `=`
//! - Family specification: `family`, `gaussian`, `binomial`, `poisson`
//!
//! ## Examples
//!
//! ```rust
//! use fiasto::internal::lexer::Token;
//! use logos::Logos;
//!
//! // Simple formula: y ~ x + z
//! let mut lexer = Token::lexer("y ~ x + z");
//! assert_eq!(lexer.next(), Some(Ok(Token::ColumnName))); // "y"
//! assert_eq!(lexer.next(), Some(Ok(Token::Tilde)));      // "~"
//! assert_eq!(lexer.next(), Some(Ok(Token::ColumnName))); // "x"
//! assert_eq!(lexer.next(), Some(Ok(Token::Plus)));       // "+"
//! assert_eq!(lexer.next(), Some(Ok(Token::ColumnName))); // "z"
//!
//! // Random effects: (1 | group)
//! let mut lexer = Token::lexer("(1 | group)");
//! assert_eq!(lexer.next(), Some(Ok(Token::FunctionStart))); // "("
//! assert_eq!(lexer.next(), Some(Ok(Token::One)));           // "1"
//! assert_eq!(lexer.next(), Some(Ok(Token::Pipe)));          // "|"
//! assert_eq!(lexer.next(), Some(Ok(Token::ColumnName)));    // "group"
//! assert_eq!(lexer.next(), Some(Ok(Token::FunctionEnd)));   // ")"
//!
//! // Function call: poly(x, 3)
//! let mut lexer = Token::lexer("poly(x, 3)");
//! assert_eq!(lexer.next(), Some(Ok(Token::Poly)));          // "poly"
//! assert_eq!(lexer.next(), Some(Ok(Token::FunctionStart))); // "("
//! assert_eq!(lexer.next(), Some(Ok(Token::ColumnName)));    // "x"
//! assert_eq!(lexer.next(), Some(Ok(Token::Comma)));         // ","
//! assert_eq!(lexer.next(), Some(Ok(Token::Integer)));       // "3"
//! assert_eq!(lexer.next(), Some(Ok(Token::FunctionEnd)));   // ")"
//! ```
//!
//! ## Token Ordering
//!
//! The order of tokens in the enum is important for correct lexing:
//! 1. Specific tokens (like `true`, `false`) must come before general regex patterns
//! 2. Longer tokens (like `||`) must come before shorter ones (like `|`)
//! 3. Function tokens are grouped together for clarity
//!
//! ## Error Handling
//!
//! The lexer uses `logos` which provides efficient error handling and recovery.
//! Invalid tokens are reported with their position in the input string.

use logos::Logos;

/// Tokens produced by the statistical formula lexer
///
/// Each token represents a meaningful unit in a statistical formula.
/// The tokens are designed to support the full range of R-style formula syntax.
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace
pub enum Token {
    // Mathematical operators and symbols
    /// Minus sign: `-`
    /// Used for subtraction and intercept suppression
    #[token("-")]
    Minus,

    /// Zero: `0`
    /// Used for intercept suppression in random effects
    #[token("0")]
    Zero,

    /// One: `1`
    /// Used for intercept terms in random effects
    #[token("1")]
    One,

    /// Integer numbers: `2`, `3`, `4`, etc.
    /// Used for polynomial degrees and other numeric parameters
    #[regex(r"[2-9]\d*")]
    Integer,

    /// String literals: `"text"`
    /// Used for string arguments in function calls
    #[regex(r#""[^"]*""#)]
    StringLiteral,

    // Boolean and null literals (must come before ColumnName regex)
    /// Boolean true (lowercase): `true`
    #[token("true")]
    True,

    /// Boolean true (uppercase): `TRUE`
    #[token("TRUE")]
    TrueUpper,

    /// Boolean false (lowercase): `false`
    #[token("false")]
    False,

    /// Boolean false (uppercase): `FALSE`
    #[token("FALSE")]
    FalseUpper,

    /// Null value (lowercase): `null`
    #[token("null")]
    Null,

    /// Null value (uppercase): `NULL`
    #[token("NULL")]
    NullUpper,

    /// Variable names and identifiers: `x`, `group`, `response_var`
    /// Matches: `[a-zA-Z][a-zA-Z0-9_]*`
    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*")]
    ColumnName,

    // Formula structure operators
    /// Tilde: `~`
    /// Separates response from predictors in formulas
    #[token("~")]
    Tilde,

    /// Plus: `+`
    /// Adds terms to the model
    #[token("+")]
    Plus,

    // Random effects operators
    /// Single pipe: `|`
    /// Used for correlated random effects grouping
    #[token("|")]
    Pipe,

    /// Double pipe: `||`
    /// Used for uncorrelated random effects grouping
    #[token("||")]
    DoublePipe,

    // Interaction operators
    /// Colon: `:`
    /// Creates interactions between variables
    #[token(":")]
    InteractionOnly,

    /// Slash: `/`
    /// Used for nested grouping in random effects
    #[token("/")]
    Slash,

    /// Asterisk: `*`
    /// Creates full interactions (main effects + interaction)
    #[token("*")]
    InteractionAndEffect,

    // Function delimiters
    /// Opening parenthesis: `(`
    /// Starts function calls and random effects
    #[token("(")]
    FunctionStart,

    /// Closing parenthesis: `)`
    /// Ends function calls and random effects
    #[token(")")]
    FunctionEnd,

    // Mathematical and statistical transformations
    /// Polynomial transformation: `poly(x, degree)`
    #[token("poly")]
    Poly,

    /// Offset term: `offset(x)`
    #[token("offset")]
    Offset,

    /// Factor/categorical variable: `factor(x)`
    #[token("factor")]
    Factor,

    /// Categorical variable with reference level: `c(x, ref=level)`
    #[token("c", priority = 3)]
    C,

    /// Scaling transformation: `scale(x)`
    #[token("scale")]
    Scale,

    /// Standardization: `standardize(x)`
    #[token("standardize")]
    Standardize,

    /// Centering transformation: `center(x)`
    #[token("center")]
    Center,

    /// Logarithmic transformation: `log(x)`
    #[token("log")]
    Log,

    /// B-splines: `bs(x)`
    #[token("bs")]
    BSplines,

    /// Gaussian process: `gp(x)`
    #[token("gp")]
    GaussianProcess,

    /// Monotonic transformation: `mono(x)`
    #[token("mono")]
    Monotonic,

    /// Measurement error: `me(x)`
    #[token("me")]
    MeasurementError,

    /// Missing values handling: `mi(x)`
    #[token("mi")]
    MissingValues,

    /// Forward fill: `forward_fill(x)`
    #[token("forward_fill")]
    ForwardFill,

    /// Backward fill: `backward_fill(x)`
    #[token("backward_fill")]
    BackwardFill,

    /// Difference: `diff(x)`
    #[token("diff")]
    Diff,

    /// Lag: `lag(x)`
    #[token("lag")]
    Lag,

    /// Lead: `lead(x)`
    #[token("lead")]
    Lead,

    /// Truncation: `trunc(x)`
    #[token("trunc")]
    Trunc,

    /// Weights: `weights(x)`
    #[token("weights")]
    Weights,

    /// Trials: `trials(x)`
    #[token("trials")]
    Trials,

    /// Censored data: `cens(x)`
    #[token("cens")]
    Censored,

    /// Multivariate binding: `bind(y1, y2)`
    #[token("bind")]
    Bind,

    // Random effects grouping functions
    /// Enhanced grouping: `gr(group, options)`
    #[token("gr")]
    Gr,

    /// Multi-membership: `mm(group1, group2)`
    #[token("mm")]
    Mm,

    /// Multi-membership with covariates: `mmc(x1, x2)`
    #[token("mmc")]
    Mmc,

    /// Category-specific: `cs(1)` or `cs(x)`
    #[token("cs")]
    Cs,

    // Punctuation and delimiters
    /// Comma: `,`
    /// Separates function arguments
    #[token(",")]
    Comma,

    /// Equals: `=`
    /// Used for named arguments
    #[token("=")]
    Equal,

    // Distribution families
    /// Family specification: `family = gaussian`
    #[token("family")]
    Family,

    /// Gaussian family: `gaussian`
    #[token("gaussian")]
    Gaussian,

    /// Binomial family: `binomial`
    #[token("binomial")]
    Binomial,

    /// Poisson family: `poisson`
    #[token("poisson")]
    Poisson,

    // gr() function argument names
    /// Correlation control: `cor = TRUE/FALSE`
    #[token("cor")]
    Cor,

    /// Grouping ID: `id = "group_id"`
    #[token("id")]
    Id,

    /// By variable: `by = NULL` or `by = "variable"`
    #[token("by")]
    By,

    /// Covariance control: `cov = TRUE/FALSE`
    #[token("cov")]
    Cov,

    /// Distribution: `dist = "student"`
    #[token("dist")]
    Dist,
}
