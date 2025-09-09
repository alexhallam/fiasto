use fiasto::parse_formula;

fn main() {
    println!("=== Testing Issue #6: Intercept and Formula Order ===\n");

    // Test the exact example from issue #6: y ~ x + poly(x, 2) + log(z)
    let formula = "y ~ x + poly(x, 2) + log(z)";
    println!("Testing formula: {}", formula);

    match parse_formula(formula) {
        Ok(metadata) => {
            println!("{}", serde_json::to_string_pretty(&metadata).unwrap());

            // Check if intercept is in all_generated_columns
            if let Some(all_columns) = metadata.get("all_generated_columns") {
                if let Some(columns_array) = all_columns.as_array() {
                    let has_intercept = columns_array
                        .iter()
                        .any(|col| col.as_str() == Some("intercept"));
                    if has_intercept {
                        println!("\n✓ Intercept is present in all_generated_columns");
                    } else {
                        println!("\n✗ Intercept is missing from all_generated_columns");
                    }
                }
            }

            // Check the formula order mapping
            if let Some(formula_order) = metadata.get("all_generated_columns_formula_order") {
                println!("\nFormula order mapping:");
                if let Some(order_map) = formula_order.as_object() {
                    for (key, value) in order_map {
                        println!("  {}: {}", key, value.as_str().unwrap_or(""));
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    println!("\n" + "=".repeat(50).as_str());

    // Test without intercept: y ~ x + poly(x, 2) + log(z) - 1
    let formula_no_intercept = "y ~ x + poly(x, 2) + log(z) - 1";
    println!(
        "\nTesting formula without intercept: {}",
        formula_no_intercept
    );

    match parse_formula(formula_no_intercept) {
        Ok(metadata) => {
            // Check if intercept is NOT in all_generated_columns
            if let Some(all_columns) = metadata.get("all_generated_columns") {
                if let Some(columns_array) = all_columns.as_array() {
                    let has_intercept = columns_array
                        .iter()
                        .any(|col| col.as_str() == Some("intercept"));
                    if has_intercept {
                        println!("✗ Intercept should NOT be present when has_intercept is false");
                    } else {
                        println!("✓ Intercept correctly absent when has_intercept is false");
                    }
                }
            }

            // Check the formula order mapping (should not have intercept)
            if let Some(formula_order) = metadata.get("all_generated_columns_formula_order") {
                println!("\nFormula order mapping (no intercept):");
                if let Some(order_map) = formula_order.as_object() {
                    for (key, value) in order_map {
                        println!("  {}: {}", key, value.as_str().unwrap_or(""));
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
