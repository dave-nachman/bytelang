pub mod parser;
pub mod ast;
pub mod semantic_analysis;
pub mod bytecode;
pub mod codegen;
pub mod vm;
pub mod runtime;
pub mod pretty_print;

use std::io::{self, Write};

use runtime::{EvaluationResult, Runtime};
use vm::ReturnValue;
use pretty_print::{pretty_print};

    
fn main() {
    let mut runtime = Runtime::new();
    loop {
        print!(">>> ");
        let _ = io::stdout().flush();
        let mut line = String::new();
        let stdin = io::stdin();

        match stdin.read_line(&mut line) {
            Ok(_) => {
                let result = runtime.evaluate(&line);
                match result {
                    EvaluationResult::ParseError(e) => println!("{:?}", e),
                    EvaluationResult::SemanticAnalysisError(e) => println!("{:?}", e),
                    EvaluationResult::EvaluationError(e) => println!("{:?}", e),
                    EvaluationResult::Success(r) => match r {
                        ReturnValue::Empty => (),
                        ReturnValue::DisplayString(s) => println!("{}", s),
                        ReturnValue::Value(v) => println!("{}", pretty_print(&v)),
                    },
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
