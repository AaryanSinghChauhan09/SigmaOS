// sigma-sh builtins: Built-in shell commands that run in-process

use std::collections::HashMap;
use crate::parser::Command;
use crate::scripting::ShellEnv;

/// Returns Some(exit_code) if cmd is a built-in, None if it should be exec'd
pub fn try_builtin(cmd: &Command, env: &mut ShellEnv, history: &[String]) -> Option<i32> {
    let name = cmd.argv.first()?.as_str();
    let args = &cmd.argv[1..];

    match name {
        "exit" | "quit" => {
            let code = args.first()
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0);
            std::process::exit(code);
        }
        "cd" => Some(builtin_cd(args)),
        "pwd" => Some(builtin_pwd()),
        "echo" => Some(builtin_echo(args)),
        "export" => Some(builtin_export(args, env)),
        "unset" => Some(builtin_unset(args, env)),
        "env" => Some(builtin_env(env)),
        "alias" => Some(builtin_alias(args, env)),
        "unalias" => Some(builtin_unalias(args, env)),
        "history" => Some(builtin_history(history)),
        "source" | "." => Some(builtin_source(args, env)),
        "help" => Some(builtin_help()),
        "type" => Some(builtin_type(args, env)),
        "which" => Some(builtin_which(args)),
        "kill" => Some(builtin_kill(args)),
        "read" => Some(builtin_read(args, env)),
        "test" | "[" => Some(builtin_test(args)),
        "true" => Some(0),
        "false" => Some(1),
        ":" => Some(0), // no-op
        _ => {
            // Check aliases
            if let Some(alias_val) = env.aliases.get(name).cloned() {
                let expanded = format!("{} {}", alias_val, args.join(" "));
                let ast = crate::parser::parse(&expanded).ok()?;
                return Some(crate::executor::execute(&ast, env));
            }
            None
        }
    }
}

fn builtin_cd(args: &[String]) -> i32 {
    let target = args.first()
        .map(|s| s.as_str())
        .unwrap_or("~");

    let path = if target == "~" {
        std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
    } else if target == "-" {
        std::env::var("OLDPWD").unwrap_or_else(|_| ".".to_string())
    } else {
        target.to_string()
    };

    let old = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_default();

    match std::env::set_current_dir(&path) {
        Ok(_) => {
            std::env::set_var("OLDPWD", old);
            0
        }
        Err(e) => {
            eprintln!("sigma-sh: cd: {}: {}", path, e);
            1
        }
    }
}

fn builtin_pwd() -> i32 {
    match std::env::current_dir() {
        Ok(path) => { println!("{}", path.display()); 0 }
        Err(e) => { eprintln!("sigma-sh: pwd: {}", e); 1 }
    }
}

fn builtin_echo(args: &[String]) -> i32 {
    let no_newline = args.first().map(|a| a == "-n").unwrap_or(false);
    let start = if no_newline { 1 } else { 0 };
    let output = args[start..].join(" ");

    // Process escape sequences
    let processed = process_escapes(&output);

    if no_newline {
        print!("{}", processed);
    } else {
        println!("{}", processed);
    }
    0
}

fn process_escapes(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('r') => out.push('\r'),
                Some('\\') => out.push('\\'),
                Some(other) => { out.push('\\'); out.push(other); }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}

fn builtin_export(args: &[String], env: &mut ShellEnv) -> i32 {
    if args.is_empty() {
        // Print all exported vars
        for (k, v) in &env.vars {
            println!("export {}={}", k, v);
        }
        return 0;
    }
    for arg in args {
        if let Some(eq) = arg.find('=') {
            let key = arg[..eq].to_string();
            let val = arg[eq + 1..].to_string();
            env.vars.insert(key.clone(), val.clone());
            std::env::set_var(&key, &val);
        } else {
            // Export existing variable
            if let Ok(v) = std::env::var(arg) {
                env.vars.insert(arg.clone(), v);
            }
        }
    }
    0
}

fn builtin_unset(args: &[String], env: &mut ShellEnv) -> i32 {
    for arg in args {
        env.vars.remove(arg);
        std::env::remove_var(arg);
    }
    0
}

fn builtin_env(env: &ShellEnv) -> i32 {
    for (k, v) in &env.vars {
        println!("{}={}", k, v);
    }
    0
}

fn builtin_alias(args: &[String], env: &mut ShellEnv) -> i32 {
    if args.is_empty() {
        for (k, v) in &env.aliases {
            println!("alias {}='{}'", k, v);
        }
        return 0;
    }
    for arg in args {
        if let Some(eq) = arg.find('=') {
            let name = arg[..eq].to_string();
            let val = arg[eq + 1..].trim_matches('\'').trim_matches('"').to_string();
            env.aliases.insert(name, val);
        } else {
            if let Some(v) = env.aliases.get(arg) {
                println!("alias {}='{}'", arg, v);
            }
        }
    }
    0
}

