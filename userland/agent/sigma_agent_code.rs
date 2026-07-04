// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/agent/sigma_agent_code.rs — Code editing + diff + git integration
// Inspiration: Claude Code (edit/diff workflow), Aider (git-aware), copilot-cli
// Language: Rust (std) — OOP via CodeAgent struct

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use super::sigma_llm::{LlmBackend, Message, GenParams, AutoBackend};

// ── Diff Types ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffKind { Added, Removed, Context }

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub kind:    DiffKind,
    pub number:  usize,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct FileDiff {
    pub path:  String,
    pub hunks: Vec<Vec<DiffLine>>,
    pub stats: DiffStats,
}

#[derive(Debug, Clone, Default)]
pub struct DiffStats { pub added: usize, pub removed: usize, pub changed: usize }

impl FileDiff {
    pub fn new(path: &str) -> Self { Self { path: path.to_owned(), hunks: Vec::new(), stats: Default::default() } }

    /// Compute a line-by-line diff between old and new content
    pub fn compute(path: &str, old: &str, new: &str) -> Self {
        let mut diff = Self::new(path);
        let old_lines: Vec<&str> = old.lines().collect();
        let new_lines: Vec<&str> = new.lines().collect();
        let mut hunk: Vec<DiffLine> = Vec::new();
        let mut i = 0; let mut j = 0;
        while i < old_lines.len() || j < new_lines.len() {
            match (old_lines.get(i), new_lines.get(j)) {
                (Some(&a), Some(&b)) if a == b => {
                    hunk.push(DiffLine { kind: DiffKind::Context, number: i+1, content: a.to_owned() });
                    i += 1; j += 1;
                }
                (Some(&a), Some(&b)) => {
                    hunk.push(DiffLine { kind: DiffKind::Removed, number: i+1, content: a.to_owned() });
                    hunk.push(DiffLine { kind: DiffKind::Added,   number: j+1, content: b.to_owned() });
                    diff.stats.removed += 1; diff.stats.added += 1; diff.stats.changed += 1;
                    i += 1; j += 1;
                }
                (Some(&a), None) => {
                    hunk.push(DiffLine { kind: DiffKind::Removed, number: i+1, content: a.to_owned() });
                    diff.stats.removed += 1; i += 1;
                }
                (None, Some(&b)) => {
                    hunk.push(DiffLine { kind: DiffKind::Added, number: j+1, content: b.to_owned() });
                    diff.stats.added += 1; j += 1;
                }
                (None, None) => break,
            }
        }
        if !hunk.is_empty() { diff.hunks.push(hunk); }
        diff
    }

    /// Render as unified diff string
    pub fn to_unified(&self) -> String {
        let mut out = format!("--- a/{}\n+++ b/{}\n", self.path, self.path);
        for hunk in &self.hunks {
            let start = hunk.first().map(|l| l.number).unwrap_or(1);
            out.push_str(&format!("@@ -{},{} +{},{} @@\n", start, hunk.len(), start, hunk.len()));
            for line in hunk {
                let prefix = match line.kind { DiffKind::Added => "+", DiffKind::Removed => "-", DiffKind::Context => " " };
                out.push_str(&format!("{}{}\n", prefix, line.content));
            }
        }
        out
    }

    /// Render with ANSI colours
    pub fn to_coloured(&self) -> String {
        let mut out = format!("\x1b[1m--- a/{}\n+++ b/{}\x1b[0m\n", self.path, self.path);
        for hunk in &self.hunks {
            let changed: Vec<&DiffLine> = hunk.iter().filter(|l| l.kind != DiffKind::Context).collect();
            if changed.is_empty() { continue; }
            for line in hunk {
                let (prefix, color) = match line.kind {
                    DiffKind::Added   => ("+", "\x1b[38;2;52;211;153m"),
                    DiffKind::Removed => ("-", "\x1b[38;2;248;113;113m"),
                    DiffKind::Context => (" ", "\x1b[38;2;107;114;128m"),
                };
                out.push_str(&format!("{}{}{}\x1b[0m\n", color, prefix, line.content));
            }
        }
        out
    }

    pub fn has_changes(&self) -> bool { self.stats.added > 0 || self.stats.removed > 0 }
}

