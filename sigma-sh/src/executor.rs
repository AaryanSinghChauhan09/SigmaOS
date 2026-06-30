// sigma-sh executor: Execute parsed AST nodes
// Handles: single commands, pipelines, background jobs, redirects

use std::fs::{File, OpenOptions};
use std::process::{Command as ProcCmd, Stdio};
use std::os::windows::io::IntoRawHandle;

use crate::parser::{Command, Pipeline, Redirect, Statement};
use crate::scripting::ShellEnv;

pub fn execute(stmt: &Statement, env: &mut ShellEnv) -> i32 {
    match stmt {
        Statement::Empty => 0,
        Statement::Command(cmd) => run_command(cmd, env),
        Statement::Pipeline(pipeline) => run_pipeline(pipeline, env),
        Statement::Sequence(stmts) => {
            let mut last = 0i32;
            for s in stmts {
                last = execute(s, env);
            }
            last
        }
        Statement::And(left, right) => {
            let code = execute(left, env);
            if code == 0 { execute(right, env) } else { code }
        }
        Statement::Or(left, right) => {
            let code = execute(left, env);
            if code != 0 { execute(right, env) } else { code }
        }
    }
}

fn run_command(cmd: &Command, env: &ShellEnv) -> i32 {
    if cmd.argv.is_empty() {
        return 0;
    }

    let program = &cmd.argv[0];
    let args = &cmd.argv[1..];

    let mut builder = ProcCmd::new(program);
    builder.args(args);

    // Apply environment
    for (k, v) in &env.vars {
        builder.env(k, v);
    }

    // Apply redirections
    for redir in &cmd.redirects {
        match redir {
            Redirect::Stdout(path) => {
                match File::create(path) {
                    Ok(f) => { builder.stdout(Stdio::from(f)); }
                    Err(e) => { eprintln!("sigma-sh: {}: {}", path, e); return 1; }
                }
            }
            Redirect::StdoutAppend(path) => {
                match OpenOptions::new().append(true).create(true).open(path) {
                    Ok(f) => { builder.stdout(Stdio::from(f)); }
                    Err(e) => { eprintln!("sigma-sh: {}: {}", path, e); return 1; }
                }
            }
            Redirect::Stdin(path) => {
                match File::open(path) {
                    Ok(f) => { builder.stdin(Stdio::from(f)); }
                    Err(e) => { eprintln!("sigma-sh: {}: {}", path, e); return 1; }
                }
            }
            Redirect::Stderr(path) => {
                match File::create(path) {
                    Ok(f) => { builder.stderr(Stdio::from(f)); }
                    Err(e) => { eprintln!("sigma-sh: {}: {}", path, e); return 1; }
                }
            }
        }
    }

    if cmd.background {
        match builder.spawn() {
            Ok(child) => {
                println!("[bg] pid {}", child.id());
                0
            }
            Err(e) => {
                eprintln!("sigma-sh: {}: {}", program, e);
                127
            }
        }
    } else {
        match builder.status() {
            Ok(status) => status.code().unwrap_or(0),
            Err(e) => {
                eprintln!("sigma-sh: {}: {}", program, e);
                127
            }
        }
    }
}

fn run_pipeline(pipeline: &Pipeline, env: &ShellEnv) -> i32 {
    let count = pipeline.commands.len();
    if count == 0 { return 0; }
    if count == 1 { return run_command(&pipeline.commands[0], env); }

    let mut children = Vec::new();
    let mut prev_stdout: Option<std::process::ChildStdout> = None;

    for (i, cmd) in pipeline.commands.iter().enumerate() {
        if cmd.argv.is_empty() { continue; }

        let mut builder = ProcCmd::new(&cmd.argv[0]);
        builder.args(&cmd.argv[1..]);
        for (k, v) in &env.vars { builder.env(k, v); }

        // Pipe stdin from previous
        if let Some(prev) = prev_stdout.take() {
            builder.stdin(Stdio::from(prev));
        }

        // All but last pipe stdout to next
        let is_last = i == count - 1;
        if !is_last {
            builder.stdout(Stdio::piped());
        }

        match builder.spawn() {
            Ok(mut child) => {
                if !is_last {
                    prev_stdout = child.stdout.take();
                }
                children.push(child);
            }
            Err(e) => {
                eprintln!("sigma-sh: {}: {}", &cmd.argv[0], e);
                return 127;
            }
        }
    }

    // Wait for all children, return last exit code
    let mut last = 0i32;
    for mut child in children {
        if let Ok(status) = child.wait() {
            last = status.code().unwrap_or(0);
        }
    }
    last
}
