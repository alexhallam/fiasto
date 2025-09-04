use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Advanced Random Effects Syntax Examples ===\n");

    // Basic Random Effects
    println!("1. Basic Random Effects:");

    // Random intercepts
    let basic_intercept = "y ~ x + (1 | group)";
    println!("   Random intercepts: {}", basic_intercept);
    match parse_formula(basic_intercept) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // Random slopes only
    let random_slopes = "y ~ x + (0 + x | group)";
    println!("   Random slopes only: {}", random_slopes);
    match parse_formula(random_slopes) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // Random intercepts + slopes (correlated)
    let correlated = "y ~ x + (x | group)";
    println!("   Random intercepts + slopes (correlated): {}", correlated);
    match parse_formula(correlated) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // Random intercepts + slopes (uncorrelated)
    let uncorrelated = "y ~ x + (x || group)";
    println!(
        "   Random intercepts + slopes (uncorrelated): {}",
        uncorrelated
    );
    match parse_formula(uncorrelated) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // // Enhanced grouping with gr() function
    // println!("2. Enhanced Grouping with gr() function:");

    // let gr_basic = "y ~ x + (1 | gr(group))";
    // println!("   Basic gr(): {}", gr_basic);
    // match parse_formula(gr_basic) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // let gr_cor_false = "y ~ x + (x | gr(group, cor = FALSE))";
    // println!("   gr() with cor = FALSE: {}", gr_cor_false);
    // match parse_formula(gr_cor_false) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // let gr_by = "y ~ x + (1 | gr(subject, by = treatment))";
    // println!("   gr() with by: {}", gr_by);
    // match parse_formula(gr_by) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // // Cross-parameter correlation
    // println!("3. Cross-Parameter Correlation:");

    // let cross_param = "y ~ x + (1 |2| group)";
    // println!("   Cross-parameter correlation: {}", cross_param);
    // match parse_formula(cross_param) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // let gr_id = "y ~ x + (1 | gr(group, id = \"2\"))";
    // println!("   gr() with id: {}", gr_id);
    // match parse_formula(gr_id) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // // Multi-membership random effects
    // println!("4. Multi-membership Random Effects:");

    // let mm_basic = "y ~ x + (1 | mm(group1, group2))";
    // println!("   Basic multi-membership: {}", mm_basic);
    // match parse_formula(mm_basic) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // let mm_varying = "y ~ x + (1 + mmc(x1, x2) | mm(group1, group2))";
    // println!(
    //     "   Multi-membership with varying covariates: {}",
    //     mm_varying
    // );
    // match parse_formula(mm_varying) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // // Category-specific random effects
    // println!("5. Category-specific Random Effects:");

    // let cs_intercept = "y ~ x + (cs(1) | group)";
    // println!("   Category-specific intercepts: {}", cs_intercept);
    // match parse_formula(cs_intercept) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // let cs_slopes = "y ~ x + (cs(x) | group)";
    // println!("   Category-specific slopes: {}", cs_slopes);
    // match parse_formula(cs_slopes) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // // Nested and crossed random effects
    // println!("6. Nested and Crossed Random Effects:");

    // let nested = "y ~ x + (1 | group1/group2)";
    // println!("   Nested random effects: {}", nested);
    // match parse_formula(nested) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // let crossed = "y ~ x + (1 | group1:group2)";
    // println!("   Crossed random effects: {}", crossed);
    // match parse_formula(crossed) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // let separate = "y ~ x + (1 | group1) + (1 | group2)";
    // println!("   Separate crossed random effects: {}", separate);
    // match parse_formula(separate) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // // Complex examples
    // println!("7. Complex Examples:");

    // let multiple_slopes = "y ~ x + (x + y + z | group)";
    // println!("   Multiple correlated random slopes: {}", multiple_slopes);
    // match parse_formula(multiple_slopes) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    let mixed_correlation = "y ~ x + (x | group) + (y || group)";
    println!("   Mixed correlation structures: {}", mixed_correlation);
    match parse_formula(mixed_correlation) {
        Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
        Err(e) => println!("   ✗ Error: {}", e),
    }
    println!();

    // let interaction_random = "y ~ x + (x*y | group)";
    // println!(
    //     "   Random effects with interactions: {}",
    //     interaction_random
    // );
    // match parse_formula(interaction_random) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // // Advanced gr() options
    // println!("8. Advanced gr() Options:");

    // let gr_cov = "y ~ x + (1 | gr(species, cov = phylo_matrix))";
    // println!("   gr() with custom covariance: {}", gr_cov);
    // match parse_formula(gr_cov) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    // let gr_dist = "y ~ x + (1 | gr(group, dist = \"student\"))";
    // println!("   gr() with non-normal distribution: {}", gr_dist);
    // match parse_formula(gr_dist) {
    //     Ok(result) => println!("   ✓ Success: {}", serde_json::to_string_pretty(&result)?),
    //     Err(e) => println!("   ✗ Error: {}", e),
    // }
    // println!();

    println!("=== All examples completed! ===");
    Ok(())
}
