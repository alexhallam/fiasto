use logos::Logos;

// ---------------------------
// LEXER
// ---------------------------

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[token("-")]
    Minus,
    #[token("0")]
    Zero,
    #[token("1")]
    One,
    #[regex(r"[2-9]\d*")]
    Integer,

    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*")]
    ColumnName,

    #[regex(r#""[^"]*""#)]
    StringLiteral,

    #[token("~")]
    Tilde,
    #[token("+")]
    Plus,

    #[token("|")]
    Pipe,
    #[token("||")]
    DoublePipe,

    #[token(":")]
    InteractionOnly,

    #[token("/")]
    Slash,

    #[token("*")]
    InteractionAndEffect,

    #[token("(")]
    FunctionStart,
    #[token(")")]
    FunctionEnd,

    // These transformations should all be followed by a "("
    #[token("poly")]
    Poly,
    #[token("offset")]
    Offset,
    #[token("factor")]
    Factor,
    // center and scale
    #[token("scale")]
    Scale,
    // standardize
    #[token("standardize")]
    Standardize,
    // center
    #[token("center")]
    Center,
    #[token("log")]
    Log,
    // B-splines
    #[token("bs")]
    BSplines,
    // Gaussian process
    #[token("gp")]
    GaussianProcess,
    // Monotonic
    #[token("mono")]
    Monotonic,
    // Measurement error
    #[token("me")]
    MeasurementError,
    // Missing values
    #[token("mi")]
    MissingValues,
    // foward Fill
    #[token("forward_fill")]
    ForwardFill,
    // backward Fill
    #[token("backward_fill")]
    BackwardFill,
    // diff
    #[token("diff")]
    Diff,
    // lag
    #[token("lag")]
    Lag,
    // lead
    #[token("lead")]
    Lead,
    // trunc
    #[token("trunc")]
    Trunc,
    // weights
    #[token("weights")]
    Weights,
    // trials
    #[token("trials")]
    Trials,
    // cens
    #[token("cens")]
    Censored,

    // Random effects functions
    #[token("gr")]
    Gr,
    #[token("mm")]
    Mm,
    #[token("mmc")]
    Mmc,
    #[token("cs")]
    Cs,

    //
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

    // gr() function arguments
    #[token("cor")]
    Cor,
    #[token("id")]
    Id,
    #[token("by")]
    By,
    #[token("cov")]
    Cov,
    #[token("dist")]
    Dist,
    #[token("true")]
    True,
    #[token("false")]
    False,
}
