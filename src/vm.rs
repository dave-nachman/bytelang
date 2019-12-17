use std::collections::HashMap;
use std::str;

use bytecode::{Heap, Instruction, SymbolTable, Value};

#[derive(Debug, PartialEq)]
pub enum EvaluationError {
    MissingVariable(String),
    InvalidOperation(String),
}

#[derive(Debug, Default)]
pub struct VM {
    pub code_stack: Vec<Instruction>,
    pub data_stack: Vec<Value>,
    pub heap: Heap,
    pub symbol_table: SymbolTable,
}

#[derive(Debug, PartialEq)]
pub enum ReturnValue {
    Value(Value),
    DisplayString(String),
    Empty,
}

impl VM {
    pub fn new() -> VM {
        VM {
            code_stack: Vec::new(),
            data_stack: Vec::new(),
            heap: Vec::with_capacity(1000),
            symbol_table: HashMap::new(),
        }
    }

    pub fn from(heap: Heap, symbol_table: SymbolTable) -> VM {
        VM {
            heap,
            symbol_table,
            code_stack: Vec::new(),
            data_stack: Vec::new(),
        }
    }

    pub fn merge(&mut self, child: VM) {
        for (k, v) in child.symbol_table {
            if self.symbol_table.contains_key(&k) {
                self.heap.push(child.heap[v.0].clone());
                let location = self.heap.len() - 1;
                self.symbol_table.insert(k, (location, 0));
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.data_stack.push(value)
    }

    fn run_instruction(&mut self, ins: &Instruction) -> Result<(), EvaluationError> {
        let mut error: Option<EvaluationError> = None;
        match ins {
            Instruction::Push(ref v) => {
                self.push(v.clone());
            }
            Instruction::AssignAndPush(ref bytes) => {
                let pointer = Value::Pointer(self.heap.len() as u8);
                self.heap.push(Value::Bytes(bytes.clone()));
                self.push(pointer);
            }
            Instruction::Lookup(ref ident) => match self.symbol_table.get(ident) {
                Some(&(location, _)) => {
                    let value = &self.heap[location];
                    self.data_stack.push(value.clone());
                }
                None => error = Some(EvaluationError::MissingVariable(ident.to_string())),
            },
            Instruction::SetVar(ref ident) => {
                let value = self.data_stack.pop().unwrap();
                self.heap.push(value);
                let location = self.heap.len() - 1;
                self.symbol_table.insert(ident.to_string(), (location, 0));
            }
            Instruction::Add => {
                let a = self.data_stack.pop().unwrap();
                let b = self.data_stack.pop().unwrap();
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => self.data_stack.push(Value::Int(x + y)),
                    (Value::Float(x), Value::Float(y)) => self.data_stack.push(Value::Float(x + y)),

                    _ => {
                        error = Some(EvaluationError::InvalidOperation(
                            "Can't add A and B".to_string(),
                        ));
                    }
                }
            }
            Instruction::Sub => {
                let _ = self.data_stack.pop().unwrap();
                let _ = self.data_stack.pop().unwrap();
            }
            Instruction::Mul => {
                let _ = self.data_stack.pop().unwrap();
                let _ = self.data_stack.pop().unwrap();
            }
            Instruction::Div => {
                let _ = self.data_stack.pop().unwrap();
                let _ = self.data_stack.pop().unwrap();
            }

            &Instruction::MakeFunction(ref function_stack) => {
                let function = Value::Function(
                    function_stack.clone(),
                    self.heap.clone(),
                    self.symbol_table.clone(),
                );
                self.data_stack.push(function);
            }

            &Instruction::Call => {
                let f = self.data_stack.pop().unwrap();

                if let Value::Function(instructions, heap, symbol_table) = f {
                    let mut vm = VM::from(heap, symbol_table);
                    let result = vm.run(instructions);

                    if let Result::Ok(ReturnValue::Value(v)) = result {
                        self.data_stack.push(v);
                    }

                    self.merge(vm);
                }
            }
        }
        match error {
            Some(e) => Result::Err(e),
            None => Result::Ok(()),
        }
    }

    pub fn run(&mut self, code_stack: Vec<Instruction>) -> Result<ReturnValue, EvaluationError> {
        self.code_stack = code_stack;
        let debug = true;
        let mut error: Option<EvaluationError> = None;
        loop {
            if self.code_stack.is_empty() {
                break;
            }
            if error.is_some() {
                break;
            }
            let ins = self.code_stack.remove(0);
            match self.run_instruction(&ins) {
                Ok(_) => (),
                Err(e) => error = Some(e),
            }
            if debug {
                println!("========");
                println!("INS: {:?}", ins);
                println!("CODE: {:?}", self.code_stack);
                println!("DATA: {:?}", self.data_stack);
                println!("SYMBOL TABLE: {:?}", self.symbol_table);
                println!("HEAP: {:?}", self.heap);
            }
        }
        match error {
            Some(e) => Result::Err(e),
            None => {
                let value = self.data_stack.pop();
                match value {
                    Option::Some(Value::Pointer(p)) => {
                        let heaped = &self.heap[p as usize];
                        match heaped {
                            Value::Bytes(ref bytes) => Result::Ok(ReturnValue::DisplayString(
                                str::from_utf8(bytes).unwrap().to_string(),
                            )),
                            _ => Result::Ok(ReturnValue::Value(heaped.clone())),
                        }
                    }
                    Option::Some(v) => Result::Ok(ReturnValue::Value(v)),
                    Option::None => Result::Ok(ReturnValue::Empty),
                }
            }
        }
    }
}
