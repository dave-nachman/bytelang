use std::collections::HashMap;

use ast::{Expr, Identifier};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Let,
    Const,
}

#[derive(Debug, Default, Clone)]
pub struct SymbolTable {
    table: HashMap<String, Type>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            table: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisError {
    UnboundIdentifier(Identifier),
    RedeclaringIdentifier(Identifier),
    ReassigningConst(Identifier),
    FunctionCallWithIncorrectArity(Identifier),
}

#[derive(Debug, Clone)]
pub enum AnalysisWarning {
    ReassigningIdentifier(Identifier),
}

#[derive(Debug)]
pub struct AnalysisResults {
    pub errors: Vec<AnalysisError>,
    pub warnings: Vec<AnalysisWarning>,
}

impl AnalysisResults {
    fn new() -> AnalysisResults {
        AnalysisResults {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn append(&mut self, other: AnalysisResults) {
        self.errors.append(&mut other.errors.clone());
        self.warnings.append(&mut other.warnings.clone());
    }
}

#[derive(Debug, Default, Clone)]
pub struct Analyzer {
    symbol_table: SymbolTable,
    function_arity: HashMap<String, usize>,
}

impl Analyzer {
    pub fn new() -> Analyzer {
        Analyzer::default()
    }

    fn add_function_arity(&mut self, ident: &Identifier, rhs: &Expr) {
        if let Expr::Function(ref idents, _) = &rhs {
            self.function_arity
                .insert(ident.clone().value, idents.len());
        }
    }

    fn analyze_expr(&mut self, expr: &Expr) -> AnalysisResults {
        let mut results = AnalysisResults::new();
        match expr {
            Expr::Literal(ref _v) => (),
            Expr::Identifier(ref ident) => {
                if !self.symbol_table.table.contains_key(&ident.value) {
                    results
                        .errors
                        .push(AnalysisError::UnboundIdentifier(ident.clone()))
                }
            }
            Expr::Assignment(ref ident, ref rhs) => {
                if !self.symbol_table.table.contains_key(&ident.value) {
                    results
                        .errors
                        .push(AnalysisError::UnboundIdentifier(ident.clone()))
                } else if self.symbol_table.table.get(&ident.value).unwrap() == &Type::Const {
                    results
                        .errors
                        .push(AnalysisError::ReassigningConst(ident.clone()))
                }

                results
                    .warnings
                    .push(AnalysisWarning::ReassigningIdentifier(ident.clone()));

                self.add_function_arity(ident, rhs);
                results.append(self.analyze_expr(rhs))
            }
            Expr::LetAssignment(ref ident, ref rhs) => {
                if self.symbol_table.table.contains_key(&ident.value) {
                    results
                        .errors
                        .push(AnalysisError::RedeclaringIdentifier(ident.clone()))
                }

                self.symbol_table
                    .table
                    .insert(ident.value.clone(), Type::Let);

                self.add_function_arity(ident, rhs);

                results.append(self.analyze_expr(rhs))
            }
            Expr::ConstAssignment(ref ident, ref rhs) => {
                if self.symbol_table.table.contains_key(&ident.value) {
                    results
                        .errors
                        .push(AnalysisError::RedeclaringIdentifier(ident.clone()))
                }
                self.symbol_table
                    .table
                    .insert(ident.value.clone(), Type::Const);

                self.add_function_arity(ident, rhs);
                results.append(self.analyze_expr(rhs))
            }
            Expr::BinaryOp(ref a, _, ref b) => {
                results.append(self.analyze_expr(a));
                results.append(self.analyze_expr(b));
            }
            Expr::Function(params, body) => {
                let mut fn_analyzer = self.clone();

                // add params to symbol table
                fn_analyzer.symbol_table.table.extend(
                    params
                    .iter()
                    .map(|param| (param.value.clone(), Type::Const)));

                let fn_results = fn_analyzer.analyze(body);
                results.append(fn_results);
            }

            Expr::CallFunction(ref ident, ref exprs) => {
                let expected_len = self.function_arity.get(&ident.value).unwrap();
                if exprs.len() != *expected_len {
                    results
                        .errors
                        .push(AnalysisError::FunctionCallWithIncorrectArity(ident.clone()))
                }
            }
        }
        results
    }

    pub fn analyze(&mut self, exprs: &[Expr]) -> AnalysisResults {
        let mut results = AnalysisResults::new();
        for expr in exprs {
            results.append(self.analyze_expr(&expr))
        }
        results
    }
}
