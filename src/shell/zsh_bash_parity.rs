use alloc::vec;
extern crate alloc;
// SigmaOS Advanced Zsh, Bash, Fish & BSD Shell Parity Engine
// Zero-dependency, #![no_std] compliant, zero-allocation shell enhancements

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// =========================================================================
// 1. STARSHIP / ZSH POWERLINE PROMPT ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptTheme {
    Minimal,
    Powerline,
    Rainbow,
    Starship,
}

#[derive(Debug, Clone)]
pub struct PromptSegment {
    pub text: String,
    pub fg_color: String,
    pub bg_color: String,
}

pub struct PowerlinePromptBuilder {
    pub theme: PromptTheme,
    pub user: String,
    pub host: String,
    pub current_dir: String,
    pub home_dir: String,
    pub last_exit_code: i32,
    pub git_branch: Option<String>,
    pub is_dirty: bool,
    pub is_root: bool,
}

impl PowerlinePromptBuilder {
    pub fn new() -> Self {
        Self {
            theme: PromptTheme::Starship,
            user: "sovereign".to_string(),
            host: "sigmaos".to_string(),
            current_dir: "/userland/home/sovereign/projects".to_string(),
            home_dir: "/userland/home/sovereign".to_string(),
            last_exit_code: 0,
            git_branch: Some("main".to_string()),
            is_dirty: false,
            is_root: false,
        }
    }

    /// Formats path replacing $HOME with '~'
    pub fn formatted_cwd(&self) -> String {
        if self.current_dir.starts_with(&self.home_dir) {
            let rel = &self.current_dir[self.home_dir.len()..];
            if rel.is_empty() {
                "~".to_string()
            } else {
                format!("~{}", rel)
            }
        } else {
            self.current_dir.clone()
        }
    }

    /// Expands Zsh-style prompt escape sequences (%n, %m, %~, %?, %#)
    pub fn expand_zsh_prompt(&self, template: &str) -> String {
        let mut result = template.to_string();
        result = result.replace("%n", &self.user);
        result = result.replace("%m", &self.host);
        result = result.replace("%~", &self.formatted_cwd());
        result = result.replace("%?", &self.last_exit_code.to_string());
        let symbol = if self.is_root { "#" } else { "%" };
        result = result.replace("%#", symbol);

        if let Some(ref branch) = self.git_branch {
            let dirty_flag = if self.is_dirty { "*" } else { "" };
            let git_str = format!(" ({}{})", branch, dirty_flag);
            result = result.replace("%g", &git_str);
        } else {
            result = result.replace("%g", "");
        }

        result
    }

    /// Renders Starship/Powerline styled color prompt string
    pub fn render_prompt(&self) -> String {
        match self.theme {
            PromptTheme::Minimal => {
                let symbol = if self.is_root { "#" } else { "$" };
                format!("{} {} ", self.formatted_cwd(), symbol)
            }
            PromptTheme::Starship => {
                let cwd = self.formatted_cwd();
                let status_symbol = if self.last_exit_code == 0 {
                    "\x1b[32m❯\x1b[0m"
                } else {
                    "\x1b[31m❯\x1b[0m"
                };
                let git_info = if let Some(ref branch) = self.git_branch {
                    let dirty = if self.is_dirty { "*" } else { "" };
                    format!(" \x1b[35mon  {}{}\x1b[0m", branch, dirty)
                } else {
                    "".to_string()
                };

                format!(
                    "\x1b[1;32m{}@{}\x1b[0m:\x1b[1;34m{}\x1b[0m{} {} ",
                    self.user, self.host, cwd, git_info, status_symbol
                )
            }
            PromptTheme::Powerline => {
                let cwd = self.formatted_cwd();
                format!(
                    "\x1b[44;30m {} \x1b[42;34m\x1b[30m {} \x1b[0;32m\x1b[0m ",
                    self.user, cwd
                )
            }
            PromptTheme::Rainbow => {
                format!(
                    "\x1b[31m{}\x1b[33m@{}\x1b[32m:\x1b[36m{}\x1b[35m❯\x1b[0m ",
                    self.user,
                    self.host,
                    self.formatted_cwd()
                )
            }
        }
    }
}

