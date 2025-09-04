use fiasto::internal::lexer::Token;

fn main() {
    let input = "y ~ x + (0 + x | group)";
    
    println!("TOKENS for: {}", input);
    let mut lex = Token::lexer(input);
    while let Some(item) = lex.next() {
        match item {
            Ok(tok) => println!("{:?}: {}", tok, lex.slice()),
            Err(()) => println!("LEX ERROR at {:?}", lex.slice()),
        }
    }
}
