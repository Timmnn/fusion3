#[derive(Debug, Clone)]
pub struct OperationNode {
    pub kind: OperationKind,
}

#[derive(Debug, Clone)]
pub enum OperationKind {
    AdditiveExpr,
}
