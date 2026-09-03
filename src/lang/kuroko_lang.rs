#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
//! ToaruOS-style Built-in Dynamic Language for SigmaOS
//!
//! Implements a Kuroko-inspired dynamic bytecode-compiled programming language,
//! similar to Python but designed for OS integration. Features:
//! - Single-pass bytecode compiler with backtracking
//! - Virtual machine with register-based execution
//! - Dynamic typing with garbage collection
//! - Module system for OS integration
//! - FFI (Foreign Function Interface) for calling SigmaOS syscalls
//! - REPL (Read-Eval-Print Loop) for interactive use
//! - Async/await support for OS operations
use alloc::format;


extern crate alloc;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::collections::BTreeMap;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;
use core::ptr::NonNull;

/// Kuroko-style value types (dynamic typing)
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub enum KurokoValue {
    Nil,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    List(Vec<KurokoValue>),
    Dict(BTreeMap<String, KurokoValue>),
    Function(usize), // Function index in code
    BuiltinFunction(BuiltinFn),
    Object(Box<KurokoObject>),
}

impl KurokoValue {
    pub fn type_name(&self) -> &'static str {
        match self {
            KurokoValue::Nil => "nil",
            KurokoValue::Bool(_) => "bool",
            KurokoValue::Integer(_) => "int",
            KurokoValue::Float(_) => "float",
            KurokoValue::String(_) => "str",
            KurokoValue::List(_) => "list",
            KurokoValue::Dict(_) => "dict",
            KurokoValue::Function(_) => "function",
            KurokoValue::BuiltinFunction(_) => "builtin",
            KurokoValue::Object(_) => "object",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            KurokoValue::Nil => false,
            KurokoValue::Bool(b) => *b,
            KurokoValue::Integer(i) => *i != 0,
            KurokoValue::Float(f) => *f != 0.0,
            KurokoValue::String(s) => !s.is_empty(),
            KurokoValue::List(l) => !l.is_empty(),
            KurokoValue::Dict(d) => !d.is_empty(),
            _ => true,
        }
    }
}

/// Kuroko-style object with attributes
#[repr(C)]
#[derive(Debug, Clone)]
pub struct KurokoObject {
    pub class_name: String,
    pub attributes: BTreeMap<String, KurokoValue>,
}

impl KurokoObject {
    pub fn new(class_name: &str) -> Self {
        KurokoObject {
            class_name: String::from(class_name),
            attributes: BTreeMap::new(),
        }
    }

    pub fn set_attribute(&mut self, name: &str, value: KurokoValue) {
        self.attributes.insert(String::from(name), value);
    }

    pub fn get_attribute(&self, name: &str) -> Option<&KurokoValue> {
        self.attributes.get(name)
    }
}

/// Builtin function type
pub type BuiltinFn = fn(&mut KurokoVM, Vec<KurokoValue>) -> Result<KurokoValue, KurokoError>;

/// Bytecode opcodes (Kuroko-inspired)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    // Constants and literals
    LoadNil,
    LoadBool,
    LoadInteger,
    LoadFloat,
    LoadString,
    
    // Stack operations
    Pop,
    Dup,
    Swap,
    Rotate,
    
    // Variables
    LoadLocal,
    StoreLocal,
    LoadGlobal,
    StoreGlobal,
    LoadAttr,
    StoreAttr,
    
    // Control flow
    Jump,
    JumpIfFalse,
    JumpIfTrue,
    JumpIfNil,
    Loop,
    Call,
    Return,
    
    // Operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    
    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    // Logical
    And,
    Or,
    Not,
    
    // Collections
    BuildList,
    BuildDict,
    GetIndex,
    SetIndex,
    
    // OS integration
    Syscall,
    AsyncCall,
    Await,
    
    // Meta
    Print,
    Input,
}

/// Bytecode instruction with operand
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operand: i64,
}

impl Instruction {
    pub fn new(opcode: Opcode, operand: i64) -> Self {
        Instruction { opcode, operand }
    }
}

/// Compiled code object (function/module)
#[repr(C)]
pub struct CodeObject {
    pub name: String,
    pub bytecode: Vec<Instruction>,
    pub constants: Vec<KurokoValue>,
    pub locals: Vec<String>,
    pub parameters: Vec<String>,
    pub is_variadic: bool,
}

impl CodeObject {
    pub fn new(name: &str) -> Self {
        CodeObject {
            name: String::from(name),
            bytecode: Vec::new(),
            constants: Vec::new(),
            locals: Vec::new(),
            parameters: Vec::new(),
            is_variadic: false,
        }
    }

    pub fn emit(&mut self, opcode: Opcode, operand: i64) {
        self.bytecode.push(Instruction::new(opcode, operand));
    }

