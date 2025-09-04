use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Testing New Improvements ===\n");

    // Test with family information
    let formula_with_family = "y ~ x + poly(x, 2), family = gaussian";
    println!("Formula with family: {}", formula_with_family);
    match parse_formula(formula_with_family) {
        Ok(result) => println!("✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();

    // Test simple interaction
    let simple_interaction = "y ~ x:z";
    println!("Simple interaction: {}", simple_interaction);
    match parse_formula(simple_interaction) {
        Ok(result) => println!("✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();

    println!("=== All tests completed! ===");
    Ok(())
}
