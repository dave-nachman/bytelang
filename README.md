# bytelang

Bytelang is a WIP implementation of a dynamically typed programming language in Rust that is compiled to bytecode and run on a stack-based virtual machine.

I wrote this as an exercise in writing a bytecode-based interpreter, and also as an opportunity to write something in Rust.

## Stages

Parser -> Semantic Analysis -> Code generation -> Virtual machine

### [Parser](./src/parser.lalrpop)

The parser is written using [lalrpop](https://github.com/lalrpop/lalrpop), outputing AST nodes

### [Semantic Analysis](./src/semantic_analysis.rs)

The semantic analyzer checks for errors (and give warnings) such as reassignment of variables, calling a function with an incorrect arity, and referring to an identifier that hasn't been defined

### [Code generation](./src/codegen.rs)

The code generation step compiles the AST into custom bytecode

### [Virtual machine evaluation](./src/vm.rs)

The virtual machine executes the bytecode, maintaining a code and data stack, a heap, and a symbol table
