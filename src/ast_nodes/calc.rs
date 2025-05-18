pub struct CalculationNode {
    pub kind: CalculationKind,
}

pub enum CalculationKind {
    Addition(AdditionNode),
    Subtraction(SubtractionNode),
    Multiplication(MultiplicationNode),
    Division(DivisionNode),
}

pub struct AdditionNode {
    pub left: Addent,
    pubright: Addent,
}

pub struct SubtractionNode {
    pub left: Addent,
    pub right: Addent,
}

pub enum Addent {
    Int(i32),
    Float(f32),
    Addition(Box<AdditionNode>),
    Multiplication(MultiplicationNode),
}

pub enum Factor {
    Int(i32),
    Float(f32),
    Multiplication(Box<MultiplicationNode>),
}

pub struct MultiplicationNode {
    pub left: Factor,
    pub right: Factor,
}

pub struct DivisionNode {
    pub divident: Factor,
    pub divisor: Factor,
}
