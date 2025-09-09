use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = vec![
        ("y ~ 1", "Should pass - intercept-only model"),
        ("y ~ 0", "Should pass - no intercept model"),
        ("y ~ 1 - 1", "Should fail - invalid syntax"),
        ("y ~ 0 + 1", "Should fail - 0 is not a valid term"),
    ];
    
    for (formula, description) in test_cases {
        println!("Testing: {} - {}", formula, description);
        
        match parse_formula(formula) {
            Ok(result) => {
                let has_intercept = result.get("metadata")
                    .and_then(|m| m.get("has_intercept"))
                    .and_then(|h| h.as_bool())
                    .unwrap_or(false);
                
                println!("  ✅ Parsing successful! has_intercept: {}", has_intercept);
            }
            Err(e) => {
                println!("  ❌ Parsing failed: {}", e);
            }
        }
        println!();
    }
    
    Ok(())
}
