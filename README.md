
[![Crates.io](https://img.shields.io/crates/v/fiasto.svg)](https://crates.io/crates/fiasto)
[![Documentation](https://docs.rs/fiasto/badge.svg)](https://docs.rs/fiasto)
[![License](https://img.shields.io/crates/l/fiasto.svg)](LICENSE)
[![Common Changelog](https://common-changelog.org/badge.svg)](https://common-changelog.org)

<h1 align="center">fiasto</h1>

<p align="center">
  <img src="img/mango_pixel2.png" alt="logo" width="240">
</p>

---
<p align="center">Pronouned like <strong>fiasco</strong>, but with a <strong>t</strong> instead of an <strong>c</strong></p>

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
- **Comprehensive Formula Support**: Full R/Wilkinson notation including complex random effects and intercept-only models
- **Variable-Centric Output**: Variables are first-class citizens with detailed metadata
- **Advanced Random Effects**: brms-style syntax with correlation control and grouping options
- **Intercept-Only Models**: Full support for `y ~ 1` and `y ~ 0` formulas with proper metadata generation
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
- **Intercept-Only Models**: Support for null models like `y ~ 1` and `y ~ 0` for baseline comparisons


## Goals

I can't think of every kind of formula that could be pasrsed. I do have a checklist to start with.

To my knowldege the `brms` formula syntax is the most complex and possibly the most complete.

I would like to start with this as a baseline then continue to extend as needed.

I also offer a clean_name for each parameter. This will all a materializer to use a simpler name for the parameter.

Polynomials for example would result in names like `x1_poly_1` or `x1_poly_2` as opposed to `[s]^2`. I keep clean_names in camel case.

### 1. Intercept-only and no-intercept models:

 `y ~ 1` -> `y ~ 1` (null model with intercept)
 `y ~ 0` -> `y ~ 0` (null model without intercept)

### 2. Mixed effects models:

 `y ~ x1*x2 + s(z) + (1+x1|1) + (1|g2) - 1` -> `y ~ x1 * x2 + s(z) + (1 + x1 | 1) + (1 | g2) - 1`

### 3. Predict `sigma`:

 `y ~ x1*x2 + s(z) + (1+x1|1) + (1|g2), sigma ~ x1 + (1|g2)` -> `y ~ x1 * x2 + s(z) + (1 + x1 | 1) + (1 | g2)` and `sigma ~ x1 + (1 | g2)`

### 4. Non-lienar models: 
`y ~ a1 - a2^x, a1 + a2 ~ 1, nl = TRUE)`

`y ~ a1 - a2^x`
`a1 ~ 1`
`a2 ~ 1`

### 5. predict a1 and a2 differently

`y ~ a1 - a2^x, a1 ~ 1, a2 ~ x + (x|g), nl = TRUE)`

`y ~ a1 - a2^x`
`a1 ~ 1`
`a2 ~ x + (x | g)`


### 6. correlated group-level effects across parameters

`y ~ a1 - a2^x, a1 ~ 1 + (1 |2| g), a2 ~ x + (x |2| g), nl = TRUE)`

`y ~ a1 - a2^x` 
`a1 ~ 1 + (1 | 2 | g)`
`a2 ~ x + (x | 2 | g)`

### 7. alternative but equivalent way to specify the above model

`y ~ a1 - a2^x, a1 ~ 1 + (1 | gr(g, id = 2)), a2 ~ x + (x | gr(g, id = 2)), nl = TRUE)`

`y ~ a1 - a2^x` 
`a1 ~ 1 + (1 | gr(g, id = 2))`
`a2 ~ x + (x | gr(g, id = 2))`

### 8. Define a multivariate model

`mvbind(y1, y2) ~ x * z + (1|g)`

`y1 ~ x * z + (1 | g)`
`y2 ~ x * z + (1 | g)`


### 9. Define a zero-inflated model also predicting the zero-inflation part
`y ~ x * z + (1+x|ID1|g), zi ~ x + (1|ID1|g))`
`y ~ x * z + (1 + x | ID1 | g)`
`zi ~ x + (1 | ID1 | g)`

### 10. Specify a predictor as monotonic
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

For detailed documentation, see [gr() Function Documentation](docs/gr_function.md).
# Trigger release workflow
