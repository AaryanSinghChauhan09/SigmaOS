// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/agent/main.rs — sigma-agent-core: Rust binary entry point
// Wires together: tools, core, extended tools, LLM backends, planner, code agent
//
// Architecture (inspired by Claude Code + Aider + azure-cli):
//
//   CLI args ──► Agent::new() ──► IntentParser ──► Tool registry
//                                       │
//                                  ReAct Planner ──► LLM Backend
//                                       │            (sigma-ai / Ollama / llama.cpp / null)
//                                  Tool Executor
//                                       │
//                                  ANSI output
//
// Usage:
//   sigma-agent-core                    Interactive REPL
//   sigma-agent-core --once "install sigma-edit"   One-shot command
//   sigma-agent-core --script setup.sa             Run .sa script
//   sigma-agent-core --dry-run "delete /tmp/old"  Preview only
//   sigma-agent-core --trust full "run rm -rf ..."  Trust level

// ── Module declarations ────────────────────────────────────────────────────────
mod sigma_agent;
mod sigma_agent_core;
mod sigma_agent_tools_ext;
mod sigma_llm;
mod sigma_agent_planner;
mod sigma_agent_code;
mod collections;

use collections::{SigmaMap, SigmaVec, SigmaStringBuilder};
use sigma_agent_core::{Agent, AgentModel};
use sigma_agent_tools_ext::{
    ExplainTool, CodeEditTool, SummariseTool, WmControlTool,
    NotifyTool, ClipboardTool, FindFilesTool, AccessibilityTool,
    VpnTool, DiskTool,
};

// ── CLI argument parser ────────────────────────────────────────────────────────

struct CliArgs {
    mode:       CliMode,
    once_cmd:   String,
    script:     String,
    verbose:    bool,
    dry_run:    bool,
    no_color:   bool,
    trust:      TrustLevel,
    model:      Option<String>,
    pipe:       bool,
}

#[derive(PartialEq, Eq)]
enum CliMode { Repl, Once, Script, Pipe }

#[derive(Clone, Copy)]
pub enum TrustLevel { Safe, Standard, Full }

impl Default for CliArgs {
    fn default() -> Self {
        Self {
            mode:     CliMode::Repl,
            once_cmd: String::new(),
            script:   String::new(),
            verbose:  false,
            dry_run:  false,
            no_color: false,
            trust:    TrustLevel::Standard,
            model:    None,
            pipe:     false,
        }
    }
}

fn parse_args() -> CliArgs {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let mut a = CliArgs::default();
    let mut i = 0;
    while i < raw.len() {
        match raw[i].as_str() {
            "--once" | "-c" => {
                i += 1;
                if i < raw.len() { a.once_cmd = raw[i].clone(); a.mode = CliMode::Once; }
            }
            "--script" | "-s" => {
                i += 1;
                if i < raw.len() { a.script = raw[i].clone(); a.mode = CliMode::Script; }
            }
            "--verbose" | "-v" => { a.verbose  = true; }
            "--dry-run"  | "-d" => { a.dry_run  = true; }
            "--no-color" | "--no-colour" => { a.no_color = true; }
            "--pipe"    => { a.pipe = true; a.mode = CliMode::Pipe; }
            "--trust"   => {
                i += 1;
                if i < raw.len() {
                    a.trust = match raw[i].as_str() {
                        "safe" => TrustLevel::Safe,
                        "full" => TrustLevel::Full,
                        _      => TrustLevel::Standard,
                    };
                }
            }
            "--model" | "-m" => {
                i += 1;
                if i < raw.len() { a.model = Some(raw[i].clone()); }
            }
            "--version" => {
                println!("sigma-agent-core 15.0.0 — SigmaOS AI CLI Agent");
                std::process::exit(0);
            }
            "--help" | "-h" => {
                print_help(); std::process::exit(0);
            }
            other if !other.starts_with('-') && a.mode == CliMode::Repl && a.once_cmd.is_empty() => {
                // Bare argument = one-shot command
                if a.once_cmd.is_empty() {
                    a.once_cmd = other.to_owned();
                    a.mode = CliMode::Once;
                } else {
                    a.once_cmd.push(' ');
                    a.once_cmd.push_str(other);
                }
            }
            _ => {}
        }
        i += 1;
    }
    a
}

