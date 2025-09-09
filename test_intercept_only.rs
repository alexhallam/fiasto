use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let formula = "y ~ 1";

    println!("Testing intercept-only model: {}", formula);

    match parse_formula(formula) {
        Ok(result) => {
            println!("✅ Parsing successful!");
            println!("Result: {}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            println!("❌ Parsing failed: {}", e);
        }
    }

    Ok(())
}
