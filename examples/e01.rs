use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "y ~ x +++";

    let result = parse_formula(input)?;
    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}