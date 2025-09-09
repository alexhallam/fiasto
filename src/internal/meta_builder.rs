//! # MetaBuilder: Variable-Centric Formula Metadata Construction
//!
//! The MetaBuilder is responsible for constructing comprehensive metadata from parsed
//! formula AST nodes. It uses a variable-centric approach where each variable is
//! tracked as a first-class citizen with detailed information about its roles,
//! transformations, interactions, and random effects.
//!
//! ## Overview
//!
//! The MetaBuilder processes AST nodes and builds a structured metadata representation
//! that makes it easy to understand the complete model structure. It handles:
//!
//! - **Variable Management**: Assigns unique IDs and tracks all variables
//! - **Role Assignment**: Determines what role each variable plays (Response, FixedEffect, etc.)
//! - **Transformation Tracking**: Records all transformations and their generated columns
//! - **Interaction Detection**: Identifies and documents variable interactions
//! - **Random Effects Processing**: Handles complex random effects structures
//! - **Metadata Generation**: Creates the final variable-centric output structure
//!
//! ## Key Features
//!
//! - **Variable-Centric Design**: Variables are the primary entities with comprehensive attributes
//! - **ID Management**: Response variable always gets ID 1, others start from ID 2
//! - **Generated Columns**: Tracks all columns that will be created for the model
//! - **Role Flexibility**: Variables can have multiple roles (e.g., both FixedEffect and RandomEffect)
//! - **Transformation Support**: Handles complex transformations with parameter tracking
//! - **Random Effects**: Supports all brms-style random effects syntax
//!
//! ## Example Usage
//!
//! ```rust
//! use fiasto::internal::meta_builder::MetaBuilder;
//! use fiasto::internal::ast::{Term, Argument, RandomEffect, Grouping, CorrelationType};
//!
//! let mut builder = MetaBuilder::new();
//!
//! // Add response variable
//! builder.push_response("y");
//!
//! // Add fixed effect
//! builder.push_plain_term("x");
//!
//! // Add transformation
//! builder.push_function_term("poly", &[Argument::Ident("x".to_string()), Argument::Integer(2)]);
//!
//! // Add random effect
//! let random_effect = RandomEffect {
//!     terms: vec![],
//!     grouping: Grouping::Simple("group".to_string()),
//!     correlation: CorrelationType::Correlated,
//!     correlation_id: None
//! };
//! builder.push_random_effect(&random_effect);
//!
//! // Build final metadata
//! let metadata = builder.build("y ~ x + poly(x, 2) + (1 | group)", true, Some("gaussian".to_string()));
//! ```
//!
//! ## Output Structure
//!
//! The MetaBuilder produces a variable-centric JSON structure where each variable
//! contains comprehensive information about its role in the model:
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
//!     },
//!     "x": {
//!       "id": 2,
//!       "roles": ["FixedEffect"],
//!       "generated_columns": ["x_poly_1", "x_poly_2"],
//!       "transformations": [...],
//!       "interactions": [],
//!       "random_effects": []
//!     }
//!   }
//! }
//! ```

use super::{
    ast::{Argument, Grouping, RandomEffect, RandomTerm},
    data_structures::{
        FormulaMetadataInfo, Interaction, RandomEffectInfo, Transformation, VariableInfo,
        VariableRole,
    },
};
use std::collections::HashMap;

/// The MetaBuilder constructs variable-centric formula metadata
///
/// This struct is responsible for building comprehensive metadata from parsed
/// formula AST nodes. It uses a variable-centric approach where each variable
/// is tracked with detailed information about its roles, transformations,
/// interactions, and random effects.
///
/// # Examples
///
/// ```rust
/// use fiasto::internal::meta_builder::MetaBuilder;
///
/// let mut builder = MetaBuilder::new();
/// builder.push_response("y");
/// builder.push_plain_term("x");
/// let metadata = builder.build("y ~ x", true, None);
/// ```
#[derive(Default)]
pub struct MetaBuilder {
    /// Maps variable names to their unique IDs
    ///
    /// # Examples
    /// - `"y"` → `1` (response always gets ID 1)
    /// - `"x"` → `2` (first predictor gets ID 2)
    /// - `"group"` → `3` (grouping variable gets ID 3)
    name_to_id: HashMap<String, u32>,

