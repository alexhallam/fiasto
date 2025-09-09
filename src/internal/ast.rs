//! # Abstract Syntax Tree (AST) Definitions
//!
//! This module defines the core data structures that represent parsed statistical formulas.
//! The AST captures the hierarchical structure of formulas including terms, functions,
//! interactions, and random effects.
//!
//! ## Overview
//!
//! The AST is designed to represent R-style statistical formulas with support for:
//! - Basic terms and variables
//! - Function calls with arguments
//! - Interactions between terms
//! - Complex random effects structures
//! - Distribution families
//!
//! ## Examples
//!
//! ### Simple Linear Model
//! ```text
//! Formula: y ~ x + z
//! AST: Term::Column("y") ~ [Term::Column("x"), Term::Column("z")]
//! ```
//!
//! ### Model with Transformations
//! ```text
//! Formula: y ~ poly(x, 3) + log(z)
//! AST: Term::Column("y") ~ [
//!   Term::Function { name: "poly", args: [Ident("x"), Integer(3)] },
//!   Term::Function { name: "log", args: [Ident("z")] }
//! ]
//! ```
//!
//! ### Random Effects Model
//! ```text
//! Formula: y ~ x + (1 | group)
//! AST: Term::Column("y") ~ [
//!   Term::Column("x"),
//!   Term::RandomEffect(RandomEffect {
//!     terms: [RandomTerm::SuppressIntercept],
//!     grouping: Grouping::Simple("group"),
//!     correlation: CorrelationType::Correlated
//!   })
//! ]
//! ```

/// Distribution families for statistical models
///
/// These represent the error distribution family in generalized linear models.
/// Each family corresponds to a specific probability distribution and link function.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::ast::Family;
///
/// // Gaussian family for linear regression
/// let gaussian = Family::Gaussian;
///
/// // Binomial family for logistic regression
/// let binomial = Family::Binomial;
///
/// // Poisson family for count data
/// let poisson = Family::Poisson;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Family {
    /// Gaussian (normal) distribution - used for linear regression
    /// Link function: identity
    /// Variance function: constant
    Gaussian,
    /// Binomial distribution - used for logistic regression
    /// Link function: logit
    /// Variance function: μ(1-μ)
    Binomial,
    /// Poisson distribution - used for count data
    /// Link function: log
    /// Variance function: μ
    Poisson,
}

/// Response variable specification
///
/// Represents either a single response variable or multiple response variables
/// bound together for multivariate models.
///
/// # Examples
/// - `y` → `Response::Single("y")`
/// - `bind(y1, y2)` → `Response::Multivariate(vec!["y1", "y2"])`
#[derive(Debug, Clone, PartialEq)]
pub enum Response {
    /// Single response variable
    Single(String),
    /// Multiple response variables bound together
    Multivariate(Vec<String>),
}

/// A term in a statistical formula
///
/// Terms represent the building blocks of statistical formulas. They can be
/// simple variables, function calls, interactions, or random effects.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::ast::{Term, Argument, RandomEffect, Grouping, CorrelationType};
///
/// // Simple variable
/// let var_term = Term::Column("x".to_string());
///
/// // Function call
/// let func_term = Term::Function {
///     name: "poly".to_string(),
///     args: vec![Argument::Ident("x".to_string()), Argument::Integer(3)]
/// };
///
/// // Interaction
/// let interaction = Term::Interaction {
///     left: Box::new(Term::Column("x".to_string())),
///     right: Box::new(Term::Column("z".to_string()))
/// };
///
/// // Random effect
/// let random_effect = Term::RandomEffect(RandomEffect {
///     terms: vec![],
///     grouping: Grouping::Simple("group".to_string()),
///     correlation: CorrelationType::Correlated,
///     correlation_id: None
/// });
/// ```
#[derive(Debug, Clone)]
pub enum Term {
    /// A simple variable or column name
    ///
    /// # Examples
    /// - `x` → `Term::Column("x")`
    /// - `response_var` → `Term::Column("response_var")`
    Column(String),

    /// A function call with arguments
    ///
    /// # Examples
    /// - `poly(x, 3)` → `Term::Function { name: "poly", args: [Ident("x"), Integer(3)] }`
    /// - `log(y)` → `Term::Function { name: "log", args: [Ident("y")] }`
    /// - `scale(z)` → `Term::Function { name: "scale", args: [Ident("z")] }`
    Function {
        /// The function name (e.g., "poly", "log", "scale")
        name: String,
        /// The function arguments
        args: Vec<Argument>,
    },

