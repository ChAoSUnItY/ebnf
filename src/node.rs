#[derive(Debug, Clone)]
pub enum Node {
    String(String),
    Int(i32),
    Terminal(String),
    Multiple(Vec<Node>),
    Symbol(Box<Node>, SymbolKind, Box<Node>),
    Group(Box<Node>),
    Unknown,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Concatenation,
    Alternation,
    Exception,
    Repetition
}