use serde::{Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Expression {
   pub lhs: String,
   pub rhs: crate::Node,
}