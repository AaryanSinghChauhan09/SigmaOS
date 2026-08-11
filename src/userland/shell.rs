#![no_std]
extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellCommand {
    Simple(Vec<String>),
    Pipe(Box<ShellCommand>, Box<ShellCommand>),
    And(Box<ShellCommand>, Box<ShellCommand>),
    Or(Box<ShellCommand>, Box<ShellCommand>),
    Sequence(Box<ShellCommand>, Box<ShellCommand>),
    Background(Box<ShellCommand>),
    Redirect(Box<ShellCommand>, Redirect),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redirect {
    pub fd: u32,
    pub path: String,
    pub append: bool,
}

pub struct Environment {
    pub vars: BTreeMap<String, String>,
}

impl Environment {
    pub fn new() -> Self {
        let mut vars = BTreeMap::new();
        vars.insert("PATH".to_string(), "/bin:/usr/bin".to_string());
        vars.insert("HOME".to_string(), "/home/user".to_string());
        Self { vars }
    }

    pub fn expand(&self, input: &str) -> String {
        let mut expanded = String::new();
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '$' {
                let mut var_name = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_alphanumeric() || next_c == '_' {
                        var_name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Some(val) = self.vars.get(&var_name) {
                    expanded.push_str(val);
                }
            } else {
                expanded.push(c);
            }
        }
        expanded
    }
}

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos..].starts_with(char::is_whitespace) {
            self.pos += self.input[self.pos..].chars().next().unwrap().len_utf8();
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    pub fn parse(&mut self) -> Option<ShellCommand> {
        self.parse_sequence()
    }

    fn parse_sequence(&mut self) -> Option<ShellCommand> {
        let mut cmd = self.parse_and_or()?;
        self.skip_whitespace();
        while let Some(c) = self.peek() {
            if c == ';' {
                self.advance();
                if let Some(next) = self.parse_and_or() {
                    cmd = ShellCommand::Sequence(Box::new(cmd), Box::new(next));
                }
            } else {
                break;
            }
            self.skip_whitespace();
        }
        Some(cmd)
    }

    fn parse_and_or(&mut self) -> Option<ShellCommand> {
        let mut cmd = self.parse_pipeline()?;
        self.skip_whitespace();
        while self.pos < self.input.len() {
            if self.input[self.pos..].starts_with("&&") {
                self.pos += 2;
                if let Some(next) = self.parse_pipeline() {
                    cmd = ShellCommand::And(Box::new(cmd), Box::new(next));
                }
            } else if self.input[self.pos..].starts_with("||") {
                self.pos += 2;
                if let Some(next) = self.parse_pipeline() {
                    cmd = ShellCommand::Or(Box::new(cmd), Box::new(next));
                }
            } else {
                break;
            }
            self.skip_whitespace();
        }
        Some(cmd)
    }

    fn parse_pipeline(&mut self) -> Option<ShellCommand> {
        let mut cmd = self.parse_command()?;
        self.skip_whitespace();
        while let Some('|') = self.peek() {
            self.advance();
            if let Some(next) = self.parse_command() {
                cmd = ShellCommand::Pipe(Box::new(cmd), Box::new(next));
            }
        }
        Some(cmd)
    }

    fn parse_command(&mut self) -> Option<ShellCommand> {
        self.skip_whitespace();
        let mut args = Vec::new();
        let mut redirects = Vec::new();

        while self.pos < self.input.len() {
            self.skip_whitespace();
            let p = self.peek();
            if p.is_none() || p == Some('|') || p == Some(';') || p == Some('&') {
                break;
            }
            
            if p == Some('>') {
                self.advance();
                let append = self.peek() == Some('>');
                if append { self.advance(); }
                self.skip_whitespace();
                let path = self.parse_word()?;
                redirects.push(Redirect { fd: 1, path, append });
                continue;
            }

            if let Some(word) = self.parse_word() {
                args.push(word);
            } else {
                break;
            }
        }

        if args.is_empty() && redirects.is_empty() {
            return None;
        }

        let mut cmd = ShellCommand::Simple(args);
        for redir in redirects {
            cmd = ShellCommand::Redirect(Box::new(cmd), redir);
        }
        Some(cmd)
    }

    fn parse_word(&mut self) -> Option<String> {
        self.skip_whitespace();
        if self.pos >= self.input.len() { return None; }
        let mut word = String::new();
        let mut in_single = false;
        let mut in_double = false;
        let mut escape = false;

        while let Some(c) = self.peek() {
            if escape {
                word.push(c);
                escape = false;
                self.advance();
                continue;
            }

            if c == '\\' {
                escape = true;
                self.advance();
                continue;
            }

            if c == '\'' && !in_double {
                in_single = !in_single;
                self.advance();
                continue;
            }

            if c == '"' && !in_single {
                in_double = !in_double;
                self.advance();
                continue;
            }

            if !in_single && !in_double && (c.is_whitespace() || c == '|' || c == '&' || c == ';' || c == '>' || c == '<') {
                break;
            }

            word.push(c);
            self.advance();
        }

        if word.is_empty() { None } else { Some(word) }
    }
}

pub struct Shell {
    pub env: Environment,
}

impl Shell {
    pub fn new() -> Self {
        Self { env: Environment::new() }
    }

    pub fn execute_line(&mut self, line: &str) -> Result<i32, &'static str> {
        let expanded = self.env.expand(line);
        let mut parser = Parser::new(&expanded);
        if let Some(cmd) = parser.parse() {
            self.execute_ast(&cmd)
        } else {
            Ok(0)
        }
    }

    fn execute_ast(&mut self, cmd: &ShellCommand) -> Result<i32, &'static str> {
        match cmd {
            ShellCommand::Simple(args) => {
                if args.is_empty() { return Ok(0); }
                match args[0].as_str() {
                    "export" => {
                        if args.len() > 1 {
                            for arg in &args[1..] {
                                if let Some(idx) = arg.find('=') {
                                    self.env.vars.insert(arg[..idx].to_string(), arg[idx+1..].to_string());
                                }
                            }
                        }
                        Ok(0)
                    }
                    _ => Ok(0) // Dispatch logic for real binaries
                }
            }
            ShellCommand::Pipe(left, right) => {
                self.execute_ast(left)?;
                self.execute_ast(right)
            }
            ShellCommand::And(left, right) => {
                let status = self.execute_ast(left)?;
                if status == 0 { self.execute_ast(right) } else { Ok(status) }
            }
            ShellCommand::Or(left, right) => {
                let status = self.execute_ast(left)?;
                if status != 0 { self.execute_ast(right) } else { Ok(status) }
            }
            ShellCommand::Sequence(left, right) => {
                self.execute_ast(left)?;
                self.execute_ast(right)
            }
            ShellCommand::Background(child) => {
                // Background execution logic
                Ok(0)
            }
            ShellCommand::Redirect(child, _) => {
                // Redirection logic
                self.execute_ast(child)
            }
        }
    }
}
