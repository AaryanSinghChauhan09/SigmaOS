// sigma-sh scripting: Variables, if/else, for, while, functions, .sigma scripts

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct ShellEnv {
    pub vars: HashMap<String, String>,
    pub aliases: HashMap<String, String>,
    pub functions: HashMap<String, Vec<String>>,
    pub last_exit: i32,
}

impl ShellEnv {
    pub fn new() -> Self {
        let mut vars = HashMap::new();
        // Seed from process environment
        for (k, v) in std::env::vars() {
            vars.insert(k, v);
        }
        Self {
            vars,
            aliases: HashMap::new(),
            functions: HashMap::new(),
            last_exit: 0,
        }
    }

    /// Expand $VAR, ${VAR}, ${VAR:-default}, $((expr)), $(...), $RANDOM, $LINENO in a string
    pub fn expand_vars(&self, input: &str) -> String {
        self.expand_vars_with_lineno(input, 0)
    }

    pub fn expand_vars_with_lineno(&self, input: &str, lineno: usize) -> String {
        let mut out = String::with_capacity(input.len());
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '$' {
                match chars.peek() {
                    // $(( arithmetic ))
                    Some('(') => {
                        // peek ahead to see if it's $(( or $(
                        let mut lookahead: Vec<char> = Vec::new();
                        // consume first '('
                        chars.next();
                        lookahead.push('(');
                        if chars.peek() == Some(&'(') {
                            chars.next(); // consume second '('
                            // collect until '))'
                            let mut expr = String::new();
                            let mut depth = 2usize;
                            loop {
                                match chars.next() {
                                    Some(')') => {
                                        depth -= 1;
                                        if depth == 0 { break; }
                                        expr.push(')');
                                    }
                                    Some(other) => expr.push(other),
                                    None => break,
                                }
                            }
                            // Consume trailing ')'
                            if chars.peek() == Some(&')') { chars.next(); }
                            let result = eval_arithmetic(&expr, self);
                            out.push_str(&result.to_string());
                        } else {
                            // $(...) command substitution
                            let mut cmd = String::new();
                            let mut depth = 1usize;
                            loop {
                                match chars.next() {
                                    Some('(') => { depth += 1; cmd.push('('); }
                                    Some(')') => {
                                        depth -= 1;
                                        if depth == 0 { break; }
                                        cmd.push(')');
                                    }
                                    Some(other) => cmd.push(other),
                                    None => break,
                                }
                            }
                            // Expand variables inside the command string first
                            let expanded_cmd = self.expand_vars(&cmd);
                            let result = command_substitution(&expanded_cmd);
                            out.push_str(&result);
                        }
                    }
                    Some('{') => {
                        chars.next(); // consume '{'
                        let mut name = String::new();
                        let mut default = None;
                        let mut colon_minus = false;

                        for ch in chars.by_ref() {
                            if ch == '}' { break; }
                            if ch == ':' && !colon_minus {
                                colon_minus = true;
                                default = Some(String::new());
                            } else if ch == '-' && colon_minus && default.as_ref().map(|d| d.is_empty()).unwrap_or(false) {
                                // ${VAR:-default} — skip the '-'
                            } else if colon_minus {
                                if let Some(ref mut d) = default { d.push(ch); }
                            } else {
                                name.push(ch);
                            }
                        }

                        let val = self.vars.get(&name).cloned()
                            .or(std::env::var(&name).ok())
                            .unwrap_or_else(|| default.unwrap_or_default());
                        out.push_str(&val);
                    }
                    Some('?') => {
                        chars.next();
                        out.push_str(&self.last_exit.to_string());
                    }
                    Some('$') => {
                        // $$ — current PID
                        chars.next();
                        out.push_str(&std::process::id().to_string());
                    }
                    Some('!') => {
                        // $! — last background PID (simplified: 0)
                        chars.next();
                        out.push('0');
                    }
                    Some('#') => {
                        // $# — argument count
                        chars.next();
                        let argc = self.vars.get("__argc__")
                            .and_then(|s| s.parse::<usize>().ok())
                            .unwrap_or(0);
                        out.push_str(&argc.to_string());
                    }
                    Some('@') | Some('*') => {
                        // $@ / $* — all positional args
                        chars.next();
                        let args = self.vars.get("__args__").cloned().unwrap_or_default();
                        out.push_str(&args);
                    }
                    Some(c) if c.is_alphanumeric() || *c == '_' => {
                        let mut name = String::new();
                        while let Some(&ch) = chars.peek() {
                            if ch.is_alphanumeric() || ch == '_' {
                                name.push(ch);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        let val = match name.as_str() {
                            // Special read-only variables
                            "RANDOM"  => {
                                use std::time::{SystemTime, UNIX_EPOCH};
                                let seed = SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .map(|d| d.subsec_nanos())
                                    .unwrap_or(0);
                                ((seed ^ (seed >> 13) ^ (seed << 7)) % 32768).to_string()
                            }
                            "LINENO"  => lineno.to_string(),
                            "SECONDS" => {
                                use std::time::{SystemTime, UNIX_EPOCH};
                                SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .map(|d| d.as_secs().to_string())
                                    .unwrap_or_else(|_| "0".to_string())
                            }
                            "PWD"     => std::env::current_dir()
                                .map(|p| p.display().to_string())
                                .unwrap_or_default(),
                            "UID"     => "0".to_string(),
                            "EUID"    => "0".to_string(),
                            "BASHPID" | "PPID" => std::process::id().to_string(),
                            _ => self.vars.get(&name).cloned()
                                .or(std::env::var(&name).ok())
                                .unwrap_or_default(),
                        };
                        out.push_str(&val);
                    }
                    _ => out.push('$'),
                }
            } else {
                out.push(c);
            }
        }
        out
    }
}

/// Evaluate a simple arithmetic expression: +, -, *, /, %, parens, variables
fn eval_arithmetic(expr: &str, env: &ShellEnv) -> i64 {
    // First expand variables in the expression
    let expanded = expr.trim().to_string();
    // Replace $VAR and VAR with their numeric values
    let tokens = tokenise_arith(&expanded, env);
    parse_arith_expr(&tokens, &mut 0)
}

fn tokenise_arith(s: &str, env: &ShellEnv) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => { chars.next(); }
            '+' | '-' | '*' | '/' | '%' | '(' | ')' => {
                tokens.push(c.to_string()); chars.next();
            }
            '$' => {
                chars.next();
                let mut name = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' { name.push(ch); chars.next(); }
                    else { break; }
                }
                let val = env.vars.get(&name).cloned()
                    .or_else(|| std::env::var(&name).ok())
                    .unwrap_or_else(|| "0".to_string());
                tokens.push(val);
            }
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() { num.push(ch); chars.next(); }
                    else { break; }
                }
                tokens.push(num);
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                // bare variable name (no $)
                let mut name = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' { name.push(ch); chars.next(); }
                    else { break; }
                }
                let val = env.vars.get(&name).cloned()
                    .or_else(|| std::env::var(&name).ok())
                    .unwrap_or_else(|| "0".to_string());
                tokens.push(val);
            }
            _ => { chars.next(); }
        }
    }
    tokens
}

