use fiasto::parse_formula;

fn main() {
    // Test case from issue #4: y ~ x1 + poly(x1,2)
    let formula = "y ~ x1 + poly(x1,2)";
    println!("Testing formula: {}", formula);

    match parse_formula(formula) {
        Ok(metadata) => {
            println!("{}", serde_json::to_string_pretty(&metadata).unwrap());

            // Check if x1 has both identity and poly roles
            if let Some(columns) = metadata.get("columns") {
                if let Some(x1_info) = columns.get("x1") {
                    if let Some(roles) = x1_info.get("roles") {
                        println!("\nCurrent roles for x1: {:?}", roles);

                        // Check if x1 has identity role
                        let has_identity = roles
                            .as_array()
                            .map(|arr| arr.iter().any(|role| role.as_str() == Some("Identity")))
                            .unwrap_or(false);

                        if has_identity {
                            println!("✓ x1 has Identity role");
                        } else {
                            println!("✗ x1 is missing Identity role");
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