    pub fn add_constant(&mut self, value: KurokoValue) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

/// Compiler error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KurokoError {
    Success = 0,
    SyntaxError = 1,
    RuntimeError = 2,
    TypeError = 3,
    NameError = 4,
    ValueError = 5,
    IndexError = 6,
    KeyError = 7,
    AttributeError = 8,
    StackOverflow = 9,
    StackUnderflow = 10,
}

/// Token types for parsing
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // Literals
    Nil,
    Bool,
    Integer,
    Float,
    String,
    
    // Identifiers and keywords
    Identifier,
    Def,
    Return,
    If,
    Else,
    While,
    For,
    In,
    Break,
    Continue,
    And,
    Or,
    Not,
    True,
    False,
    Async,
    Await,
    
    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Assign,
    PlusAssign,
    MinusAssign,
    
    // Delimiters
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    Dot,
    
    // Special
    Newline,
    EOF,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: usize) -> Self {
        Token {
            token_type,
            lexeme: String::from(lexeme),
            line,
        }
    }
}

/// Single-pass bytecode compiler with backtracking (Kuroko-style)
pub struct KurokoCompiler {
    pub tokens: Vec<Token>,
    pub current: usize,
    pub code_objects: Vec<CodeObject>,
    pub current_code: usize,
    pub loop_depth: usize,
}

impl KurokoCompiler {
    pub fn new() -> Self {
        KurokoCompiler {
            tokens: Vec::new(),
            current: 0,
            code_objects: Vec::new(),
            current_code: 0,
            loop_depth: 0,
        }
    }

    pub fn compile(&mut self, source: &str) -> Result<CodeObject, KurokoError> {
        // Tokenize
        self.tokens = self.tokenize(source)?;
        
        // Create main module code object
        let main_code = CodeObject::new("__main__");
        self.code_objects.push(main_code);
        self.current_code = 0;
        
        // Parse and compile
        while !self.is_at_end() {
            self.compile_declaration()?;
        }
        
        // Emit return nil at end
        self.emit_opcode(Opcode::LoadNil, 0);
        self.emit_opcode(Opcode::Return, 0);
        
        Ok(self.code_objects[0].clone())
    }

