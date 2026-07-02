// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/agent/sigma_agent_planner.rs — Agentic Planner + ReAct Loop
// Inspiration: Claude Code, Aider, openclaw, ai-shell
// Language: Rust (std) — OOP via Planner + Plan structs

use std::collections::BTreeMap;
use super::sigma_llm::{LlmBackend, Message, GenParams, AutoBackend};
use super::sigma_agent::{Tool, ToolResult};

// ── Plan Step ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum StepKind {
    Thought(String),       // internal reasoning
    Action { tool: String, args: BTreeMap<String,String> },
    Observation(String),   // tool result
    Answer(String),        // final answer to user
}

#[derive(Debug, Clone)]
pub struct PlanStep {
    pub kind:    StepKind,
    pub success: bool,
}

// ── Plan ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct Plan {
    pub steps:  Vec<PlanStep>,
    pub answer: Option<String>,
    pub done:   bool,
}

impl Plan {
    pub fn thoughts(&self) -> Vec<&str> {
        self.steps.iter().filter_map(|s| if let StepKind::Thought(t) = &s.kind { Some(t.as_str()) } else { None }).collect()
    }
    pub fn actions_taken(&self) -> usize {
        self.steps.iter().filter(|s| matches!(s.kind, StepKind::Action{..})).count()
    }
}

// ── System Prompt ─────────────────────────────────────────────────────────────

const SYSTEM_PROMPT: &str = r#"You are sigma-agent, the AI CLI assistant for SigmaOS.
You help users control their operating system using natural language.

You have access to these tools:
- read_file(path): Read file content
- write_file(path, content): Write to a file
- list_dir(path): List directory
- shell(command): Run a shell command
- install_package(name): Install a package
- open_app(app, args): Launch an application
- settings(action, panel, key, value): Get/set OS settings
- system_info(): System overview
- network(action, iface, ssid, pass): Network management
- process(action, pid, name): Process management
- explain(topic): Explain a concept
- code_edit(file, instruction): AI code editing
- summarise(path): Summarise file with AI
- wm_control(action, workspace, layout): Window manager
- notify(title, body, urgency): Desktop notification
- clipboard(action, content): Clipboard operations
- find_files(query, path): Search files
- accessibility(feature, state): Accessibility settings
- vpn(action, profile): VPN management
- disk(action, path): Disk management

Respond in this format:
Thought: [your reasoning]
Action: tool_name
Args: key=value, key=value
---
After getting the tool result, continue until you have a final answer.
Final Answer: [your complete answer to the user]

Always be concise. Never ask for confirmation unless the action is destructive.
For file edits, show a diff-style preview before applying.
"#;

// ── ReAct Loop (Reason + Act) ─────────────────────────────────────────────────
// Inspired by: anthropics/claude-code, Aider-AI/aider

pub struct Planner {
    pub backend:     AutoBackend,
    pub max_steps:   usize,
    pub verbose:     bool,
    pub dry_run:     bool,
}

impl Planner {
    pub fn new() -> Self {
        Self {
            backend:   AutoBackend::new(),
            max_steps: 10,
            verbose:   false,
            dry_run:   false,
        }
    }

