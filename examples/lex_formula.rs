use fiasto::lex_formula;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = "mpg ~ cyl + wt*hp + poly(disp, 4) - 1";
    let tokens = lex_formula(input)?;
    println!("{}", serde_json::to_string_pretty(&tokens)?);
    Ok(())
}