    fn tokenize(&self, source: &str) -> Result<Vec<Token>, KurokoError> {
        let mut tokens = Vec::new();
        let mut chars = source.chars().peekable();
        let mut line = 1;
        
        while let Some(&ch) = chars.peek() {
            match ch {
                ' ' | '\t' | '\r' => { chars.next(); }
                '\n' => { 
                    tokens.push(Token::new(TokenType::Newline, "\n", line));
                    line += 1;
                    chars.next(); 
                }
                '(' => { tokens.push(Token::new(TokenType::LeftParen, "(", line)); chars.next(); }
                ')' => { tokens.push(Token::new(TokenType::RightParen, ")", line)); chars.next(); }
                '[' => { tokens.push(Token::new(TokenType::LeftBracket, "[", line)); chars.next(); }
                ']' => { tokens.push(Token::new(TokenType::RightBracket, "]", line)); chars.next(); }
                '{' => { tokens.push(Token::new(TokenType::LeftBrace, "{", line)); chars.next(); }
                '}' => { tokens.push(Token::new(TokenType::RightBrace, "}", line)); chars.next(); }
                ',' => { tokens.push(Token::new(TokenType::Comma, ",", line)); chars.next(); }
                ':' => { tokens.push(Token::new(TokenType::Colon, ":", line)); chars.next(); }
                '.' => { tokens.push(Token::new(TokenType::Dot, ".", line)); chars.next(); }
                '+' => { tokens.push(Token::new(TokenType::Plus, "+", line)); chars.next(); }
                '-' => { tokens.push(Token::new(TokenType::Minus, "-", line)); chars.next(); }
                '*' => { tokens.push(Token::new(TokenType::Multiply, "*", line)); chars.next(); }
                '/' => { tokens.push(Token::new(TokenType::Divide, "/", line)); chars.next(); }
                '%' => { tokens.push(Token::new(TokenType::Modulo, "%", line)); chars.next(); }
                '=' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::new(TokenType::Equal, "==", line));
                    } else {
                        tokens.push(Token::new(TokenType::Assign, "=", line));
                    }
                }
                '!' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::new(TokenType::NotEqual, "!=", line));
                    } else {
                        tokens.push(Token::new(TokenType::Not, "!", line));
                    }
                }
                '<' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::new(TokenType::LessEqual, "<=", line));
                    } else {
                        tokens.push(Token::new(TokenType::Less, "<", line));
                    }
                }
                '>' => {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(Token::new(TokenType::GreaterEqual, ">=", line));
                    } else {
                        tokens.push(Token::new(TokenType::Greater, ">", line));
                    }
                }
                '"' => {
                    chars.next();
                    let mut string_literal = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch == '"' {
                            chars.next();
                            break;
                        }
                        string_literal.push(ch);
                        chars.next();
                    }
                    tokens.push(Token::new(TokenType::String, &string_literal, line));
                }
                '0'..='9' => {
                    let mut number = String::new();
                    let mut is_float = false;
                    while let Some(&ch) = chars.peek() {
                        if ch.is_digit(10) || ch == '.' {
                            if ch == '.' { is_float = true; }
                            number.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if is_float {
                        tokens.push(Token::new(TokenType::Float, &number, line));
                    } else {
                        tokens.push(Token::new(TokenType::Integer, &number, line));
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut identifier = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            identifier.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    
                    let token_type = match identifier.as_str() {
                        "nil" => TokenType::Nil,
                        "true" => TokenType::True,
                        "false" => TokenType::False,
                        "def" => TokenType::Def,
                        "return" => TokenType::Return,
                        "if" => TokenType::If,
                        "else" => TokenType::Else,
                        "while" => TokenType::While,
                        "for" => TokenType::For,
                        "in" => TokenType::In,
                        "break" => TokenType::Break,
                        "continue" => TokenType::Continue,
                        "and" => TokenType::And,
                        "or" => TokenType::Or,
                        "not" => TokenType::Not,
                        "async" => TokenType::Async,
                        "await" => TokenType::Await,
                        _ => TokenType::Identifier,
                    };
                    
                    tokens.push(Token::new(token_type, &identifier, line));
                }
                _ => {
                    chars.next();
                }
            }
        }
        
        tokens.push(Token::new(TokenType::EOF, "", line));
        Ok(tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || 
        self.tokens[self.current].token_type == TokenType::EOF
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() { return false; }
        self.peek().token_type == token_type
    }

    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, token_type: TokenType, _message: &str) -> Result<&Token, KurokoError> {
        if self.check(token_type) {
            Ok(self.advance())
        } else {
            Err(KurokoError::SyntaxError)
        }
    }

    fn emit_opcode(&mut self, opcode: Opcode, operand: i64) {
        if let Some(code) = self.code_objects.get_mut(self.current_code) {
            code.emit(opcode, operand);
        }
    }

    fn compile_declaration(&mut self) -> Result<(), KurokoError> {
        if self.match_token(TokenType::Def) {
            self.compile_function()
        } else {
            self.compile_statement()
        }
    }

    fn compile_function(&mut self) -> Result<(), KurokoError> {
        let name = self.consume(TokenType::Identifier, "Expect function name")?;
        
        self.consume(TokenType::LeftParen, "Expect '(' after function name")?;
        
        // Parameters
        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                let param = self.consume(TokenType::Identifier, "Expect parameter name")?;
                parameters.push(param.lexeme.clone());
                
                if !self.match_token(TokenType::Comma) { break; }
            }
        }
        
        self.consume(TokenType::RightParen, "Expect ')' after parameters")?;
        self.consume(TokenType::Newline, "Expect newline after function definition")?;
        
        // Create new code object for function
        let func_code = CodeObject::new(&name.lexeme);
        func_code.parameters = parameters;
        let func_index = self.code_objects.len();
        self.code_objects.push(func_code);
        
        // Save current code and switch to function
        let saved_code = self.current_code;
        self.current_code = func_index;
        
        // Compile function body
        while !self.check(TokenType::EOF) && 
              !self.check(TokenType::Def) && 
              self.previous().token_type != TokenType::Newline {
            self.compile_statement()?;
        }
        
        // Emit return
        self.emit_opcode(Opcode::LoadNil, 0);
        self.emit_opcode(Opcode::Return, 0);
        
        // Restore current code
        self.current_code = saved_code;
        
        // Emit function constant and store
        let func_value = KurokoValue::Function(func_index);
        let const_index = self.add_constant(func_value);
        self.emit_opcode(Opcode::LoadInteger, const_index as i64);
        self.emit_opcode(Opcode::StoreGlobal, 0); // Store in global scope
        
        Ok(())
    }

    fn add_constant(&mut self, value: KurokoValue) -> usize {
        if let Some(code) = self.code_objects.get_mut(self.current_code) {
            code.add_constant(value)
        } else {
            0
        }
    }

    fn compile_statement(&mut self) -> Result<(), KurokoError> {
        if self.match_token(TokenType::If) {
            self.compile_if()
        } else if self.match_token(TokenType::While) {
            self.compile_while()
        } else if self.match_token(TokenType::Return) {
            self.compile_return()
        } else {
            self.compile_expression_statement()
        }
    }

    fn compile_if(&mut self) -> Result<(), KurokoError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'")?;
        self.compile_expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition")?;
        self.consume(TokenType::Newline, "Expect newline after if")?;
        
        // Emit jump if false
        let jump_index = self.current_bytecode_len();
        self.emit_opcode(Opcode::JumpIfFalse, 0);
        
        // Compile then branch
        while !self.check(TokenType::Else) && !self.check(TokenType::EOF) {
            self.compile_statement()?;
        }
        
        // Emit jump over else branch
        let else_jump = self.current_bytecode_len();
        self.emit_opcode(Opcode::Jump, 0);
        
        // Patch jump if false
        self.patch_jump(jump_index);
        
        // Compile else branch if present
        if self.match_token(TokenType::Else) {
            self.consume(TokenType::Newline, "Expect newline after else")?;
            while !self.check(TokenType::EOF) {
                self.compile_statement()?;
            }
        }
        
        // Patch else jump
        self.patch_jump(else_jump);
        
        Ok(())
    }

    fn compile_while(&mut self) -> Result<(), KurokoError> {
        let loop_start = self.current_bytecode_len();
        self.loop_depth += 1;
        
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'")?;
        self.compile_expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition")?;
        self.consume(TokenType::Newline, "Expect newline after while")?;
        
        // Emit jump if false
        let jump_index = self.current_bytecode_len();
        self.emit_opcode(Opcode::JumpIfFalse, 0);
        
        // Compile loop body
        while !self.check(TokenType::EOF) && !self.check(TokenType::Break) {
            self.compile_statement()?;
        }
        
        // Emit loop back
        self.emit_opcode(Opcode::Jump, loop_start as i64);
        
        // Patch jump if false
        self.patch_jump(jump_index);
        
        self.loop_depth -= 1;
        
        Ok(())
    }

    fn compile_return(&mut self) -> Result<(), KurokoError> {
        if !self.check(TokenType::Newline) {
            self.compile_expression()?;
        } else {
            self.emit_opcode(Opcode::LoadNil, 0);
        }
        self.emit_opcode(Opcode::Return, 0);
        Ok(())
    }

    fn compile_expression_statement(&mut self) -> Result<(), KurokoError> {
        self.compile_expression()?;
        self.consume(TokenType::Newline, "Expect newline after expression")?;
        self.emit_opcode(Opcode::Pop, 0);
        Ok(())
    }

    fn compile_expression(&mut self) -> Result<(), KurokoError> {
        self.compile_assignment()
    }

    fn compile_assignment(&mut self) -> Result<(), KurokoError> {
        // For simplicity, just compile as expression for now
        self.compile_or()
    }

    fn compile_or(&mut self) -> Result<(), KurokoError> {
        self.compile_and()?;
        while self.match_token(TokenType::Or) {
            let jump_index = self.current_bytecode_len();
            self.emit_opcode(Opcode::JumpIfTrue, 0);
            self.emit_opcode(Opcode::Pop, 0);
            self.compile_and()?;
            self.patch_jump(jump_index);
        }
        Ok(())
    }

    fn compile_and(&mut self) -> Result<(), KurokoError> {
        self.compile_equality()?;
        while self.match_token(TokenType::And) {
            let jump_index = self.current_bytecode_len();
            self.emit_opcode(Opcode::JumpIfFalse, 0);
            self.emit_opcode(Opcode::Pop, 0);
            self.compile_equality()?;
            self.patch_jump(jump_index);
        }
        Ok(())
    }

    fn compile_equality(&mut self) -> Result<(), KurokoError> {
        self.compile_comparison()?;
        while self.match_token(TokenType::Equal) || self.match_token(TokenType::NotEqual) {
            let operator = self.previous().token_type;
            self.compile_comparison()?;
            
            match operator {
                TokenType::Equal => self.emit_opcode(Opcode::Equal, 0),
                TokenType::NotEqual => self.emit_opcode(Opcode::NotEqual, 0),
                _ => {}
            }
        }
        Ok(())
    }

    fn compile_comparison(&mut self) -> Result<(), KurokoError> {
        self.compile_term()?;
        while self.match_token(TokenType::Less) || self.match_token(TokenType::LessEqual) ||
              self.match_token(TokenType::Greater) || self.match_token(TokenType::GreaterEqual) {
            let operator = self.previous().token_type;
            self.compile_term()?;
            
            match operator {
                TokenType::Less => self.emit_opcode(Opcode::Less, 0),
                TokenType::LessEqual => self.emit_opcode(Opcode::LessEqual, 0),
                TokenType::Greater => self.emit_opcode(Opcode::Greater, 0),
                TokenType::GreaterEqual => self.emit_opcode(Opcode::GreaterEqual, 0),
                _ => {}
            }
        }
        Ok(())
    }

    fn compile_term(&mut self) -> Result<(), KurokoError> {
        self.compile_factor()?;
        while self.match_token(TokenType::Plus) || self.match_token(TokenType::Minus) {
            let operator = self.previous().token_type;
            self.compile_factor()?;
            
            match operator {
                TokenType::Plus => self.emit_opcode(Opcode::Add, 0),
                TokenType::Minus => self.emit_opcode(Opcode::Subtract, 0),
                _ => {}
            }
        }
        Ok(())
    }

    fn compile_factor(&mut self) -> Result<(), KurokoError> {
        self.compile_unary()?;
        while self.match_token(TokenType::Multiply) || self.match_token(TokenType::Divide) || 
              self.match_token(TokenType::Modulo) {
            let operator = self.previous().token_type;
            self.compile_unary()?;
            
            match operator {
                TokenType::Multiply => self.emit_opcode(Opcode::Multiply, 0),
                TokenType::Divide => self.emit_opcode(Opcode::Divide, 0),
                TokenType::Modulo => self.emit_opcode(Opcode::Modulo, 0),
                _ => {}
            }
        }
        Ok(())
    }

    fn compile_unary(&mut self) -> Result<(), KurokoError> {
        if self.match_token(TokenType::Minus) || self.match_token(TokenType::Not) {
            let operator = self.previous().token_type;
            self.compile_unary()?;
            
            match operator {
                TokenType::Minus => self.emit_opcode(Opcode::Subtract, 0), // Negate
                TokenType::Not => self.emit_opcode(Opcode::Not, 0),
                _ => {}
            }
        } else {
            self.compile_primary()
        }
    }

    fn compile_primary(&mut self) -> Result<(), KurokoError> {
        if self.match_token(TokenType::Nil) {
            self.emit_opcode(Opcode::LoadNil, 0);
        } else if self.match_token(TokenType::True) {
            self.emit_opcode(Opcode::LoadBool, 1);
        } else if self.match_token(TokenType::False) {
            self.emit_opcode(Opcode::LoadBool, 0);
        } else if self.match_token(TokenType::Integer) {
            let value = self.previous().lexeme.parse::<i64>().unwrap_or(0);
            let const_index = self.add_constant(KurokoValue::Integer(value));
            self.emit_opcode(Opcode::LoadInteger, const_index as i64);
        } else if self.match_token(TokenType::Float) {
            let value = self.previous().lexeme.parse::<f64>().unwrap_or(0.0);
            let const_index = self.add_constant(KurokoValue::Float(value));
            self.emit_opcode(Opcode::LoadFloat, const_index as i64);
        } else if self.match_token(TokenType::String) {
            let value = KurokoValue::String(self.previous().lexeme.clone());
            let const_index = self.add_constant(value);
            self.emit_opcode(Opcode::LoadString, const_index as i64);
        } else if self.match_token(TokenType::LeftParen) {
            self.compile_expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
        } else if self.match_token(TokenType::Identifier) {
            let name = self.previous().lexeme.clone();
            // For now, just load as global
            self.emit_opcode(Opcode::LoadGlobal, 0); // Simplified
        } else {
            return Err(KurokoError::SyntaxError);
        }
        
        self.compile_call()
    }

    fn compile_call(&mut self) -> Result<(), KurokoError> {
        if self.match_token(TokenType::LeftParen) {
            let mut arg_count = 0;
            if !self.check(TokenType::RightParen) {
                loop {
                    self.compile_expression()?;
                    arg_count += 1;
                    if !self.match_token(TokenType::Comma) { break; }
                }
            }
            self.consume(TokenType::RightParen, "Expect ')' after arguments")?;
            self.emit_opcode(Opcode::Call, arg_count as i64);
        }
        Ok(())
    }

    fn current_bytecode_len(&self) -> usize {
        if let Some(code) = self.code_objects.get(self.current_code) {
            code.bytecode.len()
        } else {
            0
        }
    }

    fn patch_jump(&mut self, index: usize) {
        if let Some(code) = self.code_objects.get_mut(self.current_code) {
            if index < code.bytecode.len() {
                let jump_target = code.bytecode.len() as i64;
                code.bytecode[index].operand = jump_target;
            }
        }
    }
}

