use alloc::vec::Vec;

use serde::Serialize;

use crate::expression::Expression;

#[derive(Debug, Clone, Serialize)]
pub struct Grammar {
    pub expressions: Vec<Expression>,
}
