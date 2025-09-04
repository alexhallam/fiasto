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
    Interaction { left: Box<Term>, right: Box<Term> },
    RandomEffect(RandomEffect),
}

#[derive(Debug, Clone)]
pub enum Argument {
    Ident(String),
    Integer(u32),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct RandomEffect {
    pub terms: Vec<RandomTerm>,
    pub grouping: Grouping,
    pub correlation: CorrelationType,
    pub correlation_id: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RandomTerm {
    Column(String),
    Function {
        name: String,
        args: Vec<Argument>,
    },
    Interaction {
        left: Box<RandomTerm>,
        right: Box<RandomTerm>,
    },
    SuppressIntercept,
}

#[derive(Debug, Clone)]
pub enum Grouping {
    Simple(String),
    Gr {
        group: String,
        options: Vec<GrOption>,
    },
    Mm {
        groups: Vec<String>,
    },
    Interaction {
        left: String,
        right: String,
    },
    Nested {
        outer: String,
        inner: String,
    },
}

#[derive(Debug, Clone)]
pub enum GrOption {
    Cor(bool),
    Id(String),
    By(Option<String>), // Can be NULL
    Cov(bool),          // Can be TRUE/FALSE
    Dist(String),
}

#[derive(Debug, Clone)]
pub enum CorrelationType {
    Correlated,
    Uncorrelated,
    CrossParameter(String),
}
