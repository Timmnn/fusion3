use super::INDENT;
use super::expression::Indent;
use super::{expression::IndentDisplay, term::VarDeclNode};
use std::fmt::{self, Display, Formatter, Result, write};

#[derive(Debug, Clone)]
pub struct VarAccessNode {
    pub name: String,
}

impl IndentDisplay for VarAccessNode {
    fn fmt_with_indent(&self, f: &mut Formatter<'_>, indent: Indent) -> Result {
        writeln!(f, "{}{}", indent.as_str(), self.name)
    }
}
