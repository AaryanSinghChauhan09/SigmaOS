// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/shell/sigma_sh_features.rs — sigma-sh missing features
// Implements: globbing, job control, command substitution, parameter expansion,
//             functions, pipes/redirects, here-docs
//
// Inspired by: bash, fish, zsh feature sets (cleanroom study)
// Language: Rust (std)

use std::collections::HashMap;
use std::process::{Command, Stdio, Child};
use std::path::Path;

// ── Glob expansion ────────────────────────────────────────────────────────
/// Expand a glob pattern (* ? [a-z]) to matching file paths
pub fn glob_expand(pattern: &str) -> Vec<String> {
    let dir = if pattern.contains('/') {
        Path::new(pattern).parent()
            .and_then(|p| p.to_str())
            .unwrap_or(".")
            .to_owned()
    } else { ".".to_owned() };

    let file_pat = if pattern.contains('/') {
        Path::new(pattern).file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(pattern)
            .to_owned()
    } else { pattern.to_owned() };

    let mut matches = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if glob_match(&name, &file_pat) {
                let full = if dir == "." { name } else { format!("{}/{}", dir, name) };
                matches.push(full);
            }
        }
    }
    matches.sort();
    if matches.is_empty() { vec![pattern.to_owned()] } else { matches }
}

pub fn glob_match(name: &str, pattern: &str) -> bool {
    let nb: Vec<char> = name.chars().collect();
    let pb: Vec<char> = pattern.chars().collect();
    glob_match_inner(&nb, &pb, 0, 0)
}

fn glob_match_inner(name: &[char], pat: &[char], ni: usize, pi: usize) -> bool {
    if pi == pat.len() { return ni == name.len(); }
    match pat[pi] {
        '*' => {
            // * matches zero or more chars
            for i in ni..=name.len() {
                if glob_match_inner(name, pat, i, pi+1) { return true; }
            }
            false
        }
        '?' => ni < name.len() && glob_match_inner(name, pat, ni+1, pi+1),
        '[' => {
            // Character class [a-z], [!abc]
            let end = pat[pi..].iter().position(|&c| c == ']').unwrap_or(1) + pi;
            let negate = pi+1 < pat.len() && pat[pi+1] == '!';
            let start = if negate { pi+2 } else { pi+1 };
            if ni >= name.len() { return false; }
            let mut matched = false;
            let mut ci = start;
            while ci < end {
                if ci+2 < end && pat[ci+1] == '-' {
                    if name[ni] >= pat[ci] && name[ni] <= pat[ci+2] { matched = true; }
                    ci += 3;
                } else {
                    if name[ni] == pat[ci] { matched = true; }
                    ci += 1;
                }
            }
            let result = matched != negate;
            result && glob_match_inner(name, pat, ni+1, end+1)
        }
        c => ni < name.len() && name[ni] == c && glob_match_inner(name, pat, ni+1, pi+1),
    }
}

// ── Parameter expansion ───────────────────────────────────────────────────
/// Expand ${VAR:-default}, ${VAR##pattern}, ${VAR%pattern}, ${#VAR}
pub fn expand_params(input: &str, env: &HashMap<String, String>) -> String {
    let mut out = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' && i+1 < chars.len() {
            if chars[i+1] == '{' {
                // Find closing }
                let start = i+2;
                let end = chars[start..].iter().position(|&c| c == '}').map(|p| p+start);
                if let Some(end) = end {
                    let expr: String = chars[start..end].iter().collect();
                    out.push_str(&expand_param_expr(&expr, env));
                    i = end + 1;
                    continue;
                }
            } else if chars[i+1].is_alphanumeric() || chars[i+1] == '_' {
                // $VAR
                let start = i+1;
                let end = chars[start..].iter().position(|c| !c.is_alphanumeric() && *c != '_')
                    .map(|p| p+start).unwrap_or(chars.len());
                let var: String = chars[start..end].iter().collect();
                out.push_str(env.get(&var).map(|s| s.as_str()).unwrap_or(""));
                i = end; continue;
            }
        }
        out.push(chars[i]); i += 1;
    }
    out
}

fn expand_param_expr(expr: &str, env: &HashMap<String, String>) -> String {
    if let Some(pos) = expr.find(":-") {
        // ${VAR:-default}
        let var = &expr[..pos]; let default = &expr[pos+2..];
        return env.get(var).filter(|v| !v.is_empty()).cloned().unwrap_or_else(|| default.to_owned());
    }
    if let Some(pos) = expr.find(":=") {
        let var = &expr[..pos]; let default = &expr[pos+2..];
        return env.get(var).cloned().unwrap_or_else(|| default.to_owned());
    }
    if let Some(pos) = expr.find("##") {
        // ${VAR##pattern} — strip longest prefix
        let var = &expr[..pos]; let pat = &expr[pos+2..];
        let val = env.get(var).map(|s| s.as_str()).unwrap_or("");
        return strip_prefix(val, pat, true);
    }
    if let Some(pos) = expr.find('#') {
        if pos == 0 {
            // ${#VAR} — length
            let var = &expr[1..];
            return env.get(var).map(|s| s.len().to_string()).unwrap_or_else(|| "0".to_owned());
        }
        // ${VAR#pattern} — strip shortest prefix
        let var = &expr[..pos]; let pat = &expr[pos+1..];
        let val = env.get(var).map(|s| s.as_str()).unwrap_or("");
        return strip_prefix(val, pat, false);
    }
    if let Some(pos) = expr.find("%%") {
        let var = &expr[..pos]; let pat = &expr[pos+2..];
        let val = env.get(var).map(|s| s.as_str()).unwrap_or("");
        return strip_suffix(val, pat, true);
    }
    if let Some(pos) = expr.find('%') {
        let var = &expr[..pos]; let pat = &expr[pos+1..];
        let val = env.get(var).map(|s| s.as_str()).unwrap_or("");
        return strip_suffix(val, pat, false);
    }
    // Plain ${VAR}
    env.get(expr).cloned().unwrap_or_default()
}

