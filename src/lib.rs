pub mod lexer;
pub mod parser;
pub mod ast;
pub mod interpreter;

//exporta de novo os tipos principais para facilitar o uso em outros arquivos e afins
pub use lexer::{Lexer, Token};
pub use parser::Parser;
pub use ast::{Stmt, Expr, Function, BinOp, LogicalOp, UnaryOp};
pub use interpreter::{Interpreter, Value, RuntimeError};