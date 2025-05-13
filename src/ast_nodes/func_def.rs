use std::fmt;

use super::expression::ExpressionNode;

#[derive(Debug, Clone)]
pub struct FuncDefNode {
    pub name: String,
    pub params: Vec<FuncParam>,
}

#[derive(Debug, Clone)]
pub struct FuncParam {
    name: String,
    param_type: String,
}

impl fmt::Display for FuncDefNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "FuncDef(NAME)")
    }
}
