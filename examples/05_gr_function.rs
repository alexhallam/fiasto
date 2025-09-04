use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Enhanced gr() Function Examples ===\n");

    // Basic gr() usage
    println!("1. Basic gr() usage:");

    let basic_gr = "y ~ x + (1 | gr(group))";
    println!("   Basic gr(): {}", basic_gr);
    match parse_formula(basic_gr) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // gr() with cov = FALSE (uncorrelated)
    println!("2. gr() with cov = FALSE (uncorrelated):");

    let gr_cov_false = "y ~ x + (x | gr(group, cov = FALSE))";
    println!("   gr() with cov = FALSE: {}", gr_cov_false);
    match parse_formula(gr_cov_false) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // gr() with cov = TRUE (correlated, default)
    println!("3. gr() with cov = TRUE (correlated):");

    let gr_cov_true = "y ~ x + (x | gr(group, cov = TRUE))";
    println!("   gr() with cov = TRUE: {}", gr_cov_true);
    match parse_formula(gr_cov_true) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // gr() with by = NULL
    println!("4. gr() with by = NULL:");

    let gr_by_null = "y ~ x + (1 | gr(group, by = NULL))";
    println!("   gr() with by = NULL: {}", gr_by_null);
    match parse_formula(gr_by_null) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // gr() with by = variable
    println!("5. gr() with by = variable:");

    let gr_by_var = "y ~ x + (1 | gr(subject, by = treatment))";
    println!("   gr() with by = variable: {}", gr_by_var);
    match parse_formula(gr_by_var) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // gr() with multiple options
    println!("6. gr() with multiple options:");

    let gr_multiple = "y ~ x + (x | gr(group, by = treatment, cov = FALSE))";
    println!("   gr() with multiple options: {}", gr_multiple);
    match parse_formula(gr_multiple) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // gr() with id for cross-parameter correlation
    println!("7. gr() with id for cross-parameter correlation:");

    let gr_id = "y ~ x + (1 | gr(group, id = 2))";
    println!("   gr() with id: {}", gr_id);
    match parse_formula(gr_id) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // gr() with dist for non-normal distributions
    println!("8. gr() with dist for non-normal distributions:");

    let gr_dist = "y ~ x + (1 | gr(group, dist = student()))";
    println!("   gr() with dist: {}", gr_dist);
    match parse_formula(gr_dist) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // Complex gr() with all options
    println!("9. Complex gr() with all options:");

    let gr_complex =
        "y ~ x + (x | gr(subject, by = treatment, cov = FALSE, id = 1, dist = student()))";
    println!("   Complex gr(): {}", gr_complex);
    match parse_formula(gr_complex) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    println!("=== All gr() examples completed! ===");
    Ok(())
}
