use super::expression::Indent;
use std::fmt::{self, Debug, Display, Formatter, Result};

use super::{
    block::BlockNode,
    expression::{ExpressionNode, IndentDisplay},
};

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

impl IndentDisplay for FuncParam {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        write!(f, "{}, {}", self.name, self.param_type);

        Ok(())
    }
}