impl Default for PowerlinePromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. FISH / ZSH FUZZY AUTO-COMPLETION & PREDICTIVE CANDIDATES
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CandidateCategory {
    Alias = 0,
    Builtin = 1,
    Executable = 2,
    File = 3,
    OptionFlag = 4,
    History = 5,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionCandidate {
    pub text: String,
    pub description: String,
    pub category: CandidateCategory,
    pub match_score: usize,
}

pub struct FuzzyCompletionEngine {
    pub history: Vec<String>,
    pub builtins: Vec<String>,
    pub aliases: BTreeMap<String, String>,
    pub files: Vec<String>,
}

impl FuzzyCompletionEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            history: Vec::new(),
            builtins: Vec::new(),
            aliases: BTreeMap::new(),
            files: Vec::new(),
        };

        // Standard builtins
        engine.builtins.push("cd".to_string());
        engine.builtins.push("ls".to_string());
        engine.builtins.push("echo".to_string());
        engine.builtins.push("pwd".to_string());
        engine.builtins.push("systemctl".to_string());
        engine.builtins.push("sigpkg".to_string());
        engine.builtins.push("sysctl".to_string());
        engine.builtins.push("pushd".to_string());
        engine.builtins.push("popd".to_string());
        engine.builtins.push("dirs".to_string());
        engine.builtins.push("jobs".to_string());

        // Sample files
        engine.files.push("Cargo.toml".to_string());
        engine.files.push("README.md".to_string());
        engine.files.push("src/lib.rs".to_string());
        engine.files.push("src/shell/mod.rs".to_string());

        engine
    }

    /// Levenshtein edit distance for fuzzy candidate matching
    pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        let m = b1.len();
        let n = b2.len();

        let mut dp = alloc::vec![vec![0usize; n + 1]; m + 1];

        for i in 0..=m {
            dp[i][0] = i;
        }
        for j in 0..=n {
            dp[0][j] = j;
        }

        for i in 1..=m {
            for j in 1..=n {
                let cost = if b1[i - 1] == b2[j - 1] { 0 } else { 1 };
                dp[i][j] = (dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j - 1] + cost);
            }
        }

        dp[m][n]
    }

    /// Queries completions for an input prefix with exact & fuzzy ranking
    pub fn get_completions(&self, prefix: &str) -> Vec<CompletionCandidate> {
        if prefix.is_empty() {
            return Vec::new();
        }

        let mut candidates = Vec::new();

        // 1. History matches
        for cmd in self.history.iter().rev() {
            if cmd.starts_with(prefix) {
                candidates.push(CompletionCandidate {
                    text: cmd.clone(),
                    description: "History".to_string(),
                    category: CandidateCategory::History,
                    match_score: 0,
                });
            }
        }

        // 2. Alias matches
        for (alias, target) in &self.aliases {
            if alias.starts_with(prefix) {
                candidates.push(CompletionCandidate {
                    text: alias.clone(),
                    description: format!("Alias -> {}", target),
                    category: CandidateCategory::Alias,
                    match_score: 0,
                });
            }
        }

        // 3. Builtin matches (exact prefix or low Levenshtein)
        for builtin in &self.builtins {
            if builtin.starts_with(prefix) {
                candidates.push(CompletionCandidate {
                    text: builtin.clone(),
                    description: "Shell Builtin".to_string(),
                    category: CandidateCategory::Builtin,
                    match_score: 0,
                });
            } else {
                let dist = Self::levenshtein_distance(prefix, builtin);
                if dist <= 4 && prefix.len() >= 3 {
                    candidates.push(CompletionCandidate {
                        text: builtin.clone(),
                        description: format!("Fuzzy match (dist {})", dist),
                        category: CandidateCategory::Builtin,
                        match_score: dist,
                    });
                }
            }
        }

        // 4. File matches
        for file in &self.files {
            if file.starts_with(prefix) {
                candidates.push(CompletionCandidate {
                    text: file.clone(),
                    description: "File".to_string(),
                    category: CandidateCategory::File,
                    match_score: 0,
                });
            }
        }

        // Deduplicate keeping highest priority category
        candidates.sort_by(|a, b| {
            a.match_score
                .cmp(&b.match_score)
                .then_with(|| a.category.cmp(&b.category))
        });

        let mut unique = Vec::new();
        for c in candidates {
            if !unique
                .iter()
                .any(|existing: &CompletionCandidate| existing.text == c.text)
            {
                unique.push(c);
            }
        }

        unique
    }

    /// Returns Fish-style inline "ghost text" completion preview
    pub fn get_ghost_suggestion(&self, current_input: &str) -> Option<String> {
        let completions = self.get_completions(current_input);
        if let Some(first) = completions.first() {
            if first.text.starts_with(current_input) && first.text.len() > current_input.len() {
                return Some(first.text[current_input.len()..].to_string());
            }
        }
        None
    }

    pub fn get_ghost_ghost_or_suggestion(&self, current_input: &str) -> Option<String> {
        self.get_ghost_suggestion(current_input)
    }
}

impl Default for FuzzyCompletionEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. ZSH COLORIZED SYNTAX HIGHLIGHTER & TOKENIZER
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxTokenKind {
    CommandValid,
    CommandUnknown,
    Builtin,
    Alias,
    OptionFlag,
    Variable,
    StringLiteral,
    Operator,
    Comment,
    Argument,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightedToken {
    pub text: String,
    pub kind: SyntaxTokenKind,
    pub ansi_color: &'static str,
}

pub struct ZshSyntaxHighlighter {
    pub valid_commands: Vec<String>,
}

impl ZshSyntaxHighlighter {
    pub fn new() -> Self {
        let mut highlighter = Self {
            valid_commands: Vec::new(),
        };

        // Builtins & common commands
        let common = [
            "ls",
            "cd",
            "pwd",
            "echo",
            "cat",
            "rm",
            "mkdir",
            "touch",
            "grep",
            "find",
            "systemctl",
            "sigpkg",
            "sysctl",
            "pushd",
            "popd",
            "dirs",
            "jobs",
            "ps",
            "whoami",
        ];
        for c in &common {
            highlighter.valid_commands.push(c.to_string());
        }

        highlighter
    }

    pub fn highlight_line(&self, line: &str) -> Vec<HighlightedToken> {
        let mut tokens = Vec::new();
        let parts: Vec<&str> = line.split_whitespace().collect();

        for (idx, &part) in parts.iter().enumerate() {
            let (kind, color) = if part.starts_with('#') {
                (SyntaxTokenKind::Comment, "\x1b[90m") // Gray
            } else if idx == 0
                || parts.get(idx.saturating_sub(1)) == Some(&"|")
                || parts.get(idx.saturating_sub(1)) == Some(&"&&")
            {
                if self.valid_commands.contains(&part.to_string()) {
                    (SyntaxTokenKind::CommandValid, "\x1b[32m") // Green
                } else {
                    (SyntaxTokenKind::CommandUnknown, "\x1b[31m") // Red
                }
            } else if part == "|"
                || part == ">"
                || part == ">>"
                || part == "<"
                || part == "&&"
                || part == "||"
            {
                (SyntaxTokenKind::Operator, "\x1b[34m") // Blue
            } else if part.starts_with('-') {
                (SyntaxTokenKind::OptionFlag, "\x1b[36m") // Cyan
            } else if part.starts_with('$') {
                (SyntaxTokenKind::Variable, "\x1b[33m") // Yellow
            } else if part.starts_with('"') || part.starts_with('\'') {
                (SyntaxTokenKind::StringLiteral, "\x1b[35m") // Magenta
            } else {
                (SyntaxTokenKind::Argument, "\x1b[37m") // White
            };

            tokens.push(HighlightedToken {
                text: part.to_string(),
                kind,
                ansi_color: color,
            });
        }

        tokens
    }

    pub fn render_ansi_string(&self, line: &str) -> String {
        let highlighted = self.highlight_line(line);
        let mut output = String::new();
        for (i, token) in highlighted.iter().enumerate() {
            if i > 0 {
                output.push(' ');
            }
            output.push_str(token.ansi_color);
            output.push_str(&token.text);
            output.push_str("\x1b[0m");
        }
        output
    }
}

impl Default for ZshSyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. BASH PARAMETER EXPANSION & WILDCARD GLOBBING
// =========================================================================

pub struct BashParameterExpansion;

impl BashParameterExpansion {
    /// Expands bash parameter syntax: ${VAR:-default}, ${VAR#prefix}, ${VAR%suffix}, ${#VAR}, ${VAR//pattern/replacement}, ${VAR:offset:length}
    pub fn expand(expr: &str, env: &BTreeMap<String, String>) -> String {
        if !expr.starts_with("${") || !expr.ends_with('}') {
            return expr.to_string();
        }

        let inner = &expr[2..expr.len() - 1];

        // 1. ${#VAR} - string length
        if inner.starts_with('#') {
            let var_name = &inner[1..];
            let val = env.get(var_name).cloned().unwrap_or_default();
            return val.len().to_string();
        }

        // 2. ${VAR//search/replace} - global replace
        if let Some(pos) = inner.find("//") {
            let var_name = &inner[..pos];
            let rest = &inner[pos + 2..];
            let val = env.get(var_name).cloned().unwrap_or_default();
            if let Some(sep) = rest.find('/') {
                let pattern = &rest[..sep];
                let replacement = &rest[sep + 1..];
                return val.replace(pattern, replacement);
            } else {
                return val.replace(rest, "");
            }
        }

        // 3. ${VAR:offset:length} - substring slicing
        if let Some(pos) = inner.find(':') {
            if !inner.contains(":-") {
                let var_name = &inner[..pos];
                let slice_spec = &inner[pos + 1..];
                let val = env.get(var_name).cloned().unwrap_or_default();
                if let Some(len_pos) = slice_spec.find(':') {
                    let offset: usize = slice_spec[..len_pos].parse().unwrap_or(0);
                    let length: usize = slice_spec[len_pos + 1..].parse().unwrap_or(val.len());
                    if offset < val.len() {
                        let end = (offset + length).min(val.len());
                        return val[offset..end].to_string();
                    }
                    return String::new();
                } else if let Ok(offset) = slice_spec.parse::<usize>() {
                    if offset < val.len() {
                        return val[offset..].to_string();
                    }
                    return String::new();
                }
            }
        }

        // 4. ${VAR:-default} - default value
        if let Some(pos) = inner.find(":-") {
            let var_name = &inner[..pos];
            let default_val = &inner[pos + 2..];
            if let Some(val) = env.get(var_name) {
                if !val.is_empty() {
                    return val.clone();
                }
            }
            return default_val.to_string();
        }

        // 5. ${VAR#prefix} - strip prefix
        if let Some(pos) = inner.find('#') {
            let var_name = &inner[..pos];
            let prefix = &inner[pos + 1..];
            let val = env.get(var_name).cloned().unwrap_or_default();
            if val.starts_with(prefix) {
                return val[prefix.len()..].to_string();
            }
            return val;
        }

        // 6. ${VAR%suffix} - strip suffix
        if let Some(pos) = inner.find('%') {
            let var_name = &inner[..pos];
            let suffix = &inner[pos + 1..];
            let val = env.get(var_name).cloned().unwrap_or_default();
            if val.ends_with(suffix) {
                return val[..val.len() - suffix.len()].to_string();
            }
            return val;
        }

        // Direct variable lookup
        env.get(inner).cloned().unwrap_or_default()
    }
}

pub struct WildcardGlobMatcher;

impl WildcardGlobMatcher {
    /// Glob pattern matching supporting `*` and `?`
    pub fn is_match(pattern: &str, text: &str) -> bool {
        let p_bytes = pattern.as_bytes();
        let t_bytes = text.as_bytes();
        let p_len = p_bytes.len();
        let t_len = t_bytes.len();

        let mut dp = alloc::vec![vec![false; t_len + 1]; p_len + 1];
        dp[0][0] = true;

        for i in 1..=p_len {
            if p_bytes[i - 1] == b'*' {
                dp[i][0] = dp[i - 1][0];
            }
        }

        for i in 1..=p_len {
            for j in 1..=t_len {
                if p_bytes[i - 1] == b'*' {
                    dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
                } else if p_bytes[i - 1] == b'?' || p_bytes[i - 1] == t_bytes[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }

        dp[p_len][t_len]
    }

    /// Filters a list of filenames matching a glob pattern
    pub fn filter_matches<'a>(pattern: &str, candidates: &[&'a str]) -> Vec<&'a str> {
        candidates
            .iter()
            .copied()
            .filter(|c| Self::is_match(pattern, c))
            .collect()
    }
}

