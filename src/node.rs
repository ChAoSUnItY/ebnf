#[derive(Debug)]
pub enum Node {
    String(String),
    Int(i32),
    Symbol(String),
    Unknown
}