use super::{
    block::BlockNode,
    func_call::FuncCallNode,
    func_def::FuncDefNode,
    term::{StructInitNode, TermNode, VarDeclNode},
};
use colored::Colorize;
use std::fmt::{self, Display, Formatter, Result};

// Define a constant for indentation increment
const INDENT_SIZE: usize = 2;
// Use a dedicated struct to track indentation level
#[derive(Debug, Clone, Copy)]
pub struct Indent(pub usize);

impl Indent {
    pub fn new() -> Self {
        Indent(0)
    }

    pub fn increment(&self) -> Self {
        Indent(self.0 + INDENT_SIZE)
    }

    // Helper to get indent as string
    pub fn as_str(&self) -> String {
        " ".repeat(self.0)
    }
}

// Helper trait to display AST nodes with indentation
pub trait IndentDisplay {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result;
}

#[derive(Debug, Clone)]
pub struct ExpressionNode {
    pub kind: ExpressionKind,
}

impl IndentDisplay for ExpressionNode {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        writeln!(
            f,
            "{}{}",
            indent.as_str(),
            "Expression".on_color("#fc627d").black()
        )?;
        self.kind.fmt_with_indent(f, indent.increment())
    }
}

impl Display for ExpressionNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.fmt_with_indent(f, Indent::new())
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    VarDecl(VarDeclNode),
    AddExpr(AddExprNode),
}

