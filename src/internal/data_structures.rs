use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------
// DATA STRUCTURES
// ---------------------------

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VariableRole {
    Response,
    FixedEffect,
    RandomEffect,
    GroupingVariable,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transformation {
    pub function: String,
    pub parameters: serde_json::Value, // Flexible parameters object
    pub generates_columns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interaction {
    pub with: Vec<String>,
    pub order: u32,
    pub context: String,                   // "fixed_effects" or "random_effects"
    pub grouping_variable: Option<String>, // Only for random effects
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RandomEffectInfo {
    pub kind: String, // "intercept", "slope", "grouping"
    pub grouping_variable: String,
    pub has_intercept: bool,
    pub correlated: bool,
    pub includes_interactions: Vec<String>,
    pub variables: Option<Vec<String>>, // For grouping kind
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VariableInfo {
    pub id: u32,
    pub roles: Vec<VariableRole>,
    pub transformations: Vec<Transformation>,
    pub interactions: Vec<Interaction>,
    pub random_effects: Vec<RandomEffectInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormulaMetadataInfo {
    pub has_intercept: bool,
    pub is_random_effects_model: bool,
    pub has_uncorrelated_slopes_and_intercepts: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormulaMetaData {
    pub formula: String,
    pub metadata: FormulaMetadataInfo,
    pub columns: HashMap<String, VariableInfo>,
}

// Legacy structures for backward compatibility
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnNameStruct {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransformationStruct {
    pub column_name_struct_id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnSuggestedNameStruct {
    pub column_name_struct_id: u32,
    pub name: String,
}
