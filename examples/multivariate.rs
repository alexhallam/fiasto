use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Multivariate Model Examples ===\n");

    // Example 1: Basic multivariate response
    println!("1. Basic multivariate response: bind(y1, y2) ~ x");
    let formula1 = "bind(y1, y2) ~ x";
    let result1 = parse_formula(formula1)?;
    println!("   Input: {}", formula1);
    println!("   Output: {}", serde_json::to_string_pretty(&result1)?);
    println!();

    // Example 2: Multivariate response with multiple predictors
    println!("2. Multivariate response with multiple predictors: bind(y1, y2, y3) ~ x + z");
    let formula2 = "bind(y1, y2, y3) ~ x + z";
    let result2 = parse_formula(formula2)?;
    println!("   Input: {}", formula2);
    println!("   Output: {}", serde_json::to_string_pretty(&result2)?);
    println!();

    // Example 3: Multivariate response with family specification
    println!("3. Multivariate response with family: bind(y1, y2) ~ x, family = gaussian");
    let formula3 = "bind(y1, y2) ~ x, family = gaussian";
    let result3 = parse_formula(formula3)?;
    println!("   Input: {}", formula3);
    println!("   Output: {}", serde_json::to_string_pretty(&result3)?);
    println!();

    // Example 4: Multivariate response with complex terms
    println!("4. Multivariate response with complex terms: bind(y1, y2) ~ poly(x, 2) + (1 | group)");
    let formula4 = "bind(y1, y2) ~ poly(x, 2) + (1 | group)";
    let result4 = parse_formula(formula4)?;
    println!("   Input: {}", formula4);
    println!("   Output: {}", serde_json::to_string_pretty(&result4)?);
    println!();

    // Example 5: Show what happens with invalid syntax
    println!("5. Invalid syntax (should fail): bind(y1) ~ x");
    let formula5 = "bind(y1) ~ x";
    match parse_formula(formula5) {
        Ok(_) => println!("   Unexpected: This should have failed!"),
        Err(e) => println!("   Expected error: {}", e),
    }

    Ok(())
}
