use std::fmt;

use super::{block::BlockNode, expression::ExpressionNode};

#[derive(Debug, Clone)]
pub struct FuncDefNode {
    pub name: String,
    pub params: Vec<FuncParam>,
    pub body: BlockNode,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FuncParam {
    pub name: String,
    pub param_type: String,
}

impl fmt::Display for FuncDefNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "FuncDef(NAME)")
    }
}
