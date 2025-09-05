
[![Crates.io](https://img.shields.io/crates/v/fiasto.svg)](https://crates.io/crates/fiasto)
[![Documentation](https://docs.rs/fiasto/badge.svg)](https://docs.rs/fiasto)
[![License](https://img.shields.io/crates/l/fiasto.svg)](LICENSE)
[![Common Changelog](https://common-changelog.org/badge.svg)](https://common-changelog.org)

<h1 align="center">fiasto</h1>

<p align="center">
  <img src="img/mango_pixel2.png" alt="logo" width="240">
</p>

---
<p align="center">Pronouned like **fiasco**, but with a **t** instead of an **c**</p>

---

<p align="center">(F)ormulas (I)n (AST) (O)ut</p>

A Language-Agnostic modern Wilkinson's formula parser and lexer.

## ⭕ In Testing

This library is in test and actively changing.

## Motivation

Formula parsing and materialization is normally done in a single library. 
Python, for example, has `patsy`/`formulaic`/`formulae` which all do parsing & materialization.
R's `model.matrix` also handles formula parsing and design matrix creation.
There is nothing wrong with this coupling. I wanted to try decoupling the parsing and materialization.
I thought this would allow a focused library that could be used in multiple languages or dataframe libraries.
This package has a clear path, to parse and/or lex formulas and return structured JSON metadata.
Note: Technically an AST is not returned. A simplified/structured intermediate representation (IR) in the form of json is returned. This json IR ought to be easy for many language bindings to use.

## 🎯 Simple API
The library exposes a clean, focused API:
- `parse_formula()` - Takes a Wilkinson's formula string and returns structured JSON metadata
- `lex_formula()` - Tokenizes a formula string and returns JSON describing each token
"Only two functions?! What kind of library is this?!"
An easy to maintain library with a small surface area. The best kind.

## Output Format
The parser returns a variable-centric JSON structure where each variable
is described with its roles, transformations, interactions, and random effects.
This makes it easy to understand the complete model structure and generate
appropriate design matrices. [wayne](https://github.com/alexhallam/wayne) is a python package
that can take this JSON and generates design matrices for use in statistical modeling.
## Features
- **Comprehensive Formula Support**: Full R/Wilkinson notation including complex random effects
- **Variable-Centric Output**: Variables are first-class citizens with detailed metadata
- **Advanced Random Effects**: brms-style syntax with correlation control and grouping options
- **High Performance**: Zero-copy processing and efficient tokenization
- **Pretty Error Messages**: Colored, contextual error reporting with syntax highlighting
- **Robust Error Recovery**: Graceful handling of malformed formulas with specific error types
- **Language Agnostic Output**: JSON format for easy integration with various programming languages
- **Comprehensive Documentation**: Detailed usage examples and grammar rules
- **Comprehensive Metadata**: Variable roles, transformations, interactions, and relationships
- **Automatic Naming For Generated Columns**: Consistent, descriptive names for transformed and interaction terms
- **Dual API**: Both parsing and lexing functions for flexibility
- **Efficient tokenization**: using one of the fastest lexer generators for Rust ([logos](https://docs.rs/logos/0.15.1/logos/index.html) crate)
- **Fast pattern matching**: using match statements and enum-based token handling. Rust match statements are zero-cost abstractions.
- **Minimal string copying**: with extensive use of string slices (`&str`) where possible

## Use Cases:

- **Formula Validation**: Check if formulas are valid against datasets before expensive computation
- **Cross-Platform Model Specs**: Define models once, implement in multiple statistical frameworks


## Goals

I can't think of every kind of formula that could be pasrsed. I do have a checklist to start with.

To my knowldege the `brms` formula syntax is the most complex and possibly the most complete.

I would like to start with this as a baseline then continue to extend as needed.

I also offer a clean_name for each parameter. This will all a materializer to use a simpler name for the parameter.

Polynomials for example would result in names like `x1_poly_1` or `x1_poly_2` as opposed to `[s]^2`. I keep clean_names in camel case.

### 1. Mixed effects models:

 `y ~ x1*x2 + s(z) + (1+x1|1) + (1|g2) - 1` -> `y ~ x1 * x2 + s(z) + (1 + x1 | 1) + (1 | g2) - 1`

### Predict `sigma`:

 `y ~ x1*x2 + s(z) + (1+x1|1) + (1|g2), sigma ~ x1 + (1|g2)` -> `y ~ x1 * x2 + s(z) + (1 + x1 | 1) + (1 | g2)` and `sigma ~ x1 + (1 | g2)`

### Non-lienar models: 
`y ~ a1 - a2^x, a1 + a2 ~ 1, nl = TRUE)`

`y ~ a1 - a2^x`
`a1 ~ 1`
`a2 ~ 1`

### predict a1 and a2 differently

`y ~ a1 - a2^x, a1 ~ 1, a2 ~ x + (x|g), nl = TRUE)`

`y ~ a1 - a2^x`
`a1 ~ 1`
`a2 ~ x + (x | g)`


###correlated group-level effects across parameters

`y ~ a1 - a2^x, a1 ~ 1 + (1 |2| g), a2 ~ x + (x |2| g), nl = TRUE)`

`y ~ a1 - a2^x` 
`a1 ~ 1 + (1 | 2 | g)`
`a2 ~ x + (x | 2 | g)`

### alternative but equivalent way to specify the above model

`y ~ a1 - a2^x, a1 ~ 1 + (1 | gr(g, id = 2)), a2 ~ x + (x | gr(g, id = 2)), nl = TRUE)`

`y ~ a1 - a2^x` 
`a1 ~ 1 + (1 | gr(g, id = 2))`
`a2 ~ x + (x | gr(g, id = 2))`

### Define a multivariate model

`mvbind(y1, y2) ~ x * z + (1|g)`

`y1 ~ x * z + (1 | g)`
`y2 ~ x * z + (1 | g)`


### Define a zero-inflated model also predicting the zero-inflation part
`y ~ x * z + (1+x|ID1|g), zi ~ x + (1|ID1|g))`
`y ~ x * z + (1 + x | ID1 | g)`
`zi ~ x + (1 | ID1 | g)`

### Specify a predictor as monotonic
`y ~ mo(x) + more_predictors)`
`y ~ mo(x) + more_predictors`

### for ordinal models only specify a predictor as category specific
`y ~ cs(x) + more_predictors)`
`y ~ cs(x) + more_predictors`


### Add a category specific group-level intercept
`y ~ cs(x) + (cs(1)|g))`
`y ~ cs(x) + (cs(1) | g)`

### Specify parameter 'disc'
`y ~ person + item, disc ~ item)
`y ~ person + item`
`disc ~ item`
`disc ~ item`

### Specify variables containing measurement error
`y ~ me(x, sdx))`
`y ~ me(x, sdx)`

### Specify predictors on all parameters of the wiener diffusion model the main formula models the drift rate 'delta'
`rt | dec(decision) ~ x, bs ~ x, ndt ~ x, bias ~ x)`
`rt | dec(decision) ~ x`
`bs ~ x`
`ndt ~ x`
`bias ~ x`

  # fix the bias parameter to 0.5
`rt | dec(decision) ~ x, bias = 0.5)`
`rt | dec(decision) ~ x`
`bias = 0.5`

### Specify different predictors for different mixture components
`mix <- mixture(gaussian, gaussian)`
`mix <- mixture(gaussian, gaussian)`
`y ~ 1, mu1 ~ x, mu2 ~ z, family = mix)`
`y ~ 1`
`mu1 ~ x`
`mu2 ~ z`

### Fix both residual standard deviations to the same value
`y ~ x, sigma2 = "sigma1", family = mix)`
`y ~ x`
`sigma2 = sigma1`

### Use the '+' operator to specify models
`(y ~ 1) +nlf(sigma ~ a * exp(b * x), a ~ x) + lf(b ~ z + (1|g), dpar = "sigma") + gaussian()`
`y ~ 1`
`sigma ~ a * exp(b * x)`
`a ~ x`
`b ~ z + (1 | g)`

### Specify a multivariate model using the '+' operator
`(y1 ~ x + (1|g)) + gaussian() + cor_ar(~1|g) + bf(y2 ~ z) + poisson()`

`y1 ~ x + (1 | g)` 
`autocor ~ arma(time = NA, gr = g, p = 1, q = 0, cov = FALSE)`
`y2 ~ z` 

### Specify correlated residuals of a gaussian and a poisson model

`(y1 ~ 1 + x + (1|c|obs), sigma = 1) + gaussian()`
`y2 ~ 1 + x + (1|c|obs)) + poisson()`

# model missing values in predictors
`bmi ~ age * mi(chl)) + bf(chl | mi() ~ age) + set_rescor(FALSE)`
`bmi ~ age * mi(chl)`
`chl | mi() ~ age`

### model sigma as a function of the mean
`y ~ eta, nl = TRUE) + lf(eta ~ 1 + x) + nlf(sigma ~ tau * sqrt(eta)) + lf(tau ~ 1)`
`y ~ eta`
`eta ~ 1 + x`
`sigma ~ tau * sqrt(eta)`
`tau ~ 1`

### Multivariate models

`(y1 ~ x + (1|g) + (y2 ~ s(z))`
`y1 ~ x + (1 | g)` 
`y2 ~ s(z)` 

### Fill method
`y ~ x + (1 | g), fill = "mean"`



## Helper functions

`rescor()`: Logical. Indicates if residual correlation between the response variables should be modeled.
`nl()`: Logical. Indicates whether formula should be treated as specifying a non-linear model.
`lf()`: Logical. Indicates if the model is linear.
`nlf()`: Logical. Indicates if the model is nonlinear.
`set_rescor()`: Logical. Indicates if residual correlation between the response variables should be modeled.
`set_nl()`: Logical. Indicates if the model is nonlinear.
`set_lf()`: Logical. Indicates if the model is linear.
`set_nlf()`: Logical. Indicates if the model is nonlinear.
`autocor()`: A one sided formula containing autocorrelation terms. All none autocorrelation terms in autocor will be silently ignored.
`seasonal()`: Takes a day parameter.
`monotonic()`: Indicates that the predictor should be treated as monotonic.
`decomp()`: Optional name of the decomposition used for the population-level design matrix. Defaults to NULL that is no decomposition. Other options currently available are "QR" for the QR decomposition that helps in fitting models with highly correlated predictors.
`sparse()`: Logical; indicates whether the population-level design matrices should be treated as sparse (defaults to FALSE). For design matrices with many zeros, this can considerably reduce required memory. Sampling speed is currently not improved or even slightly decreased.
`center()` Logical; Indicates if the population-level design matrix should be centered, which usually increases sampling efficiency. See the 'Details' section for more information. Defaults to TRUE for distributional parameters and to FALSE for non-linear parameters.
`me()`: Indicates that the predictor should be treated as containing measurement error.
`mi()`: Indicates that the predictor should be treated as containing missing values.
`cs()`: Indicates that the predictor should be treated as categorical specific.
`gr()`: Enhanced grouping factor with advanced options for controlling random effects structure.
`time_index()`: Indicates that the predictor should be treated as a time index and not used in modeling, but for seasonal patterns.






## 📦 Installation

Run `cargo add fiasto` or add this to your `Cargo.toml`:


## 📖 Example Usage

```rust
use fiasto::parse_formula;

let formula = "y ~ x1 + x2 + x1:x2 + poly(x1, 3) - 1";
let metadata = parse_formula(formula).unwrap();
println!("{}", serde_json::to_string_pretty(&metadata).unwrap());
```

### Inspect Tokens: `lex_formula`

If you want to inspect how the lexer tokenizes a formula (useful when debugging parse errors
or understanding how functions and interactions are split), use `lex_formula` which returns a
JSON array of token objects with `token` and `lexeme` fields.

```rust
use fiasto::lex_formula;

let input = "mpg ~ cyl + wt*hp + poly(disp, 4) - 1";
let tokens = lex_formula(input).unwrap();
println!("{}", serde_json::to_string_pretty(&tokens).unwrap());
```

This prints objects like:

```json
{ "token": "ColumnName", "lexeme": "mpg" }
{ "token": "Tilde", "lexeme": "~" }
{ "token": "Plus", "lexeme": "+" }
```

### Basic Formula
```rust
use fiasto::parse_formula;

// Simple linear model
let result = parse_formula("y ~ x + z").unwrap();
println!("Has intercept: {}", result.has_intercept);
println!("Columns: {:?}", result.column_names);
```

### With Family Specification
```rust
use fiasto::parse_formula;

// Generalized linear model
let result = parse_formula("y ~ x + z, family = gaussian").unwrap();
println!("Family: {:?}", result.family);
```

### Enhanced gr() Function for Random Effects
```rust
use fiasto::parse_formula;

// Basic gr() usage (equivalent to standard random effects)
let result = parse_formula("y ~ x + (1 | gr(group))").unwrap();

// Uncorrelated random effects
let result = parse_formula("y ~ x + (x | gr(group, cov = FALSE))").unwrap();

// Additional grouping with by parameter
let result = parse_formula("y ~ x + (1 | gr(subject, by = treatment))").unwrap();

// Cross-parameter correlation with id
let result = parse_formula("y ~ x + (1 | gr(group, id = 2))").unwrap();

// Non-normal distribution for random effects
let result = parse_formula("y ~ x + (1 | gr(group, dist = student))").unwrap();

// Complex example with multiple options
let result = parse_formula("y ~ x + (x | gr(subject, by = treatment, cov = FALSE, id = 1, dist = student))").unwrap();
```

#### gr() Function Options:
- **`by`**: Additional grouping variable (can be `NULL` or a variable name)
- **`cov`**: Control correlation structure (`TRUE` for correlated, `FALSE` for independent)
- **`id`**: String identifier for cross-parameter correlations
- **`dist`**: Distribution specification for random effects (e.g., `"student"`)

For detailed documentation, see [gr() Function Documentation](docs/gr_function.md).
# Trigger release workflow
