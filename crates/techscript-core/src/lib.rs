// ── TechScript Core Library ──────────────────────────────────────────
// The heart of TechScript: lexer, parser, compiler, VM, and stdlib.

pub mod ansi;

pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod error;
pub mod opcode;
pub mod chunk;
pub mod value;
pub mod compiler;
pub mod vm;
pub mod builtins;
pub mod stdlib;
pub mod tests;
pub mod repl;
pub mod span;
pub mod formatter;
pub mod linter;
pub mod bytecode_file;
pub mod module_resolver;
