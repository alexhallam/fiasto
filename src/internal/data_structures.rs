//! # Data Structures for Formula Metadata
//!
//! This module defines the data structures used to represent the parsed formula metadata.
//! The structures are designed to be serializable to JSON and provide comprehensive
//! information about variables, their roles, transformations, and relationships.
//!
//! ## Overview
//!
//! The metadata structure is variable-centric, meaning each variable is a first-class
//! citizen with detailed information about its role in the model. This approach makes
//! it easy to understand the complete model structure and generate appropriate design matrices.
//!
//! ## Key Concepts
//!
//! - **Variable Roles**: Each variable can have multiple roles (Response, FixedEffect, etc.)
//! - **Transformations**: Functions applied to variables that generate new columns
//! - **Interactions**: Relationships between variables in fixed or random effects
//! - **Random Effects**: Information about grouping structures and correlation patterns
//! - **Generated Columns**: All columns that will be created for the model
//!
//! ## Example Output Structure
//!
//! ```json
//! {
//!   "formula": "y ~ x + poly(x, 2) + (1 | group), family = gaussian",
//!   "metadata": {
//!     "has_intercept": true,
//!     "is_random_effects_model": true,
//!     "has_uncorrelated_slopes_and_intercepts": false,
//!     "family": "gaussian"
//!   },
//!   "all_generated_columns": ["y", "x", "x_poly_1", "x_poly_2", "group"],
//!   "columns": {
//!     "y": {
//!       "id": 1,
//!       "roles": ["Response"],
//!       "generated_columns": ["y"],
//!       "transformations": [],
//!       "interactions": [],
//!       "random_effects": []
//!     }
//!   }
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Roles that variables can play in a statistical model
///
/// Variables can have multiple roles, allowing for complex model specifications
/// where a variable might be both a fixed effect and have random effects.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::data_structures::VariableRole;
///
/// // Response variable
/// let response = VariableRole::Response;
///
/// // Fixed effect predictor
/// let fixed = VariableRole::FixedEffect;
///
/// // Variable with random effects
/// let random = VariableRole::RandomEffect;
///
/// // Grouping variable for random effects
/// let grouping = VariableRole::GroupingVariable;
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VariableRole {
    /// The dependent/response variable (always gets ID 1)
    ///
    /// # Examples
    /// - `y` in `y ~ x + z`
    /// - `response` in `response ~ predictor + (1 | group)`
    Response,

    /// A predictor variable in the fixed effects part
    ///
    /// # Examples
    /// - `x` in `y ~ x + z`
    /// - `treatment` in `y ~ treatment + (1 | subject)`
    FixedEffect,

    /// A variable that has random effects
    ///
    /// # Examples
    /// - `x` in `y ~ x + (x | group)`
    /// - `time` in `y ~ time + (time | subject)`
    RandomEffect,

    /// A variable used for grouping in random effects
    ///
    /// # Examples
    /// - `group` in `(1 | group)`
    /// - `subject` in `(x | subject)`
    /// - `site` in `(1 | site)`
    GroupingVariable,

    /// A variable used in its raw form without any transformation
    ///
    /// # Examples
    /// - `x1` in `y ~ x1 + poly(x1, 2)` (x1 appears both as identity and in poly)
    /// - `x` in `y ~ x + log(x)` (x appears both as identity and in log)
    Identity,

    /// A variable that represents an interaction term
    ///
    /// # Examples
    /// - `x1_x2` for interaction `x1:x2`
    /// - `x1_x2_x3` for interaction `x1:x2:x3`
    InteractionTerm,

    /// A categorical variable with reference level specification
    ///
    /// # Examples
    /// - `c(treatment, ref=control)` for categorical treatment with control as reference
    /// - `c(group, ref="group1")` for categorical group with "group1" as reference
    Categorical,
}

