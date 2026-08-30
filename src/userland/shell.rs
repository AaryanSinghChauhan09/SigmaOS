use alloc::vec;
extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Comprehensive redirection specifications inspired by Linux (Bash/Zsh/Fish) and BSD (Ksh/Sh)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedirectSpec {
    /// Redirect output to file: > or >> or >| (FD, path, append mode, force overwrite/clobber)
    Output {
        fd: u32,
        path: String,
        append: bool,
        force: bool,
    },
    /// Redirect input from file: < (FD, path)
    Input {
        fd: u32,
        path: String,
    },
    /// Duplicate output file descriptor: N>&M (e.g. 2>&1)
    DupOutput {
        src_fd: u32,
        target_fd: u32,
    },
    /// Duplicate input file descriptor: N<&M (e.g. 0<&3)
    DupInput {
        src_fd: u32,
        target_fd: u32,
    },
    /// Close file descriptor: N>&- or N<&-
    CloseFd {
        fd: u32,
    },
    /// Here-document: << DELIM or <<- DELIM (FD, delimiter, strip leading tabs, body content)
    HereDoc {
        fd: u32,
        delimiter: String,
        strip_leading_tabs: bool,
        content: String,
    },
    /// Here-string: <<< "string" (FD, inline string)
    HereString {
        fd: u32,
        content: String,
    },
    /// Combined stdout and stderr output redirection: &> or >&
    CombinedOutput {
        path: String,
        append: bool,
    },
    /// Process substitution input: <(cmd)
    ProcessSubInput {
        fd: u32,
        command: Box<ShellCommand>,
    },
    /// Process substitution output: >(cmd)
    ProcessSubOutput {
        fd: u32,
        command: Box<ShellCommand>,
    },
}

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

/// Legacy alias for backward compatibility

/// Target stream binding for file descriptor redirection
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedirectKind {
    Output,        // >
    Append,        // >>
    Input,         // <
    HereDoc,       // <<
    HereString,    // <<<
    DupOutput,     // >& or 2>&1
    DupInput,      // <&
    CloseFd,       // >&-
    CombinedOutput, // &>
    ProcessSubInput, // < <(command)
    ProcessSubOutput, // > >(command)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redirect {
    pub src_fd: u32,
    pub target_fd: Option<u32>,
    pub path: String,
    pub kind: RedirectKind,
}

impl From<RedirectSpec> for Redirect {
    fn from(spec: RedirectSpec) -> Self {
        match spec {
            RedirectSpec::Output { fd, path, append, .. } => Redirect {
                src_fd: fd,
                target_fd: None,
                path,
                kind: if append { RedirectKind::Append } else { RedirectKind::Output },
            },
            RedirectSpec::Input { fd, path } => Redirect {
                src_fd: fd,
                target_fd: None,
                path,
                kind: RedirectKind::Input,
            },
            RedirectSpec::DupOutput { src_fd, target_fd } => Redirect {
                src_fd,
                target_fd: Some(target_fd),
                path: target_fd.to_string(),
                kind: RedirectKind::DupOutput,
            },
            RedirectSpec::DupInput { src_fd, target_fd } => Redirect {
                src_fd,
                target_fd: Some(target_fd),
                path: target_fd.to_string(),
                kind: RedirectKind::DupInput,
            },
            RedirectSpec::CloseFd { fd } => Redirect {
                src_fd: fd,
                target_fd: None,
                path: String::new(),
                kind: RedirectKind::Output,
            },
            RedirectSpec::HereDoc { fd, delimiter, .. } => Redirect {
                src_fd: fd,
                target_fd: None,
                path: delimiter,
                kind: RedirectKind::HereDoc,
            },
            RedirectSpec::HereString { fd, content } => Redirect {
                src_fd: fd,
                target_fd: None,
                path: content,
                kind: RedirectKind::HereString,
            },
            RedirectSpec::CombinedOutput { path, .. } => Redirect {
                src_fd: 1,
                target_fd: None,
                path,
                kind: RedirectKind::Output,
            },
            RedirectSpec::ProcessSubInput { fd, .. } | RedirectSpec::ProcessSubOutput { fd, .. } => Redirect {
                src_fd: fd,
                target_fd: None,
                path: String::new(),
                kind: RedirectKind::Output,
            },
        }
    }
}


