use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Intercept-Only Model Examples ===\n");

    // Example 1: Basic intercept-only model
    println!("1. Basic intercept-only model: y ~ 1");
    let formula1 = "y ~ 1";
    let result1 = parse_formula(formula1)?;
    println!("   Input: {}", formula1);
    println!("   Output: {}", serde_json::to_string_pretty(&result1)?);
    println!();

    // Example 2: Intercept-only model with family
    println!("2. Intercept-only model with family: y ~ 1, family = gaussian");
    let formula2 = "y ~ 1, family = gaussian";
    let result2 = parse_formula(formula2)?;
    println!("   Input: {}", formula2);
    println!("   Output: {}", serde_json::to_string_pretty(&result2)?);
    println!();

    // Example 3: No-intercept model (y ~ 0)
    println!("3. No-intercept model: y ~ 0");
    let formula3 = "y ~ 0";
    let result3 = parse_formula(formula3)?;
    println!("   Input: {}", formula3);
    println!("   Output: {}", serde_json::to_string_pretty(&result3)?);
    println!();

    // Example 4: Show what happens with invalid syntax
    println!("4. Invalid syntax (should fail): y ~ 1 - 1");
    let formula4 = "y ~ 1 - 1";
    match parse_formula(formula4) {
        Ok(_) => println!("   Unexpected: This should have failed!"),
        Err(e) => println!("   Expected error: {}", e),
    }
    println!();

    // Example 5: Show what happens with invalid zero combination
    println!("5. Invalid syntax (should fail): y ~ 0 + 1");
    let formula5 = "y ~ 0 + 1";
    match parse_formula(formula5) {
        Ok(_) => println!("   Unexpected: This should have failed!"),
        Err(e) => println!("   Expected error: {}", e),
    }

    Ok(())
}
