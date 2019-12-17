#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub value: String,
}

impl Identifier {
    pub fn new(value: String) -> Identifier {
        Identifier { value }
    }
}

#[derive(Debug)]
pub enum Expr {
    Literal(Value),
    Identifier(Identifier),
    Assignment(Identifier, Box<Expr>),
    LetAssignment(Identifier, Box<Expr>),
    ConstAssignment(Identifier, Box<Expr>),
    BinaryOp(Box<Expr>, Operator, Box<Expr>),
    Function(Vec<Identifier>, Vec<Expr>),
    CallFunction(Identifier, Vec<Expr>),
}

#[derive(Debug)]
pub enum Value {
    Int(i32),
    Float(f64),
    Bool(bool),
    String(String),
}
