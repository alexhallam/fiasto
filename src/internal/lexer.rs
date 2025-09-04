use logos::Logos;

// ---------------------------
// LEXER
// ---------------------------

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("-")]
    Minus,
    #[token("1")]
    One,
    #[regex(r"[2-9]\d*")]
    Integer,

    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*")]
    ColumnName,

    #[token("~")]
    Tilde,
    #[token("+")]
    Plus,

    #[token("(")]
    FunctionStart,
    #[token(")")]
    FunctionEnd,

    #[token("poly")]
    Poly,

    #[token(",")]
    Comma,
    #[token("=")]
    Equal,

    #[token("family")]
    Family,

    #[token("gaussian")]
    Gaussian,
    #[token("binomial")]
    Binomial,
    #[token("poisson")]
    Poisson,
}
