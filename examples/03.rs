use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "y ~ x + poly(x, 2) + poly(x1, 4) + log(x1) - 1, family = gaussian";

    println!("Testing public parse_formula function:");
    println!("Input: {}", input);

    let result = parse_formula(input)?;

    println!("FORMULA METADATA (as JSON):");
    println!("{}", result);
    println!("{}", serde_json::to_string_pretty(&result)?);

    println!("\n\n");

    Ok(())
}