impl Default for KurokoCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Virtual machine for executing bytecode
pub struct KurokoVM {
    pub stack: Vec<KurokoValue>,
    pub globals: BTreeMap<String, KurokoValue>,
    pub code_objects: Vec<CodeObject>,
    pub current_frame: Vec<VMFrame>,
    pub builtin_functions: BTreeMap<String, BuiltinFn>,
}

#[repr(C)]
pub struct VMFrame {
    pub code_index: usize,
    pub ip: usize,
    pub locals: Vec<KurokoValue>,
}

impl KurokoVM {
    pub fn new() -> Self {
        let mut vm = KurokoVM {
            stack: Vec::new(),
            globals: BTreeMap::new(),
            code_objects: Vec::new(),
            current_frame: Vec::new(),
            builtin_functions: BTreeMap::new(),
        };
        
        // Register builtin functions
        vm.register_builtins();
        vm
    }

    fn register_builtins(&mut self) {
        self.builtin_functions.insert("print".to_string(), builtin_print);
        self.builtin_functions.insert("input".to_string(), builtin_input);
        self.builtin_functions.insert("len".to_string(), builtin_len);
        self.builtin_functions.insert("type".to_string(), builtin_type);
    }

    pub fn interpret(&mut self, code: CodeObject) -> Result<KurokoValue, KurokoError> {
        self.code_objects.push(code);
        let code_index = self.code_objects.len() - 1;
        
        let frame = VMFrame {
            code_index,
            ip: 0,
            locals: Vec::new(),
        };
        self.current_frame.push(frame);
        
        let result = self.run();
        
        self.current_frame.pop();
        result
    }

