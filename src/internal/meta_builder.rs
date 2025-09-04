use super::{
    ast::{Argument, Grouping, RandomEffect, RandomTerm},
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
                Argument::String(s) => format!("\"{}\"", s),
                Argument::Boolean(b) => b.to_string(),
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

    // This function handles random effects
    pub fn push_random_effect(&mut self, random_effect: &RandomEffect) {
        // Process each term in the random effect
        for term in &random_effect.terms {
            match term {
                RandomTerm::Column(name) => {
                    if name != "1" {
                        let id = self.ensure_col(name);
                        self.random_cols.push(ColumnSuggestedNameStruct {
                            column_name_struct_id: id,
                            name: name.clone(),
                        });
                    }
                }
                RandomTerm::Function { name, args } => {
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
                            Argument::String(s) => format!("\"{}\"", s),
                            Argument::Boolean(b) => b.to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    let suggested = format!("{name}({arg_str})");

                    if base_id != 0 {
                        self.transformations.push(TransformationStruct {
                            column_name_struct_id: base_id,
                            name: name.clone(),
                        });
                        self.random_cols.push(ColumnSuggestedNameStruct {
                            column_name_struct_id: base_id,
                            name: suggested,
                        });
                    } else {
                        self.random_cols.push(ColumnSuggestedNameStruct {
                            column_name_struct_id: 0,
                            name: suggested,
                        });
                    }
                }
                RandomTerm::Interaction { left, right } => {
                    // Handle interactions in random effects
                    let left_name = match left.as_ref() {
                        RandomTerm::Column(name) => name.clone(),
                        _ => "interaction".to_string(),
                    };
                    let right_name = match right.as_ref() {
                        RandomTerm::Column(name) => name.clone(),
                        _ => "interaction".to_string(),
                    };
                    let interaction_name = format!("{}:{}", left_name, right_name);
                    let id = self.ensure_col(&interaction_name);
                    self.random_cols.push(ColumnSuggestedNameStruct {
                        column_name_struct_id: id,
                        name: interaction_name,
                    });
                }
                RandomTerm::SuppressIntercept => {
                    // Intercept suppression - no column to add
                }
            }
        }

        // Process grouping variables - these represent the random effect structure
        match &random_effect.grouping {
            Grouping::Simple(group) => {
                let id = self.ensure_col(group);
                self.random_cols.push(ColumnSuggestedNameStruct {
                    column_name_struct_id: id,
                    name: group.clone(),
                });
            }
            Grouping::Gr { group, .. } => {
                let id = self.ensure_col(group);
                self.random_cols.push(ColumnSuggestedNameStruct {
                    column_name_struct_id: id,
                    name: group.clone(),
                });
            }
            Grouping::Mm { groups } => {
                for group in groups {
                    let id = self.ensure_col(group);
                    self.random_cols.push(ColumnSuggestedNameStruct {
                        column_name_struct_id: id,
                        name: group.clone(),
                    });
                }
            }
            Grouping::Interaction { left, right } => {
                let left_id = self.ensure_col(left);
                let right_id = self.ensure_col(right);
                self.random_cols.push(ColumnSuggestedNameStruct {
                    column_name_struct_id: left_id,
                    name: left.clone(),
                });
                self.random_cols.push(ColumnSuggestedNameStruct {
                    column_name_struct_id: right_id,
                    name: right.clone(),
                });
            }
            Grouping::Nested { outer, inner } => {
                let outer_id = self.ensure_col(outer);
                let inner_id = self.ensure_col(inner);
                self.random_cols.push(ColumnSuggestedNameStruct {
                    column_name_struct_id: outer_id,
                    name: outer.clone(),
                });
                self.random_cols.push(ColumnSuggestedNameStruct {
                    column_name_struct_id: inner_id,
                    name: inner.clone(),
                });
            }
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