// =========================================================================
// 5. PIPELINE PARSER & BSD DIRECTORY STACK / JOB CONTROL
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCommand {
    pub program: String,
    pub args: Vec<String>,
    pub stdin_file: Option<String>,
    pub stdout_file: Option<String>,
    pub append_stdout: bool,
    pub stderr_file: Option<String>,
    pub here_string: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellPipeline {
    pub stages: Vec<PipelineCommand>,
    pub run_in_background: bool,
    pub condition_next: Option<String>, // "&&" or "||"
}

pub struct ShellPipelineParser;

impl ShellPipelineParser {
    pub fn parse(line: &str) -> ShellPipeline {
        let trimmed = line.trim();
        let is_bg = trimmed.ends_with('&') && !trimmed.ends_with("&&");
        let clean_line = if is_bg {
            trimmed[..trimmed.len() - 1].trim()
        } else {
            trimmed
        };

        let raw_stages: Vec<&str> = clean_line.split('|').collect();
        let mut stages = Vec::new();

        for stage in raw_stages {
            let tokens: Vec<&str> = stage.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            let mut program = String::new();
            let mut args = Vec::new();
            let mut stdin_file = None;
            let mut stdout_file = None;
            let mut append_stdout = false;
            let mut stderr_file = None;
            let mut here_string = None;

            let mut i = 0;
            while i < tokens.len() {
                let tok = tokens[i];
                if tok == "<<<" && i + 1 < tokens.len() {
                    here_string = Some(
                        tokens[i + 1]
                            .trim_matches('"')
                            .trim_matches('\'')
                            .to_string(),
                    );
                    i += 2;
                } else if tok == "<" && i + 1 < tokens.len() {
                    stdin_file = Some(tokens[i + 1].to_string());
                    i += 2;
                } else if tok == ">" && i + 1 < tokens.len() {
                    stdout_file = Some(tokens[i + 1].to_string());
                    append_stdout = false;
                    i += 2;
                } else if tok == ">>" && i + 1 < tokens.len() {
                    stdout_file = Some(tokens[i + 1].to_string());
                    append_stdout = true;
                    i += 2;
                } else if tok == "2>" && i + 1 < tokens.len() {
                    stderr_file = Some(tokens[i + 1].to_string());
                    i += 2;
                } else {
                    if program.is_empty() {
                        program = tok.to_string();
                    } else {
                        args.push(tok.to_string());
                    }
                    i += 1;
                }
            }

            if !program.is_empty() {
                stages.push(PipelineCommand {
                    program,
                    args,
                    stdin_file,
                    stdout_file,
                    append_stdout,
                    stderr_file,
                    here_string,
                });
            }
        }

        ShellPipeline {
            stages,
            run_in_background: is_bg,
            condition_next: None,
        }
    }
}

pub struct BsdDirectoryStack {
    pub stack: Vec<String>,
    pub current_dir: String,
}

impl BsdDirectoryStack {
    pub fn new(initial_dir: &str) -> Self {
        Self {
            stack: Vec::new(),
            current_dir: initial_dir.to_string(),
        }
    }

    pub fn pushd(&mut self, new_dir: &str) -> String {
        self.stack.push(self.current_dir.clone());
        self.current_dir = new_dir.to_string();
        self.dirs()
    }

    pub fn popd(&mut self) -> Result<String, &'static str> {
        if let Some(dir) = self.stack.pop() {
            self.current_dir = dir;
            Ok(self.dirs())
        } else {
            Err("popd: Directory stack empty")
        }
    }

    pub fn dirs(&self) -> String {
        let mut list = Vec::new();
        list.push(self.current_dir.as_str());
        for d in self.stack.iter().rev() {
            list.push(d.as_str());
        }
        list.join(" ")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Running,
    Stopped,
    Terminated,
}

#[derive(Debug, Clone)]
pub struct ShellJob {
    pub id: usize,
    pub pid: usize,
    pub command: String,
    pub state: JobState,
}

pub struct ShellJobControl {
    pub jobs: Vec<ShellJob>,
    pub next_job_id: usize,
}

impl ShellJobControl {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            next_job_id: 1,
        }
    }

    pub fn add_job(&mut self, pid: usize, command: &str) -> usize {
        let id = self.next_job_id;
        self.next_job_id += 1;
        self.jobs.push(ShellJob {
            id,
            pid,
            command: command.to_string(),
            state: JobState::Running,
        });
        id
    }

    pub fn list_jobs(&self) -> String {
        if self.jobs.is_empty() {
            return "No active jobs.".to_string();
        }
        let mut out = String::new();
        for job in &self.jobs {
            let state_str = match job.state {
                JobState::Running => "Running",
                JobState::Stopped => "Stopped",
                JobState::Terminated => "Terminated",
            };
            out.push_str(&format!(
                "[{}] PID {}  {}  {}\n",
                job.id, job.pid, state_str, job.command
            ));
        }
        out
    }

    pub fn bring_to_foreground(&mut self, id: usize) -> Result<String, String> {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == id) {
            job.state = JobState::Running;
            Ok(format!(
                "[{}] Job '{}' (PID {}) brought to foreground.",
                job.id, job.command, job.pid
            ))
        } else {
            Err(format!("Job %{} not found", id))
        }
    }

    pub fn send_to_background(&mut self, id: usize) -> Result<String, String> {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == id) {
            job.state = JobState::Running;
            Ok(format!(
                "[{}] Job '{}' (PID {}) running in background.",
                job.id, job.command, job.pid
            ))
        } else {
            Err(format!("Job %{} not found", id))
        }
    }

    pub fn stop_job(&mut self, id: usize) -> bool {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == id) {
            job.state = JobState::Stopped;
            true
        } else {
            false
        }
    }

    pub fn resume_job(&mut self, id: usize) -> bool {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == id) {
            job.state = JobState::Running;
            true
        } else {
            false
        }
    }

    pub fn remove_job(&mut self, id: usize) -> bool {
        if let Some(pos) = self.jobs.iter().position(|j| j.id == id) {
            self.jobs.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for ShellJobControl {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 6. ZSH/FISH-INSPIRED SCRIPT HOOKS & ARITHMETIC EVALUATOR
// =========================================================================

pub struct ShellScriptHookEngine {
    pub precmd_hooks: Vec<String>,
    pub preexec_hooks: Vec<String>,
    pub chpwd_hooks: Vec<String>,
}

impl ShellScriptHookEngine {
    pub fn new() -> Self {
        Self {
            precmd_hooks: Vec::new(),
            preexec_hooks: Vec::new(),
            chpwd_hooks: Vec::new(),
        }
    }

    pub fn add_precmd_hook(&mut self, cmd: &str) {
        self.precmd_hooks.push(cmd.to_string());
    }

    pub fn add_preexec_hook(&mut self, cmd: &str) {
        self.preexec_hooks.push(cmd.to_string());
    }

    pub fn add_chpwd_hook(&mut self, cmd: &str) {
        self.chpwd_hooks.push(cmd.to_string());
    }

    pub fn trigger_precmd(&self) -> Vec<String> {
        self.precmd_hooks.clone()
    }

    pub fn trigger_preexec(&self, command_line: &str) -> Vec<String> {
        self.preexec_hooks
            .iter()
            .map(|hook| format!("{} {}", hook, command_line))
            .collect()
    }

    pub fn trigger_chpwd(&self, old_pwd: &str, new_pwd: &str) -> Vec<String> {
        self.chpwd_hooks
            .iter()
            .map(|hook| format!("{} {} {}", hook, old_pwd, new_pwd))
            .collect()
    }
}

impl Default for ShellScriptHookEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ShellArithmeticEvaluator;

impl ShellArithmeticEvaluator {
    /// Evaluates Bash `$(( expr ))` arithmetic expressions (e.g. `$(( 10 + 20 * 2 ))`)
    pub fn evaluate(expr: &str) -> Result<i64, &'static str> {
        let clean = expr.trim();
        let inner = if clean.starts_with("$(( ") || clean.starts_with("$(((") {
            clean
                .trim_start_matches("$(( ")
                .trim_start_matches("$(((")
                .trim_end_matches(" ))")
                .trim_end_matches(")))")
        } else if clean.starts_with("$(( ") || clean.starts_with("$(( ") {
            clean.trim_start_matches("$(( ").trim_end_matches(" ))")
        } else if clean.starts_with("$((") && clean.ends_with("))") {
            &clean[3..clean.len() - 2]
        } else {
            clean
        };

        let tokens: Vec<&str> = inner.split_whitespace().collect();
        if tokens.is_empty() {
            return Ok(0);
        }

        if tokens.len() == 1 {
            return tokens[0].parse::<i64>().map_err(|_| "Invalid integer");
        }

        if tokens.len() == 3 {
            let left = tokens[0]
                .parse::<i64>()
                .map_err(|_| "Invalid left operand")?;
            let op = tokens[1];
            let right = tokens[2]
                .parse::<i64>()
                .map_err(|_| "Invalid right operand")?;

            match op {
                "+" => Ok(left + right),
                "-" => Ok(left - right),
                "*" => Ok(left * right),
                "/" => {
                    if right == 0 {
                        Err("Division by zero")
                    } else {
                        Ok(left / right)
                    }
                }
                "%" => {
                    if right == 0 {
                        Err("Modulo by zero")
                    } else {
                        Ok(left % right)
                    }
                }
                _ => Err("Unsupported arithmetic operator"),
            }
        } else {
            Err("Complex arithmetic expression not supported")
        }
    }
}

// =========================================================================
// 6. UNIVERSAL SHELL COMPATIBILITY ENGINE (Bash, Zsh, Fish, Tcsh, Ksh, Dash)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellDialect {
    Bash,
    Zsh,
    Fish,
    Tcsh,
    Ksh,
    Dash,
    BsdSh,
}

pub struct FishAbbreviationEngine {
    pub abbreviations: Vec<(String, String)>,
}

impl FishAbbreviationEngine {
    pub fn new() -> Self {
        Self {
            abbreviations: Vec::new(),
        }
    }

    pub fn add_abbreviation(&mut self, abbr: &str, expansion: &str) {
        self.abbreviations
            .push((abbr.to_string(), expansion.to_string()));
    }

    pub fn expand_line(&self, line: &str) -> String {
        let trimmed = line.trim();
        let mut parts = trimmed.split_whitespace();
        if let Some(first) = parts.next() {
            for (abbr, expansion) in &self.abbreviations {
                if first == abbr {
                    let rest: Vec<&str> = parts.collect();
                    if rest.is_empty() {
                        return expansion.clone();
                    } else {
                        return format!("{} {}", expansion, rest.join(" "));
                    }
                }
            }
        }
        line.to_string()
    }
}

pub struct TcshHistorySubstitutionEngine {
    pub history: Vec<String>,
}

impl TcshHistorySubstitutionEngine {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn push_history(&mut self, cmd: &str) {
        self.history.push(cmd.to_string());
    }

    pub fn substitute(&self, line: &str) -> Result<String, &'static str> {
        let trimmed = line.trim();
        if !trimmed.contains('!') {
            return Ok(line.to_string());
        }

        let last_cmd = self.history.last().ok_or("No event in history")?;
        let last_parts: Vec<&str> = last_cmd.split_whitespace().collect();

        if trimmed == "!!" {
            return Ok(last_cmd.clone());
        }
        if trimmed == "!$" {
            let last_arg = last_parts.last().ok_or("No argument in history")?;
            return Ok(last_arg.to_string());
        }
        if trimmed == "!*" {
            if last_parts.len() > 1 {
                return Ok(last_parts[1..].join(" "));
            } else {
                return Ok(String::new());
            }
        }
        if trimmed.starts_with("!") {
            let prefix = &trimmed[1..];
            for prev in self.history.iter().rev() {
                if prev.starts_with(prefix) {
                    return Ok(prev.clone());
                }
            }
            return Err("No matching history entry");
        }

        Ok(line.to_string())
    }
}

