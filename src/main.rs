pub mod ast;
pub mod bytecode;
pub mod codegen;
pub mod parser;
pub mod pretty_print;
pub mod runtime;
pub mod semantic_analysis;
pub mod vm;

use std::io::{self, Write};

use pretty_print::pretty_print;
use runtime::{EvaluationResult, Runtime};
use vm::ReturnValue;

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
