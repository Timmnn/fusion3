pub mod expression;
pub mod func_call;
pub mod statement;

use expression::*;
use func_call::*;
use statement::*;

use std::fmt::{self, write};

// Type alias for integers
pub type Integer = i32;

// ProgramNode struct to represent a program (sequence of statements)
#[derive(Debug)]
pub struct ProgramNode {
    pub statements: Vec<StatementNode>,
}

// AstNode enum to represent AST nodes
#[derive(Debug)]
pub enum AstNode {
    Statement(Box<StatementNode>),
    Program(Box<ProgramNode>),
    Integer(Integer),
    Expression(Box<ExpressionNode>),
}

// Helper function to generate indentation
fn indent(level: usize) -> String {
    "  ".repeat(level)
}

// Implement Display for ProgramNode
impl fmt::Display for ProgramNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Program")?;
        for stmt in &self.statements {
            writeln!(f, "  {}", stmt)?;
        }
        Ok(())
    }
}

// Implement Display for AstNode
impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Statement(stmt) => write!(f, "{}", stmt),
            AstNode::Program(prog) => write!(f, "{}", prog),
            AstNode::Integer(val) => write!(f, "{}", val),
            AstNode::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockNode {
    pub statements: Vec<StatementNode>,
}

impl fmt::Display for BlockNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Block({:?})", self.statements)
    }
}