impl IndentDisplay for ExpressionKind {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        match self {
            ExpressionKind::AddExpr(node) => {
                writeln!(
                    f,
                    "{}{}",
                    indent.as_str(),
                    "AddExpr".on_truecolor(100, 149, 237).black()
                )?;
                node.fmt_with_indent(f, indent.increment())
            }
            ExpressionKind::VarDecl(node) => {
                writeln!(
                    f,
                    "{}{}",
                    indent.as_str(),
                    "VarDecl".on_color("#32CD32").black()
                )?;
                node.fmt_with_indent(f, indent.increment())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddExprNode {
    pub left: MulExprNode,
    pub addent: Vec<AddExprPart>,
}

impl IndentDisplay for AddExprNode {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        // Display left part
        writeln!(
            f,
            "{}{}:",
            indent.as_str(),
            "Left".black().on_truecolor(200, 177, 54)
        )?;
        self.left.fmt_with_indent(f, indent.increment())?;

        // Display addents if any
        if !self.addent.is_empty() {
            writeln!(f, "{}Addents:", indent.as_str())?;
            let inner_indent = indent.increment();
            for (i, part) in self.addent.iter().enumerate() {
                writeln!(f, "{}[{}]:", inner_indent.as_str(), i)?;
                part.fmt_with_indent(f, inner_indent.increment())?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct MulExprPart {
    pub op: MulOp,
    pub value: MulExprNode,
}

impl IndentDisplay for MulExprPart {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        writeln!(
            f,
            "{}{}: {}",
            indent.as_str(),
            "Operator".black().on_truecolor(44, 78, 211),
            self.op
        )?;
        writeln!(f, "{}Value:", indent.as_str())?;
        self.value.fmt_with_indent(f, indent.increment())
    }
}

#[derive(Debug, Clone)]
pub struct AddExprPart {
    pub op: AddOp,
    pub value: AddExprNode,
}

impl IndentDisplay for AddExprPart {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        writeln!(
            f,
            "{}{}({})",
            indent.as_str(),
            "Operator".black().on_truecolor(44, 78, 211),
            self.op
        )?;
        writeln!(f, "{}Value:", indent.as_str())?;
        self.value.fmt_with_indent(f, indent.increment())
    }
}

#[derive(Debug, Clone)]
pub struct MulExprNode {
    pub left: PrimaryNode,
    pub factor: Vec<MulExprPart>,
}

impl IndentDisplay for MulExprNode {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        writeln!(
            f,
            "{}{}",
            indent.as_str(),
            "MulExpr".on_truecolor(255, 165, 0).black()
        )?;

        let inner_indent = indent.increment();
        writeln!(
            f,
            "{}{}:",
            inner_indent.as_str(),
            "Left".black().on_truecolor(200, 177, 54)
        )?;
        self.left.fmt_with_indent(f, inner_indent.increment())?;

        if !self.factor.is_empty() {
            writeln!(f, "{}Factors:", inner_indent.as_str())?;
            let factors_indent = inner_indent.increment();
            for (i, factor) in self.factor.iter().enumerate() {
                writeln!(f, "{}[{}]:", factors_indent.as_str(), i)?;
                factor.fmt_with_indent(f, factors_indent.increment())?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PrimaryNode {
    pub kind: PrimaryKind,
}

impl IndentDisplay for PrimaryNode {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        writeln!(
            f,
            "{}{}",
            indent.as_str(),
            "Primary".on_truecolor(147, 112, 219).black()
        )?;

        let inner_indent = indent.increment();
        match &self.kind {
            PrimaryKind::IntLit(val) => {
                writeln!(
                    f,
                    "{}{}({})",
                    inner_indent.as_str(),
                    "IntLiteral".black().on_truecolor(200, 85, 85),
                    val
                )
            }
            PrimaryKind::FloatLit(val) => {
                writeln!(f, "{}FloatLiteral({})", inner_indent.as_str(), val)
            }
            PrimaryKind::StrLit(val) => {
                writeln!(f, "{}StringLiteral(\"{}\")", inner_indent.as_str(), val)
            }
            PrimaryKind::FuncCall(node) => {
                writeln!(f, "{}FunctionCall:", inner_indent.as_str())?;
                node.fmt_with_indent(f, inner_indent.increment())
            }
            PrimaryKind::StructInit(node) => {
                writeln!(f, "{}StructInit:", inner_indent.as_str())?;
                node.fmt_with_indent(f, inner_indent.increment())
            }
            PrimaryKind::Block(node) => {
                writeln!(f, "{}Block:", inner_indent.as_str())?;
                node.fmt_with_indent(f, inner_indent.increment())
            }
            PrimaryKind::FuncDef(node) => {
                writeln!(f, "{}FunctionDef:", inner_indent.as_str())?;
                node.fmt_with_indent(f, inner_indent.increment())
            }
            PrimaryKind::Expression(expr) => {
                writeln!(f, "{}Expression:", inner_indent.as_str())?;
                expr.fmt_with_indent(f, inner_indent.increment())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum PrimaryKind {
    IntLit(i32),
    FloatLit(f32),
    StrLit(String),
    FuncCall(FuncCallNode),
    StructInit(StructInitNode),
    Block(BlockNode),
    FuncDef(FuncDefNode),
    Expression(Box<ExpressionNode>),
}

#[derive(Debug, Clone)]
pub enum MulOp {
    Multiply,
    Divide,
}

impl Display for MulOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MulOp::Divide => write!(f, "/"),
            MulOp::Multiply => write!(f, "*"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AddOp {
    Add,
    Subtract, // Fixed typo in the enum variant name
}

impl Display for AddOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AddOp::Add => write!(f, "+"),
            AddOp::Subtract => write!(f, "-"),
        }
    }
}

// You would need to implement IndentDisplay for other node types as well
// For example:

impl IndentDisplay for VarDeclNode {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        // Implementation would depend on the actual structure of VarDeclNode
        writeln!(f, "{}Name: {}", indent.as_str(), self.name)?;
        writeln!(f, "{}Value:", indent.as_str())?;
        self.value.fmt_with_indent(f, indent.increment())
    }
}

// Stub implementations for the other types
// You would need to replace these with actual implementations

impl IndentDisplay for FuncCallNode {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        writeln!(f, "{}FuncCall", indent.as_str())
    }
}

impl IndentDisplay for StructInitNode {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        writeln!(f, "{}StructInit", indent.as_str())
    }
}

impl IndentDisplay for BlockNode {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        writeln!(f, "{}Block", indent.as_str())
    }
}

impl IndentDisplay for FuncDefNode {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        writeln!(f, "{}FuncDef", indent.as_str())
    }
}