pub struct RedirectionEngine {
    pub streams: BTreeMap<u32, Vec<u8>>,
    pub redirection_log: Vec<String>,
}

impl RedirectionEngine {
    pub fn new() -> Self {
        Self {
            streams: BTreeMap::new(),
            redirection_log: Vec::new(),
        }
    }

    pub fn write_fd(&mut self, fd: u32, data: &[u8]) {
        self.streams.entry(fd).or_default().extend_from_slice(data);
    }

    pub fn read_fd(&self, fd: u32) -> Option<&[u8]> {
        self.streams.get(&fd).map(|v| v.as_slice())
    }

    pub fn get_captured_output(&self, fd: u32) -> Option<&[u8]> {
        self.read_fd(fd)
    }
}

impl Default for RedirectionEngine {
    fn default() -> Self {
        Self::new()
    }
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

    /// Bash/Zsh-inspired arithmetic expansion `$(( 1 + 2 ))` evaluation
    pub fn eval_arithmetic_expr(expr: &str) -> i64 {
        let trimmed = expr.trim();
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        if tokens.len() == 3 {
            let left = tokens[0].parse::<i64>().unwrap_or(0);
            let right = tokens[2].parse::<i64>().unwrap_or(0);
            match tokens[1] {
                "+" => left + right,
                "-" => left - right,
                "*" => left * right,
                "/" if right != 0 => left / right,
                "%" if right != 0 => left % right,
                _ => 0,
            }
        } else if let Ok(val) = trimmed.parse::<i64>() {
            val
        } else {
            0
        }
    }

    /// Performs environment variable and $(( arithmetic )) expansion
    pub fn expand(&self, input: &str) -> String {
        let mut expanded = String::new();
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '$' {
                if chars.peek() == Some(&'(') {
                    chars.next(); // consume '('
                    if chars.peek() == Some(&'(') {
                        chars.next(); // consume '('
                        let mut arith_body = String::new();
                        let mut closed = false;
                        while let Some(ac) = chars.next() {
                            if ac == ')' && chars.peek() == Some(&')') {
                                chars.next(); // consume second ')'
                                closed = true;
                                break;
                            }
                            arith_body.push(ac);
                        }
                        if closed {
                            let val = Self::eval_arithmetic_expr(&arith_body);
                            expanded.push_str(&val.to_string());
                            continue;
                        }
                    }
                }

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

    /// Bash/Zsh-inspired brace expansion: `file_{a,b}.txt` -> `["file_a.txt", "file_b.txt"]`
    pub fn expand_braces(input: &str) -> Vec<String> {
        if let (Some(start), Some(end)) = (input.find('{'), input.find('}')) {
            if start < end {
                let prefix = &input[..start];
                let suffix = &input[end + 1..];
                let body = &input[start + 1..end];
                let options: Vec<&str> = body.split(',').collect();
                let mut results = Vec::new();
                for opt in options {
                    let combined = alloc::format!("{}{}{}", prefix, opt.trim(), suffix);
                    results.extend(Self::expand_braces(&combined));
                }
                return results;
            }
        }
        alloc::vec![input.to_string()]
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
        while self.pos < self.input.len()
            && self.input[self.pos..].starts_with(char::is_whitespace)
        {
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

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
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
            if self.starts_with("&&") {
                self.pos += 2;
                if let Some(next) = self.parse_pipeline() {
                    cmd = ShellCommand::And(Box::new(cmd), Box::new(next));
                }
            } else if self.starts_with("||") {
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
                    redirects.push(Redirect::from(RedirectSpec::Output {
                        fd: src_fd,
                        path,
                        append: true,
                        force: false,
                    }));
                } else if self.peek() == Some('&') {
                    self.advance();
                    self.skip_whitespace();
                    let target = self.parse_word()?;
                    let target_fd = target.parse::<u32>().unwrap_or(1);
                    redirects.push(Redirect::from(RedirectSpec::DupOutput {
                        src_fd,
                        target_fd,
                    }));
                } else {
                    self.skip_whitespace();
                    let path = self.parse_word()?;
                    redirects.push(Redirect::from(RedirectSpec::Output {
                        fd: src_fd,
                        path,
                        append: false,
                        force: false,
                    }));
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
                        redirects.push(Redirect::from(RedirectSpec::HereString {
                            fd: src_fd,
                            content,
                        }));
                    } else {
                        // << HereDoc
                        self.skip_whitespace();
                        let delimiter = self.parse_word()?;
                        redirects.push(Redirect::from(RedirectSpec::HereDoc {
                            fd: src_fd,
                            delimiter: delimiter.clone(),
                            strip_leading_tabs: false,
                            content: delimiter,
                        }));
                    }
                } else if self.peek() == Some('&') {
                    self.advance();
                    self.skip_whitespace();
                    let target = self.parse_word()?;
                    let target_fd = target.parse::<u32>().unwrap_or(0);
                    redirects.push(Redirect::from(RedirectSpec::DupInput {
                        src_fd,
                        target_fd,
                    }));
                } else {
                    self.skip_whitespace();
                    let path = self.parse_word()?;
                    redirects.push(Redirect::from(RedirectSpec::Input {
                        fd: src_fd,
                        path,
                    }));
                }
                continue;
            }

            // Check for explicit FD prefix e.g., "2>", "1>", "2>&1", "0<"
            let mut explicit_fd: Option<u32> = None;
            let current_pos = self.pos;
            if let Some(fd_val) = self.try_parse_fd_prefix() {
                explicit_fd = Some(fd_val);
            }

            let peek_c = self.peek();
            if peek_c == Some('>') || peek_c == Some('<') || self.starts_with("&>") || self.starts_with(">&") {
                if let Some(spec) = self.parse_redirect_operator(explicit_fd) {
                    redirects.push(Redirect::from(spec));
                    continue;
                } else {
                    self.pos = current_pos;
                }
            } else if explicit_fd.is_some() {
                // Not a redirection, rewind
                self.pos = current_pos;
            }

            if let Some(word) = self.parse_word() {
                let expanded_words = Environment::expand_braces(&word);
                args.extend(expanded_words);
            } else {
                break;
            }
        }

        if args.is_empty() && redirects.is_empty() {
            return None;
        }

        let mut cmd = ShellCommand::Simple(args);
        for redir in redirects {
            cmd = ShellCommand::Redirect(Box::new(cmd), redir.into());
        }
        Some(cmd)
    }