/// A transformation applied to a variable
///
/// Transformations represent functions that are applied to variables to create
/// new columns for the model. Each transformation specifies the function name,
/// its parameters, and the columns it generates.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::data_structures::Transformation;
/// use serde_json::json;
///
/// // Polynomial transformation: poly(x, 3)
/// let poly_transform = Transformation {
///     function: "poly".to_string(),
///     parameters: json!({
///         "degree": 3,
///         "orthogonal": true
///     }),
///     generates_columns: vec!["x_poly_1".to_string(), "x_poly_2".to_string(), "x_poly_3".to_string()]
/// };
///
/// // Logarithm transformation: log(y)
/// let log_transform = Transformation {
///     function: "log".to_string(),
///     parameters: json!({}),
///     generates_columns: vec!["y_log".to_string()]
/// };
///
/// // Scaling transformation: scale(z)
/// let scale_transform = Transformation {
///     function: "scale".to_string(),
///     parameters: json!({
///         "center": true,
///         "scale": true
///     }),
///     generates_columns: vec!["z_scaled".to_string()]
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transformation {
    /// The name of the transformation function
    ///
    /// # Examples
    /// - `"poly"` for polynomial transformations
    /// - `"log"` for logarithmic transformations
    /// - `"scale"` for scaling transformations
    pub function: String,

    /// Parameters for the transformation function
    ///
    /// # Examples
    /// - `{"degree": 3, "orthogonal": true}` for poly()
    /// - `{}` for log() (no parameters)
    /// - `{"center": true, "scale": true}` for scale()
    pub parameters: serde_json::Value, // Flexible parameters object

    /// The column names generated by this transformation
    ///
    /// # Examples
    /// - `["x_poly_1", "x_poly_2", "x_poly_3"]` for poly(x, 3)
    /// - `["y_log"]` for log(y)
    /// - `["z_scaled"]` for scale(z)
    pub generates_columns: Vec<String>,
}

/// An interaction between variables
///
/// Interactions represent relationships between variables in either fixed effects
/// or random effects contexts. They specify which variables interact and provide
/// context about the interaction.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::data_structures::Interaction;
///
/// // Fixed effects interaction: x:z
/// let fixed_interaction = Interaction {
///     with: vec!["z".to_string()],
///     order: 2,
///     context: "fixed_effects".to_string(),
///     grouping_variable: None
/// };
///
/// // Random effects interaction: (x:z | group)
/// let random_interaction = Interaction {
///     with: vec!["z".to_string()],
///     order: 2,
///     context: "random_effects".to_string(),
///     grouping_variable: Some("group".to_string())
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interaction {
    /// The variables that this variable interacts with
    ///
    /// # Examples
    /// - `["z"]` for interaction `x:z`
    /// - `["y", "z"]` for interaction `x:y:z`
    pub with: Vec<String>,

    /// The order of the interaction (number of variables involved)
    ///
    /// # Examples
    /// - `2` for `x:z` (two-way interaction)
    /// - `3` for `x:y:z` (three-way interaction)
    pub order: u32,

    /// The context where this interaction occurs
    ///
    /// # Examples
    /// - `"fixed_effects"` for interactions in the fixed effects part
    /// - `"random_effects"` for interactions in random effects
    pub context: String, // "fixed_effects" or "random_effects"

    /// The grouping variable for random effects interactions
    ///
    /// # Examples
    /// - `None` for fixed effects interactions
    /// - `Some("group")` for `(x:z | group)`
    pub grouping_variable: Option<String>, // Only for random effects
}

