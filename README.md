
[![Crates.io](https://img.shields.io/crates/v/fiasto.svg)](https://crates.io/crates/fiasto)
[![Documentation](https://docs.rs/fiasto/badge.svg)](https://docs.rs/fiasto)
[![License](https://img.shields.io/crates/l/fiasto.svg)](LICENSE)
[![Common Changelog](https://common-changelog.org/badge.svg)](https://common-changelog.org)

<h1 align="center">fiasto</h1>

<p align="center">
  <img src="img/mango_pixel2.png" alt="logo" width="240">
</p>

---
<p align="center">Pronouned like "fiasco", but with a "t" instead of an "c"</p>

---

<p align="center">(F)ormulas (I)n (AST) (O)ut. High-performance modern Wilkinson's formula parsing as seen in R/brms/formulaic/formulae. Supports linear
models and mixed effects models</p>

## 🚀 Ready for Production

This library is production-ready and actively maintained.

## Motivation

Formula parsing and materialization are normally done in a single library or package or coupled to a package.

There is nothing wrong with this coupling. I had some personal projects that would benefit from this decoupling the parsing and materialization.

This library was the results. A formula goes in, a json IR comes out.

Technically an AST is not returned. A simplified/structured intermediate representation (IR) in the form of json is returned. This json IR ought to be easy for many language bindings to use.

## 🎯 Simple API

The library exposes a clean, focused API:

- `parse_formula()` - Takes a Wilkinson's formula string and returns structured JSON metadata
- Additional utility functions for working with parsed formulas

"Only two functions?! What kind of library is this?!"

An easy to maintain library with a small surface area. The best kind.

Maybe I will also add

`print_canonical_formula()` - Pretty prints the canonical formula.


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
`gr()`: Indicates that the predictor should be treated as a grouping factor.
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