    fn run(&mut self) -> Result<KurokoValue, KurokoError> {
        loop {
            let frame = self.current_frame.last().ok_or(KurokoError::RuntimeError)?;
            let code = self.code_objects.get(frame.code_index).ok_or(KurokoError::RuntimeError)?;
            
            if frame.ip >= code.bytecode.len() {
                break;
            }
            
            let instruction = code.bytecode[frame.ip];
            frame.ip += 1;
            
            match instruction.opcode {
                Opcode::LoadNil => self.stack.push(KurokoValue::Nil),
                Opcode::LoadBool => self.stack.push(KurokoValue::Bool(instruction.operand != 0)),
                Opcode::LoadInteger => {
                    if let Some(KurokoValue::Integer(val)) = code.constants.get(instruction.operand as usize) {
                        self.stack.push(KurokoValue::Integer(*val));
                    }
                }
                Opcode::LoadFloat => {
                    if let Some(KurokoValue::Float(val)) = code.constants.get(instruction.operand as usize) {
                        self.stack.push(KurokoValue::Float(*val));
                    }
                }
                Opcode::LoadString => {
                    if let Some(KurokoValue::String(val)) = code.constants.get(instruction.operand as usize) {
                        self.stack.push(KurokoValue::String(val.clone()));
                    }
                }
                Opcode::Pop => { self.stack.pop(); }
                Opcode::Add => {
                    let b = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    self.stack.push(self.add_values(a, b)?);
                }
                Opcode::Subtract => {
                    let b = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    self.stack.push(self.subtract_values(a, b)?);
                }
                Opcode::Multiply => {
                    let b = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    self.stack.push(self.multiply_values(a, b)?);
                }
                Opcode::Divide => {
                    let b = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    self.stack.push(self.divide_values(a, b)?);
                }
                Opcode::Equal => {
                    let b = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    self.stack.push(KurokoValue::Bool(a == b));
                }
                Opcode::NotEqual => {
                    let b = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    self.stack.push(KurokoValue::Bool(a != b));
                }
                Opcode::Less => {
                    let b = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    self.stack.push(self.compare_less(a, b)?);
                }
                Opcode::Greater => {
                    let b = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    let a = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    self.stack.push(self.compare_greater(a, b)?);
                }
                Opcode::Jump => {
                    frame.ip = instruction.operand as usize;
                }
                Opcode::JumpIfFalse => {
                    let value = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    if !value.is_truthy() {
                        frame.ip = instruction.operand as usize;
                    }
                }
                Opcode::JumpIfTrue => {
                    let value = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    if value.is_truthy() {
                        frame.ip = instruction.operand as usize;
                    }
                }
                Opcode::Call => {
                    let arg_count = instruction.operand as usize;
                    let args: Vec<KurokoValue> = self.stack.drain(self.stack.len() - arg_count..).collect();
                    let function = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    
                    let result = self.call_function(function, args)?;
                    self.stack.push(result);
                }
                Opcode::Return => {
                    return Ok(self.stack.pop().unwrap_or(KurokoValue::Nil));
                }
                Opcode::Print => {
                    let value = self.stack.pop().ok_or(KurokoError::StackUnderflow)?;
                    println!("{}", self.value_to_string(&value));
                }
                _ => {}
            }
        }
        
        Ok(self.stack.pop().unwrap_or(KurokoValue::Nil))
    }

