use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing Categorical Function c() in Fiasto");
    println!("==========================================");
    println!();

    // Test 1: Basic categorical function without reference level
    println!("=== Test 1: Basic categorical function ===");
    let formula1 = "y ~ c(treatment)";
    println!("Formula: {}", formula1);
    match parse_formula(formula1) {
        Ok(result) => {
            println!("✓ Parsed successfully!");
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            println!("✗ Error parsing formula: {}", e);
        }
    }
    println!();

    // Test 2: Categorical function with reference level (unquoted)
    println!("=== Test 2: Categorical with reference level (unquoted) ===");
    let formula2 = "y ~ c(treatment, ref=control)";
    println!("Formula: {}", formula2);
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

    // Test 3: Categorical function with reference level (quoted)
    println!("=== Test 3: Categorical with reference level (quoted) ===");
    let formula3 = r#"y ~ c(group, ref="group1")"#;
    println!("Formula: {}", formula3);
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

    // Test 4: Categorical function with other variables
    println!("=== Test 4: Categorical with other variables ===");
    let formula4 = "y ~ x1 + c(treatment, ref=control) + x2";
    println!("Formula: {}", formula4);
    match parse_formula(formula4) {
        Ok(result) => {
            println!("✓ Parsed successfully!");
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            println!("✗ Error parsing formula: {}", e);
        }
    }
    println!();

    // Test 5: Multiple categorical variables
    println!("=== Test 5: Multiple categorical variables ===");
    let formula5 = "y ~ c(treatment, ref=control) + c(group, ref=\"group1\")";
    println!("Formula: {}", formula5);
    match parse_formula(formula5) {
        Ok(result) => {
            println!("✓ Parsed successfully!");
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            println!("✗ Error parsing formula: {}", e);
        }
    }
    println!();

    println!("All tests completed!");
    Ok(())
}