pub struct KshParameterExpansionEngine;

impl KshParameterExpansionEngine {
    pub fn expand_parameter(expr: &str, env: &[(String, String)]) -> String {
        // Handle ${var:-default}
        if expr.contains(":-") {
            if let (Some(start), Some(end)) = (expr.find("${"), expr.find('}')) {
                let inner = &expr[start + 2..end];
                if let Some(pos) = inner.find(":-") {
                    let var_name = &inner[..pos];
                    let default_val = &inner[pos + 2..];
                    for (k, v) in env {
                        if k == var_name && !v.is_empty() {
                            return format!("{}{}{}", &expr[..start], v, &expr[end + 1..]);
                        }
                    }
                    return format!("{}{}{}", &expr[..start], default_val, &expr[end + 1..]);
                }
            }
        }

        // Handle ${#var} (String length)
        if expr.contains("${#") {
            if let (Some(start), Some(end)) = (expr.find("${#"), expr.find('}')) {
                let var_name = &expr[start + 3..end];
                for (k, v) in env {
                    if k == var_name {
                        let len_str = v.len().to_string();
                        return format!("{}{}{}", &expr[..start], len_str, &expr[end + 1..]);
                    }
                }
                return format!("{}0{}", &expr[..start], &expr[end + 1..]);
            }
        }

        // Handle ${var//search/replace}
        if expr.contains("//") {
            if let (Some(start), Some(end)) = (expr.find("${"), expr.find('}')) {
                let inner = &expr[start + 2..end];
                if let Some(pos) = inner.find("//") {
                    let var_name = &inner[..pos];
                    let rest = &inner[pos + 2..];
                    if let Some(slash_idx) = rest.find('/') {
                        let search = &rest[..slash_idx];
                        let replace = &rest[slash_idx + 1..];
                        for (k, v) in env {
                            if k == var_name {
                                let substituted = v.replace(search, replace);
                                return format!(
                                    "{}{}{}",
                                    &expr[..start],
                                    substituted,
                                    &expr[end + 1..]
                                );
                            }
                        }
                    }
                }
            }
        }

        expr.to_string()
    }
}