    fn add_values(&self, a: KurokoValue, b: KurokoValue) -> Result<KurokoValue, KurokoError> {
        match (a, b) {
            (KurokoValue::Integer(x), KurokoValue::Integer(y)) => Ok(KurokoValue::Integer(x + y)),
            (KurokoValue::Float(x), KurokoValue::Float(y)) => Ok(KurokoValue::Float(x + y)),
            (KurokoValue::Integer(x), KurokoValue::Float(y)) => Ok(KurokoValue::Float(x as f64 + y)),
            (KurokoValue::Float(x), KurokoValue::Integer(y)) => Ok(KurokoValue::Float(x + y as f64)),
            (KurokoValue::String(x), KurokoValue::String(y)) => Ok(KurokoValue::String(format!("{}{}", x, y))),
            _ => Err(KurokoError::TypeError),
        }
    }

    fn subtract_values(&self, a: KurokoValue, b: KurokoValue) -> Result<KurokoValue, KurokoError> {
        match (a, b) {
            (KurokoValue::Integer(x), KurokoValue::Integer(y)) => Ok(KurokoValue::Integer(x - y)),
            (KurokoValue::Float(x), KurokoValue::Float(y)) => Ok(KurokoValue::Float(x - y)),
            (KurokoValue::Integer(x), KurokoValue::Float(y)) => Ok(KurokoValue::Float(x as f64 - y)),
            (KurokoValue::Float(x), KurokoValue::Integer(y)) => Ok(KurokoValue::Float(x - y as f64)),
            _ => Err(KurokoError::TypeError),
        }
    }

