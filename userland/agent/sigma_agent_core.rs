// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/agent/sigma_agent_core.rs — Agent core: intent parsing, agentic loop, REPL
// Language: Rust (std) — OOP via Agent struct

use std::collections::BTreeMap;
use std::io::{self, BufRead, Write};
use super::sigma_agent::{Tool, ToolResult};

// ── Intent ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Intent {
    pub tool_name: String,
    pub args:      BTreeMap<String, String>,
    pub raw:       String,
}

// ── Simple NL → Intent parser ─────────────────────────────────────────────────

pub struct IntentParser {
    // keyword → (tool_name, field_mappings)
    patterns: Vec<(Vec<&'static str>, &'static str, Vec<(&'static str, Option<usize>)>)>,
}

impl IntentParser {
    pub fn new() -> Self {
        // pattern: (keywords, tool_name, [(arg_key, word_index)])
        // word_index = nth word after the keyword phrase; None = rest of line
        Self { patterns: vec![
            (vec!["read","show","cat","open file","view file"], "read_file",
             vec![("path", Some(1))]),
            (vec!["write","save","create file"], "write_file",
             vec![("path", Some(1)), ("content", None)]),
            (vec!["list","ls","dir","what files","show files"], "list_dir",
             vec![("path", Some(1))]),
            (vec!["install","add package","pkg install"], "install_package",
             vec![("name", Some(1))]),
            (vec!["open app","launch","start app","run app"], "open_app",
             vec![("app", Some(1)), ("args", None)]),
            (vec!["system info","sysinfo","neofetch","about this system","what's running"], "system_info",
             vec![]),
            (vec!["set theme","change theme","dark mode","light mode"], "settings",
             vec![("action",  Some(0)), // will be overridden
                  ("panel",   Some(0)),
                  ("key",     Some(0)),
                  ("value",   Some(0))]),
            (vec!["network","wifi","internet","connect wifi","network status"], "network",
             vec![("action", Some(1))]),
            (vec!["processes","running apps","kill process","ps","list processes"], "process",
             vec![("action", Some(1))]),
            (vec!["run","exec","execute","bash","shell"], "shell",
             vec![("command", None)]),
        ]}
    }

    pub fn parse(&self, input: &str) -> Option<Intent> {
        let lower = input.trim().to_ascii_lowercase();
        let words: Vec<&str> = input.trim().split_whitespace().collect();

        // Special cases for natural language settings commands
        if lower.contains("theme") {
            let mut args = BTreeMap::new();
            args.insert("action".to_owned(), "set".to_owned());
            args.insert("panel".to_owned(),  "appearance".to_owned());
            args.insert("key".to_owned(),    "theme".to_owned());
            let value = if lower.contains("dark") { "zenith-dark" }
                        else if lower.contains("light") { "zenith-light" }
                        else if lower.contains("high contrast") { "high-contrast" }
                        else { "zenith-dark" };
            args.insert("value".to_owned(), value.to_owned());
            return Some(Intent { tool_name: "settings".to_owned(), args, raw: input.to_owned() });
        }

        if lower.starts_with("install ") {
            let pkg = input[8..].trim().to_owned();
            let mut args = BTreeMap::new(); args.insert("name".to_owned(), pkg);
            return Some(Intent { tool_name: "install_package".to_owned(), args, raw: input.to_owned() });
        }

        for (keywords, tool, field_map) in &self.patterns {
            for kw in keywords {
                if lower.contains(kw) {
                    let mut args = BTreeMap::new();
                    // Extract position-based args
                    let kw_words: usize = kw.split_whitespace().count();
                    let rest_words: Vec<&str> = words.iter().skip(kw_words).copied().collect();
                    for (field, idx) in field_map {
                        if let Some(i) = idx {
                            if *i < rest_words.len() {
                                args.insert(field.to_string(), rest_words[*i].to_owned());
                            }
                        } else {
                            // None means "rest of line"
                            if !rest_words.is_empty() {
                                args.insert(field.to_string(), rest_words.join(" "));
                            }
                        }
                    }
                    return Some(Intent { tool_name: tool.to_string(), args, raw: input.to_owned() });
                }
            }
        }

        // Fallback: treat as shell command
        if !input.trim().is_empty() {
            let mut args = BTreeMap::new();
            args.insert("command".to_owned(), input.trim().to_owned());
            return Some(Intent { tool_name: "shell".to_owned(), args, raw: input.to_owned() });
        }
        None
    }
}

// ── Conversation History ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Turn {
    pub role:    Role,
    pub content: String,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role { User, Assistant, Tool }

pub struct ConversationHistory {
    turns:    Vec<Turn>,
    max_len:  usize,
}

impl ConversationHistory {
    pub fn new(max_len: usize) -> Self { Self { turns: Vec::new(), max_len } }
    pub fn push(&mut self, role: Role, content: &str) {
        self.turns.push(Turn { role, content: content.to_owned() });
        if self.turns.len() > self.max_len { self.turns.remove(0); }
    }
    pub fn last_user(&self)      -> Option<&str> {
        self.turns.iter().rev().find(|t| t.role == Role::User).map(|t| t.content.as_str())
    }
    pub fn summary(&self, last_n: usize) -> String {
        self.turns.iter().rev().take(last_n).rev()
            .map(|t| format!("[{:?}] {}", t.role, &t.content[..t.content.len().min(80)]))
            .collect::<Vec<_>>().join("\n")
    }
}

// ── Agent ─────────────────────────────────────────────────────────────────────

pub struct Agent {
    tools:   BTreeMap<String, Box<dyn Tool>>,
    aliases: BTreeMap<String, String>,
    parser:  IntentParser,
    history: ConversationHistory,
    verbose: bool,
    model:   AgentModel,
}

#[derive(Clone, Copy)]
pub enum AgentModel { Local, Remote }  // local = sigma-ai, remote = API

impl Agent {
    pub fn new() -> Self {
        let mut a = Self {
            tools:   BTreeMap::new(),
            aliases: BTreeMap::new(),
            parser:  IntentParser::new(),
            history: ConversationHistory::new(50),
            verbose: false,
            model:   AgentModel::Local,
        };
        a.register_defaults();
        a
    }

    pub fn set_verbose(&mut self, v: bool)     { self.verbose = v; }
    pub fn set_model(&mut self, m: AgentModel) { self.model = m; }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        let name = tool.name().to_owned();
        for alias in tool.aliases() {
            self.aliases.insert(alias.to_string(), name.clone());
        }
        self.tools.insert(name, tool);
    }

    fn register_defaults(&mut self) {
        use super::sigma_agent::*;
        self.register(Box::new(ReadFileTool));
        self.register(Box::new(WriteFileTool));
        self.register(Box::new(ListDirTool));
        self.register(Box::new(ShellTool));
        self.register(Box::new(InstallPkgTool));
        self.register(Box::new(OpenAppTool));
        self.register(Box::new(SettingsTool));
        self.register(Box::new(SystemInfoTool));
        self.register(Box::new(NetworkTool));
        self.register(Box::new(ProcessTool));
    }

    fn resolve_tool(&self, name: &str) -> Option<&dyn Tool> {
        let canonical = self.aliases.get(name).map(|s| s.as_str()).unwrap_or(name);
        self.tools.get(canonical).map(|t| t.as_ref())
    }

    /// Process a single user message and return the agent response
    pub fn process(&mut self, input: &str) -> String {
        self.history.push(Role::User, input);

        // Built-in meta-commands
        match input.trim().to_ascii_lowercase().as_str() {
            "help" | "?" | "/help"    => return self.help_text(),
            "tools" | "/tools"        => return self.list_tools(),
            "history" | "/history"    => return self.history.summary(10),
            "clear" | "/clear"        => { self.history = ConversationHistory::new(50); return "History cleared.".to_owned(); }
            "quit" | "exit" | "/quit" => std::process::exit(0),
            _ => {}
        }

        // Parse intent
        let intent = match self.parser.parse(input) {
            Some(i) => i,
            None    => { let r = "I'm not sure what you'd like to do. Type 'help' for examples.".to_owned(); self.history.push(Role::Assistant, &r); return r; }
        };

        if self.verbose { eprintln!("[agent] intent: {} {:?}", intent.tool_name, intent.args); }

        // Execute tool
        let tool = match self.resolve_tool(&intent.tool_name) {
            Some(t) => t,
            None => {
                let r = format!("Unknown tool: {}. Type 'tools' to see available tools.", intent.tool_name);
                self.history.push(Role::Assistant, &r); return r;
            }
        };

        let result = tool.execute(&intent.args);
        self.history.push(Role::Tool, &format!("{}: {}", intent.tool_name, if result.success { "ok" } else { "err" }));

        let response = if result.success {
            let mut r = result.output;
            if let Some(next) = result.next { r = format!("{}\n\nSuggestion: {}", r, next); }
            r
        } else {
            format!("Error: {}", result.error.unwrap_or_else(|| "Unknown error".to_owned()))
        };

        self.history.push(Role::Assistant, &response);
        response
    }

    fn help_text(&self) -> String {
        let examples = vec![
            ("Files & Filesystem",   vec!["read /home/user/notes.md", "list /usr/bin", "write /tmp/hello.txt content: Hello world"]),
            ("Apps & Packages",      vec!["install sigma-edit", "open app sigma-terminal", "launch sigma-files"]),
            ("System",               vec!["system info", "what's running", "kill process 1234"]),
            ("Settings / Themes",    vec!["set dark mode", "change theme to zenith-light", "settings get appearance theme"]),
            ("Network",              vec!["network status", "connect wifi MyNetwork secret123", "list network interfaces"]),
            ("Shell Commands",       vec!["run ls -la /home", "exec sigma-pkg list", "bash echo hello"]),
        ];
        let mut lines = vec!["Σ sigma-agent — AI CLI Agent for SigmaOS\n".to_owned()];
        lines.push("Natural language commands:\n".to_owned());
        for (section, cmds) in &examples {
            lines.push(format!("  {}:", section));
            for cmd in cmds { lines.push(format!("    > {}", cmd)); }
            lines.push(String::new());
        }
        lines.push("Special commands: help | tools | history | clear | quit".to_owned());
        lines.join("\n")
    }

    fn list_tools(&self) -> String {
        let mut lines = vec!["Available tools:\n".to_owned()];
        for (name, tool) in &self.tools {
            let aliases = tool.aliases().join(", ");
            lines.push(format!("  {:20} — {}", name, tool.description()));
            if !aliases.is_empty() { lines.push(format!("    aliases: {}", aliases)); }
        }
        lines.join("\n")
    }

    /// Run the interactive REPL
    pub fn run_repl(&mut self) {
        let stdin = io::stdin();
        println!("{}", self.help_text());
        println!("{}", "─".repeat(60));
        loop {
            print!("σ> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            match stdin.lock().read_line(&mut line) {
                Ok(0) | Err(_) => break,
                Ok(_) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() { continue; }
                    let response = self.process(trimmed);
                    println!("{}\n", response);
                }
            }
        }
    }

    /// Run a single non-interactive command
    pub fn run_once(&mut self, input: &str) {
        let result = self.process(input);
        println!("{}", result);
    }

    /// Execute a script file line by line
    pub fn run_script(&mut self, path: &str) {
        match std::fs::read_to_string(path) {
            Ok(content) => {
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') { continue; }
                    println!("σ> {}", line);
                    let result = self.process(line);
                    println!("{}\n", result);
                }
            }
            Err(e) => eprintln!("Cannot read script {}: {}", path, e),
        }
    }
}
