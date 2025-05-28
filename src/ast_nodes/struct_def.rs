use super::expression::Indent;
use super::{
    block::BlockNode,
    expression::{ExpressionNode, IndentDisplay},
};
use std::fmt::{self, Debug, Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct StructDefNode {
    pub name: String,
    pub fields: Vec<StructFieldNode>,
}

#[derive(Debug, Clone)]
pub struct StructFieldNode {
    pub name: String,
    pub type_name: String,
}

impl IndentDisplay for StructDefNode {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        writeln!(f, "{}{}", indent.as_str(), self.name)
    }
}
