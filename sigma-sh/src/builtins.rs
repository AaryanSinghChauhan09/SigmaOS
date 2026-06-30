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
    println!("\x1b[1;36msigma-sh\x1b[0m — SigmaOS Sovereign Shell v0.2.0");
    println!();
    println!("\x1b[1mBuilt-in commands:\x1b[0m");
    println!("  cd [dir]         Change directory (- goes to OLDPWD, ~ goes home)");
    println!("  pwd              Print working directory");
    println!("  echo [-n] [...]  Print text (supports \\n, \\t escapes)");
    println!("  export [K=V]     Set/export environment variables");
    println!("  unset [K]        Remove environment variable");
    println!("  env              List all environment variables");
    println!("  alias [K=V]      Define or list aliases");
    println!("  unalias [-a] K   Remove alias(es)");
    println!("  history          Show command history");
    println!("  source <file>    Execute script in current shell context");
    println!("  help             Show this help");
    println!("  exit [code]      Exit sigma-sh");
    println!();
    println!("\x1b[1mFeatures:\x1b[0m");
    println!("  Pipes: cmd1 | cmd2 | cmd3");
    println!("  Redirects: > >> < 2>");
    println!("  Background: cmd &");
    println!("  Sequences: cmd1 ; cmd2");
    println!("  Conditionals: cmd1 && cmd2  |  cmd1 || cmd2");
    println!("  Variables: \\$VAR, \\${{VAR:-default}}");
    println!("  Scripts: sigma-sh script.sigma");
    0
}
