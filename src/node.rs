use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Node {
    String(String),
    RegexString(String),
    Terminal(String),
    Multiple(Vec<Node>),
    RegexExt(Box<Node>, RegexExtKind),
    Symbol(Box<Node>, SymbolKind, Box<Node>),
    Group(Box<Node>),
    Optional(Box<Node>),
    Repeat(Box<Node>),
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub enum RegexExtKind {
    Repeat0,
    Repeat1,
    Optional,
}

#[derive(Debug, Clone, Serialize)]
pub enum SymbolKind {
    Concatenation,
    Alternation,
}
