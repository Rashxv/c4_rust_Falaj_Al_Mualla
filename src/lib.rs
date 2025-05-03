//! This is the library entry point for the C4 Rust compiler backend.
//! 
//! It organizes the compilation pipeline into modular components:
//! - `lexer`: Converts raw source code into a sequence of tokens.
//! - `token`: Defines the token kinds used by the lexer and parser.
//! - `parser`: Transforms tokens into an abstract syntax tree (AST) and then into bytecode.
//! - `instruction`: Contains the virtual machine instruction set.
//! - `vm`: Provides the stack-based virtual machine that executes bytecode.
//!
//! These modules together support parsing, compiling, and interpreting a small C-like language.

pub mod lexer;
pub mod token;
pub mod parser;
pub mod instruction;
pub mod vm;