    /// Execute a user goal using the ReAct (Reason+Act) pattern
    pub fn execute(&self, goal: &str, tools: &BTreeMap<String, Box<dyn Tool>>) -> Plan {
        let mut plan = Plan::default();
        let mut messages = vec![
            Message::system(SYSTEM_PROMPT),
            Message::user(goal),
        ];

        for step_num in 0..self.max_steps {
            // Generate next thought + action from LLM
            let params = GenParams::precise();
            let response = match self.backend.generate(&messages, &params) {
                Ok(r)  => r,
                Err(e) => {
                    // Fallback: use rule-based planner
                    let ans = self.rule_based_plan(goal, tools);
                    plan.steps.push(PlanStep { kind: StepKind::Answer(ans.clone()), success: true });
                    plan.answer = Some(ans);
                    plan.done   = true;
                    return plan;
                }
            };

            if self.verbose { eprintln!("[planner step {}]\n{}", step_num, response); }

            // Parse the response
            if let Some(answer) = Self::extract_final_answer(&response) {
                plan.steps.push(PlanStep { kind: StepKind::Answer(answer.clone()), success: true });
                plan.answer = Some(answer);
                plan.done   = true;
                return plan;
            }

            let (thought, tool_name, args) = Self::parse_react_response(&response);

            if let Some(t) = thought {
                plan.steps.push(PlanStep { kind: StepKind::Thought(t.clone()), success: true });
                messages.push(Message::assistant(&format!("Thought: {}", t)));
            }

            if let Some(tool_name) = tool_name {
                // Execute tool
                let result = if let Some(tool) = tools.get(&tool_name) {
                    if self.dry_run && Self::is_destructive(&tool_name) {
                        ToolResult { success: true, output: format!("[dry-run] Would call: {} {:?}", tool_name, args), error: None, next: None }
                    } else {
                        tool.execute(&args)
                    }
                } else {
                    ToolResult::err(format!("Tool not found: {}", tool_name))
                };

                let obs = if result.success { result.output.clone() } else { format!("Error: {}", result.error.unwrap_or_default()) };
                plan.steps.push(PlanStep { kind: StepKind::Action { tool: tool_name.clone(), args: args.clone() }, success: result.success });
                plan.steps.push(PlanStep { kind: StepKind::Observation(obs.clone()), success: result.success });

                // Add observation to message history
                messages.push(Message::assistant(&format!("Action: {}\nArgs: {}", tool_name,
                    args.iter().map(|(k,v)| format!("{}={}", k, v)).collect::<Vec<_>>().join(", "))));
                messages.push(Message { role: super::sigma_llm::MessageRole::Tool, content: format!("Result: {}", obs) });

                // Follow up hint
                if let Some(next) = result.next {
                    messages.push(Message { role: super::sigma_llm::MessageRole::Tool,
                        content: format!("Suggestion: {}", next) });
                }
            } else {
                // No action parsed → treat whole response as final answer
                plan.steps.push(PlanStep { kind: StepKind::Answer(response.clone()), success: true });
                plan.answer = Some(response);
                plan.done   = true;
                return plan;
            }
        }

        // Max steps reached
        plan.answer = Some(format!("Reached max steps ({}). Last observations:\n{}",
            self.max_steps,
            plan.steps.iter().rev()
                .filter_map(|s| if let StepKind::Observation(o) = &s.kind { Some(o.as_str()) } else { None })
                .take(3).collect::<Vec<_>>().join("\n")));
        plan.done = true;
        plan
    }

    fn parse_react_response(text: &str) -> (Option<String>, Option<String>, BTreeMap<String,String>) {
        let mut thought = None;
        let mut tool    = None;
        let mut args    = BTreeMap::new();

        for line in text.lines() {
            let line = line.trim();
            if let Some(t) = line.strip_prefix("Thought:") { thought = Some(t.trim().to_owned()); }
            else if let Some(a) = line.strip_prefix("Action:") { tool = Some(a.trim().to_owned()); }
            else if let Some(a) = line.strip_prefix("Args:") {
                for pair in a.split(',') {
                    let pair = pair.trim();
                    if let Some(eq) = pair.find('=') {
                        let k = pair[..eq].trim().to_owned();
                        let v = pair[eq+1..].trim().trim_matches('"').to_owned();
                        args.insert(k, v);
                    }
                }
            }
        }
        (thought, tool, args)
    }

    fn extract_final_answer(text: &str) -> Option<String> {
        for line in text.lines() {
            if let Some(ans) = line.strip_prefix("Final Answer:") {
                return Some(ans.trim().to_owned());
            }
        }
        if text.contains("---") {
            // Everything after --- is the answer
            if let Some(pos) = text.rfind("---") {
                let after = text[pos+3..].trim();
                if !after.is_empty() { return Some(after.to_owned()); }
            }
        }
        None
    }

    fn is_destructive(tool: &str) -> bool {
        matches!(tool, "write_file" | "shell" | "install_package" | "process")
    }