/// Information about random effects for a variable
///
/// Random effects information describes how a variable participates in random effects
/// structures, including the type of random effect and grouping information.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::data_structures::RandomEffectInfo;
///
/// // Random intercept: (1 | group)
/// let random_intercept = RandomEffectInfo {
///     kind: "grouping".to_string(),
///     grouping_variable: "group".to_string(),
///     has_intercept: true,
///     correlated: true,
///     includes_interactions: vec![],
///     variables: Some(vec![])
/// };
///
/// // Random slope: (x | group)
/// let random_slope = RandomEffectInfo {
///     kind: "slope".to_string(),
///     grouping_variable: "group".to_string(),
///     has_intercept: false,
///     correlated: true,
///     includes_interactions: vec![],
///     variables: None
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RandomEffectInfo {
    /// The type of random effect
    ///
    /// # Examples
    /// - `"intercept"` for random intercepts
    /// - `"slope"` for random slopes
    /// - `"grouping"` for grouping variables
    pub kind: String, // "intercept", "slope", "grouping"

    /// The grouping variable for this random effect
    ///
    /// # Examples
    /// - `"group"` for `(x | group)`
    /// - `"subject"` for `(time | subject)`
    pub grouping_variable: String,

    /// Whether this random effect includes an intercept
    ///
    /// # Examples
    /// - `true` for `(1 | group)` or `(x | group)`
    /// - `false` for `(0 + x | group)`
    pub has_intercept: bool,

    /// Whether random effects are correlated
    ///
    /// # Examples
    /// - `true` for `(x | group)` (correlated)
    /// - `false` for `(x || group)` (uncorrelated)
    pub correlated: bool,

    /// Interactions included in this random effect
    ///
    /// # Examples
    /// - `[]` for simple random effects
    /// - `["z"]` for `(x:z | group)`
    pub includes_interactions: Vec<String>,

    /// Variables involved in this random effect (for grouping kind)
    ///
    /// # Examples
    /// - `Some(vec![])` for `(1 | group)`
    /// - `Some(vec!["x"])` for `(x | group)`
    /// - `None` for slope random effects
    pub variables: Option<Vec<String>>, // For grouping kind
}

/// Complete information about a variable in the model
///
/// VariableInfo provides comprehensive information about each variable in the model,
/// including its roles, transformations, interactions, and random effects.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::data_structures::{VariableInfo, VariableRole, Transformation, Interaction, RandomEffectInfo};
/// use serde_json::json;
///
/// // Response variable
/// let response_var = VariableInfo {
///     id: 1,
///     roles: vec![VariableRole::Response],
///     transformations: vec![],
///     interactions: vec![],
///     random_effects: vec![],
///     generated_columns: vec!["y".to_string()]
/// };
///
/// // Variable with transformation and random effects
/// let complex_var = VariableInfo {
///     id: 2,
///     roles: vec![VariableRole::FixedEffect, VariableRole::RandomEffect],
///     transformations: vec![Transformation {
///         function: "poly".to_string(),
///         parameters: json!({"degree": 2}),
///         generates_columns: vec!["x_poly_1".to_string(), "x_poly_2".to_string()]
///     }],
///     interactions: vec![Interaction {
///         with: vec!["z".to_string()],
///         order: 2,
///         context: "fixed_effects".to_string(),
///         grouping_variable: None
///     }],
///     random_effects: vec![RandomEffectInfo {
///         kind: "slope".to_string(),
///         grouping_variable: "group".to_string(),
///         has_intercept: false,
///         correlated: true,
///         includes_interactions: vec![],
///         variables: None
///     }],
///     generated_columns: vec!["x_poly_1".to_string(), "x_poly_2".to_string()]
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VariableInfo {
    /// Unique identifier for this variable (response always gets ID 1)
    pub id: u32,

    /// All roles this variable plays in the model
    pub roles: Vec<VariableRole>,

    /// Transformations applied to this variable
    pub transformations: Vec<Transformation>,

    /// Interactions this variable participates in
    pub interactions: Vec<Interaction>,

    /// Random effects information for this variable
    pub random_effects: Vec<RandomEffectInfo>,

    /// All column names generated for this variable
    pub generated_columns: Vec<String>,
}

/// Metadata about the overall formula
///
/// FormulaMetadataInfo provides high-level information about the formula structure,
/// including whether it has an intercept, uses random effects, and specifies a family.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::data_structures::FormulaMetadataInfo;
///
/// // Simple linear model
/// let linear_meta = FormulaMetadataInfo {
///     has_intercept: true,
///     is_random_effects_model: false,
///     has_uncorrelated_slopes_and_intercepts: false,
///     family: Some("gaussian".to_string()),
///     response_variable_count: 1
/// };
///
/// // Mixed effects model with uncorrelated effects
/// let mixed_meta = FormulaMetadataInfo {
///     has_intercept: true,
///     is_random_effects_model: true,
///     has_uncorrelated_slopes_and_intercepts: true,
///     family: Some("gaussian".to_string()),
///     response_variable_count: 1
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormulaMetadataInfo {
    /// Whether the model includes an intercept term
    pub has_intercept: bool,

    /// Whether the model includes random effects
    pub is_random_effects_model: bool,

    /// Whether the model uses uncorrelated random slopes and intercepts (|| syntax)
    pub has_uncorrelated_slopes_and_intercepts: bool,

    /// The distribution family for the model (if specified)
    pub family: Option<String>,

    /// Number of response variables (1 for single response, >1 for multivariate)
    pub response_variable_count: u32,
}