    /// An interaction between two terms
    ///
    /// # Examples
    /// - `x:z` → `Term::Interaction { left: Column("x"), right: Column("z") }`
    /// - `poly(x,2):log(y)` → `Term::Interaction { left: Function{...}, right: Function{...} }`
    Interaction {
        /// The left-hand side of the interaction
        left: Box<Term>,
        /// The right-hand side of the interaction
        right: Box<Term>,
    },

    /// A random effects term
    ///
    /// # Examples
    /// - `(1 | group)` → `Term::RandomEffect(RandomEffect{...})`
    /// - `(x | group)` → `Term::RandomEffect(RandomEffect{...})`
    /// - `(x || group)` → `Term::RandomEffect(RandomEffect{...})`
    RandomEffect(RandomEffect),

    /// An intercept term
    ///
    /// # Examples
    /// - `1` → `Term::Intercept`
    /// - Used in formulas like `y ~ 1` for intercept-only models
    Intercept,

    /// A zero term (no intercept)
    ///
    /// # Examples
    /// - `0` → `Term::Zero`
    /// - Used in formulas like `y ~ 0` for models without intercept
    Zero,
}

/// Arguments to function calls
///
/// Function arguments can be identifiers, integers, strings, or boolean values.
/// These are used in function calls like `poly(x, 3)` or `gr(group, cor = TRUE)`.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::ast::Argument;
///
/// // Identifier argument
/// let ident_arg = Argument::Ident("x".to_string());
///
/// // Integer argument
/// let int_arg = Argument::Integer(3);
///
/// // String argument
/// let str_arg = Argument::String("student".to_string());
///
/// // Boolean argument
/// let bool_arg = Argument::Boolean(true);
/// ```
#[derive(Debug, Clone)]
pub enum Argument {
    /// An identifier (variable name)
    ///
    /// # Examples
    /// - `x` → `Argument::Ident("x")`
    /// - `group_var` → `Argument::Ident("group_var")`
    Ident(String),

    /// An integer value
    ///
    /// # Examples
    /// - `3` → `Argument::Integer(3)`
    /// - `0` → `Argument::Integer(0)`
    Integer(u32),

    /// A string literal
    ///
    /// # Examples
    /// - `"student"` → `Argument::String("student")`
    /// - `"group_id"` → `Argument::String("group_id")`
    String(String),

    /// A boolean value
    ///
    /// # Examples
    /// - `TRUE` → `Argument::Boolean(true)`
    /// - `FALSE` → `Argument::Boolean(false)`
    Boolean(bool),
}

/// A random effects specification
///
/// Random effects define the grouping structure and correlation patterns
/// for mixed-effects models. They specify which variables have random effects
/// and how those effects are grouped and correlated.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::ast::{RandomEffect, RandomTerm, Grouping, CorrelationType};
///
/// // Random intercepts: (1 | group)
/// let random_intercept = RandomEffect {
///     terms: vec![RandomTerm::SuppressIntercept],
///     grouping: Grouping::Simple("group".to_string()),
///     correlation: CorrelationType::Correlated,
///     correlation_id: None
/// };
///
/// // Random slopes: (x | group)
/// let random_slope = RandomEffect {
///     terms: vec![RandomTerm::Column("x".to_string())],
///     grouping: Grouping::Simple("group".to_string()),
///     correlation: CorrelationType::Correlated,
///     correlation_id: None
/// };
///
/// // Uncorrelated effects: (x || group)
/// let uncorrelated = RandomEffect {
///     terms: vec![RandomTerm::Column("x".to_string())],
///     grouping: Grouping::Simple("group".to_string()),
///     correlation: CorrelationType::Uncorrelated,
///     correlation_id: None
/// };
/// ```
#[derive(Debug, Clone)]
pub struct RandomEffect {
    /// The terms that have random effects
    ///
    /// # Examples
    /// - `(1 | group)` → `[RandomTerm::SuppressIntercept]`
    /// - `(x | group)` → `[RandomTerm::Column("x")]`
    /// - `(x + z | group)` → `[RandomTerm::Column("x"), RandomTerm::Column("z")]`
    pub terms: Vec<RandomTerm>,

    /// The grouping structure for the random effects
    ///
    /// # Examples
    /// - `(1 | group)` → `Grouping::Simple("group")`
    /// - `(1 | gr(group, cor=FALSE))` → `Grouping::Gr{...}`
    /// - `(1 | group1/group2)` → `Grouping::Nested{...}`
    pub grouping: Grouping,

