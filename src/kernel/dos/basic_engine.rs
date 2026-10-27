// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - BASIC Interpreter Engine (GW-BASIC / QBASIC compatible)
// Principles: INTERPRETER DESIGN, MODULARITY, COMPOSITION OVER INHERITANCE

use alloc::{collections::BTreeMap, string::String, vec::Vec};

/// BASIC token types
#[derive(Debug, Clone, PartialEq)]
pub enum BasicToken {
    // Keywords
    Print,
    Input,
    Let,
    If,
    Then,
    Else,
    For,
    To,
    Step,
    Next,
    Goto,
    Gosub,
    Return,
    Rem,
    Dim,
    End,
    Run,
    List,
    New,
    Save,
    Load,
    And,
    Or,
    Not,
    // Literals
    NumberLit(i64),
    FloatLit(f64),
    StringLit(String),
    Ident(String),
    LineNumber(u32),
    // Operators & punctuation
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Exp,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    LParen,
    RParen,
    Comma,
    Semicolon,
    Colon,
    Quote,
    Newline,
    EOF,
}

/// Runtime value
#[derive(Debug, Clone, PartialEq)]
pub enum BasicValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl BasicValue {
    pub fn to_bool(&self) -> bool {
        match self {
            BasicValue::Bool(b) => *b,
            BasicValue::Int(n) => *n != 0,
            BasicValue::Float(f) => *f != 0.0,
            BasicValue::Str(s) => !s.is_empty(),
            BasicValue::Nil => false,
        }
    }

