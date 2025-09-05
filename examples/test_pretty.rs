use fiasto::parse_formula;

fn main() {
    // Test different error scenarios
    let test_cases = vec![
        "y ~ x +",           // trailing plus
        "y ~ + x",           // leading plus  
        "~ x",               // missing response
        "y ~ x (",           // unmatched paren
        "y ~",               // incomplete formula
    ];
    
    for (i, formula) in test_cases.iter().enumerate() {
        println!("\n=== Test case {} ===", i + 1);
        println!("Formula: {}", formula);
        if let Err(_) = parse_formula(formula) {
            // Error is already printed by parse_formula via eprintln!
        }
    }
}
