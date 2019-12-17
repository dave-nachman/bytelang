extern crate lalrpop_util;

use codegen;
use parser;
use semantic_analysis::{AnalysisError, Analyzer};
use vm::{EvaluationError, ReturnValue, VM};

#[derive(Debug, Default)]
pub struct Runtime {
    analyzer: Analyzer,
    vm: VM,
}

#[derive(Debug, PartialEq)]
pub enum EvaluationResult {
    ParseError(String),
    SemanticAnalysisError(Vec<AnalysisError>),
    EvaluationError(EvaluationError),
    Success(ReturnValue),
}

impl Runtime {
    pub fn new() -> Runtime {
        Runtime::default()
    }

    pub fn evaluate(&mut self, source: &str) -> EvaluationResult {
        match source {
            ":env\n" => {
                let mut env_string = String::new();
                for (k, v) in &self.vm.symbol_table {
                    env_string.push_str(&format!("{:?}: {:?}\n", k, self.vm.heap[v.0]));
                }

                EvaluationResult::Success(ReturnValue::DisplayString(env_string))
            }

            _ => {
                let parse_result = parser::ExprOrStmtParser::new().parse(source);
                match parse_result {
                    Ok(node) => {
                        let program = vec![*node];
                        let analysis = self.analyzer.analyze(&program);

                        if analysis.errors.is_empty() {
                            if !analysis.warnings.is_empty() {
                                println!("warning: {:?}", analysis.warnings)
                            }
                            let bytecode = codegen::generate(program);
                            let result = self.vm.run(bytecode);
                            match result {
                                Ok(r) => EvaluationResult::Success(r),
                                Err(e) => EvaluationResult::EvaluationError(e),
                            }
                        } else {
                            EvaluationResult::SemanticAnalysisError(analysis.errors)
                        }
                    }
                    Err(e) => EvaluationResult::ParseError(format!("{:?}", e)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytecode::Value;

    #[test]
    fn test_int() {
        let mut runtime = Runtime::new();
        assert_eq!(
            runtime.evaluate("3"),
            EvaluationResult::Success(ReturnValue::Value(Value::Int(3)))
        );
    }

    #[test]
    fn test_float() {
        let mut runtime = Runtime::new();
        assert_eq!(
            runtime.evaluate("3.0"),
            EvaluationResult::Success(ReturnValue::Value(Value::Float(3.0)))
        );
    }

    #[test]
    fn test_bool() {
        let mut runtime = Runtime::new();
        assert_eq!(
            runtime.evaluate("true"),
            EvaluationResult::Success(ReturnValue::Value(Value::Bool(true)))
        );
    }

    #[test]
    fn test_string() {
        let mut runtime = Runtime::new();
        assert_eq!(
            runtime.evaluate("\"hello world\""),
            EvaluationResult::Success(ReturnValue::DisplayString("\"hello world\"".to_string()))
        );
    }

    #[test]
    fn test_identifier_lookup() {
        let mut runtime = Runtime::new();
        let _ = runtime.evaluate("let x = 3");
        assert_eq!(
            runtime.evaluate("x"),
            EvaluationResult::Success(ReturnValue::Value(Value::Int(3)))
        );
    }

    #[test]
    fn test_addition() {
        let mut runtime = Runtime::new();
        assert_eq!(
            runtime.evaluate("3 + 3"),
            EvaluationResult::Success(ReturnValue::Value(Value::Int(6)))
        );
    }

    #[test]
    fn test_function() {
        let mut runtime = Runtime::new();
        let _ = runtime.evaluate("let x = () => 3;");
        assert_eq!(
            runtime.evaluate("x()"),
            EvaluationResult::Success(ReturnValue::Value(Value::Int(3)))
        );
    }
}