    /// The correlation structure between random effects
    ///
    /// # Examples
    /// - `(x | group)` → `CorrelationType::Correlated`
    /// - `(x || group)` → `CorrelationType::Uncorrelated`
    /// - `(x |ID| group)` → `CorrelationType::CrossParameter("ID")`
    pub correlation: CorrelationType,

    /// Optional correlation ID for cross-parameter correlations
    ///
    /// # Examples
    /// - `(x | group)` → `None`
    /// - `(x |ID| group)` → `Some("ID")`
    pub correlation_id: Option<String>,
}

/// Terms within random effects specifications
///
/// Random terms specify which variables or functions have random effects
/// within a grouping structure.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::ast::{RandomTerm, Argument};
///
/// // Simple variable with random effect
/// let var_term = RandomTerm::Column("x".to_string());
///
/// // Function with random effect
/// let func_term = RandomTerm::Function {
///     name: "poly".to_string(),
///     args: vec![Argument::Ident("x".to_string()), Argument::Integer(2)]
/// };
///
/// // Interaction with random effect
/// let interaction = RandomTerm::Interaction {
///     left: Box::new(RandomTerm::Column("x".to_string())),
///     right: Box::new(RandomTerm::Column("z".to_string()))
/// };
///
/// // Suppress intercept (0 + or -1 +)
/// let suppress = RandomTerm::SuppressIntercept;
/// ```
#[derive(Debug, Clone)]
pub enum RandomTerm {
    /// A simple variable with random effects
    ///
    /// # Examples
    /// - `x` → `RandomTerm::Column("x")`
    /// - `response` → `RandomTerm::Column("response")`
    Column(String),

    /// A function call with random effects
    ///
    /// # Examples
    /// - `poly(x, 2)` → `RandomTerm::Function { name: "poly", args: [...] }`
    /// - `log(y)` → `RandomTerm::Function { name: "log", args: [...] }`
    Function {
        /// The function name
        name: String,
        /// The function arguments
        args: Vec<Argument>,
    },

    /// An interaction between random terms
    ///
    /// # Examples
    /// - `x:z` → `RandomTerm::Interaction { left: Column("x"), right: Column("z") }`
    /// - `x*y` → `RandomTerm::Interaction { left: Column("x"), right: Column("y") }`
    Interaction {
        /// The left-hand side of the interaction
        left: Box<RandomTerm>,
        /// The right-hand side of the interaction
        right: Box<RandomTerm>,
    },

    /// Suppress the random intercept (0 + or -1 +)
    ///
    /// # Examples
    /// - `(0 + x | group)` → `[SuppressIntercept, Column("x")]`
    /// - `(-1 + x | group)` → `[SuppressIntercept, Column("x")]`
    SuppressIntercept,
}

/// Grouping structures for random effects
///
/// Grouping defines how observations are grouped for random effects.
/// Different grouping structures support various statistical modeling scenarios.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::ast::{Grouping, GrOption};
///
/// // Simple grouping: (1 | group)
/// let simple = Grouping::Simple("group".to_string());
///
/// // Enhanced grouping with options: (1 | gr(group, cor=FALSE))
/// let gr_grouping = Grouping::Gr {
///     group: "group".to_string(),
///     options: vec![GrOption::Cor(false)]
/// };
///
/// // Multi-membership: (1 | mm(group1, group2))
/// let mm_grouping = Grouping::Mm {
///     groups: vec!["group1".to_string(), "group2".to_string()]
/// };
///
/// // Interaction grouping: (1 | group1:group2)
/// let interaction = Grouping::Interaction {
///     left: "group1".to_string(),
///     right: "group2".to_string()
/// };
///
/// // Nested grouping: (1 | group1/group2)
/// let nested = Grouping::Nested {
///     outer: "group1".to_string(),
///     inner: "group2".to_string()
/// };
/// ```
#[derive(Debug, Clone)]
pub enum Grouping {
    /// Simple grouping by a single variable
    ///
    /// # Examples
    /// - `(1 | group)` → `Grouping::Simple("group")`
    /// - `(x | site)` → `Grouping::Simple("site")`
    Simple(String),

