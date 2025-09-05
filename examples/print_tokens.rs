use fiasto::internal::lexer::Token;
use logos::Logos;

fn main() {
    let input = "mpg ~ cyl + wt*hp + poly(disp, 4) - 1";
    let mut lexer = Token::lexer(input);
    while let Some(tok) = lexer.next() {
        println!("{:?}", tok);
    }
}