// ── Edit Request ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct EditRequest {
    pub file:        PathBuf,
    pub instruction: String,
    pub context:     Option<String>,   // extra context (error message, test output, etc.)
    pub apply:       bool,             // true = apply; false = preview only
    pub git_commit:  bool,             // auto-commit after applying
    pub commit_msg:  Option<String>,
}

impl EditRequest {
    pub fn new(file: &str, instruction: &str) -> Self {
        Self { file: PathBuf::from(file), instruction: instruction.to_owned(),
               context: None, apply: false, git_commit: false, commit_msg: None }
    }
    pub fn with_context(mut self, ctx: &str) -> Self { self.context = Some(ctx.to_owned()); self }
    pub fn apply(mut self) -> Self { self.apply = true; self }
    pub fn with_commit(mut self, msg: &str) -> Self { self.git_commit = true; self.commit_msg = Some(msg.to_owned()); self }
}

// ── Edit Result ───────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct EditResult {
    pub original:    String,
    pub modified:    String,
    pub diff:        FileDiff,
    pub applied:     bool,
    pub committed:   bool,
    pub commit_hash: Option<String>,
    pub explanation: String,
}

// ── Code Agent ────────────────────────────────────────────────────────────────
// Architecture inspired by Claude Code + Aider

pub struct CodeAgent {
    backend:  AutoBackend,
    pub verbose:  bool,
}

impl CodeAgent {
    pub fn new() -> Self { Self { backend: AutoBackend::new(), verbose: false } }

    /// Main entry: analyse file + apply instruction via LLM
    pub fn edit(&self, req: &EditRequest) -> Result<EditResult, String> {
        let original = fs::read_to_string(&req.file)
            .map_err(|e| format!("Cannot read {:?}: {}", req.file, e))?;

        let file_ext = req.file.extension().and_then(|e| e.to_str()).unwrap_or("txt");
        let modified  = self.generate_edit(&original, &req.instruction, file_ext, req.context.as_deref())?;
        let diff      = FileDiff::compute(req.file.to_str().unwrap_or(""), &original, &modified);
        let explanation = self.explain_changes(&diff);

        let mut applied   = false;
        let mut committed = false;
        let mut hash      = None;

        if req.apply && diff.has_changes() {
            fs::write(&req.file, &modified).map_err(|e| format!("Write failed: {}", e))?;
            applied = true;
            if req.git_commit {
                let msg = req.commit_msg.clone().unwrap_or_else(|| format!("{}: {}", req.file.display(), req.instruction));
                if let Some(h) = self.git_commit_file(&req.file, &msg) { hash = Some(h); committed = true; }
            }
        }

        Ok(EditResult { original, modified, diff, applied, committed, commit_hash: hash, explanation })
    }

    fn generate_edit(&self, original: &str, instruction: &str, lang: &str, context: Option<&str>) -> Result<String, String> {
        let ctx_section = context.map(|c| format!("\n\nContext:\n{}", c)).unwrap_or_default();
        let prompt = format!(
            "Edit the following {} file according to this instruction: {}{}\n\
             Return ONLY the complete modified file content, no explanation, no markdown fences.\n\n\
             Original:\n```{}\n{}\n```",
            lang, instruction, ctx_section, lang, original
        );
        let messages = vec![
            Message::system("You are a precise code editor. When given a file and instruction, return only the complete modified file."),
            Message::user(&prompt),
        ];
        let params = GenParams::code();
        self.backend.generate(&messages, &params)
            .map_err(|e| format!("LLM error: {}", e))
            .map(|r| self.strip_fences(&r, lang))
    }

    fn strip_fences(&self, content: &str, lang: &str) -> String {
        // Remove markdown code fences if present
        let c = content.trim();
        let fence_open  = format!("```{}", lang);
        let fence_open2 = "```";
        let fence_close = "```";
        if c.starts_with(&fence_open) {
            let after = &c[fence_open.len()..];
            let end = after.rfind(fence_close).unwrap_or(after.len());
            return after[..end].trim().to_owned();
        }
        if c.starts_with(fence_open2) {
            let after = &c[fence_open2.len()..];
            let end = after.rfind(fence_close).unwrap_or(after.len());
            return after[..end].trim().to_owned();
        }
        c.to_owned()
    }

    fn explain_changes(&self, diff: &FileDiff) -> String {
        if !diff.has_changes() { return "No changes needed.".to_owned(); }
        let changed: Vec<String> = diff.hunks.iter().flat_map(|h| {
            h.iter().filter(|l| l.kind == DiffKind::Added).map(|l| l.content.trim().to_owned())
        }).take(5).collect();
        format!("{} lines added, {} removed. Changed: {}", diff.stats.added, diff.stats.removed, changed.join("; "))
    }

