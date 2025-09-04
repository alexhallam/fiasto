use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Variable-Centric Metadata Examples ===\n");

    // Test the example from the user's request
    let complex_formula = "y ~ poly(x, 3) + x:z + (x*z | group) + (0 + y | site)";
    println!("Complex formula: {}", complex_formula);
    match parse_formula(complex_formula) {
        Ok(result) => println!("✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();

    // Test a simpler example
    let simple_formula = "y ~ x + (1 | group)";
    println!("Simple formula: {}", simple_formula);
    match parse_formula(simple_formula) {
        Ok(result) => println!("✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();

    // Test with transformations
    let transform_formula = "y ~ x + poly(x, 2) + log(z)";
    println!("With transformations: {}", transform_formula);
    match parse_formula(transform_formula) {
        Ok(result) => println!("✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("✗ Error: {}", e),
    }
    println!();

    println!("=== All examples completed! ===");
    Ok(())
}
