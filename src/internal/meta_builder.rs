use super::{
    ast::Argument,
    data_structures::{
        ColumnNameStruct, ColumnSuggestedNameStruct, FormulaMetaData, TransformationStruct,
    },
};
use std::collections::HashMap;

// ---------------------------
// META BUILDER
// ---------------------------

#[derive(Default)]
/// The MetaBuilder is responsible for building the formula metadata
/// The name_to_id is a map of the column names to their id. Useful for joins
/// The columns is a vector of the column names
/// The transformations is a vector of the transformations
/// The response_cols is a vector of the response columns
/// The fixed_cols is a vector of the fixed columns
/// The random_cols is a vector of the random columns
pub struct MetaBuilder {
    name_to_id: HashMap<String, u32>,
    columns: Vec<ColumnNameStruct>,
    transformations: Vec<TransformationStruct>,
    response_cols: Vec<ColumnSuggestedNameStruct>,
    fixed_cols: Vec<ColumnSuggestedNameStruct>,
    random_cols: Vec<ColumnSuggestedNameStruct>,
}

/// MetaBuilder does the following
/// `new()` instantance
/// ensure_col() - this function
impl MetaBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    // ensure_col(), this function will ...
    pub fn ensure_col(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.name_to_id.get(name) {
            return id;
        }
        let id = self.columns.len() as u32 + 1;
        self.columns.push(ColumnNameStruct {
            id,
            name: name.to_string(),
        });
        self.name_to_id.insert(name.to_string(), id);
        id
    }

    // This function will ...
    pub fn push_response(&mut self, name: &str) {
        let id = self.ensure_col(name);
        self.response_cols.push(ColumnSuggestedNameStruct {
            column_name_struct_id: id,
            name: name.to_string(),
        });
    }

    // this function pushes fixed cols to terms
    pub fn push_plain_term(&mut self, name: &str) {
        let id = self.ensure_col(name);
        self.fixed_cols.push(ColumnSuggestedNameStruct {
            column_name_struct_id: id,
            name: name.to_string(),
        });
    }

    // This function returns the transformation associated with a column name
    pub fn push_function_term(&mut self, fname: &str, args: &[Argument]) {
        let base_ident = args.iter().find_map(|a| match a {
            Argument::Ident(s) => Some(s.as_str()),
            _ => None,
        });

        let base_id = base_ident.map(|col| self.ensure_col(col)).unwrap_or(0);

        let arg_str = args
            .iter()
            .map(|a| match a {
                Argument::Ident(s) => s.clone(),
                Argument::Integer(n) => n.to_string(),
            })
            .collect::<Vec<_>>()
            .join(", ");
        let suggested = format!("{fname}({arg_str})");

        if base_id != 0 {
            self.transformations.push(TransformationStruct {
                column_name_struct_id: base_id,
                name: fname.to_string(),
            });
            self.fixed_cols.push(ColumnSuggestedNameStruct {
                column_name_struct_id: base_id,
                name: suggested,
            });
        } else {
            self.fixed_cols.push(ColumnSuggestedNameStruct {
                column_name_struct_id: 0,
                name: suggested,
            });
        }
    }

    pub fn build(self, input: &str, has_intercept: bool) -> FormulaMetaData {
        FormulaMetaData {
            transformations: self.transformations,
            column_names: self.columns,
            has_intercept,
            formula: input.to_string(),
            response_columns: self.response_cols,
            fix_effects_columns: self.fixed_cols,
            random_effects_columns: self.random_cols,
        }
    }
}