    fn try_parse_fd_prefix(&mut self) -> Option<u32> {
        let mut save_pos = self.pos;
        let mut num: u32 = 0;
        let mut digits = 0;

        while save_pos < self.input.len() {
            let c = self.input[save_pos..].chars().next().unwrap();
            if c.is_ascii_digit() {
                num = num * 10 + (c as u32 - '0' as u32);
                digits += 1;
                save_pos += c.len_utf8();
            } else {
                break;
            }
        }

        if digits > 0 && save_pos < self.input.len() {
            let next_c = self.input[save_pos..].chars().next().unwrap();
            if next_c == '>' || next_c == '<' {
                self.pos = save_pos;
                return Some(num);
            }
        }

        None
    }

    fn parse_redirect_operator(&mut self, explicit_fd: Option<u32>) -> Option<RedirectSpec> {
        // Combined stdout & stderr: &> or &>>
        if self.starts_with("&>") {
            let append = self.starts_with("&>>");
            self.pos += if append { 3 } else { 2 };
            self.skip_whitespace();
            let path = self.parse_word()?;
            return Some(RedirectSpec::CombinedOutput { path, append });
        }

        // Here-string: <<<
        if self.starts_with("<<<") {
            self.pos += 3;
            self.skip_whitespace();
            let content = self.parse_word()?;
            let fd = explicit_fd.unwrap_or(0);
            return Some(RedirectSpec::HereString { fd, content });
        }

        // Here-doc: <<- or <<
        if self.starts_with("<<-") || self.starts_with("<<") {
            let strip_tabs = self.starts_with("<<-");
            self.pos += if strip_tabs { 3 } else { 2 };
            self.skip_whitespace();
            let delimiter = self.parse_word()?;
            let fd = explicit_fd.unwrap_or(0);
            let content = self.collect_here_doc_body(&delimiter);
            return Some(RedirectSpec::HereDoc {
                fd,
                delimiter,
                strip_leading_tabs: strip_tabs,
                content,
            });
        }

        // FD duplication / closing: >&N, <&N, >&-, <&-
        if self.starts_with(">&") || self.starts_with("<&") {
            let is_output = self.starts_with(">&");
            let op_pos = self.pos;
            self.pos += 2;
            self.skip_whitespace();

            if self.peek() == Some('-') {
                self.advance();
                let fd = explicit_fd.unwrap_or(if is_output { 1 } else { 0 });
                return Some(RedirectSpec::CloseFd { fd });
            }

            if let Some(target_fd) = self.parse_u32() {
                let src_fd = explicit_fd.unwrap_or(if is_output { 1 } else { 0 });
                if is_output {
                    return Some(RedirectSpec::DupOutput { src_fd, target_fd });
                } else {
                    return Some(RedirectSpec::DupInput { src_fd, target_fd });
                }
            }

            // If not followed by digits or `-`, rewind and treat `>& file` as CombinedOutput
            self.pos = op_pos + 2;
            self.skip_whitespace();
            if let Some(path) = self.parse_word() {
                return Some(RedirectSpec::CombinedOutput { path, append: false });
            }
        }

        // Output redirection: >>, >|, >
        if self.starts_with(">>") {
            self.pos += 2;
            self.skip_whitespace();
            let path = self.parse_word()?;
            let fd = explicit_fd.unwrap_or(1);
            return Some(RedirectSpec::Output {
                fd,
                path,
                append: true,
                force: false,
            });
        }

        if self.starts_with(">|") {
            self.pos += 2;
            self.skip_whitespace();
            let path = self.parse_word()?;
            let fd = explicit_fd.unwrap_or(1);
            return Some(RedirectSpec::Output {
                fd,
                path,
                append: false,
                force: true,
            });
        }

        // Standard output redirect: >
        if self.peek() == Some('>') {
            self.advance();
            self.skip_whitespace();
            let path = self.parse_word()?;
            let fd = explicit_fd.unwrap_or(1);
            return Some(RedirectSpec::Output {
                fd,
                path,
                append: false,
                force: false,
            });
        }

        // Standard input redirect: <
        if self.peek() == Some('<') {
            self.advance();
            self.skip_whitespace();
            let path = self.parse_word()?;
            let fd = explicit_fd.unwrap_or(0);
            return Some(RedirectSpec::Input { fd, path });
        }

        None
    }

