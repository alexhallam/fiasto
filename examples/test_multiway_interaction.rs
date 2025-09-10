use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Multi-way Interactions in Fiasto");
    println!("=========================================");
    println!();

    // Test 2-way interaction
    println!("=== Testing 2-way interaction ===");
    let formula2 = "y ~ x1*x2";
    println!("Formula: {}", formula2);
    println!("Expected: [x1, x2, x1_x2]");
    match parse_formula(formula2) {
        Ok(result) => {
            println!("✓ Parsed successfully!");
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            println!("✗ Error parsing formula: {}", e);
        }
    }
    println!();

    // Test 3-way interaction
    println!("=== Testing 3-way interaction ===");
    let formula3 = "y ~ x1*x2*x3";
    println!("Formula: {}", formula3);
    println!("Expected: [x1, x2, x3, x1_x2, x1_x3, x2_x3, x1_x2_x3]");
    match parse_formula(formula3) {
        Ok(result) => {
            println!("✓ Parsed successfully!");
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            println!("✗ Error parsing formula: {}", e);
        }
    }
    println!();

    // Test 4-way interaction
    println!("=== Testing 4-way interaction ===");
    let formula4 = "y ~ x1*x2*x3*x4";
    println!("Formula: {}", formula4);
    println!("Expected: [x1, x2, x3, x4, x1_x2, x1_x3, x1_x4, x2_x3, x2_x4, x3_x4, x1_x2_x3, x1_x2_x4, x1_x3_x4, x2_x3_x4, x1_x2_x3_x4]");
    match parse_formula(formula4) {
        Ok(result) => {
            println!("✓ Parsed successfully!");
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            println!("✗ Error parsing formula: {}", e);
        }
    }

    Ok(())
}
