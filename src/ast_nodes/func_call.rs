use std::fmt;

#[derive(Debug, Clone)]
pub struct FuncCallNode {
    pub name: String,
}

impl fmt::Display for FuncCallNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "FuncCall(NAME)")
    }
}