    /// Enhanced grouping with gr() function and options
    ///
    /// # Examples
    /// - `(1 | gr(group))` → `Grouping::Gr { group: "group", options: [] }`
    /// - `(1 | gr(group, cor=FALSE))` → `Grouping::Gr { group: "group", options: [Cor(false)] }`
    Gr {
        /// The grouping variable name
        group: String,
        /// Additional options for the grouping
        options: Vec<GrOption>,
    },

    /// Multi-membership grouping
    ///
    /// # Examples
    /// - `(1 | mm(group1, group2))` → `Grouping::Mm { groups: ["group1", "group2"] }`
    Mm {
        /// The multiple grouping variables
        groups: Vec<String>,
    },

    /// Interaction of grouping factors
    ///
    /// # Examples
    /// - `(1 | group1:group2)` → `Grouping::Interaction { left: "group1", right: "group2" }`
    Interaction {
        /// The left grouping factor
        left: String,
        /// The right grouping factor
        right: String,
    },

    /// Nested grouping structure
    ///
    /// # Examples
    /// - `(1 | group1/group2)` → `Grouping::Nested { outer: "group1", inner: "group2" }`
    Nested {
        /// The outer (higher-level) grouping factor
        outer: String,
        /// The inner (lower-level) grouping factor
        inner: String,
    },
}

/// Options for the gr() grouping function
///
/// The gr() function provides enhanced grouping capabilities with various
/// options to control correlation structures and grouping behavior.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::ast::GrOption;
///
/// // Control correlation: cor = FALSE
/// let cor_option = GrOption::Cor(false);
///
/// // Set grouping ID: id = "group_1"
/// let id_option = GrOption::Id("group_1".to_string());
///
/// // Set by variable: by = NULL
/// let by_option = GrOption::By(None);
///
/// // Control covariance: cov = TRUE
/// let cov_option = GrOption::Cov(true);
///
/// // Set distribution: dist = "student"
/// let dist_option = GrOption::Dist("student".to_string());
/// ```
#[derive(Debug, Clone)]
pub enum GrOption {
    /// Control correlation between random effects
    ///
    /// # Examples
    /// - `cor = TRUE` → `GrOption::Cor(true)`
    /// - `cor = FALSE` → `GrOption::Cor(false)`
    Cor(bool),

    /// Set a grouping ID for the random effects
    ///
    /// # Examples
    /// - `id = "group_1"` → `GrOption::Id("group_1")`
    /// - `id = "site_effects"` → `GrOption::Id("site_effects")`
    Id(String),

    /// Set a by variable (can be NULL)
    ///
    /// # Examples
    /// - `by = NULL` → `GrOption::By(None)`
    /// - `by = "treatment"` → `GrOption::By(Some("treatment"))`
    By(Option<String>), // Can be NULL

    /// Control covariance structure
    ///
    /// # Examples
    /// - `cov = TRUE` → `GrOption::Cov(true)`
    /// - `cov = FALSE` → `GrOption::Cov(false)`
    Cov(bool), // Can be TRUE/FALSE

    /// Set the distribution for random effects
    ///
    /// # Examples
    /// - `dist = "student"` → `GrOption::Dist("student")`
    /// - `dist = "normal"` → `GrOption::Dist("normal")`
    Dist(String),
}

/// Correlation types for random effects
///
/// Defines how random effects are correlated within and across grouping levels.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::ast::CorrelationType;
///
/// // Correlated random effects: (x | group)
/// let correlated = CorrelationType::Correlated;
///
/// // Uncorrelated random effects: (x || group)
/// let uncorrelated = CorrelationType::Uncorrelated;
///
/// // Cross-parameter correlation: (x |ID| group)
/// let cross_param = CorrelationType::CrossParameter("ID".to_string());
/// ```
#[derive(Debug, Clone)]
pub enum CorrelationType {
    /// Random effects are correlated (default)
    ///
    /// # Examples
    /// - `(x | group)` → `CorrelationType::Correlated`
    /// - `(1 | group)` → `CorrelationType::Correlated`
    Correlated,

    /// Random effects are uncorrelated
    ///
    /// # Examples
    /// - `(x || group)` → `CorrelationType::Uncorrelated`
    /// - `(1 || group)` → `CorrelationType::Uncorrelated`
    Uncorrelated,

    /// Cross-parameter correlation with specific ID
    ///
    /// # Examples
    /// - `(x |ID| group)` → `CorrelationType::CrossParameter("ID")`
    /// - `(x |CORR| group)` → `CorrelationType::CrossParameter("CORR")`
    CrossParameter(String),
}
