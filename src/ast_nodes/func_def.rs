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
