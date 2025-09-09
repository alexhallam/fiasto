use fiasto::internal::parser::Parser;

// This example intentionally uses a malformed formula so we can see the pretty error output.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // trailing '+' will cause a parse error
    let input = "y ~ x +";
    let mut parser = Parser::new(input)?;
    match parser.parse_formula() {
        Ok(_) => println!("parsed ok (unexpected)"),
        Err(e) => {
            // Print the colored, pretty error to stderr
            eprintln!("{}", parser.pretty_error(&e));
        }
    }
    Ok(())
}