pub struct DashPosixShValidator;

impl DashPosixShValidator {
    pub fn validate_posix_compliance(script: &str) -> Vec<(u32, &'static str)> {
        let mut non_posix_warnings = Vec::new();
        for (line_num, line) in script.lines().enumerate() {
            let l = line.trim();
            if l.contains("[[") {
                non_posix_warnings.push((
                    (line_num + 1) as u32,
                    "Non-POSIX conditional [[ ]]; use [ ] instead",
                ));
            }
            if l.contains("<<<") {
                non_posix_warnings.push((
                    (line_num + 1) as u32,
                    "Non-POSIX here-string <<<; use echo | or cat <<",
                ));
            }
            if l.starts_with("function ") {
                non_posix_warnings.push((
                    (line_num + 1) as u32,
                    "Non-POSIX 'function' keyword; use foo() { ... }",
                ));
            }
            if l.contains("=(") {
                non_posix_warnings.push((
                    (line_num + 1) as u32,
                    "Non-POSIX array syntax foo=(...); arrays not supported in dash/sh",
                ));
            }
        }
        non_posix_warnings
    }
}

pub struct UniversalShellCompatibilityEngine {
    pub fish_abbr: FishAbbreviationEngine,
    pub tcsh_history: TcshHistorySubstitutionEngine,
    pub environment: Vec<(String, String)>,
}

impl UniversalShellCompatibilityEngine {
    pub fn new() -> Self {
        Self {
            fish_abbr: FishAbbreviationEngine::new(),
            tcsh_history: TcshHistorySubstitutionEngine::new(),
            environment: Vec::new(),
        }
    }