/// Complete formula metadata structure
///
/// FormulaMetaData is the top-level structure that contains all information
/// about a parsed statistical formula. It provides both the original formula
/// and comprehensive metadata about all variables and their relationships.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::data_structures::{FormulaMetaData, FormulaMetadataInfo, VariableInfo, VariableRole};
/// use std::collections::HashMap;
///
/// let mut columns = HashMap::new();
/// columns.insert("y".to_string(), VariableInfo {
///     id: 1,
///     roles: vec![VariableRole::Response],
///     transformations: vec![],
///     interactions: vec![],
///     random_effects: vec![],
///     generated_columns: vec!["y".to_string()]
/// });
///
/// let metadata = FormulaMetaData {
///     formula: "y ~ x + (1 | group), family = gaussian".to_string(),
///     metadata: FormulaMetadataInfo {
///         has_intercept: true,
///         is_random_effects_model: true,
///         has_uncorrelated_slopes_and_intercepts: false,
///         family: Some("gaussian".to_string()),
///         response_variable_count: 1
///     },
///     columns,
///     all_generated_columns: vec!["y".to_string(), "intercept".to_string(), "x".to_string(), "group".to_string()],
///     all_generated_columns_formula_order: {
///         let mut map = HashMap::new();
///         map.insert("1".to_string(), "y".to_string());
///         map.insert("2".to_string(), "intercept".to_string());
///         map.insert("3".to_string(), "x".to_string());
///         map.insert("4".to_string(), "group".to_string());
///         map
///     }
/// };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormulaMetaData {
    /// The original formula string
    pub formula: String,

    /// High-level metadata about the formula
    pub metadata: FormulaMetadataInfo,

    /// Detailed information about each variable
    pub columns: HashMap<String, VariableInfo>,

    /// All generated column names ordered by variable ID, including intercept if present
    pub all_generated_columns: Vec<String>,

    /// Mapping of formula order to column names
    ///
    /// This field provides a mapping from formula order (as string keys "1", "2", etc.)
    /// to the corresponding column names. The order follows the formula structure:
    /// 1. Response variable
    /// 2. Intercept (if present)
    /// 3. Variables in order of appearance in the formula
    ///
    /// # Examples
    ///
    /// For formula `y ~ x + poly(x, 2) + log(z)`:
    /// ```json
    /// {
    ///   "1": "y",
    ///   "2": "intercept",
    ///   "3": "x",
    ///   "4": "x_poly_1",
    ///   "5": "x_poly_2",
    ///   "6": "z_log"
    /// }
    /// ```
    pub all_generated_columns_formula_order: HashMap<String, String>,
}

// Legacy structures for backward compatibility
// These structures are maintained for compatibility with older versions
// but are not used in the current variable-centric approach.

/// Legacy structure for column names (deprecated)
///
/// This structure was used in the old effect-centric metadata format.
/// It is maintained for backward compatibility but should not be used
/// in new code. Use `VariableInfo` instead.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnNameStruct {
    /// Unique identifier for the column
    pub id: u32,
    /// The column name
    pub name: String,
}

/// Legacy structure for transformations (deprecated)
///
/// This structure was used in the old effect-centric metadata format.
/// It is maintained for backward compatibility but should not be used
/// in new code. Use `Transformation` within `VariableInfo` instead.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformationStruct {
    /// ID of the associated column name struct
    pub column_name_struct_id: u32,
    /// The transformation name
    pub name: String,
}

/// Legacy structure for suggested column names (deprecated)
///
/// This structure was used in the old effect-centric metadata format.
/// It is maintained for backward compatibility but should not be used
/// in new code. Use `generated_columns` within `VariableInfo` instead.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnSuggestedNameStruct {
    /// ID of the associated column name struct
    pub column_name_struct_id: u32,
    /// The suggested column name
    pub name: String,
}