    pub fn to_int(&self) -> i64 {
        match self {
            BasicValue::Int(n) => *n,
            BasicValue::Float(f) => *f as i64,
            BasicValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub fn to_string_repr(&self) -> String {
        match self {
            BasicValue::Int(n) => alloc::format!("{}", n),
            BasicValue::Float(f) => alloc::format!("{}", f),
            BasicValue::Bool(b) => {
                if *b {
                    String::from("-1")
                } else {
                    String::from("0")
                }
            }
            BasicValue::Str(s) => s.clone(),
            BasicValue::Nil => String::new(),
        }
    }
}

/// A single BASIC program line
#[derive(Debug, Clone)]
pub struct BasicLine {
    pub number: u32,
    pub source: String,
}

/// Stored BASIC program
#[derive(Debug, Default)]
pub struct BasicProgram {
    pub lines: BTreeMap<u32, BasicLine>,
}

impl BasicProgram {
    pub fn new() -> Self {
        Self {
            lines: BTreeMap::new(),
        }
    }

    pub fn add_line(&mut self, number: u32, source: &str) {
        if source.trim().is_empty() {
            self.lines.remove(&number);
        } else {
            self.lines.insert(
                number,
                BasicLine {
                    number,
                    source: source.into(),
                },
            );
        }
    }

    pub fn list(&self) -> Vec<&BasicLine> {
        self.lines.values().collect()
    }

    pub fn line_numbers(&self) -> Vec<u32> {
        self.lines.keys().copied().collect()
    }
}

/// BASIC interpreter state
pub struct BasicInterpreter {
    pub program: BasicProgram,
    pub variables: BTreeMap<String, BasicValue>,
    pub output: Vec<String>,
    pub call_stack: Vec<u32>, // GOSUB return addresses
    pub data_pointer: usize,
    pub running: bool,
    pub for_stacks: Vec<ForLoop>,
}

#[derive(Debug, Clone)]
pub struct ForLoop {
    pub var: String,
    pub limit: i64,
    pub step: i64,
    pub return_line: u32,
}

impl BasicInterpreter {
    pub fn new() -> Self {
        Self {
            program: BasicProgram::new(),
            variables: BTreeMap::new(),
            output: Vec::new(),
            call_stack: Vec::new(),
            data_pointer: 0,
            running: false,
            for_stacks: Vec::new(),
        }
    }

    pub fn load_program(&mut self, program: BasicProgram) {
        self.program = program;
    }

    pub fn set_variable(&mut self, name: &str, value: BasicValue) {
        self.variables.insert(name.to_ascii_uppercase(), value);
    }

    pub fn get_variable(&self, name: &str) -> BasicValue {
        self.variables
            .get(&name.to_ascii_uppercase())
            .cloned()
            .unwrap_or(BasicValue::Int(0))
    }

    /// Execute a single BASIC statement line
    pub fn execute_line(&mut self, source: &str) -> Result<Option<u32>, String> {
        let trimmed = source.trim();
        let upper = trimmed.to_ascii_uppercase();

        if upper.starts_with("REM") || upper.starts_with("'") {
            return Ok(None); // comment, skip
        }

        if upper.starts_with("PRINT") {
            let expr = trimmed[5..].trim();
            let val = self.eval_expr(expr);
            self.output.push(val.to_string_repr());
            return Ok(None);
        }

        if upper.starts_with("LET ")
            || (!upper.contains(' ') == false && upper.contains('=') && !upper.starts_with("IF"))
        {
            let stmt = if upper.starts_with("LET ") {
                &trimmed[4..]
            } else {
                trimmed
            };
            if let Some(eq_pos) = stmt.find('=') {
                let var = stmt[..eq_pos].trim();
                let expr = stmt[eq_pos + 1..].trim();
                let val = self.eval_expr(expr);
                self.set_variable(var, val);
            }
            return Ok(None);
        }

        if upper.starts_with("END") {
            self.running = false;
            return Ok(None);
        }

        if upper.starts_with("GOTO ") {
            let target: u32 = trimmed[5..]
                .trim()
                .parse()
                .map_err(|_| String::from("invalid GOTO target"))?;
            return Ok(Some(target));
        }

        if upper.starts_with("GOSUB ") {
            let target: u32 = trimmed[6..]
                .trim()
                .parse()
                .map_err(|_| String::from("invalid GOSUB target"))?;
            return Ok(Some(target));
        }

        if upper.starts_with("RETURN") {
            let ret = self
                .call_stack
                .pop()
                .ok_or_else(|| String::from("RETURN without GOSUB"))?;
            return Ok(Some(ret));
        }

        Ok(None)
    }

    /// Evaluate an expression to a BasicValue
    pub fn eval_expr(&self, expr: &str) -> BasicValue {
        let trimmed = expr.trim();

        // String literal
        if trimmed.starts_with('"') && trimmed.ends_with('"') {
            return BasicValue::Str(trimmed[1..trimmed.len() - 1].into());
        }

        // Numeric literal
        if let Ok(n) = trimmed.parse::<i64>() {
            return BasicValue::Int(n);
        }
        if let Ok(f) = trimmed.parse::<f64>() {
            return BasicValue::Float(f);
        }

        // Simple addition/subtraction
        if let Some(pos) = trimmed.rfind('+') {
            let left = self.eval_expr(&trimmed[..pos]);
            let right = self.eval_expr(&trimmed[pos + 1..]);
            return match (left, right) {
                (BasicValue::Int(a), BasicValue::Int(b)) => BasicValue::Int(a + b),
                (BasicValue::Float(a), BasicValue::Float(b)) => BasicValue::Float(a + b),
                (BasicValue::Str(a), BasicValue::Str(b)) => BasicValue::Str(a + &b),
                (BasicValue::Int(a), BasicValue::Float(b)) => BasicValue::Float(a as f64 + b),
                _ => BasicValue::Int(0),
            };
        }

        // Variable lookup
        self.get_variable(trimmed)
    }

    /// Run the loaded program
    pub fn run(&mut self) -> Vec<String> {
        self.running = true;
        self.output.clear();
        let line_numbers = self.program.line_numbers();
        let mut idx = 0;

        while self.running && idx < line_numbers.len() {
            let ln = line_numbers[idx];
            let source = self.program.lines[&ln].source.clone();
            match self.execute_line(&source) {
                Ok(Some(goto_target)) => {
                    if let Some(pos) = line_numbers.iter().position(|&n| n == goto_target) {
                        idx = pos;
                        continue;
                    }
                }
                Ok(None) => {}
                Err(_) => {
                    self.running = false;
                    break;
                }
            }
            idx += 1;
        }
        self.running = false;
        self.output.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_string_literal() {
        let mut interp = BasicInterpreter::new();
        let mut prog = BasicProgram::new();
        prog.add_line(10, "PRINT \"HELLO WORLD\"");
        prog.add_line(20, "END");
        interp.load_program(prog);
        let out = interp.run();
        assert_eq!(out[0], "HELLO WORLD");
    }

    #[test]
    fn test_let_and_print_variable() {
        let mut interp = BasicInterpreter::new();
        let mut prog = BasicProgram::new();
        prog.add_line(10, "LET X = 42");
        prog.add_line(20, "PRINT X");
        prog.add_line(30, "END");
        interp.load_program(prog);
        let out = interp.run();
        assert_eq!(out[0], "42");
    }

    #[test]
    fn test_expression_addition() {
        let interp = BasicInterpreter::new();
        let result = interp.eval_expr("10+32");
        assert_eq!(result, BasicValue::Int(42));
    }

    #[test]
    fn test_rem_is_skipped() {
        let mut interp = BasicInterpreter::new();
        let mut prog = BasicProgram::new();
        prog.add_line(10, "REM this is a comment");
        prog.add_line(20, "END");
        interp.load_program(prog);
        let out = interp.run();
        assert!(out.is_empty());
    }
}