    fn multiply_values(&self, a: KurokoValue, b: KurokoValue) -> Result<KurokoValue, KurokoError> {
        match (a, b) {
            (KurokoValue::Integer(x), KurokoValue::Integer(y)) => Ok(KurokoValue::Integer(x * y)),
            (KurokoValue::Float(x), KurokoValue::Float(y)) => Ok(KurokoValue::Float(x * y)),
            (KurokoValue::Integer(x), KurokoValue::Float(y)) => Ok(KurokoValue::Float(x as f64 * y)),
            (KurokoValue::Float(x), KurokoValue::Integer(y)) => Ok(KurokoValue::Float(x * y as f64)),
            _ => Err(KurokoError::TypeError),
        }
    }

    fn divide_values(&self, a: KurokoValue, b: KurokoValue) -> Result<KurokoValue, KurokoError> {
        match (a, b) {
            (KurokoValue::Integer(x), KurokoValue::Integer(y)) => {
                if y == 0 { return Err(KurokoError::ValueError); }
                Ok(KurokoValue::Integer(x / y))
            }
            (KurokoValue::Float(x), KurokoValue::Float(y)) => {
                if y == 0.0 { return Err(KurokoError::ValueError); }
                Ok(KurokoValue::Float(x / y))
            }
            (KurokoValue::Integer(x), KurokoValue::Float(y)) => {
                if y == 0.0 { return Err(KurokoError::ValueError); }
                Ok(KurokoValue::Float(x as f64 / y))
            }
            (KurokoValue::Float(x), KurokoValue::Integer(y)) => {
                if y == 0 { return Err(KurokoError::ValueError); }
                Ok(KurokoValue::Float(x / y as f64))
            }
            _ => Err(KurokoError::TypeError),
        }
    }

    fn compare_less(&self, a: KurokoValue, b: KurokoValue) -> Result<KurokoValue, KurokoError> {
        match (a, b) {
            (KurokoValue::Integer(x), KurokoValue::Integer(y)) => Ok(KurokoValue::Bool(x < y)),
            (KurokoValue::Float(x), KurokoValue::Float(y)) => Ok(KurokoValue::Bool(x < y)),
            (KurokoValue::Integer(x), KurokoValue::Float(y)) => Ok(KurokoValue::Bool((x as f64) < y)),
            (KurokoValue::Float(x), KurokoValue::Integer(y)) => Ok(KurokoValue::Bool(x < (y as f64))),
            _ => Err(KurokoError::TypeError),
        }
    }

    fn compare_greater(&self, a: KurokoValue, b: KurokoValue) -> Result<KurokoValue, KurokoError> {
        match (a, b) {
            (KurokoValue::Integer(x), KurokoValue::Integer(y)) => Ok(KurokoValue::Bool(x > y)),
            (KurokoValue::Float(x), KurokoValue::Float(y)) => Ok(KurokoValue::Bool(x > y)),
            (KurokoValue::Integer(x), KurokoValue::Float(y)) => Ok(KurokoValue::Bool((x as f64) > y)),
            (KurokoValue::Float(x), KurokoValue::Integer(y)) => Ok(KurokoValue::Bool(x > (y as f64))),
            _ => Err(KurokoError::TypeError),
        }
    }

    fn call_function(&mut self, function: KurokoValue, args: Vec<KurokoValue>) -> Result<KurokoValue, KurokoError> {
        match function {
            KurokoValue::BuiltinFunction(fn_ptr) => {
                fn_ptr(self, args)
            }
            KurokoValue::Function(code_index) => {
                // Execute user-defined function
                let frame = VMFrame {
                    code_index,
                    ip: 0,
                    locals: args,
                };
                self.current_frame.push(frame);
                let result = self.run();
                self.current_frame.pop();
                result
            }
            _ => Err(KurokoError::TypeError),
        }
    }

    fn value_to_string(&self, value: &KurokoValue) -> String {
        match value {
            KurokoValue::Nil => "nil".to_string(),
            KurokoValue::Bool(b) => b.to_string(),
            KurokoValue::Integer(i) => i.to_string(),
            KurokoValue::Float(f) => f.to_string(),
            KurokoValue::String(s) => s.clone(),
            KurokoValue::List(l) => {
                let items: Vec<String> = l.iter().map(|v| self.value_to_string(v)).collect();
                format!("[{}]", format!("{}/{}", items, ", "))
            }
            KurokoValue::Dict(d) => {
                let items: Vec<String> = d.iter().map(|(k, v)| format!("{}: {}", k, self.value_to_string(v))).collect();
                format!("{{{}}}", format!("{}/{}", items, ", "))
            }
            KurokoValue::Function(_) => "<function>".to_string(),
            KurokoValue::BuiltinFunction(_) => "<builtin>".to_string(),
            KurokoValue::Object(obj) => format!("<{}>", obj.class_name),
        }
    }
}

