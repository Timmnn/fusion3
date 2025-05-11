use super::Integer;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ExpressionNode {
    pub kind: ExpressionKind,
}

#[derive(Debug, Clone)]
pub enum ExpressionKind {
    Addition(Integer, Integer),
    Subtraction(Integer, Integer),
    Integer(i32),
}

// Implement Display for ExpressionKind
impl fmt::Display for ExpressionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpressionKind::Addition(left, right) => write!(f, "Addition({}, {})", left, right),
            ExpressionKind::Subtraction(left, right) => {
                write!(f, "Subtraction({}, {})", left, right)
            }
            ExpressionKind::Integer(val) => write!(f, "Integer({})", val),
        }
    }
}

// Implement Display for ExpressionNode
impl fmt::Display for ExpressionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}
