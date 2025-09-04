use serde::{Deserialize, Serialize};

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
pub struct ColumnNameStruct {
    pub id: u32,
    pub name: String,
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
pub struct TransformationStruct {
    pub column_name_struct_id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ColumnSuggestedNameStruct {
    pub column_name_struct_id: u32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormulaMetaData {
    pub transformations: Vec<TransformationStruct>,
    pub column_names: Vec<ColumnNameStruct>,
    pub has_intercept: bool,
    pub has_uncorrelated_slopes_and_intercepts: bool,
    pub formula: String,
    pub response_columns: Vec<ColumnSuggestedNameStruct>,
    pub fix_effects_columns: Vec<ColumnSuggestedNameStruct>,
    pub random_effects_columns: Vec<ColumnSuggestedNameStruct>,
}
