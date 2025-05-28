use super::expression::ExpressionNode;

#[derive(Debug, Clone)]
pub struct VarDeclNode {
    pub name: String,
    pub value: Box<ExpressionNode>,
}

#[derive(Debug, Clone)]
pub struct StructInitNode {}