    fn git_commit_file(&self, path: &Path, msg: &str) -> Option<String> {
        let _ = Command::new("git").args(["add", path.to_str()?]).status().ok()?;
        let _ = Command::new("git").args(["commit", "-m", msg]).status().ok()?;
        let out = Command::new("git").args(["rev-parse", "--short", "HEAD"]).output().ok()?;
        Some(String::from_utf8_lossy(&out.stdout).trim().to_owned())
    }

    /// Review a file for issues without modifying it
    pub fn review(&self, path: &str) -> Result<String, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let ext = Path::new(path).extension().and_then(|e| e.to_str()).unwrap_or("txt");
        let messages = vec![
            Message::system("You are a SigmaOS code reviewer. List specific bugs, security issues, and improvements in the given file. Be concise."),
            Message::user(&format!("Review this {} file:\n```{}\n{}\n```", ext, ext, &content[..content.len().min(4000)])),
        ];
        self.backend.generate(&messages, &GenParams::precise()).map_err(|e| e.to_string())
    }

    /// Generate tests for a file
    pub fn generate_tests(&self, path: &str) -> Result<String, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let ext = Path::new(path).extension().and_then(|e| e.to_str()).unwrap_or("rs");
        let messages = vec![
            Message::system("Generate comprehensive unit tests for the following code. Return only the test code."),
            Message::user(&format!("Generate {} tests:\n```{}\n{}\n```", ext, ext, &content[..content.len().min(3000)])),
        ];
        self.backend.generate(&messages, &GenParams::code()).map_err(|e| e.to_string())
    }

    /// Explain code in plain English
    pub fn explain_code(&self, path: &str, selection: Option<(usize,usize)>) -> Result<String, String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let code = if let Some((start, end)) = selection {
            content.lines().skip(start.saturating_sub(1)).take(end-start+1).collect::<Vec<_>>().join("\n")
        } else { content[..content.len().min(3000)].to_owned() };
        let messages = vec![
            Message::system("Explain the following code in plain English. Be concise and highlight the key logic."),
            Message::user(&code),
        ];
        self.backend.generate(&messages, &GenParams::default()).map_err(|e| e.to_string())
    }

    /// Fix compiler/runtime errors in a file
    pub fn fix_errors(&self, path: &str, error_output: &str) -> Result<EditResult, String> {
        let req = EditRequest::new(path, &format!("Fix these errors:\n{}", &error_output[..error_output.len().min(1000)]))
            .with_context(error_output)
            .apply();
        self.edit(&req)
    }

    /// Refactor: rename, extract, or reorganize
    pub fn refactor(&self, path: &str, instruction: &str) -> Result<EditResult, String> {
        let req = EditRequest::new(path, instruction).apply();
        self.edit(&req)
    }
}

// ── Git Helper ────────────────────────────────────────────────────────────────

pub struct GitHelper;

impl GitHelper {
    pub fn is_git_repo() -> bool {
        Command::new("git").args(["rev-parse", "--git-dir"]).status()
            .map(|s| s.success()).unwrap_or(false)
    }

    pub fn current_branch() -> Option<String> {
        Command::new("git").args(["branch", "--show-current"]).output().ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_owned())
    }

    pub fn uncommitted_files() -> Vec<String> {
        Command::new("git").args(["diff", "--name-only"]).output().ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.lines().map(|l| l.to_owned()).collect())
            .unwrap_or_default()
    }

    pub fn last_commit_msg() -> Option<String> {
        Command::new("git").args(["log", "-1", "--pretty=%s"]).output().ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_owned())
    }

    pub fn auto_commit_message(files: &[&str], llm: &AutoBackend) -> String {
        if files.is_empty() { return "Update files".to_owned(); }
        let file_list = files.join(", ");
        let messages = vec![
            Message::system("Generate a concise git commit message (50 chars max) for the given changed files. Start with a verb."),
            Message::user(&format!("Changed files: {}", file_list)),
        ];
        let params = GenParams { max_tokens: 32, temperature: 0.3, ..GenParams::default() };
        llm.generate(&messages, &params).unwrap_or_else(|_| format!("Update {}", files[0]))
    }
}
