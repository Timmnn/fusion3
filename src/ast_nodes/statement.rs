use std::fmt;

use super::{BlockNode, expression::ExpressionNode};

// StatementKind enum to specify different types of statements
#[derive(Debug, Clone)]
pub enum StatementKind {
    Expr(Box<ExpressionNode>),  // Expression statement
    Block(BlockNode),           // Block of statements
    FuncDef(String, BlockNode), // Function definition: name and body (block)
    CImport(String),
}

// StatementNode struct to represent a statement in the AST
#[derive(Debug, Clone)]
pub struct StatementNode {
    pub kind: StatementKind,
}

// Implement Display for StatementNode
impl fmt::Display for StatementNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

// Implement Display for StatementKind
impl fmt::Display for StatementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatementKind::Expr(expr) => write!(f, "Expr{}", expr),
            StatementKind::Block(block) => {
                writeln!(f, "Block(")?;
                for stmt in &block.statements {
                    writeln!(f, "  {}", stmt)?;
                }
                write!(f, ")")
            }
            StatementKind::FuncDef(name, body) => {
                writeln!(f, "FuncDef({})", name)?;
                writeln!(f, "    {}", body)?;
                write!(f, "")
            }
            StatementKind::CImport(lib) => {
                write!(f, "c_import_lib")
            }
        }
    }
}
