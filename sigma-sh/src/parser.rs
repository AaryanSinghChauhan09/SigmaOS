// sigma-sh parser: Tokenizer + AST builder
// Supports: pipes, redirects, background jobs, variable expansion, quoting

#[derive(Debug, Clone)]
pub enum Redirect {
    Stdout(String),       // > file
    StdoutAppend(String), // >> file
    Stdin(String),        // < file
    Stderr(String),       // 2> file
}

#[derive(Debug, Clone)]
pub struct Command {
    pub argv: Vec<String>,
    pub redirects: Vec<Redirect>,
    pub background: bool,
}

#[derive(Debug, Clone)]
pub struct Pipeline {
    pub commands: Vec<Command>,
    pub background: bool,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Command(Command),
    Pipeline(Pipeline),
    Sequence(Vec<Statement>),   // cmd1 ; cmd2
    And(Box<Statement>, Box<Statement>), // cmd1 && cmd2
    Or(Box<Statement>, Box<Statement>),  // cmd1 || cmd2
    Empty,
}

/// Parse a raw input line into a Statement AST
pub fn parse(input: &str) -> Result<Statement, String> {
    let input = input.trim();
    if input.is_empty() || input.starts_with('#') {
        return Ok(Statement::Empty);
    }

    // Split on semicolons first (lowest precedence outside quotes)
    let parts = split_unquoted(input, ';');
    if parts.len() > 1 {
        let stmts: Result<Vec<_>, _> = parts.iter().map(|p| parse(p.trim())).collect();
        return Ok(Statement::Sequence(stmts?));
    }

    // && / || (short-circuit)
    if let Some(idx) = find_unquoted(input, "&&") {
        let left = parse(&input[..idx])?;
        let right = parse(&input[idx + 2..])?;
        return Ok(Statement::And(Box::new(left), Box::new(right)));
    }
    if let Some(idx) = find_unquoted(input, "||") {
        let left = parse(&input[..idx])?;
        let right = parse(&input[idx + 2..])?;
        return Ok(Statement::Or(Box::new(left), Box::new(right)));
    }

    // Pipe
    let pipe_parts = split_unquoted(input, '|');
    if pipe_parts.len() > 1 {
        let background = input.trim_end().ends_with('&');
        let commands: Result<Vec<_>, _> = pipe_parts.iter()
            .map(|p| parse_command(p.trim()))
            .collect();
        return Ok(Statement::Pipeline(Pipeline {
            commands: commands?,
            background,
        }));
    }

    // Single command
    Ok(Statement::Command(parse_command(input)?))
}

fn parse_command(input: &str) -> Result<Command, String> {
    let mut input = input.trim();
    let background = input.ends_with('&') && !input.ends_with("&&");
    if background {
        input = input[..input.len() - 1].trim();
    }

    let tokens = tokenize(input)?;
    let mut argv: Vec<String> = Vec::new();
    let mut redirects: Vec<Redirect> = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        let tok = &tokens[i];
        if tok == ">" {
            i += 1;
            redirects.push(Redirect::Stdout(tokens.get(i).cloned().ok_or("expected filename after '>'")?));
        } else if tok == ">>" {
            i += 1;
            redirects.push(Redirect::StdoutAppend(tokens.get(i).cloned().ok_or("expected filename after '>>'")?));
        } else if tok == "<" {
            i += 1;
            redirects.push(Redirect::Stdin(tokens.get(i).cloned().ok_or("expected filename after '<'")?));
        } else if tok == "2>" {
            i += 1;
            redirects.push(Redirect::Stderr(tokens.get(i).cloned().ok_or("expected filename after '2>'")?));
        } else {
            argv.push(tok.clone());
        }
        i += 1;
    }

    Ok(Command { argv, redirects, background })
}

/// Tokenize respecting single/double quotes and backslash escapes
fn tokenize(input: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            '"' => {
                loop {
                    match chars.next() {
                        Some('"') => break,
                        Some('\\') => {
                            if let Some(esc) = chars.next() {
                                current.push(esc);
                            }
                        }
                        Some(c) => current.push(c),
                        None => return Err("unterminated double quote".to_string()),
                    }
                }
            }
            '\'' => {
                loop {
                    match chars.next() {
                        Some('\'') => break,
                        Some(c) => current.push(c),
                        None => return Err("unterminated single quote".to_string()),
                    }
                }
            }
            ' ' | '\t' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}

fn split_unquoted(input: &str, sep: char) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            '\\' => {
                current.push(c);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
                continue;
            }
            _ if c == sep && !in_single && !in_double => {
                parts.push(current.clone());
                current.clear();
                continue;
            }
            _ => {}
        }
        current.push(c);
    }
    parts.push(current);
    parts
}

fn find_unquoted(input: &str, needle: &str) -> Option<usize> {
    let mut in_single = false;
    let mut in_double = false;
    let bytes = input.as_bytes();
    let needle = needle.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\'' && !in_double { in_single = !in_single; i += 1; continue; }
        if b == b'"' && !in_single { in_double = !in_double; i += 1; continue; }
        if !in_single && !in_double && bytes[i..].starts_with(needle) {
            return Some(i);
        }
        i += 1;
    }
    None
}