fn print_help() {
    println!(r#"sigma-agent-core 15.0.0 — SigmaOS AI CLI Agent (Rust engine)

USAGE:
  sigma-agent-core                        Interactive REPL
  sigma-agent-core "install sigma-edit"   One-shot command
  sigma-agent-core --once "<command>"     One-shot (explicit)
  sigma-agent-core --script <file>        Run .sa script file
  sigma-agent-core --pipe                 Read commands from stdin

FLAGS:
  --verbose, -v       Show reasoning steps and tool calls
  --dry-run, -d       Preview actions without executing
  --no-color          Disable ANSI colour output
  --trust safe        Read-only operations only
  --trust standard    Default: reads + installs + settings (default)
  --trust full        All operations including shell execution
  --model <name>      Force specific LLM model
  --version           Print version and exit

EXAMPLES:
  sigma-agent-core "install sigma-edit"
  sigma-agent-core "set dark mode"
  sigma-agent-core "system info"
  sigma-agent-core "find sigma_net.rs in /home/user/code"
  sigma-agent-core "fix userland/agent/main.rs add error handling"
  sigma-agent-core --dry-run "delete /tmp/old"
  sigma-agent-core --trust full "run rm -rf /tmp/cache"
  sigma-agent-core --script ~/setup.sa

GUI → CLI QUICK REFERENCE:
  Open Terminal        sigma-agent-core "open app sigma-terminal"
  Dark Mode            sigma-agent-core "set dark mode"
  Install App          sigma-agent-core "install <name>"
  System Info          sigma-agent-core "system info"
  Network Status       sigma-agent-core "network status"
  Kill Process         sigma-agent-core "kill process <pid>"
  High Contrast        sigma-agent-core "accessibility high-contrast on"

Inspired by: Claude Code · Aider · llama.cpp · azure-cli · copilot-cli
"#);
}

// ── Build the agent with all 20 tools ─────────────────────────────────────────

fn build_agent(args: &CliArgs) -> Agent {
    let mut agent = Agent::new();
    agent.set_verbose(args.verbose);

    // Register the 10 extended tools (tools 11-20)
    agent.register(Box::new(ExplainTool));
    agent.register(Box::new(CodeEditTool));
    agent.register(Box::new(SummariseTool));
    agent.register(Box::new(WmControlTool));
    agent.register(Box::new(NotifyTool));
    agent.register(Box::new(ClipboardTool));
    agent.register(Box::new(FindFilesTool));
    agent.register(Box::new(AccessibilityTool));
    agent.register(Box::new(VpnTool));
    agent.register(Box::new(DiskTool));

    agent
}

// ── Dry-run / trust enforcement ───────────────────────────────────────────────

fn check_trust(cmd: &str, trust: TrustLevel) -> bool {
    let lower = cmd.to_ascii_lowercase();
    match trust {
        TrustLevel::Safe => {
            // Only allow reads
            let safe = lower.starts_with("read") || lower.starts_with("list") ||
                       lower.starts_with("cat")  || lower.starts_with("show") ||
                       lower.starts_with("system info") || lower.starts_with("disk") ||
                       lower == "help" || lower == "tools" || lower == "history";
            if !safe {
                eprintln!("\x1b[33m[trust:safe] Blocked: '{}'. Safe mode allows read-only operations.\x1b[0m", cmd);
            }
            safe
        }
        TrustLevel::Standard => {
            // Block dangerous shell operations
            let blocked = lower.contains("rm -rf") || lower.contains("dd if=") ||
                          lower.contains("mkfs") || lower.contains("wipefs") ||
                          lower.contains("> /dev/") || lower.contains("shutdown") ||
                          lower.contains("reboot");
            if blocked {
                eprintln!("\x1b[33m[trust:standard] Blocked potentially dangerous command. Use --trust full to allow.\x1b[0m");
            }
            !blocked
        }
        TrustLevel::Full => true,
    }
}

// ── REPL with tool invocation tracking ────────────────────────────────────────

fn run_repl(agent: &mut Agent, args: &CliArgs) {
    use std::io::{self, BufRead, Write};
    let stdin = io::stdin();
    let no_color = args.no_color;

    // Banner
    if !no_color {
        println!("\x1b[38;2;69;243;255m\x1b[1mΣ sigma-agent-core\x1b[0m \x1b[38;2;107;114;128mv15.0 — SigmaOS AI CLI Agent\x1b[0m");
        println!("\x1b[38;2;107;114;128m  Type a natural language command, or 'help' for examples.\x1b[0m");
        println!();
    } else {
        println!("sigma-agent-core v15.0 — SigmaOS AI CLI Agent");
        println!("Type 'help' for examples.");
    }

    loop {
        // Prompt
        let prompt = if no_color { "σ> ".to_owned() }
                     else { "\x1b[38;2;69;243;255m\x1b[1mσ\x1b[0m \x1b[38;2;107;114;128m>\x1b[0m ".to_owned() };
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        let line = line.trim().to_owned();
        if line.is_empty() { continue; }

        // Trust check
        if !check_trust(&line, args.trust) { continue; }

        // Dry-run marker
        if args.dry_run {
            if !no_color { print!("\x1b[38;2;251;191;36m[dry-run] Would execute: {}\x1b[0m\n", line); }
            else { println!("[dry-run] Would execute: {}", line); }
            continue;
        }

        let result = agent.process(&line);
        if !no_color { println!("\n{}\n", colorise(&result)); }
        else { println!("\n{}\n", result); }
    }
}

fn colorise(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        let colored = if line.starts_with('✓') {
            format!("\x1b[38;2;52;211;153m{}\x1b[0m", line)
        } else if line.starts_with('✗') || line.to_ascii_lowercase().starts_with("error") {
            format!("\x1b[38;2;248;113;113m{}\x1b[0m", line)
        } else if line.starts_with('Σ') || line.starts_with("sigma") {
            format!("\x1b[38;2;69;243;255m{}\x1b[0m", line)
        } else {
            line.to_owned()
        };
        out.push_str(&colored);
        out.push('\n');
    }
    out.trim_end().to_owned()
}

