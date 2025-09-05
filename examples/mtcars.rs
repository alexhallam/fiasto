use fiasto::parse_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "mpg ~ cyl + wt*hp + poly(disp, 4) - 1";
    let result = parse_formula(input)?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}