fn builtin_unalias(args: &[String], env: &mut ShellEnv) -> i32 {
    if args.first().map(|a| a == "-a").unwrap_or(false) {
        env.aliases.clear();
    } else {
        for arg in args {
            env.aliases.remove(arg);
        }
    }
    0
}

fn builtin_history(history: &[String]) -> i32 {
    for (i, cmd) in history.iter().enumerate() {
        println!("{:5}  {}", i + 1, cmd);
    }
    0
}

fn builtin_source(args: &[String], env: &mut ShellEnv) -> i32 {
    let path = match args.first() {
        Some(p) => p,
        None => {
            eprintln!("sigma-sh: source: filename argument required");
            return 1;
        }
    };
    match crate::scripting::run_script_env(path, env) {
        Ok(code) => code,
        Err(e) => { eprintln!("sigma-sh: source: {}: {}", path, e); 1 }
    }
}

fn builtin_help() -> i32 {
    println!("\x1b[1;36msigma-sh\x1b[0m — SigmaOS Sovereign Shell v0.3.0");
    println!();
    println!("\x1b[1mNavigation:\x1b[0m");
    println!("  cd [dir|-|~]         Change directory (- = OLDPWD, ~ = HOME)");
    println!("  pwd                  Print working directory");
    println!("  ls [dir]             List directory (delegates to external ls)");
    println!();
    println!("\x1b[1mOutput:\x1b[0m");
    println!("  echo [-n] [...]      Print text (supports \\n \\t escapes)");
    println!("  printf <fmt> [...]   Formatted output");
    println!();
    println!("\x1b[1mVariables & Environment:\x1b[0m");
    println!("  export [K=V ...]     Set/export environment variables");
    println!("  unset [K ...]        Remove environment variable");
    println!("  env                  List all environment variables");
    println!("  read <VAR>           Read a line from stdin into VAR");
    println!();
    println!("\x1b[1mAliases & Functions:\x1b[0m");
    println!("  alias [K='V']        Define or list aliases");
    println!("  unalias [-a] K       Remove alias(es)");
    println!("  type <name>          Show how name would be interpreted");
    println!("  which <name>         Locate an executable on PATH");
    println!();
    println!("\x1b[1mHistory & Session:\x1b[0m");
    println!("  history              Show command history");
    println!("  source <file>        Execute script in current shell context");
    println!("  kill [-SIGNAL] <pid> Send a signal to a process");
    println!("  exit [code]          Exit sigma-sh");
    println!("  help                 Show this help");
    println!();
    println!("\x1b[1mFlow Control (in scripts):\x1b[0m");
    println!("  if … then … fi       Conditional");
    println!("  for VAR in … ; do … done  Loop over items");
    println!("  while … ; do … done  While loop");
    println!("  func() {{ … }}        Define a function");
    println!("  test / [            Evaluate conditions (delegates to external)");
    println!();
    println!("\x1b[1mSyntax Features:\x1b[0m");
    println!("  Pipes:      cmd1 | cmd2 | cmd3");
    println!("  Redirects:  > >> < 2>");
    println!("  Background: cmd &");
    println!("  Sequences:  cmd1 ; cmd2");
    println!("  Logic:      cmd1 && cmd2  |  cmd1 || cmd2");
    println!("  Variables:  $VAR  ${{VAR:-default}}  $?");
    println!("  Scripts:    sigma-sh script.sigma");
    0
}

fn builtin_type(args: &[String], env: &ShellEnv) -> i32 {
    if args.is_empty() {
        eprintln!("sigma-sh: type: usage: type <name>");
        return 1;
    }
    let mut code = 0i32;
    for name in args {
        // Check built-ins
        let builtins = [
            "cd", "pwd", "echo", "export", "unset", "env", "alias", "unalias",
            "history", "source", "help", "type", "which", "kill", "read", "exit",
            "true", "false",
        ];
        if builtins.contains(&name.as_str()) {
            println!("{} is a shell builtin", name);
        } else if let Some(v) = env.aliases.get(name) {
            println!("{} is aliased to '{}'", name, v);
        } else if env.functions.contains_key(name) {
            println!("{} is a shell function", name);
        } else if let Some(path) = find_in_path(name) {
            println!("{} is {}", name, path);
        } else {
            eprintln!("sigma-sh: type: {}: not found", name);
            code = 1;
        }
    }
    code
}

fn builtin_which(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("sigma-sh: which: usage: which <name>");
        return 1;
    }
    let mut code = 0i32;
    for name in args {
        match find_in_path(name) {
            Some(path) => println!("{}", path),
            None => {
                eprintln!("{}: not found", name);
                code = 1;
            }
        }
    }
    code
}