    pub fn detect_shebang_dialect(script: &str) -> ShellDialect {
        if let Some(first_line) = script.lines().next() {
            let trimmed = first_line.trim();
            if trimmed.starts_with("#!") {
                if trimmed.contains("bash") {
                    return ShellDialect::Bash;
                } else if trimmed.contains("zsh") {
                    return ShellDialect::Zsh;
                } else if trimmed.contains("fish") {
                    return ShellDialect::Fish;
                } else if trimmed.contains("tcsh") || trimmed.contains("csh") {
                    return ShellDialect::Tcsh;
                } else if trimmed.contains("ksh") {
                    return ShellDialect::Ksh;
                } else if trimmed.contains("dash") {
                    return ShellDialect::Dash;
                } else if trimmed.contains("sh") {
                    return ShellDialect::BsdSh;
                }
            }
        }
        ShellDialect::Bash
    }

    pub fn process_input_line(&mut self, line: &str) -> Result<ShellPipeline, &'static str> {
        let hist_sub = self.tcsh_history.substitute(line)?;
        let abbr_expanded = self.fish_abbr.expand_line(&hist_sub);
        let param_expanded =
            KshParameterExpansionEngine::expand_parameter(&abbr_expanded, &self.environment);
        self.tcsh_history.push_history(line);
        Ok(ShellPipelineParser::parse(&param_expanded))
    }

    /// Transpiles any script into a vector of executable POSIX `/bin/sh` pipelines
    pub fn execute_script_as_sh(&mut self, script: &str) -> Result<Vec<ShellPipeline>, &'static str> {
        let dialect = Self::detect_shebang_dialect(script);
        let posix_script = UniversalScriptTranspiler::transpile_to_posix_sh(script, dialect);

        let mut pipelines = Vec::new();
        for line in posix_script.lines() {
            let l = line.trim();
            if l.is_empty() || l.starts_with('#') {
                continue;
            }
            if let Ok(pipeline) = self.process_input_line(l) {
                if !pipeline.stages.is_empty() {
                    pipelines.push(pipeline);
                }
            }
        }
        Ok(pipelines)
    }
}

pub struct UniversalScriptTranspiler;

impl UniversalScriptTranspiler {
    /// Transpiles multi-dialect shell scripts (Fish, Tcsh, Bash, Zsh, Ksh) into POSIX /bin/sh compliant syntax
    pub fn transpile_to_posix_sh(script: &str, dialect: ShellDialect) -> String {
        let mut transpiled = String::new();
        let mut in_function = false;

        for line in script.lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("#!") {
                transpiled.push_str("#!/bin/sh\n");
                continue;
            }

            let converted_line = match dialect {
                ShellDialect::Fish => Self::transpile_fish_line(trimmed, &mut in_function),
                ShellDialect::Tcsh => Self::transpile_tcsh_line(trimmed),
                ShellDialect::Bash | ShellDialect::Zsh | ShellDialect::Ksh => Self::transpile_bash_zsh_line(trimmed),
                ShellDialect::Dash | ShellDialect::BsdSh => trimmed.to_string(),
            };

            transpiled.push_str(&converted_line);
            transpiled.push('\n');
        }

        transpiled
    }

    fn transpile_fish_line(line: &str, in_function: &mut bool) -> String {
        let mut l = line.to_string();

        // 1. Fish 'set -g VAR val' or 'set VAR val' -> 'VAR=val' / 'export VAR=val'
        if l.starts_with("set -x ") || l.starts_with("set -gx ") {
            let rest = l.trim_start_matches("set -x ").trim_start_matches("set -gx ");
            if let Some(space_idx) = rest.find(' ') {
                let var = &rest[..space_idx];
                let val = &rest[space_idx + 1..];
                return format!("export {}={}", var, val);
            }
        } else if l.starts_with("set -g ") || l.starts_with("set ") {
            let rest = l.trim_start_matches("set -g ").trim_start_matches("set ");
            if let Some(space_idx) = rest.find(' ') {
                let var = &rest[..space_idx];
                let val = &rest[space_idx + 1..];
                return format!("{}={}", var, val);
            }
        }

        // 2. Fish 'and' / 'or' -> '&&' / '||'
        if l.starts_with("and ") {
            l = format!("&& {}", &l[4..]);
        } else if l.starts_with("or ") {
            l = format!("|| {}", &l[3..]);
        }

        // 3. Fish 'function foo' -> 'foo() {'
        if l.starts_with("function ") {
            let func_name = l.trim_start_matches("function ").trim();
            *in_function = true;
            return format!("{}() {{", func_name);
        }

        // 4. Fish 'end' -> '}' if in function
        if l == "end" && *in_function {
            *in_function = false;
            return "}".to_string();
        }

        l
    }

    fn transpile_tcsh_line(line: &str) -> String {
        let l = line.to_string();

        // 1. Tcsh 'setenv VAR val' -> 'export VAR=val'
        if l.starts_with("setenv ") {
            let rest = l.trim_start_matches("setenv ");
            let parts: Vec<&str> = rest.split_whitespace().collect();
            if parts.len() >= 2 {
                return format!("export {}={}", parts[0], parts[1..].join(" "));
            } else if parts.len() == 1 {
                return format!("export {}=", parts[0]);
            }
        }

        // 2. Tcsh 'alias foo bar' -> 'alias foo="bar"'
        if l.starts_with("alias ") {
            let rest = l.trim_start_matches("alias ");
            if let Some(space_idx) = rest.find(' ') {
                let name = &rest[..space_idx];
                let cmd = &rest[space_idx + 1..];
                return format!("alias {}={}", name, cmd);
            }
        }

        l
    }

    fn transpile_bash_zsh_line(line: &str) -> String {
        let mut l = line.to_string();

        // 1. [[ expr ]] -> [ expr ]
        if l.contains("[[") && l.contains("]]") {
            l = l.replace("[[", "[").replace("]]", "]");
        }

        // 2. <<< "here string" -> echo "here string" |
        if l.contains("<<<") {
            if let Some(pos) = l.find("<<<") {
                let cmd = &l[..pos].trim();
                let string_val = &l[pos + 3..].trim();
                return format!("echo {} | {}", string_val, cmd);
            }
        }

        // 3. function foo() -> foo()
        if l.starts_with("function ") {
            let rest = l.trim_start_matches("function ").trim();
            if !rest.contains("()") {
                if let Some(idx) = rest.find(' ') {
                    let name = &rest[..idx];
                    let body = &rest[idx..];
                    return format!("{}() {}", name, body);
                } else {
                    return format!("{}()", rest);
                }
            }
        }

        l
    }
}

impl Default for UniversalShellCompatibilityEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_powerline_prompt_expansions() {
        let mut builder = PowerlinePromptBuilder::new();
        builder.user = "root".to_string();
        builder.host = "sigmaos".to_string();
        builder.current_dir = "/userland/home/sovereign/docs".to_string();
        builder.home_dir = "/userland/home/sovereign".to_string();
        builder.last_exit_code = 127;
        builder.is_root = true;

        let expanded = builder.expand_zsh_prompt("[%n@%m %~ (%?)]%#");
        assert_eq!(expanded, "[root@sigmaos ~/docs (127)]#");

        let rendered_starship = builder.render_prompt();
        assert!(rendered_starship.contains("root@sigmaos"));
        assert!(rendered_starship.contains("~/docs"));
    }

    #[test]
    fn test_fuzzy_completion_engine() {
        let mut engine = FuzzyCompletionEngine::new();
        engine.history.push("systemctl status nginx".to_string());
        engine
            .aliases
            .insert("ll".to_string(), "ls -la".to_string());

        // Exact matches
        let comps_sys = engine.get_completions("sys");
        assert!(comps_sys.iter().any(|c| c.text == "systemctl"));

        // Fuzzy edit distance match
        let comps_fuz = engine.get_completions("systmctl");
        assert!(comps_fuz.iter().any(|c| c.text == "systemctl"));

        // Inline ghost text
        let ghost = engine.get_ghost_suggestion("systemctl status ngin");
        assert_eq!(ghost, Some("x".to_string()));
    }

    #[test]
    fn test_zsh_syntax_highlighter() {
        let highlighter = ZshSyntaxHighlighter::new();

        let line = "ls -la $HOME | grep \"Cargo\" # comment";
        let tokens = highlighter.highlight_line(line);

        assert_eq!(tokens[0].kind, SyntaxTokenKind::CommandValid);
        assert_eq!(tokens[1].kind, SyntaxTokenKind::OptionFlag);
        assert_eq!(tokens[2].kind, SyntaxTokenKind::Variable);
        assert_eq!(tokens[3].kind, SyntaxTokenKind::Operator);
        assert_eq!(tokens[4].kind, SyntaxTokenKind::CommandValid);
        assert_eq!(tokens[5].kind, SyntaxTokenKind::StringLiteral);
        assert_eq!(tokens[6].kind, SyntaxTokenKind::Comment);

        let ansi = highlighter.render_ansi_string("ls -la");
        assert!(ansi.contains("\x1b[32mls"));
        assert!(ansi.contains("\x1b[36m-la"));
    }

    #[test]
    fn test_bash_parameter_expansions() {
        let mut env = BTreeMap::new();
        env.insert("USER".to_string(), "sovereign".to_string());
        env.insert("FILE".to_string(), "document.txt".to_string());
        env.insert("PREFIX_PATH".to_string(), "/usr/local/bin".to_string());

        // Default value
        assert_eq!(
            BashParameterExpansion::expand("${USER:-guest}", &env),
            "sovereign"
        );
        assert_eq!(
            BashParameterExpansion::expand("${EMPTY:-default}", &env),
            "default"
        );

        // String length
        assert_eq!(BashParameterExpansion::expand("${#USER}", &env), "9");

        // Prefix stripping
        assert_eq!(
            BashParameterExpansion::expand("${PREFIX_PATH#/usr/local/}", &env),
            "bin"
        );

        // Suffix stripping
        assert_eq!(
            BashParameterExpansion::expand("${FILE%.txt}", &env),
            "document"
        );

        // Global replacement
        assert_eq!(
            BashParameterExpansion::expand("${FILE//doc/file}", &env),
            "fileument.txt"
        );

        // Substring slicing
        assert_eq!(BashParameterExpansion::expand("${USER:0:5}", &env), "sover");
    }

    #[test]
    fn test_wildcard_glob_matching() {
        assert!(WildcardGlobMatcher::is_match("*.rs", "main.rs"));
        assert!(WildcardGlobMatcher::is_match("src/*.rs", "src/lib.rs"));
        assert!(WildcardGlobMatcher::is_match("file_?.txt", "file_1.txt"));
        assert!(!WildcardGlobMatcher::is_match("file_?.txt", "file_10.txt"));

        let files = ["main.rs", "test.py", "lib.rs", "Cargo.toml"];
        let matched = WildcardGlobMatcher::filter_matches("*.rs", &files);
        assert_eq!(matched, vec!["main.rs", "lib.rs"]);
    }

    #[test]
    fn test_pipeline_parser_and_redirections() {
        let pipeline = ShellPipelineParser::parse(
            "cat < input.txt | grep -i \"error\" > output.txt 2> error.log &",
        );

        assert!(pipeline.run_in_background);
        assert_eq!(pipeline.stages.len(), 2);

        let stage1 = &pipeline.stages[0];
        assert_eq!(stage1.program, "cat");
        assert_eq!(stage1.stdin_file, Some("input.txt".to_string()));

        let stage2 = &pipeline.stages[1];
        assert_eq!(stage2.program, "grep");
        assert_eq!(stage2.args, vec!["-i", "\"error\""]);
        assert_eq!(stage2.stdout_file, Some("output.txt".to_string()));
        assert_eq!(stage2.stderr_file, Some("error.log".to_string()));
    }

    #[test]
    fn test_here_string_parsing() {
        let pipeline = ShellPipelineParser::parse("grep hello <<< \"hello_world\"");
        assert_eq!(pipeline.stages[0].program, "grep");
        assert_eq!(
            pipeline.stages[0].here_string,
            Some("hello_world".to_string())
        );
    }

    #[test]
    fn test_bsd_directory_stack() {
        let mut dstack = BsdDirectoryStack::new("/home/user");
        assert_eq!(dstack.dirs(), "/home/user");

        dstack.pushd("/etc");
        assert_eq!(dstack.dirs(), "/etc /home/user");

        dstack.pushd("/var/log");
        assert_eq!(dstack.dirs(), "/var/log /etc /home/user");

        assert_eq!(dstack.popd().unwrap(), "/etc /home/user");
        assert_eq!(dstack.current_dir, "/etc");
    }

    #[test]
    fn test_shell_job_control() {
        let mut jc = ShellJobControl::new();

        let id1 = jc.add_job(101, "sleep 100");
        let id2 = jc.add_job(102, "cargo build &");

        assert_eq!(jc.jobs.len(), 2);

        jc.stop_job(id1);
        assert_eq!(jc.jobs[0].state, JobState::Stopped);

        let listing = jc.list_jobs();
        assert!(listing.contains("sleep 100"));
        assert!(listing.contains("Stopped"));

        assert!(jc.remove_job(id2));
        assert_eq!(jc.jobs.len(), 1);
    }

    #[test]
    fn test_shell_script_hooks() {
        let mut hooks = ShellScriptHookEngine::new();
        hooks.add_precmd_hook("update_status_line");
        hooks.add_preexec_hook("log_cmd");
        hooks.add_chpwd_hook("auto_ls");

        assert_eq!(hooks.trigger_precmd(), vec!["update_status_line"]);
        assert_eq!(hooks.trigger_preexec("ls -l"), vec!["log_cmd ls -l"]);
        assert_eq!(
            hooks.trigger_chpwd("/home", "/etc"),
            vec!["auto_ls /home /etc"]
        );
    }

    #[test]
    fn test_arithmetic_evaluator() {
        assert_eq!(ShellArithmeticEvaluator::evaluate("$(( 10 + 20 ))"), Ok(30));
        assert_eq!(ShellArithmeticEvaluator::evaluate("$(( 50 - 15 ))"), Ok(35));
        assert_eq!(ShellArithmeticEvaluator::evaluate("$(( 6 * 7 ))"), Ok(42));
        assert_eq!(ShellArithmeticEvaluator::evaluate("$(( 100 / 5 ))"), Ok(20));
        assert_eq!(
            ShellArithmeticEvaluator::evaluate("$(( 100 / 0 ))"),
            Err("Division by zero")
        );
    }

    #[test]
    fn test_fish_abbreviation_engine() {
        let mut abbr_engine = FishAbbreviationEngine::new();
        abbr_engine.add_abbreviation("g", "git");
        abbr_engine.add_abbreviation("gc", "git commit -m");

        assert_eq!(
            abbr_engine.expand_line("g checkout main"),
            "git checkout main"
        );
        assert_eq!(
            abbr_engine.expand_line("gc 'initial commit'"),
            "git commit -m 'initial commit'"
        );
        assert_eq!(abbr_engine.expand_line("ls -la"), "ls -la");
    }

    #[test]
    fn test_tcsh_history_substitution() {
        let mut tcsh_hist = TcshHistorySubstitutionEngine::new();
        tcsh_hist.push_history("cargo build --release");

        assert_eq!(tcsh_hist.substitute("!!").unwrap(), "cargo build --release");
        assert_eq!(tcsh_hist.substitute("!$").unwrap(), "--release");
        assert_eq!(tcsh_hist.substitute("!*").unwrap(), "build --release");
        assert_eq!(
            tcsh_hist.substitute("!car").unwrap(),
            "cargo build --release"
        );
    }

    #[test]
    fn test_ksh_parameter_expansion() {
        let env = vec![
            ("USER".to_string(), "sigma".to_string()),
            ("EMPTY".to_string(), "".to_string()),
            ("PATH".to_string(), "/usr/bin:/bin".to_string()),
        ];

        assert_eq!(
            KshParameterExpansionEngine::expand_parameter("echo ${EMPTY:-fallback}", &env),
            "echo fallback"
        );
        assert_eq!(
            KshParameterExpansionEngine::expand_parameter("echo ${USER:-fallback}", &env),
            "echo sigma"
        );
        assert_eq!(
            KshParameterExpansionEngine::expand_parameter("len=${#USER}", &env),
            "len=5"
        );
        assert_eq!(
            KshParameterExpansionEngine::expand_parameter("echo ${PATH//bin/sbin}", &env),
            "echo /usr/sbin:/sbin"
        );
    }

    #[test]
    fn test_dash_posix_sh_validator() {
        let non_posix_script = "#!/bin/sh\n[[ -f /etc/sigma ]] && echo ok\necho <<< 'hello'\nfunction foo() {\n  arr=(1 2 3)\n}\n";
        let warnings = DashPosixShValidator::validate_posix_compliance(non_posix_script);
        assert_eq!(warnings.len(), 4);
        assert_eq!(warnings[0].0, 2);
        assert_eq!(warnings[1].0, 3);
        assert_eq!(warnings[2].0, 4);
        assert_eq!(warnings[3].0, 5);
    }

    #[test]
    fn test_universal_shell_compatibility_engine() {
        let mut engine = UniversalShellCompatibilityEngine::new();
        engine.fish_abbr.add_abbreviation("co", "checkout");
        engine
            .environment
            .push(("NAME".to_string(), "SigmaOS".to_string()));

        let bash_script = "#!/bin/bash\necho hello";
        assert_eq!(
            UniversalShellCompatibilityEngine::detect_shebang_dialect(bash_script),
            ShellDialect::Bash
        );

        let fish_script = "#!/usr/bin/env fish\nco main";
        assert_eq!(
            UniversalShellCompatibilityEngine::detect_shebang_dialect(fish_script),
            ShellDialect::Fish
        );

        let pipeline = engine.process_input_line("co ${NAME:-default}").unwrap();
        assert_eq!(pipeline.stages[0].program, "checkout");
        assert_eq!(pipeline.stages[0].args, vec!["SigmaOS".to_string()]);
    }

    #[test]
    fn test_universal_script_transpiler_and_sh_execution() {
        let fish_script = "#!/usr/bin/env fish\nset -gx TARGET /usr/bin\nfunction build_all\n  echo building\nend\nand echo done";
        let posix_fish = UniversalScriptTranspiler::transpile_to_posix_sh(fish_script, ShellDialect::Fish);
        assert!(posix_fish.contains("#!/bin/sh"));
        assert!(posix_fish.contains("export TARGET=/usr/bin"));
        assert!(posix_fish.contains("build_all() {"));
        assert!(posix_fish.contains("}"));
        assert!(posix_fish.contains("&& echo done"));

        let tcsh_script = "#!/bin/tcsh\nsetenv PORT 8080\nalias ll ls -la";
        let posix_tcsh = UniversalScriptTranspiler::transpile_to_posix_sh(tcsh_script, ShellDialect::Tcsh);
        assert!(posix_tcsh.contains("export PORT=8080"));
        assert!(posix_tcsh.contains("alias ll=ls -la"));

        let bash_script = "#!/bin/bash\ngrep test <<< \"test_string\"\n[[ -f /tmp/foo ]]";
        let posix_bash = UniversalScriptTranspiler::transpile_to_posix_sh(bash_script, ShellDialect::Bash);
        assert!(posix_bash.contains("echo \"test_string\" | grep test"));
        assert!(posix_bash.contains("[ -f /tmp/foo ]"));

        let mut engine = UniversalShellCompatibilityEngine::new();
        let pipelines = engine.execute_script_as_sh(tcsh_script).unwrap();
        assert!(!pipelines.is_empty());
    }
}