    /// Rule-based planner fallback when LLM is offline
    /// Maps natural language goals directly to tool calls
    fn rule_based_plan(&self, goal: &str, tools: &BTreeMap<String, Box<dyn Tool>>) -> String {
        let lower = goal.to_ascii_lowercase();
        let mut args = BTreeMap::new();

        if lower.starts_with("install ") {
            args.insert("name".to_owned(), goal[8..].trim().to_owned());
            if let Some(t) = tools.get("install_package") { return t.execute(&args).output; }
        }
        if lower.contains("system info") || lower.contains("sysinfo") {
            if let Some(t) = tools.get("system_info") { return t.execute(&args).output; }
        }
        if lower.starts_with("list ") || lower.starts_with("ls ") {
            let path = goal.splitn(2,' ').nth(1).unwrap_or(".").to_owned();
            args.insert("path".to_owned(), path);
            if let Some(t) = tools.get("list_dir") { return t.execute(&args).output; }
        }
        if lower.starts_with("read ") || lower.starts_with("cat ") {
            let path = goal.splitn(2,' ').nth(1).unwrap_or("").to_owned();
            args.insert("path".to_owned(), path);
            if let Some(t) = tools.get("read_file") { return t.execute(&args).output; }
        }
        // Fallback: run as shell command
        args.insert("command".to_owned(), goal.to_owned());
        if let Some(t) = tools.get("shell") { return t.execute(&args).output; }
        format!("I would: {}", goal)
    }
}

// ── Command Suggestion Engine ──────────────────────────────────────────────────
// Inspired by: BuilderIO/ai-shell, github/copilot-cli

pub struct CommandSuggestor {
    backend: AutoBackend,
}

impl CommandSuggestor {
    pub fn new() -> Self { Self { backend: AutoBackend::new() } }

    /// Given a natural language description, suggest the sigma-* CLI command
    pub fn suggest(&self, description: &str) -> Vec<String> {
        let messages = vec![
            Message::system("You are a SigmaOS CLI expert. Given a task description, output 1-3 sigma-* commands that accomplish it. Output only the commands, one per line, no explanation."),
            Message::user(description),
        ];
        let params = GenParams { max_tokens: 128, temperature: 0.3, ..GenParams::default() };
        match self.backend.generate(&messages, &params) {
            Ok(r) => r.lines().filter(|l| !l.trim().is_empty()).map(|l| l.trim().to_owned()).collect(),
            Err(_) => self.rule_based_suggest(description),
        }
    }

    fn rule_based_suggest(&self, desc: &str) -> Vec<String> {
        let lower = desc.to_ascii_lowercase();
        if lower.contains("install")  { return vec![format!("sigma-pkg install <package-name>")]; }
        if lower.contains("theme")    { return vec!["sigma-agent \"set dark mode\"".to_owned()]; }
        if lower.contains("network")  { return vec!["sigma-netctl list".to_owned(), "sigma-netctl status".to_owned()]; }
        if lower.contains("process")  { return vec!["sigma-agent \"show processes\"".to_owned()]; }
        if lower.contains("disk")     { return vec!["sigma-disks list".to_owned()]; }
        if lower.contains("vpn")      { return vec!["sigma-vpn list".to_owned(), "sigma-vpn connect <profile>".to_owned()]; }
        vec![format!("sigma-agent \"{}\"", desc)]
    }

    /// Explain what a command does
    pub fn explain_command(&self, cmd: &str) -> String {
        let messages = vec![
            Message::system("You are a SigmaOS CLI expert. Briefly explain what the given command does in 1-2 sentences."),
            Message::user(cmd),
        ];
        let params = GenParams { max_tokens: 128, temperature: 0.2, ..GenParams::default() };
        self.backend.generate(&messages, &params)
            .unwrap_or_else(|_| format!("Runs the command: {}", cmd))
    }

    /// Fix a failing command
    pub fn fix_command(&self, cmd: &str, error: &str) -> Option<String> {
        let messages = vec![
            Message::system("You are a SigmaOS CLI expert. A command failed. Suggest the corrected command only, no explanation."),
            Message::user(&format!("Command: {}\nError: {}", cmd, error)),
        ];
        let params = GenParams { max_tokens: 64, temperature: 0.1, ..GenParams::default() };
        self.backend.generate(&messages, &params).ok()
            .map(|s| s.trim().to_owned())
    }
}
