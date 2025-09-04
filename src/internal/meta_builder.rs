use super::{
    ast::{Argument, Grouping, RandomEffect, RandomTerm},
    data_structures::{
        FormulaMetadataInfo, Interaction, RandomEffectInfo, Transformation, VariableInfo,
        VariableRole,
    },
};
use std::collections::HashMap;

// ---------------------------
// META BUILDER
// ---------------------------

#[derive(Default)]
/// The MetaBuilder is responsible for building the formula metadata
/// Uses a variable-centric approach where each variable is tracked with its roles,
/// transformations, interactions, and random effects
pub struct MetaBuilder {
    name_to_id: HashMap<String, u32>,
    columns: HashMap<String, VariableInfo>,
    has_intercept: bool,
    has_uncorrelated_slopes_and_intercepts: bool,
    is_random_effects_model: bool,
    next_id: u32,
}

impl MetaBuilder {
    pub fn new() -> Self {
        Self {
            name_to_id: HashMap::new(),
            columns: HashMap::new(),
            has_intercept: true,
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
            // Update generated columns with the transformation's generated columns
            var_info.generated_columns = transformation.generates_columns;
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

    /// Adds a fixed effect variable
    pub fn push_plain_term(&mut self, name: &str) {
        self.ensure_variable(name);
        self.add_role(name, VariableRole::FixedEffect);
    }

    /// Adds an interaction term
    pub fn push_interaction(
        &mut self,
        left: &crate::internal::ast::Term,
        right: &crate::internal::ast::Term,
    ) {
        // Extract variable names from the interaction terms
        let left_name = self.extract_variable_name(left);
        let right_name = self.extract_variable_name(right);

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
    fn extract_variable_name(&self, term: &crate::internal::ast::Term) -> Option<String> {
        match term {
            crate::internal::ast::Term::Column(name) => Some(name.clone()),
            crate::internal::ast::Term::Function { name, args } => {
                // For functions, extract the first argument if it's an identifier
                args.iter().find_map(|arg| match arg {
                    Argument::Ident(s) => Some(s.clone()),
                    _ => None,
                })
            }
            crate::internal::ast::Term::Interaction { left, right } => {
                // For nested interactions, we'll use the left side for now
                self.extract_variable_name(left)
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

    /// Builds the final FormulaMetaData
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
