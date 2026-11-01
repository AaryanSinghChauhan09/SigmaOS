//! Language runtime modules for SigmaOS
//!
//! Contains implementations of dynamic programming languages and scripting environments
//! for OS integration and user scripting capabilities.

pub mod kuroko_lang;

pub use kuroko_lang::{
    BuiltinFn, CodeObject, Instruction, KurokoCompiler, KurokoError, KurokoObject, KurokoREPL,
    KurokoVM, KurokoValue, Opcode, Token, TokenType,
};
pub mod i18n;
pub use i18n::{KeyboardLayout, Locale, LocaleManager, KEYBOARD_LAYOUTS, SUPPORTED_LOCALES};