fn parse_arith_expr(tokens: &[String], pos: &mut usize) -> i64 {
    let mut left = parse_arith_term(tokens, pos);
    while *pos < tokens.len() {
        match tokens[*pos].as_str() {
            "+" => { *pos += 1; left += parse_arith_term(tokens, pos); }
            "-" => { *pos += 1; left -= parse_arith_term(tokens, pos); }
            _ => break,
        }
    }
    left
}

fn parse_arith_term(tokens: &[String], pos: &mut usize) -> i64 {
    let mut left = parse_arith_factor(tokens, pos);
    while *pos < tokens.len() {
        match tokens[*pos].as_str() {
            "*" => { *pos += 1; left *= parse_arith_factor(tokens, pos); }
            "/" => {
                *pos += 1;
                let r = parse_arith_factor(tokens, pos);
                left = if r != 0 { left / r } else { 0 };
            }
            "%" => {
                *pos += 1;
                let r = parse_arith_factor(tokens, pos);
                left = if r != 0 { left % r } else { 0 };
            }
            _ => break,
        }
    }
    left
}

fn parse_arith_factor(tokens: &[String], pos: &mut usize) -> i64 {
    if *pos >= tokens.len() { return 0; }
    match tokens[*pos].as_str() {
        "(" => {
            *pos += 1;
            let val = parse_arith_expr(tokens, pos);
            if *pos < tokens.len() && tokens[*pos] == ")" { *pos += 1; }
            val
        }
        "-" => {
            *pos += 1;
            -parse_arith_factor(tokens, pos)
        }
        _ => {
            let val = tokens[*pos].parse::<i64>().unwrap_or(0);
            *pos += 1;
            val
        }
    }
}

/// Run a command string and return its trimmed stdout (for $(...) substitution)
fn command_substitution(cmd: &str) -> String {
    use std::process::Command;
    // Try the shell first; fall back gracefully
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output();
    match output {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            s.trim_end_matches('\n').to_string()
        }
        Err(_) => String::new(),
    }
}

/// Run a .sigma script file from path
pub fn run_script(path: &str) -> Result<i32, String> {
    let mut env = ShellEnv::new();
    run_script_env(path, &mut env)
}

/// Run a script in an existing shell environment
pub fn run_script_env(path: &str, env: &mut ShellEnv) -> Result<i32, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("{}: {}", path, e))?;

    let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    run_lines(&lines, env)
}

