//! Language runtime modules for SigmaOS
//! 
//! Contains implementations of dynamic programming languages and scripting environments
//! for OS integration and user scripting capabilities.

pub mod kuroko_lang;

pub use kuroko_lang::{
    KurokoValue, KurokoObject, KurokoError, Opcode, Instruction, CodeObject,
    KurokoCompiler, KurokoVM, KurokoREPL, BuiltinFn,
    Token, TokenType,
};