    fn parse_u32(&mut self) -> Option<u32> {
        let mut num: u32 = 0;
        let mut digits = 0;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                num = num * 10 + (c as u32 - '0' as u32);
                digits += 1;
                self.advance();
            } else {
                break;
            }
        }

        if digits > 0 {
            Some(num)
        } else {
            None
        }
    }

    fn collect_here_doc_body(&mut self, delimiter: &str) -> String {
        let mut body = String::new();
        self.skip_whitespace();

        while self.pos < self.input.len() {
            let rest = &self.input[self.pos..];
            if rest.starts_with(delimiter) {
                let after = &rest[delimiter.len()..];
                if after.is_empty() || after.starts_with('\n') || after.starts_with(';') {
                    self.pos += delimiter.len();
                    break;
                }
            }

            if let Some(c) = self.advance() {
                body.push(c);
            }
        }

        body
    }

    fn parse_until_matching_paren(&mut self) -> Option<String> {
        let mut depth = 1;
        let mut result = String::new();

        while let Some(c) = self.advance() {
            if c == '(' {
                depth += 1;
            } else if c == ')' {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            result.push(c);
        }

        if depth == 0 {
            Some(result)
        } else {
            None
        }
    }

    fn parse_word(&mut self) -> Option<String> {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return None;
        }
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

            if !in_single
                && !in_double
                && (c.is_whitespace() || c == '|' || c == '&' || c == ';' || c == '>' || c == '<')
            {
                break;
            }

            word.push(c);
            self.advance();
        }

        if word.is_empty() {
            None
        } else {
            Some(word)
        }
    }
}