/// Execute a list of script lines with full scripting support
fn run_lines(lines: &[String], env: &mut ShellEnv) -> Result<i32, String> {
    let mut i = 0;
    let mut last_exit = 0i32;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip comments and empty
        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        // Function definition
        if line.ends_with("() {") || line.ends_with("(){") {
            let name = line.trim_end_matches("() {").trim_end_matches("(){").trim().to_string();
            let mut body: Vec<String> = Vec::new();
            i += 1;
            while i < lines.len() {
                let bl = lines[i].trim();
                if bl == "}" { break; }
                body.push(lines[i].clone());
                i += 1;
            }
            env.functions.insert(name, body);
            i += 1;
            continue;
        }

        // if statement
        if line.starts_with("if ") {
            let (then_lines, else_lines, end_i) = parse_if_block(lines, i)?;
            let cond_line = line["if ".len()..].trim_end_matches("; then").trim_end_matches(" then").to_string();
            let cond_code = execute_script_line(&cond_line, env);
            if cond_code == 0 {
                last_exit = run_lines(&then_lines, env)?;
            } else if let Some(else_body) = else_lines {
                last_exit = run_lines(&else_body, env)?;
            }
            i = end_i;
            continue;
        }

        // for loop
        if line.starts_with("for ") {
            let (var, items, body, end_i) = parse_for_loop(lines, i)?;
            for item in &items {
                env.vars.insert(var.clone(), item.clone());
                last_exit = run_lines(&body, env)?;
            }
            i = end_i;
            continue;
        }

        // while loop
        if line.starts_with("while ") {
            let (cond, body, end_i) = parse_while_loop(lines, i)?;
            loop {
                let code = execute_script_line(&cond, env);
                if code != 0 { break; }
                last_exit = run_lines(&body, env)?;
            }
            i = end_i;
            continue;
        }

        // Variable assignment: KEY=value
        if let Some(eq) = line.find('=') {
            let key = &line[..eq];
            if key.chars().all(|c| c.is_alphanumeric() || c == '_') && !key.is_empty() {
                let val = env.expand_vars(&line[eq + 1..]);
                env.vars.insert(key.to_string(), val);
                i += 1;
                continue;
            }
        }

        // Regular command
        last_exit = execute_script_line_with_lineno(line, env, i + 1);
        i += 1;
    }

    Ok(last_exit)
}

fn execute_script_line(line: &str, env: &mut ShellEnv) -> i32 {
    execute_script_line_with_lineno(line, env, 0)
}

fn execute_script_line_with_lineno(line: &str, env: &mut ShellEnv, lineno: usize) -> i32 {
    let expanded = env.expand_vars_with_lineno(line, lineno);
    let ast = match crate::parser::parse(&expanded) {
        Ok(a) => a,
        Err(e) => { eprintln!("sigma-sh: {}", e); return 1; }
    };

    // Check built-ins
    if let crate::parser::Statement::Command(ref cmd) = ast {
        if let Some(code) = crate::builtins::try_builtin(cmd, env, &[]) {
            env.last_exit = code;
            return code;
        }
        // Check user-defined functions
        if let Some(name) = cmd.argv.first() {
            if let Some(body) = env.functions.get(name).cloned() {
                let code = run_lines(&body, env).unwrap_or(1);
                env.last_exit = code;
                return code;
            }
        }
    }

    let code = crate::executor::execute(&ast, env);
    env.last_exit = code;
    code
}

// ---- Parsing helpers for control flow ----

fn parse_if_block(lines: &[String], start: usize) -> Result<(Vec<String>, Option<Vec<String>>, usize), String> {
    let mut then_lines = Vec::new();
    let mut else_lines: Option<Vec<String>> = None;
    let mut in_else = false;
    let mut depth = 1;
    let mut i = start + 1;

    while i < lines.len() {
        let l = lines[i].trim();
        if l.starts_with("if ") { depth += 1; }
        if l == "fi" {
            depth -= 1;
            if depth == 0 { return Ok((then_lines, else_lines, i + 1)); }
        }
        if depth == 1 && l == "else" {
            in_else = true;
            i += 1;
            else_lines = Some(Vec::new());
            continue;
        }
        if in_else {
            else_lines.as_mut().unwrap().push(lines[i].clone());
        } else {
            if l != "then" { then_lines.push(lines[i].clone()); }
        }
        i += 1;
    }
    Err("unterminated if block".to_string())
}

fn parse_for_loop(lines: &[String], start: usize) -> Result<(String, Vec<String>, Vec<String>, usize), String> {
    // "for VAR in item1 item2 ...; do"
    let header = lines[start].trim()["for ".len()..].to_string();
    let in_pos = header.find(" in ").ok_or("invalid for syntax")?;
    let var = header[..in_pos].trim().to_string();
    let rest = header[in_pos + 4..].trim_end_matches("; do").trim_end_matches(" do").to_string();
    let items: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();

    let mut body = Vec::new();
    let mut i = start + 1;
    while i < lines.len() {
        let l = lines[i].trim();
        if l == "done" { return Ok((var, items, body, i + 1)); }
        if l != "do" { body.push(lines[i].clone()); }
        i += 1;
    }
    Err("unterminated for loop".to_string())
}

fn parse_while_loop(lines: &[String], start: usize) -> Result<(String, Vec<String>, usize), String> {
    let cond = lines[start].trim()["while ".len()..].trim_end_matches("; do").trim_end_matches(" do").to_string();
    let mut body = Vec::new();
    let mut i = start + 1;
    while i < lines.len() {
        let l = lines[i].trim();
        if l == "done" { return Ok((cond, body, i + 1)); }
        if l != "do" { body.push(lines[i].clone()); }
        i += 1;
    }
    Err("unterminated while loop".to_string())
}
