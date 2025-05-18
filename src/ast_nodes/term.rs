use super::{expression::ExpressionNode, operation::OperationNode};
use crate::ast_nodes::INDENT;
use std::fmt::{self, Pointer, write};

use super::{block::BlockNode, func_call::FuncCallNode, func_def::FuncDefNode};

#[derive(Debug, Clone)]
pub struct TermNode {
    pub kind: TermKind,
}

#[derive(Debug, Clone)]
pub enum TermKind {
    FuncDef(FuncDefNode),
    Block(BlockNode),
    Operation(OperationNode),
    IntLit(i32),
    StrLit(String),
    FloatLit(f32),
    FuncCall(FuncCallNode),
    StructInit(StructInitNode),
    VarDecl(VarDeclNode),
    Assignment(AssignmentNode),
}

impl fmt::Display for TermKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TermKind::FuncDef(n) => n.fmt(f),
            _ => write!(f, "Not implemented"),
        }
    }
}

// Implement Display for ExpressionNode
impl fmt::Display for TermNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "TermNode").unwrap();
        write!(f, "{}", INDENT).unwrap();
        self.kind.fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct AssignmentNode {}

#[derive(Debug, Clone)]
pub struct VarDeclNode {
    pub name: String,
    pub value: Box<ExpressionNode>,
}

#[derive(Debug, Clone)]
pub struct StructInitNode {}
