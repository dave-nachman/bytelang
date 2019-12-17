use std::collections::HashMap;

pub type Heap = Vec<Value>;
pub type SymbolTable = HashMap<String, (usize, i8)>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f64),
    Bool(bool),
    Pointer(u8),
    Bytes(Vec<u8>),
    Function(Vec<Instruction>, Heap, SymbolTable),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Push(Value),
    AssignAndPush(Vec<u8>),
    Add,
    Sub,
    Mul,
    Div,
    Lookup(String),
    SetVar(String),
    MakeFunction(Vec<Instruction>),
    Call,
}