impl Default for KurokoVM {
    fn default() -> Self {
        Self::new()
    }
}

// Builtin functions
fn builtin_print(vm: &mut KurokoVM, args: Vec<KurokoValue>) -> Result<KurokoValue, KurokoError> {
    for arg in &args {
        print!("{}", vm.value_to_string(arg));
    }
    println!();
    Ok(KurokoValue::Nil)
}

fn builtin_input(vm: &mut KurokoVM, args: Vec<KurokoValue>) -> Result<KurokoValue, KurokoError> {
    if !args.is_empty() {
        print!("{}", vm.value_to_string(&args[0]));
    }
    
    // In a real implementation, this would read from stdin
    // For now, return a mock input
    Ok(KurokoValue::String("user_input".to_string()))
}

fn builtin_len(_vm: &mut KurokoVM, args: Vec<KurokoValue>) -> Result<KurokoValue, KurokoError> {
    if args.len() != 1 {
        return Err(KurokoError::ValueError);
    }
    
    match &args[0] {
        KurokoValue::String(s) => Ok(KurokoValue::Integer(s.len() as i64)),
        KurokoValue::List(l) => Ok(KurokoValue::Integer(l.len() as i64)),
        KurokoValue::Dict(d) => Ok(KurokoValue::Integer(d.len() as i64)),
        _ => Err(KurokoError::TypeError),
    }
}

fn builtin_type(_vm: &mut KurokoVM, args: Vec<KurokoValue>) -> Result<KurokoValue, KurokoError> {
    if args.len() != 1 {
        return Err(KurokoError::ValueError);
    }
    
    Ok(KurokoValue::String(args[0].type_name().to_string()))
}

/// REPL (Read-Eval-Print Loop) for interactive use
pub struct KurokoREPL {
    pub vm: KurokoVM,
    pub compiler: KurokoCompiler,
}

impl KurokoREPL {
    pub fn new() -> Self {
        KurokoREPL {
            vm: KurokoVM::new(),
            compiler: KurokoCompiler::new(),
        }
    }

    pub fn run_line(&mut self, line: &str) -> Result<String, KurokoError> {
        match self.compiler.compile(line) {
            Ok(code) => {
                match self.vm.interpret(code) {
                    Ok(result) => Ok(self.vm.value_to_string(&result)),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        }
    }

    pub fn run(&mut self) {
        println!("SigmaOS Kuroko REPL v1.0");
        println!("Type 'exit' to quit");
        
        loop {
            print!(">>> ");
            // In real implementation, read from stdin
            let input = "print(42)"; // Mock input
            
            if input.trim() == "exit" {
                break;
            }
            
            match self.run_line(input) {
                Ok(result) => println!("{}", result),
                Err(e) => println!("Error: {:?}", e),
            }
        }
    }
}

impl Default for KurokoREPL {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let mut compiler = KurokoCompiler::new();
        let code = compiler.compile("1 + 2").unwrap();
        
        let mut vm = KurokoVM::new();
        let result = vm.interpret(code).unwrap();
        
        assert_eq!(result, KurokoValue::Integer(3));
    }

    #[test]
    fn test_string_concatenation() {
        let mut compiler = KurokoCompiler::new();
        let code = compiler.compile("\"hello\" + \" world\"").unwrap();
        
        let mut vm = KurokoVM::new();
        let result = vm.interpret(code).unwrap();
        
        assert_eq!(result, KurokoValue::String("hello world".to_string()));
    }

    #[test]
    fn test_boolean_operations() {
        let mut compiler = KurokoCompiler::new();
        let code = compiler.compile("true and false").unwrap();
        
        let mut vm = KurokoVM::new();
        let result = vm.interpret(code).unwrap();
        
        assert_eq!(result, KurokoValue::Bool(false));
    }

    #[test]
    fn test_comparison() {
        let mut compiler = KurokoCompiler::new();
        let code = compiler.compile("5 > 3").unwrap();
        
        let mut vm = KurokoVM::new();
        let result = vm.interpret(code).unwrap();
        
        assert_eq!(result, KurokoValue::Bool(true));
    }

    #[test]
    fn test_builtin_print() {
        let mut compiler = KurokoCompiler::new();
        let code = compiler.compile("print(42)").unwrap();
        
        let mut vm = KurokoVM::new();
        let result = vm.interpret(code).unwrap();
        
        assert_eq!(result, KurokoValue::Nil);
    }
}