pub struct Shell {
    pub env: Environment,
    pub redirection_engine: RedirectionEngine,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            redirection_engine: RedirectionEngine::new(),
        }
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

    pub fn execute_ast(&mut self, cmd: &ShellCommand) -> Result<i32, &'static str> {
        match cmd {
            ShellCommand::Simple(args) => {
                if args.is_empty() {
                    return Ok(0);
                }
                match args[0].as_str() {
                    "export" => {
                        if args.len() > 1 {
                            for arg in &args[1..] {
                                if let Some(idx) = arg.find('=') {
                                    self.env
                                        .vars
                                        .insert(arg[..idx].to_string(), arg[idx + 1..].to_string());
                                }
                            }
                        }
                        Ok(0)
                    }
                    "echo" => {
                        let text = args[1..].join(" ");
                        let out_str = format!("{}\n", text);
                        self.redirection_engine.write_fd(1, out_str.as_bytes());
                        Ok(0)
                    }
                    "cat" => {
                        let stdin_bytes = self.redirection_engine.read_fd(0).map(|b: &[u8]| b.to_vec());
                        if let Some(bytes) = stdin_bytes {
                            self.redirection_engine.write_fd(1, &bytes);
                        } else {
                            self.redirection_engine
                                .write_fd(1, b"[cat: reading standard input]\n");
                        }
                        Ok(0)
                    }
                    "pwd" => {
                        let cwd = self
                            .env
                            .vars
                            .get("PWD")
                            .cloned()
                            .unwrap_or_else(|| "/home/user".to_string());
                        let out_str = format!("{}\n", cwd);
                        self.redirection_engine.write_fd(1, out_str.as_bytes());
                        Ok(0)
                    }
                    _ => Ok(0), // Builtin / external binary dispatch
                }
            }
            ShellCommand::Pipe(left, right) => {
                self.execute_ast(left)?;
                self.execute_ast(right)
            }
            ShellCommand::And(left, right) => {
                let status = self.execute_ast(left)?;
                if status == 0 {
                    self.execute_ast(right)
                } else {
                    Ok(status)
                }
            }
            ShellCommand::Or(left, right) => {
                let status = self.execute_ast(left)?;
                if status != 0 {
                    self.execute_ast(right)
                } else {
                    Ok(status)
                }
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
                    RedirectKind::CloseFd => {
                        self.env.vars.insert(format!("FD_{}_REDIRECT", redir.src_fd), "CLOSED".to_string());
                    }
                    RedirectKind::CombinedOutput => {
                        self.env.vars.insert("FD_COMBINED_REDIRECT".to_string(), format!("FILE:{}", redir.path));
                    }
                    _ => {}
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
                assert_eq!(redir.path, "output.txt");
                assert_eq!(redir.kind, RedirectKind::Output);
            }
            _ => panic!("Expected Redirect command"),
        }

        let mut parser2 = Parser::new("cat < input.txt");
        let cmd2 = parser2.parse().unwrap();
        match cmd2 {
            ShellCommand::Redirect(_, redir) => {
                assert_eq!(redir.src_fd, 0);
                assert_eq!(redir.path, "input.txt");
                assert_eq!(redir.kind, RedirectKind::Input);
            }
            _ => panic!("Expected Redirect command"),
        }

        let mut parser3 = Parser::new("ls 2> error.log");
        let cmd3 = parser3.parse().unwrap();
        match cmd3 {
            ShellCommand::Redirect(_, redir) => {
                assert_eq!(redir.src_fd, 2);
                assert_eq!(redir.path, "error.log");
                assert_eq!(redir.kind, RedirectKind::Output);
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

        // POSIX: `<<<` consumes exactly one word, so a multi-word here-string
        // must be quoted. parse_word strips the quotes.
        let mut parser5 = Parser::new("grep fn <<< \"fn main()\"");
        let cmd5 = parser5.parse().unwrap();
        match cmd5 {
            ShellCommand::Redirect(_, redir) => {
                assert_eq!(redir.src_fd, 0);
                assert_eq!(redir.path, "fn main()");
                assert_eq!(redir.kind, RedirectKind::HereString);
            }
            _ => panic!("Expected Redirect command"),
        }

        // Unquoted here-strings stop at the first word, matching bash.
        let mut parser6 = Parser::new("grep fn <<< sovereign");
        let cmd6 = parser6.parse().unwrap();
        match cmd6 {
            ShellCommand::Redirect(_, redir) => {
                assert_eq!(redir.path, "sovereign");
                assert_eq!(redir.kind, RedirectKind::HereString);
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
