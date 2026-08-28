#![no_std]
extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
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
pub enum RedirectKind {
    Output,        // >
    Append,        // >>
    Input,         // <
    HereDoc,       // <<
    HereString,    // <<<
    DupOutput,     // >& or 2>&1
    DupInput,      // <&
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redirect {
    pub src_fd: u32,
    pub target_fd: Option<u32>,
    pub path: String,
    pub kind: RedirectKind,
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
            if p.is_none() || p == Some('|') || p == Some(';') {
                break;
            }

            // Check if there is an explicit FD number before redirection symbol (e.g., 2>, 1>)
            let save_pos = self.pos;
            let mut explicit_fd = None;
            let mut digit_count = 0;
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() {
                    digit_count += 1;
                    self.advance();
                } else {
                    break;
                }
            }

            let next_p = self.peek();
            if digit_count > 0 && (next_p == Some('>') || next_p == Some('<')) {
                if let Ok(fd_val) = self.input[save_pos..self.pos].parse::<u32>() {
                    explicit_fd = Some(fd_val);
                }
            } else {
                self.pos = save_pos; // Restore if not redirection prefix
            }

            let cur_p = self.peek();
            if cur_p == Some('>') {
                self.advance();
                let src_fd = explicit_fd.unwrap_or(1);
                if self.peek() == Some('>') {
                    self.advance();
                    self.skip_whitespace();
                    let path = self.parse_word()?;
                    redirects.push(Redirect {
                        src_fd,
                        target_fd: None,
                        path,
                        kind: RedirectKind::Append,
                    });
                } else if self.peek() == Some('&') {
                    self.advance();
                    self.skip_whitespace();
                    let target = self.parse_word()?;
                    let target_fd = target.parse::<u32>().ok();
                    redirects.push(Redirect {
                        src_fd,
                        target_fd,
                        path: target,
                        kind: RedirectKind::DupOutput,
                    });
                } else {
                    self.skip_whitespace();
                    let path = self.parse_word()?;
                    redirects.push(Redirect {
                        src_fd,
                        target_fd: None,
                        path,
                        kind: RedirectKind::Output,
                    });
                }
                continue;
            } else if cur_p == Some('<') {
                self.advance();
                let src_fd = explicit_fd.unwrap_or(0);
                if self.peek() == Some('<') {
                    self.advance();
                    if self.peek() == Some('<') {
                        // <<< HereString
                        self.advance();
                        self.skip_whitespace();
                        let content = self.parse_word()?;
                        redirects.push(Redirect {
                            src_fd,
                            target_fd: None,
                            path: content,
                            kind: RedirectKind::HereString,
                        });
                    } else {
                        // << HereDoc
                        self.skip_whitespace();
                        let delimiter = self.parse_word()?;
                        redirects.push(Redirect {
                            src_fd,
                            target_fd: None,
                            path: delimiter,
                            kind: RedirectKind::HereDoc,
                        });
                    }
                } else if self.peek() == Some('&') {
                    self.advance();
                    self.skip_whitespace();
                    let target = self.parse_word()?;
                    let target_fd = target.parse::<u32>().ok();
                    redirects.push(Redirect {
                        src_fd,
                        target_fd,
                        path: target,
                        kind: RedirectKind::DupInput,
                    });
                } else {
                    self.skip_whitespace();
                    let path = self.parse_word()?;
                    redirects.push(Redirect {
                        src_fd,
                        target_fd: None,
                        path,
                        kind: RedirectKind::Input,
                    });
                }
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
            ShellCommand::Background(_child) => {
                // Background execution logic
                Ok(0)
            }
            ShellCommand::Redirect(child, redir) => {
                // Execute child with redirection context logged in environment / streams
                let status = self.execute_ast(child)?;
                match redir.kind {
                    RedirectKind::Output => {
                        self.env.vars.insert(format!("FD_{}_REDIRECT", redir.src_fd), format!("FILE:{}", redir.path));
                    }
                    RedirectKind::Append => {
                        self.env.vars.insert(format!("FD_{}_REDIRECT", redir.src_fd), format!("APPEND:{}", redir.path));
                    }
                    RedirectKind::Input => {
                        self.env.vars.insert(format!("FD_{}_REDIRECT", redir.src_fd), format!("INPUT:{}", redir.path));
                    }
                    RedirectKind::HereDoc => {
                        self.env.vars.insert(format!("FD_{}_HEREDOC", redir.src_fd), redir.path.clone());
                    }
                    RedirectKind::HereString => {
                        self.env.vars.insert(format!("FD_{}_HERESTRING", redir.src_fd), redir.path.clone());
                    }
                    RedirectKind::DupOutput | RedirectKind::DupInput => {
                        if let Some(target) = redir.target_fd {
                            self.env.vars.insert(format!("FD_{}_REDIRECT", redir.src_fd), format!("FD:{}", target));
                        }
                    }
                }
                Ok(status)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_redirection_parsing() {
        let mut parser = Parser::new("echo hello > output.txt");
        let cmd = parser.parse().unwrap();
        match cmd {
            ShellCommand::Redirect(_child, redir) => {
                assert_eq!(redir.src_fd, 1);
                assert_eq!(redir.kind, RedirectKind::Output);
                assert_eq!(redir.path, "output.txt");
            }
            _ => panic!("Expected Redirect command"),
        }

        let mut parser2 = Parser::new("cat < input.txt");
        let cmd2 = parser2.parse().unwrap();
        match cmd2 {
            ShellCommand::Redirect(_, redir) => {
                assert_eq!(redir.src_fd, 0);
                assert_eq!(redir.kind, RedirectKind::Input);
                assert_eq!(redir.path, "input.txt");
            }
            _ => panic!("Expected Redirect command"),
        }

        let mut parser3 = Parser::new("ls 2> error.log");
        let cmd3 = parser3.parse().unwrap();
        match cmd3 {
            ShellCommand::Redirect(_, redir) => {
                assert_eq!(redir.src_fd, 2);
                assert_eq!(redir.kind, RedirectKind::Output);
                assert_eq!(redir.path, "error.log");
            }
            _ => panic!("Expected Redirect command"),
        }

        let mut parser4 = Parser::new("command 2>&1");
        let cmd4 = parser4.parse().unwrap();
        match cmd4 {
            ShellCommand::Redirect(_, redir) => {
                assert_eq!(redir.src_fd, 2);
                assert_eq!(redir.target_fd, Some(1));
                assert_eq!(redir.kind, RedirectKind::DupOutput);
            }
            _ => panic!("Expected Redirect command"),
        }

        let mut parser5 = Parser::new("grep fn <<< 'fn main()'");
        let cmd5 = parser5.parse().unwrap();
        match cmd5 {
            ShellCommand::Redirect(_, redir) => {
                assert_eq!(redir.src_fd, 0);
                assert_eq!(redir.kind, RedirectKind::HereString);
                assert_eq!(redir.path, "fn main()");
            }
            _ => panic!("Expected Redirect command"),
        }
    }

    #[test]
    fn test_shell_redirection_execution() {
        let mut shell = Shell::new();
        assert!(shell.execute_line("echo hello 2>&1").is_ok());
        assert_eq!(shell.env.vars.get("FD_2_REDIRECT").unwrap(), "FD:1");

        assert!(shell.execute_line("cat << EOF").is_ok());
        assert_eq!(shell.env.vars.get("FD_0_HEREDOC").unwrap(), "EOF");
    }
}