fn strip_prefix(val: &str, pat: &str, greedy: bool) -> String {
    if greedy {
        for end in (0..=val.len()).rev() {
            if glob_match(&val[..end], pat) { return val[end..].to_owned(); }
        }
    } else {
        for end in 0..=val.len() {
            if glob_match(&val[..end], pat) { return val[end..].to_owned(); }
        }
    }
    val.to_owned()
}

fn strip_suffix(val: &str, pat: &str, greedy: bool) -> String {
    if greedy {
        for start in 0..=val.len() {
            if glob_match(&val[start..], pat) { return val[..start].to_owned(); }
        }
    } else {
        for start in (0..=val.len()).rev() {
            if glob_match(&val[start..], pat) { return val[..start].to_owned(); }
        }
    }
    val.to_owned()
}

// ── Command substitution $(...) ───────────────────────────────────────────
pub fn command_substitution(cmd: &str) -> String {
    let out = Command::new("sh").arg("-c").arg(cmd)
        .stdout(Stdio::piped()).output();
    match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim_end_matches('\n').to_owned(),
        Err(_) => String::new(),
    }
}

/// Expand all $(...) in a string
pub fn expand_subshells(input: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '$' && i+1 < chars.len() && chars[i+1] == '(' {
            let depth_start = i+2; let mut depth = 1; let mut j = depth_start;
            while j < chars.len() && depth > 0 {
                if chars[j] == '(' { depth += 1; }
                if chars[j] == ')' { depth -= 1; }
                if depth > 0 { j += 1; }
            }
            let cmd: String = chars[depth_start..j].iter().collect();
            out.push_str(&command_substitution(&cmd));
            i = j + 1; continue;
        }
        out.push(chars[i]); i += 1;
    }
    out
}

// ── Job control ────────────────────────────────────────────────────────────
#[derive(Debug)]
pub struct Job {
    pub id:      usize,
    pub command: String,
    pub child:   Child,
    pub stopped: bool,
}

pub struct JobTable {
    jobs: Vec<Job>,
    next_id: usize,
}

impl JobTable {
    pub fn new() -> Self { Self { jobs: Vec::new(), next_id: 1 } }

    pub fn add(&mut self, command: String, child: Child) -> usize {
        let id = self.next_id; self.next_id += 1;
        self.jobs.push(Job { id, command, child, stopped: false });
        id
    }

    pub fn fg(&mut self, id: usize) -> Option<i32> {
        let pos = self.jobs.iter().position(|j| j.id == id)?;
        let status = self.jobs[pos].child.wait().ok()?.code().unwrap_or(-1);
        self.jobs.remove(pos);
        Some(status)
    }

    pub fn list(&self) {
        for job in &self.jobs {
            let state = if job.stopped { "Stopped" } else { "Running" };
            println!("[{}] {} {}", job.id, state, job.command);
        }
    }

    pub fn wait_all(&mut self) {
        for job in self.jobs.iter_mut() {
            let _ = job.child.wait();
        }
        self.jobs.clear();
    }
}

// ── Shell function definitions ────────────────────────────────────────────
pub struct ShellFunction {
    pub name: String,
    pub body: Vec<String>,  // lines of the function body
}

pub struct FunctionTable {
    funcs: HashMap<String, ShellFunction>,
}

impl FunctionTable {
    pub fn new() -> Self { Self { funcs: HashMap::new() } }

    pub fn define(&mut self, name: &str, body: Vec<String>) {
        self.funcs.insert(name.to_owned(), ShellFunction { name: name.to_owned(), body });
    }

    pub fn get(&self, name: &str) -> Option<&ShellFunction> {
        self.funcs.get(name)
    }

    pub fn remove(&mut self, name: &str) -> bool {
        self.funcs.remove(name).is_some()
    }

    pub fn list(&self) {
        for name in self.funcs.keys() { println!("{}", name); }
    }
}

// ── Here-document parsing ──────────────────────────────────────────────────
pub fn parse_heredoc(delimiter: &str, lines: &mut impl Iterator<Item=String>) -> String {
    let mut content = String::new();
    for line in lines.by_ref() {
        if line.trim() == delimiter { break; }
        content.push_str(&line);
        content.push('\n');
    }
    content
}

// ── Pipeline execution ─────────────────────────────────────────────────────
/// Execute a pipeline: cmd1 | cmd2 | cmd3
pub fn execute_pipeline(commands: &[&str]) -> i32 {
    if commands.is_empty() { return 0; }
    if commands.len() == 1 {
        return Command::new("sh").arg("-c").arg(commands[0]).status()
            .map(|s| s.code().unwrap_or(-1)).unwrap_or(-1);
    }
    let mut children: Vec<Child> = Vec::new();
    let mut prev_stdout: Option<std::process::ChildStdout> = None;
    for (i, cmd) in commands.iter().enumerate() {
        let is_last = i == commands.len() - 1;
        let mut builder = Command::new("sh");
        builder.arg("-c").arg(cmd);
        if let Some(prev) = prev_stdout.take() {
            builder.stdin(Stdio::from(prev));
        }
        if !is_last { builder.stdout(Stdio::piped()); }
        match builder.spawn() {
            Ok(mut child) => {
                if !is_last { prev_stdout = child.stdout.take(); }
                children.push(child);
            }
            Err(_) => return -1,
        }
    }
    let mut last_status = 0;
    for mut child in children {
        last_status = child.wait().map(|s| s.code().unwrap_or(-1)).unwrap_or(-1);
    }
    last_status
}
