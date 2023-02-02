use crate::expression::Expression;
use serde::{Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Grammar {
    pub expressions: Vec<Expression>
}