fn find_in_path(name: &str) -> Option<String> {
    // If it contains a slash, treat as a direct path
    if name.contains('/') || name.contains('\\') {
        if std::path::Path::new(name).is_file() {
            return Some(name.to_string());
        }
        return None;
    }
    let path_var = std::env::var("PATH").unwrap_or_default();
    let sep = if cfg!(windows) { ';' } else { ':' };
    for dir in path_var.split(sep) {
        let candidate = std::path::Path::new(dir).join(name);
        // On Windows also try with .exe
        #[cfg(windows)]
        {
            let with_exe = std::path::Path::new(dir).join(format!("{}.exe", name));
            if with_exe.is_file() {
                return Some(with_exe.display().to_string());
            }
        }
        if candidate.is_file() {
            return Some(candidate.display().to_string());
        }
    }
    None
}

fn builtin_kill(args: &[String]) -> i32 {
    if args.is_empty() {
        eprintln!("sigma-sh: kill: usage: kill [-SIGNAL] <pid> [...]");
        return 1;
    }

    let mut signal = "TERM";
    let mut pids: Vec<&str> = Vec::new();

    for arg in args {
        if let Some(sig) = arg.strip_prefix('-') {
            signal = sig;
        } else {
            pids.push(arg);
        }
    }

    if pids.is_empty() {
        eprintln!("sigma-sh: kill: no PID specified");
        return 1;
    }

    // On unix we'd use libc::kill; here we delegate to the system kill command
    let status = std::process::Command::new("kill")
        .arg(format!("-{}", signal))
        .args(&pids)
        .status();

    match status {
        Ok(s) => s.code().unwrap_or(0),
        Err(e) => {
            eprintln!("sigma-sh: kill: {}", e);
            1
        }
    }
}

fn builtin_read(args: &[String], env: &mut ShellEnv) -> i32 {
    let var_name = match args.first() {
        Some(v) => v.clone(),
        None => {
            eprintln!("sigma-sh: read: usage: read <VARNAME>");
            return 1;
        }
    };

    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(0) => {
            // EOF
            env.vars.insert(var_name, String::new());
            return 1;
        }
        Ok(_) => {}
        Err(e) => {
            eprintln!("sigma-sh: read: {}", e);
            return 1;
        }
    }

    let value = line.trim_end_matches('\n').trim_end_matches('\r').to_string();
    env.vars.insert(var_name, value);
    0
}

fn builtin_test(args: &[String]) -> i32 {
    // Strip trailing ']' if called as '['
    let args: Vec<&str> = args.iter()
        .map(|s| s.as_str())
        .filter(|s| *s != "]")
        .collect();

    if args.is_empty() {
        return 1; // false
    }

    match args.as_slice() {
        // Unary: -e FILE, -f FILE, -d FILE, -z STR, -n STR
        ["-e", path] => if std::path::Path::new(path).exists() { 0 } else { 1 },
        ["-f", path] => if std::path::Path::new(path).is_file() { 0 } else { 1 },
        ["-d", path] => if std::path::Path::new(path).is_dir() { 0 } else { 1 },
        ["-z", s]    => if s.is_empty() { 0 } else { 1 },
        ["-n", s]    => if !s.is_empty() { 0 } else { 1 },
        // Binary: STR = STR, STR != STR, NUM -eq NUM, -lt, -gt, -le, -ge
        [a, "=",  b] => if a == b { 0 } else { 1 },
        [a, "!=", b] => if a != b { 0 } else { 1 },
        [a, "-eq", b] => match (a.parse::<i64>(), b.parse::<i64>()) {
            (Ok(x), Ok(y)) => if x == y { 0 } else { 1 },
            _ => 1,
        },
        [a, "-ne", b] => match (a.parse::<i64>(), b.parse::<i64>()) {
            (Ok(x), Ok(y)) => if x != y { 0 } else { 1 },
            _ => 1,
        },
        [a, "-lt", b] => match (a.parse::<i64>(), b.parse::<i64>()) {
            (Ok(x), Ok(y)) => if x < y { 0 } else { 1 },
            _ => 1,
        },
        [a, "-gt", b] => match (a.parse::<i64>(), b.parse::<i64>()) {
            (Ok(x), Ok(y)) => if x > y { 0 } else { 1 },
            _ => 1,
        },
        [a, "-le", b] => match (a.parse::<i64>(), b.parse::<i64>()) {
            (Ok(x), Ok(y)) => if x <= y { 0 } else { 1 },
            _ => 1,
        },
        [a, "-ge", b] => match (a.parse::<i64>(), b.parse::<i64>()) {
            (Ok(x), Ok(y)) => if x >= y { 0 } else { 1 },
            _ => 1,
        },
        // Negation
        ["!", rest @ ..] => {
            let inner = builtin_test(&rest.iter().map(|s| s.to_string()).collect::<Vec<_>>());
            if inner == 0 { 1 } else { 0 }
        }
        // Single string: non-empty = true
        [s] => if !s.is_empty() { 0 } else { 1 },
        _ => {
            eprintln!("sigma-sh: test: unsupported expression");
            1
        }
    }
}