// ── Pipe mode — read commands from stdin, one per line ─────────────────────────

fn run_pipe(agent: &mut Agent, args: &CliArgs) {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(l) => {
                let l = l.trim().to_owned();
                if l.is_empty() || l.starts_with('#') { continue; }
                if !check_trust(&l, args.trust) { continue; }
                if args.dry_run { println!("[dry-run] {}", l); continue; }
                println!("{}", agent.process(&l));
            }
            Err(_) => break,
        }
    }
}

// ── Entry point ────────────────────────────────────────────────────────────────

fn main() {
    let args = parse_args();
    let mut agent = build_agent(&args);

    match args.mode {
        CliMode::Repl => {
            run_repl(&mut agent, &args);
        }
        CliMode::Once => {
            let cmd = args.once_cmd.trim().to_owned();
            if !check_trust(&cmd, args.trust) { std::process::exit(1); }
            if args.dry_run {
                println!("[dry-run] Would execute: {}", cmd);
            } else {
                let result = agent.process(&cmd);
                if args.no_color { println!("{}", result); }
                else { println!("{}", colorise(&result)); }
            }
        }
        CliMode::Script => {
            let path = args.script.clone();
            if args.dry_run {
                // Show script contents without running
                match std::fs::read_to_string(&path) {
                    Ok(content) => {
                        for line in content.lines() {
                            let l = line.trim();
                            if !l.is_empty() && !l.starts_with('#') {
                                println!("[dry-run] Would execute: {}", l);
                            }
                        }
                    }
                    Err(e) => { eprintln!("Cannot read script {}: {}", path, e); std::process::exit(1); }
                }
            } else {
                agent.run_script(&path);
            }
        }
        CliMode::Pipe => {
            run_pipe(&mut agent, &args);
        }
    }
}
