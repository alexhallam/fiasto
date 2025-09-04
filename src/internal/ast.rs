// ---------------------------
// SIMPLE AST
// ---------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum Family {
    Gaussian,
    Binomial,
    Poisson,
}

#[derive(Debug, Clone)]
pub enum Term {
    Column(String),
    Function { name: String, args: Vec<Argument> },
}

#[derive(Debug, Clone)]
pub enum Argument {
    Ident(String),
    Integer(u32),
}
