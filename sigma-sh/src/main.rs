// sigma-sh: The Sovereign Shell for SigmaOS
// Entry point — REPL loop + signal handling

mod builtins;
mod executor;
mod parser;
mod scripting;

use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Script mode: sigma-sh script.sigma
    if args.len() >= 2 {
        let path = &args[1];
        if let Err(e) = scripting::run_script(path) {
            eprintln!("sigma-sh: script error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Interactive REPL
    repl();
}

fn repl() {
    let stdin = io::stdin();
    let mut history: Vec<String> = Vec::new();
    let mut env = scripting::ShellEnv::new();

    println!("sigma-sh v0.2.0 — SigmaOS Sovereign Shell");
    println!("Type 'help' for built-in commands. Type 'exit' to quit.\n");

    loop {
        print_prompt(&env);
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {}
            Err(e) => {
                eprintln!("sigma-sh: read error: {}", e);
                break;
            }
        }

        let line = line.trim_end_matches('\n').trim_end_matches('\r').to_string();
        if line.is_empty() {
            continue;
        }

        history.push(line.clone());
        env.last_exit = execute_line(&line, &mut env, &history);
    }
}

fn execute_line(line: &str, env: &mut scripting::ShellEnv, history: &[String]) -> i32 {
    // Variable expansion
    let expanded = env.expand_vars(line);

    // Parse into AST
    let ast = match parser::parse(&expanded) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("sigma-sh: parse error: {}", e);
            return 1;
        }
    };

    // Handle built-ins
    if let parser::Statement::Command(ref cmd) = ast {
        if let Some(code) = builtins::try_builtin(cmd, env, history) {
            return code;
        }
    }

    // Execute external commands / pipelines
    executor::execute(&ast, env)
}

fn print_prompt(env: &scripting::ShellEnv) {
    let cwd = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "?".to_string());

    let host = std::env::var("HOSTNAME").unwrap_or_else(|_| "sigmaos".to_string());
    let user = std::env::var("USER").unwrap_or_else(|_| "sigma".to_string());
    let status = if env.last_exit == 0 { "❯" } else { "✗" };

    print!("\x1b[1;34m{}@{}\x1b[0m:\x1b[1;36m{}\x1b[0m {} ", user, host, cwd, status);
}