    /// Maps variable names to their complete information
    ///
    /// Contains all variables with their roles, transformations,
    /// interactions, and random effects information.
    columns: HashMap<String, VariableInfo>,

    /// Whether the model uses uncorrelated random slopes and intercepts (|| syntax)
    ///
    /// # Examples
    /// - `true` for `(x || group)` (uncorrelated effects)
    /// - `false` for `(x | group)` (correlated effects)
    has_uncorrelated_slopes_and_intercepts: bool,

    /// Whether the model includes any random effects
    ///
    /// # Examples
    /// - `true` for `y ~ x + (1 | group)`
    /// - `false` for `y ~ x + z`
    is_random_effects_model: bool,

    /// The next available ID for new variables
    ///
    /// Starts at 2 (since response gets ID 1) and increments
    /// for each new variable added.
    next_id: u32,
}

impl MetaBuilder {
    /// Creates a new MetaBuilder instance
    ///
    /// Initializes the builder with empty collections and default values.
    /// The next_id starts at 1, but the response variable will be assigned ID 1,
    /// so other variables will start from ID 2.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fiasto::internal::meta_builder::MetaBuilder;
    ///
    /// let builder = MetaBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            name_to_id: HashMap::new(),
            columns: HashMap::new(),
            has_uncorrelated_slopes_and_intercepts: false,
            is_random_effects_model: false,
            next_id: 1,
        }
    }

    /// Ensures a variable exists in the columns map and returns its ID
    pub fn ensure_variable(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.name_to_id.get(name) {
            id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            self.name_to_id.insert(name.to_string(), id);
            self.columns.insert(
                name.to_string(),
                VariableInfo {
                    id,
                    roles: Vec::new(),
                    transformations: Vec::new(),
                    interactions: Vec::new(),
                    random_effects: Vec::new(),
                    generated_columns: vec![name.to_string()], // Default to the variable name itself
                },
            );
            id
        }
    }

    /// Adds a role to a variable
    ///
    /// Adds a new role to the variable if it doesn't already have that role.
    /// Variables can have multiple roles (e.g., both FixedEffect and RandomEffect).
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable
    /// * `role` - The role to add to the variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fiasto::internal::meta_builder::MetaBuilder;
    /// use fiasto::internal::data_structures::VariableRole;
    ///
    /// let mut builder = MetaBuilder::new();
    /// builder.ensure_variable("x");
    /// builder.add_role("x", VariableRole::FixedEffect);
    /// ```
    pub fn add_role(&mut self, name: &str, role: VariableRole) {
        if let Some(var_info) = self.columns.get_mut(name) {
            if !var_info.roles.contains(&role) {
                var_info.roles.push(role);
            }
        }
    }

    /// Adds a transformation to a variable
    pub fn add_transformation(&mut self, name: &str, transformation: Transformation) {
        if let Some(var_info) = self.columns.get_mut(name) {
            var_info.transformations.push(transformation.clone());
            
            // If the variable has an Identity role, preserve the original variable name
            // and add the transformation's generated columns
            if var_info.roles.contains(&VariableRole::Identity) {
                let mut new_columns = vec![name.to_string()]; // Keep the original variable name
                new_columns.extend(transformation.generates_columns);
                var_info.generated_columns = new_columns;
            } else {
                // Update generated columns with the transformation's generated columns
                var_info.generated_columns = transformation.generates_columns;
            }
        }
    }

    /// Adds an interaction to a variable
    pub fn add_interaction(&mut self, name: &str, interaction: Interaction) {
        if let Some(var_info) = self.columns.get_mut(name) {
            var_info.interactions.push(interaction);
        }
    }

    /// Adds random effect info to a variable
    pub fn add_random_effect(&mut self, name: &str, random_effect: RandomEffectInfo) {
        if let Some(var_info) = self.columns.get_mut(name) {
            var_info.random_effects.push(random_effect);
        }
    }

    /// Adds a response variable (always gets ID 1)
    ///
    /// The response variable is always assigned ID 1, and all other variables
    /// will be assigned IDs starting from 2. This ensures consistent ordering
    /// in the output metadata.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the response variable
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fiasto::internal::meta_builder::MetaBuilder;
    ///
    /// let mut builder = MetaBuilder::new();
    /// builder.push_response("y");
    /// // y will have ID 1 and role Response
    /// ```
    pub fn push_response(&mut self, name: &str) {
        // Ensure response variable gets ID 1
        if !self.name_to_id.contains_key(name) {
            self.name_to_id.insert(name.to_string(), 1);
            self.columns.insert(
                name.to_string(),
                VariableInfo {
                    id: 1,
                    roles: vec![VariableRole::Response],
                    transformations: Vec::new(),
                    interactions: Vec::new(),
                    random_effects: Vec::new(),
                    generated_columns: vec![name.to_string()],
                },
            );
            self.next_id = 2; // Start other variables from ID 2
        } else {
            self.add_role(name, VariableRole::Response);
        }
    }

    /// Adds a plain variable term (identity transformation)
    ///
    /// Adds a simple variable that appears without any transformation.
    /// The variable will be assigned the next available ID and
    /// given the Identity role to indicate it's used in its raw form.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variable to add as a plain term
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fiasto::internal::meta_builder::MetaBuilder;
    ///
    /// let mut builder = MetaBuilder::new();
    /// builder.push_plain_term("x");
    /// // x will be added with Identity role
    /// ```
    pub fn push_plain_term(&mut self, name: &str) {
        self.ensure_variable(name);
        self.add_role(name, VariableRole::Identity);
    }

    /// Adds an interaction term
    pub fn push_interaction(
        &mut self,
        left: &crate::internal::ast::Term,
        right: &crate::internal::ast::Term,
    ) {
        // Extract variable names from the interaction terms
        let left_name = Self::extract_variable_name(left);
        let right_name = Self::extract_variable_name(right);

        if let (Some(left_var), Some(right_var)) = (left_name, right_name) {
            // Ensure both variables exist
            self.ensure_variable(&left_var);
            self.ensure_variable(&right_var);

            // Add fixed effect role to both variables
            self.add_role(&left_var, VariableRole::FixedEffect);
            self.add_role(&right_var, VariableRole::FixedEffect);

            // Generate interaction column name
            let interaction_name = format!("{}_z", left_var);

            // Add interaction info to both variables
            let interaction = Interaction {
                with: vec![right_var.clone()],
                order: 2,
                context: "fixed_effects".to_string(),
                grouping_variable: None,
            };
            self.add_interaction(&left_var, interaction);

            let interaction = Interaction {
                with: vec![left_var.clone()],
                order: 2,
                context: "fixed_effects".to_string(),
                grouping_variable: None,
            };
            self.add_interaction(&right_var, interaction);

            // Update generated columns for the left variable to include the interaction
            if let Some(var_info) = self.columns.get_mut(&left_var) {
                if !var_info.generated_columns.contains(&interaction_name) {
                    var_info.generated_columns.push(interaction_name);
                }
            }
        }
    }

    /// Extracts variable name from a term
    fn extract_variable_name(term: &crate::internal::ast::Term) -> Option<String> {
        match term {
            crate::internal::ast::Term::Column(name) => Some(name.clone()),
            crate::internal::ast::Term::Function { name: _name, args } => {
                // For functions, extract the first argument if it's an identifier
                args.iter().find_map(|arg| match arg {
                    Argument::Ident(s) => Some(s.clone()),
                    _ => None,
                })
            }
            crate::internal::ast::Term::Interaction {
                left,
                right: _right,
            } => {
                // For nested interactions, we'll use the left side for now
                Self::extract_variable_name(left)
            }
            crate::internal::ast::Term::RandomEffect(_) => None,
        }
    }

    /// Adds a function/transformation term
    pub fn push_function_term(&mut self, fname: &str, args: &[Argument]) {
        let base_ident = args.iter().find_map(|a| match a {
            Argument::Ident(s) => Some(s.as_str()),
            _ => None,
        });

        if let Some(base_col) = base_ident {
            self.ensure_variable(base_col);
            // Add FixedEffect role for the transformed version
            self.add_role(base_col, VariableRole::FixedEffect);

            // Create transformation info
            let parameters = self.extract_function_parameters(fname, args);
            let generates_columns = self.generate_transformation_columns(fname, args);

            let transformation = Transformation {
                function: fname.to_string(),
                parameters,
                generates_columns,
            };

            self.add_transformation(base_col, transformation);
        }
    }

    /// Handles random effects with variable-centric approach
    pub fn push_random_effect(&mut self, random_effect: &RandomEffect) {
        self.is_random_effects_model = true;

        // Check if this random effect uses uncorrelated syntax (||)
        if matches!(
            random_effect.correlation,
            crate::internal::ast::CorrelationType::Uncorrelated
        ) {
            self.has_uncorrelated_slopes_and_intercepts = true;
        }

        // Extract grouping variable name
        let grouping_var = match &random_effect.grouping {
            Grouping::Simple(group) => group.clone(),
            Grouping::Gr { group, .. } => group.clone(),
            Grouping::Mm { groups } => groups.join("_"),
            Grouping::Interaction { left, right } => format!("{}:{}", left, right),
            Grouping::Nested { outer, inner } => format!("{}/{}", outer, inner),
        };

        // Ensure grouping variable exists and mark it as such
        self.ensure_variable(&grouping_var);
        self.add_role(&grouping_var, VariableRole::GroupingVariable);

        // Determine if this random effect has an intercept
        let has_intercept = random_effect
            .terms
            .iter()
            .any(|term| matches!(term, RandomTerm::Column(name) if name == "1"));

        // Determine correlation status
        let correlated = !matches!(
            random_effect.correlation,
            crate::internal::ast::CorrelationType::Uncorrelated
        );

        // Process each term in the random effect
        let mut variables_in_random_effect = Vec::new();
        let mut interactions_in_random_effect = Vec::new();

        for term in &random_effect.terms {
            match term {
                RandomTerm::Column(name) => {
                    if name != "1" {
                        self.ensure_variable(name);
                        self.add_role(name, VariableRole::RandomEffect);
                        variables_in_random_effect.push(name.clone());

                        // Add random effect info to the variable
                        let random_effect_info = RandomEffectInfo {
                            kind: "slope".to_string(),
                            grouping_variable: grouping_var.clone(),
                            has_intercept,
                            correlated,
                            includes_interactions: Vec::new(),
                            variables: None,
                        };
                        self.add_random_effect(name, random_effect_info);
                    }
                }
                RandomTerm::Function {
                    name: func_name,
                    args,
                } => {
                    let base_ident = args.iter().find_map(|a| match a {
                        Argument::Ident(s) => Some(s.as_str()),
                        _ => None,
                    });

                    if let Some(base_col) = base_ident {
                        self.ensure_variable(base_col);
                        self.add_role(base_col, VariableRole::RandomEffect);
                        variables_in_random_effect.push(base_col.to_string());

                        // Add transformation
                        let parameters = self.extract_function_parameters(func_name, args);
                        let generates_columns =
                            self.generate_transformation_columns(func_name, args);

                        let transformation = Transformation {
                            function: func_name.clone(),
                            parameters,
                            generates_columns,
                        };
                        self.add_transformation(base_col, transformation);

                        // Add random effect info
                        let random_effect_info = RandomEffectInfo {
                            kind: "slope".to_string(),
                            grouping_variable: grouping_var.clone(),
                            has_intercept,
                            correlated,
                            includes_interactions: Vec::new(),
                            variables: None,
                        };
                        self.add_random_effect(base_col, random_effect_info);
                    }
                }
                RandomTerm::Interaction { left, right } => {
                    let left_name = match left.as_ref() {
                        RandomTerm::Column(name) => name.clone(),
                        _ => "interaction".to_string(),
                    };
                    let right_name = match right.as_ref() {
                        RandomTerm::Column(name) => name.clone(),
                        _ => "interaction".to_string(),
                    };

                    let interaction_name = format!("{}:{}", left_name, right_name);
                    interactions_in_random_effect.push(interaction_name.clone());

                    // Add interaction info to both variables
                    let interaction = Interaction {
                        with: vec![right_name.clone()],
                        order: 2,
                        context: "random_effects".to_string(),
                        grouping_variable: Some(grouping_var.clone()),
                    };
                    self.add_interaction(&left_name, interaction);

                    let interaction = Interaction {
                        with: vec![left_name.clone()],
                        order: 2,
                        context: "random_effects".to_string(),
                        grouping_variable: Some(grouping_var.clone()),
                    };
                    self.add_interaction(&right_name, interaction);
                }
                RandomTerm::SuppressIntercept => {
                    // Intercept suppression - no column to add
                }
            }
        }

        // Add grouping random effect info to the grouping variable
        let grouping_random_effect = RandomEffectInfo {
            kind: "grouping".to_string(),
            grouping_variable: grouping_var.clone(),
            has_intercept,
            correlated,
            includes_interactions: interactions_in_random_effect,
            variables: Some(variables_in_random_effect),
        };
        self.add_random_effect(&grouping_var, grouping_random_effect);
    }

    /// Extracts function parameters into a JSON value
    fn extract_function_parameters(&self, fname: &str, args: &[Argument]) -> serde_json::Value {
        let mut params = serde_json::Map::new();

        match fname {
            "poly" => {
                if let Some(Argument::Integer(degree)) = args.get(1) {
                    params.insert(
                        "degree".to_string(),
                        serde_json::Value::Number((*degree).into()),
                    );
                    params.insert("orthogonal".to_string(), serde_json::Value::Bool(true));
                }
            }
            "log" => {
                // No additional parameters for log
            }
            _ => {
                // Generic parameter handling
                for (i, arg) in args.iter().enumerate() {
                    let key = format!("arg_{}", i);
                    let value = match arg {
                        Argument::Integer(n) => serde_json::Value::Number((*n).into()),
                        Argument::String(s) => serde_json::Value::String(s.clone()),
                        Argument::Boolean(b) => serde_json::Value::Bool(*b),
                        Argument::Ident(s) => serde_json::Value::String(s.clone()),
                    };
                    params.insert(key, value);
                }
            }
        }

        serde_json::Value::Object(params)
    }

    /// Generates column names for transformations
    fn generate_transformation_columns(&self, fname: &str, args: &[Argument]) -> Vec<String> {
        let base_name = args
            .iter()
            .find_map(|a| match a {
                Argument::Ident(s) => Some(s.as_str()),
                _ => None,
            })
            .unwrap_or("unknown");

        match fname {
            "poly" => {
                if let Some(Argument::Integer(degree)) = args.get(1) {
                    (1..=*degree as usize)
                        .map(|i| format!("{}_poly_{}", base_name, i))
                        .collect()
                } else {
                    vec![format!("{}_poly", base_name)]
                }
            }
            "log" => vec![format!("{}_log", base_name)],
            _ => vec![format!("{}_{}", base_name, fname)],
        }
    }

    /// Builds the final FormulaMetaData structure
    ///
    /// This method consumes the MetaBuilder and creates the final metadata structure
    /// that contains all information about the parsed formula. It generates the
    /// `all_generated_columns` array ordered by variable ID and creates the complete
    /// variable-centric metadata structure.
    ///
    /// # Arguments
    ///
    /// * `self` - Consumes the MetaBuilder
    /// * `input` - The original formula string
    /// * `has_intercept` - Whether the model includes an intercept
    /// * `family` - The distribution family (if specified)
    ///
    /// # Returns
    ///
    /// A complete `FormulaMetaData` structure with all variable information
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fiasto::internal::meta_builder::MetaBuilder;
    ///
    /// let mut builder = MetaBuilder::new();
    /// builder.push_response("y");
    /// builder.push_plain_term("x");
    ///
    /// let metadata = builder.build("y ~ x", true, Some("gaussian".to_string()));
    /// // metadata contains complete variable-centric information
    /// ```
    pub fn build(
        self,
        input: &str,
        has_intercept: bool,
        family: Option<String>,
    ) -> crate::internal::data_structures::FormulaMetaData {
        // Generate all_generated_columns ordered by ID
        let mut all_generated_columns = Vec::new();
        let mut sorted_vars: Vec<_> = self.columns.values().collect();
        sorted_vars.sort_by_key(|v| v.id);

        for var in sorted_vars {
            all_generated_columns.extend(var.generated_columns.clone());
        }

        crate::internal::data_structures::FormulaMetaData {
            formula: input.to_string(),
            metadata: FormulaMetadataInfo {
                has_intercept,
                is_random_effects_model: self.is_random_effects_model,
                has_uncorrelated_slopes_and_intercepts: self.has_uncorrelated_slopes_and_intercepts,
                family,
            },
            columns: self.columns,
            all_generated_columns,
        }
    }
}
