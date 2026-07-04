// sigma-sh: The Sovereign Shell for SigmaOS
// Entry point — REPL loop + signal handling

mod builtins;
mod executor;
mod parser;
mod scripting;

use std::io::{self, BufRead, Write};

const VERSION: &str = "0.3.0";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Flag handling before dropping into REPL or script mode
    for arg in &args[1..] {
        match arg.as_str() {
            "--version" | "-V" => {
                println!("sigma-sh {}", VERSION);
                println!("SigmaOS Sovereign Shell — GPL-2.0-or-later");
                std::process::exit(0);
            }
            "--help" | "-h" => {
                print_shell_help();
                std::process::exit(0);
            }
            _ => {}
        }
    }

    // Script mode: sigma-sh script.sigma [args...]
    if args.len() >= 2 && !args[1].starts_with('-') {
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

fn print_shell_help() {
    println!("sigma-sh {} — SigmaOS Sovereign Shell", VERSION);
    println!();
    println!("USAGE:");
    println!("  sigma-sh                 Start interactive REPL");
    println!("  sigma-sh <script>        Execute a .sigma script");
    println!("  sigma-sh --version       Print version and exit");
    println!("  sigma-sh --help          Show this help");
    println!();
    println!("Inside the shell, type 'help' for built-in commands.");
}

fn repl() {
    let stdin = io::stdin();
    let mut history: Vec<String> = Vec::new();
    let mut env = scripting::ShellEnv::new();

    // Print welcome banner
    println!("\x1b[1;36msigma-sh\x1b[0m {} — SigmaOS Sovereign Shell", VERSION);
    println!("Type \x1b[1mhelp\x1b[0m for built-ins. Type \x1b[1mexit\x1b[0m to quit.\n");

    loop {
        print_prompt(&env);
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => {
                println!(); // Ctrl-D / EOF — print newline before exit
                break;
            }
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

        // Deduplicate consecutive identical history entries
        if history.last().map(|l| l != &line).unwrap_or(true) {
            history.push(line.clone());
        }

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
        .map(|p| {
            // Shorten home dir to ~
            let home = std::env::var("HOME").unwrap_or_default();
            let s = p.display().to_string();
            if !home.is_empty() && s.starts_with(&home) {
                format!("~{}", &s[home.len()..])
            } else {
                s
            }
        })
        .unwrap_or_else(|_| "?".to_string());

    let host = std::env::var("HOSTNAME")
        .unwrap_or_else(|_| "sigmaos".to_string());
    let user = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "sigma".to_string());

    let git_branch = get_git_branch();
    let branch_part = match &git_branch {
        Some(b) => format!(" \x1b[1;33m({})\x1b[0m", b),
        None    => String::new(),
    };

    let status_glyph = if env.last_exit == 0 {
        "\x1b[1;32m❯\x1b[0m"
    } else {
        "\x1b[1;31m✗\x1b[0m"
    };

    print!(
        "\x1b[1;34m{}@{}\x1b[0m:\x1b[1;36m{}\x1b[0m{} {} ",
        user, host, cwd, branch_part, status_glyph
    );
}

/// Try to get the current git branch name cheaply (no subprocess).
fn get_git_branch() -> Option<String> {
    let mut dir = std::env::current_dir().ok()?;
    loop {
        let head = dir.join(".git").join("HEAD");
        if head.exists() {
            let content = std::fs::read_to_string(&head).ok()?;
            let content = content.trim();
            if let Some(branch) = content.strip_prefix("ref: refs/heads/") {
                return Some(branch.to_string());
            }
            // Detached HEAD — show short hash
            return Some(content.get(..7).unwrap_or(content).to_string());
        }
        if !dir.pop() {
            break;
        }
    }
    None
}
