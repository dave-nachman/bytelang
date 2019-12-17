use ast::{Expr, Operator, Value};
use bytecode;
use bytecode::Instruction;

fn generate_from_node(node: &Expr) -> Vec<Instruction> {
    let mut code: Vec<Instruction> = Vec::new();
    match node {
        Expr::Literal(ref value) => match value {
            Value::Int(ref v) => code.push(Instruction::Push(bytecode::Value::Int(*v))),
            Value::Float(ref v) => code.push(Instruction::Push(bytecode::Value::Float(*v))),
            Value::Bool(ref v) => code.push(Instruction::Push(bytecode::Value::Bool(*v))),
            Value::String(ref v) => {
                let as_bytes = v.as_bytes();
                code.push(Instruction::AssignAndPush(as_bytes.to_vec()));
            }
        },
        Expr::BinaryOp(ref a, Operator::Add, ref b) => {
            code.extend(generate_from_node(a.as_ref()));
            code.extend(generate_from_node(b.as_ref()));
            code.push(Instruction::Add)
        }
        Expr::BinaryOp(ref a, Operator::Sub, ref b) => {
            code.extend(generate_from_node(a.as_ref()));
            code.extend(generate_from_node(b.as_ref()));
            code.push(Instruction::Sub)
        }
        Expr::BinaryOp(ref a, Operator::Div, ref b) => {
            code.extend(generate_from_node(a.as_ref()));
            code.extend(generate_from_node(b.as_ref()));
            code.push(Instruction::Div)
        }
        Expr::BinaryOp(ref a, Operator::Mul, ref b) => {
            code.extend(generate_from_node(a.as_ref()));
            code.extend(generate_from_node(b.as_ref()));
            code.push(Instruction::Mul)
        }
        Expr::LetAssignment(ref ident, ref expr) => {
            code.extend(generate_from_node(expr.as_ref()));
            code.push(Instruction::SetVar(ident.value.clone()))
        }
        Expr::ConstAssignment(ref ident, ref expr) => {
            code.extend(generate_from_node(expr.as_ref()));
            code.push(Instruction::SetVar(ident.value.clone()))
        }
        Expr::Assignment(ref ident, ref expr) => {
            code.extend(generate_from_node(expr.as_ref()));
            code.push(Instruction::SetVar(ident.value.clone()))
        }
        Expr::Identifier(ref ident) => code.push(Instruction::Lookup(ident.value.clone())),
        Expr::Function(ref params, ref body) => {
            let mut function_stack: Vec<Instruction> = Vec::new();
            for param in params {
                function_stack.push(Instruction::SetVar(param.value.clone()))
            }
            for node in body {
                function_stack.append(&mut generate_from_node(node))
            }

            code.push(Instruction::MakeFunction(function_stack));
        }

        Expr::CallFunction(ref ident, ref _args) => {
            code.push(Instruction::Lookup(ident.value.clone()));
            code.push(Instruction::Call)
        }
    }
    code
}

pub fn generate(nodes: Vec<Expr>) -> Vec<Instruction> {
    let mut code: Vec<Instruction> = Vec::new();
    for node in nodes {
        code.extend(generate_from_node(&node))
    }
    code
}
