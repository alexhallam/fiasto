use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "y ~ x + poly(x, 2) + poly(x1, 4) - 1, family = gaussian";

    println!("Testing public parse_formula function:");
    println!("Input: {}", input);
    println!();

    let result = parse_formula(input)?;

    println!("FORMULA METADATA (as JSON):");
    println!("{}", result);
    println!("{}", serde_json::to_string_pretty(&result)?);

    println!("\n\n");

    // Test with a simpler formula
    let simple_input = "y ~ x + z";
    println!("Testing with simpler formula:");
    println!("Input: {}", simple_input);
    println!();

    let simple_result = parse_formula(simple_input)?;
    println!("FORMULA METADATA (as JSON):");
    println!("{}", serde_json::to_string_pretty(&simple_result)?);

    Ok(())
}
