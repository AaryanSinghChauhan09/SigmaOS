// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/agent/sigma_llm.rs — Local LLM inference via llama.cpp-style runtime
// Inspiration: ggml-org/llama.cpp, BuilderIO/ai-shell, j178/chatgpt
// Language: Rust (std) — OOP via LlmBackend trait + providers

use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};

// ── Message Types ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Message {
    pub role:    MessageRole,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MessageRole { System, User, Assistant, Tool }

impl Message {
    pub fn system(content: &str)    -> Self { Self { role: MessageRole::System,    content: content.to_owned() } }
    pub fn user(content: &str)      -> Self { Self { role: MessageRole::User,      content: content.to_owned() } }
    pub fn assistant(content: &str) -> Self { Self { role: MessageRole::Assistant, content: content.to_owned() } }
}

// ── Generation Parameters ─────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct GenParams {
    pub max_tokens:   u32,
    pub temperature:  f32,
    pub top_p:        f32,
    pub top_k:        u32,
    pub repeat_penalty: f32,
    pub stream:       bool,
    pub stop:         Vec<String>,
}

impl Default for GenParams {
    fn default() -> Self {
        Self {
            max_tokens:   512,
            temperature:  0.7,
            top_p:        0.9,
            top_k:        40,
            repeat_penalty: 1.1,
            stream:       true,
            stop:         vec!["</s>".to_owned(), "Human:".to_owned(), "User:".to_owned()],
        }
    }
}

impl GenParams {
    pub fn creative() -> Self { Self { temperature: 0.9, top_p: 0.95, ..Self::default() } }
    pub fn precise()  -> Self { Self { temperature: 0.2, top_p: 0.7,  ..Self::default() } }
    pub fn code()     -> Self { Self { temperature: 0.1, stop: vec!["```".to_owned()], ..Self::default() } }
}

// ── LLM Backend Trait (OOP) ───────────────────────────────────────────────────

pub trait LlmBackend: Send + Sync {
    fn name(&self)       -> &'static str;
    fn is_available(&self) -> bool;
    fn generate(&self, messages: &[Message], params: &GenParams) -> Result<String, LlmError>;
    fn generate_stream(&self, messages: &[Message], params: &GenParams, on_token: &mut dyn FnMut(&str)) -> Result<(), LlmError>;
}

#[derive(Debug)]
pub enum LlmError {
    NotAvailable,
    Timeout,
    ParseError(String),
    BackendError(String),
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotAvailable     => write!(f, "LLM backend not available"),
            Self::Timeout          => write!(f, "LLM request timed out"),
            Self::ParseError(e)    => write!(f, "Parse error: {}", e),
            Self::BackendError(e)  => write!(f, "Backend error: {}", e),
        }
    }
}

// ── llama.cpp Backend (local, sigma-ai via llama-cli) ─────────────────────────
// Inspired by: ggml-org/llama.cpp

pub struct LlamaCppBackend {
    pub model_path:  String,
    pub cli_path:    String,   // path to llama-cli binary
    pub n_ctx:       u32,
    pub n_threads:   u32,
    pub gpu_layers:  i32,      // -1 = all on GPU, 0 = CPU only
}

impl LlamaCppBackend {
    pub fn new(model_path: &str) -> Self {
        Self {
            model_path:  model_path.to_owned(),
            cli_path:    which_llama_cli(),
            n_ctx:       4096,
            n_threads:   num_cpus(),
            gpu_layers:  0,
        }
    }

    pub fn auto() -> Option<Self> {
        // Auto-detect model from known locations
        let candidates = [
            "/usr/share/sigma/models/tinyllama-1.1b-chat-q4_0.gguf",
            "/var/cache/sigma/models/tinyllama.gguf",
            &format!("{}/.cache/sigma/models/tinyllama.gguf", home_dir()),
        ];
        for path in &candidates {
            if std::path::Path::new(path).exists() {
                return Some(Self::new(path));
            }
        }
        None
    }

    fn format_prompt(&self, messages: &[Message]) -> String {
        // ChatML format (compatible with TinyLlama + most fine-tunes)
        let mut prompt = String::new();
        for msg in messages {
            match msg.role {
                MessageRole::System    => prompt.push_str(&format!("<|system|>\n{}\n", msg.content)),
                MessageRole::User      => prompt.push_str(&format!("<|user|>\n{}\n", msg.content)),
                MessageRole::Assistant => prompt.push_str(&format!("<|assistant|>\n{}\n", msg.content)),
                MessageRole::Tool      => prompt.push_str(&format!("<|tool|>\n{}\n", msg.content)),
            }
        }
        prompt.push_str("<|assistant|>\n");
        prompt
    }
}

impl LlmBackend for LlamaCppBackend {
    fn name(&self) -> &'static str { "llama.cpp (local)" }

    fn is_available(&self) -> bool {
        !self.cli_path.is_empty()
            && std::path::Path::new(&self.model_path).exists()
    }

    fn generate(&self, messages: &[Message], params: &GenParams) -> Result<String, LlmError> {
        if !self.is_available() { return Err(LlmError::NotAvailable); }
        let prompt = self.format_prompt(messages);
        let out = Command::new(&self.cli_path)
            .args([
                "-m", &self.model_path,
                "-p", &prompt,
                "--n-predict", &params.max_tokens.to_string(),
                "--temp", &params.temperature.to_string(),
                "--top-p", &params.top_p.to_string(),
                "--top-k", &params.top_k.to_string(),
                "--repeat-penalty", &params.repeat_penalty.to_string(),
                "--threads", &self.n_threads.to_string(),
                "--ctx-size", &self.n_ctx.to_string(),
                "--no-display-prompt",
                "--log-disable",
            ])
            .output()
            .map_err(|e| LlmError::BackendError(e.to_string()))?;
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).trim().to_owned())
        } else {
            Err(LlmError::BackendError(String::from_utf8_lossy(&out.stderr).to_string()))
        }
    }

    fn generate_stream(&self, messages: &[Message], params: &GenParams, on_token: &mut dyn FnMut(&str)) -> Result<(), LlmError> {
        if !self.is_available() { return Err(LlmError::NotAvailable); }
        let prompt = self.format_prompt(messages);
        let mut child = Command::new(&self.cli_path)
            .args([
                "-m", &self.model_path,
                "-p", &prompt,
                "--n-predict", &params.max_tokens.to_string(),
                "--temp", &params.temperature.to_string(),
                "--threads", &self.n_threads.to_string(),
                "--no-display-prompt",
                "--log-disable",
            ])
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| LlmError::BackendError(e.to_string()))?;

        if let Some(stdout) = child.stdout.take() {
            let reader = BufReader::new(stdout);
            for line in reader.lines().flatten() {
                on_token(&line);
                on_token("\n");
            }
        }
        let _ = child.wait();
        Ok(())
    }
}

// ── sigma-ai Daemon Backend (IPC socket) ──────────────────────────────────────
// Connects to the running sigma-ai daemon via Unix socket

pub struct SigmaAiBackend {
    socket_path: String,
}

impl SigmaAiBackend {
    pub fn new() -> Self {
        Self { socket_path: "/run/sigma/ai.sock".to_owned() }
    }
}

impl LlmBackend for SigmaAiBackend {
    fn name(&self) -> &'static str { "sigma-ai daemon" }

    fn is_available(&self) -> bool {
        std::path::Path::new(&self.socket_path).exists()
    }

    fn generate(&self, messages: &[Message], params: &GenParams) -> Result<String, LlmError> {
        if !self.is_available() { return Err(LlmError::NotAvailable); }
        // Send JSON request over Unix socket
        use std::os::unix::net::UnixStream;
        let stream = UnixStream::connect(&self.socket_path)
            .map_err(|e| LlmError::BackendError(e.to_string()))?;
        let mut stream_w = stream.try_clone().map_err(|e| LlmError::BackendError(e.to_string()))?;

        // Serialize messages
        let msgs_json: String = messages.iter().map(|m| {
            let role = match m.role {
                MessageRole::System    => "system",
                MessageRole::User      => "user",
                MessageRole::Assistant => "assistant",
                MessageRole::Tool      => "tool",
            };
            format!(r#"{{"role":"{}","content":"{}"}}"#, role, m.content.replace('"', "\\\""))
        }).collect::<Vec<_>>().join(",");
        let req = format!(r#"{{"messages":[{}],"max_tokens":{},"temperature":{}}}"#,
                          msgs_json, params.max_tokens, params.temperature);
        write!(stream_w, "{}\n", req).map_err(|e| LlmError::BackendError(e.to_string()))?;
        stream_w.flush().map_err(|e| LlmError::BackendError(e.to_string()))?;

        // Read response
        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader.read_line(&mut response).map_err(|e| LlmError::BackendError(e.to_string()))?;
        // Parse simple JSON: {"content": "..."}
        if let Some(start) = response.find("\"content\":\"") {
            let rest = &response[start+11..];
            if let Some(end) = rest.find("\"}") {
                return Ok(rest[..end].replace("\\n", "\n").replace("\\\"", "\""));
            }
        }
        Ok(response.trim().to_owned())
    }

    fn generate_stream(&self, messages: &[Message], params: &GenParams, on_token: &mut dyn FnMut(&str)) -> Result<(), LlmError> {
        // Fallback: generate then call on_token with full response
        let response = self.generate(messages, params)?;
        for word in response.split_whitespace() { on_token(word); on_token(" "); }
        Ok(())
    }
}

// ── Ollama Backend (HTTP API) ─────────────────────────────────────────────────

pub struct OllamaBackend {
    pub base_url: String,
    pub model:    String,
}

impl OllamaBackend {
    pub fn new(model: &str) -> Self {
        Self { base_url: "http://localhost:11434".to_owned(), model: model.to_owned() }
    }
    pub fn is_running() -> bool {
        Command::new("curl").args(["-s", "-o", "/dev/null", "-w", "%{http_code}",
                                   "http://localhost:11434/api/tags"]).output()
            .ok().and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim() == "200").unwrap_or(false)
    }
}

impl LlmBackend for OllamaBackend {
    fn name(&self)          -> &'static str { "Ollama (local HTTP)" }
    fn is_available(&self)  -> bool { Self::is_running() }

    fn generate(&self, messages: &[Message], params: &GenParams) -> Result<String, LlmError> {
        if !self.is_available() { return Err(LlmError::NotAvailable); }
        let msgs_json: String = messages.iter().map(|m| {
            let role = match m.role { MessageRole::System => "system", MessageRole::User => "user",
                                      MessageRole::Assistant => "assistant", _ => "user" };
            format!(r#"{{"role":"{}","content":"{}"}}"#, role, m.content.replace('"', "\\\""))
        }).collect::<Vec<_>>().join(",");
        let body = format!(r#"{{"model":"{}","messages":[{}],"stream":false,"options":{{"temperature":{},"num_predict":{}}}}}"#,
                           self.model, msgs_json, params.temperature, params.max_tokens);
        let out = Command::new("curl").args([
            "-s", "-X", "POST",
            &format!("{}/api/chat", self.base_url),
            "-H", "Content-Type: application/json",
            "-d", &body,
        ]).output().map_err(|e| LlmError::BackendError(e.to_string()))?;
        let resp = String::from_utf8_lossy(&out.stdout).to_string();
        // Parse: {"message":{"content":"..."}}
        if let Some(start) = resp.find("\"content\":\"") {
            let rest = &resp[start+11..];
            if let Some(end) = rest.find("\"}") {
                return Ok(rest[..end].replace("\\n", "\n"));
            }
        }
        Err(LlmError::ParseError(resp))
    }

    fn generate_stream(&self, messages: &[Message], params: &GenParams, on_token: &mut dyn FnMut(&str)) -> Result<(), LlmError> {
        let response = self.generate(messages, params)?;
        for ch in response.chars() { let s = ch.to_string(); on_token(&s); }
        Ok(())
    }
}

// ── Auto-selecting Backend ────────────────────────────────────────────────────

pub struct AutoBackend { inner: Box<dyn LlmBackend> }

impl AutoBackend {
    pub fn new() -> Self {
        // Priority: sigma-ai daemon → Ollama → llama.cpp
        if SigmaAiBackend::new().is_available() {
            return Self { inner: Box::new(SigmaAiBackend::new()) };
        }
        if OllamaBackend::is_running() {
            return Self { inner: Box::new(OllamaBackend::new("tinyllama")) };
        }
        if let Some(llamacpp) = LlamaCppBackend::auto() {
            return Self { inner: Box::new(llamacpp) };
        }
        // No backend available — use null backend that returns canned responses
        Self { inner: Box::new(NullBackend) }
    }
    pub fn backend_name(&self) -> &'static str { self.inner.name() }
}

impl LlmBackend for AutoBackend {
    fn name(&self)         -> &'static str { "auto" }
    fn is_available(&self) -> bool { self.inner.is_available() }
    fn generate(&self, messages: &[Message], params: &GenParams) -> Result<String, LlmError> {
        self.inner.generate(messages, params)
    }
    fn generate_stream(&self, messages: &[Message], params: &GenParams, on_token: &mut dyn FnMut(&str)) -> Result<(), LlmError> {
        self.inner.generate_stream(messages, params, on_token)
    }
}

// ── Null Backend (offline fallback) ──────────────────────────────────────────

struct NullBackend;
impl LlmBackend for NullBackend {
    fn name(&self)         -> &'static str { "none (offline)" }
    fn is_available(&self) -> bool { false }
    fn generate(&self, messages: &[Message], _: &GenParams) -> Result<String, LlmError> {
        let last = messages.iter().rev().find(|m| m.role == MessageRole::User)
            .map(|m| m.content.as_str()).unwrap_or("");
        Ok(format!("(sigma-ai offline) I understood your request: \"{}\"\n\
Install sigma-ai with: sigma-pkg install sigma-ai\n\
Or use sigma-agent in tool mode: sigma-agent run <command>", &last[..last.len().min(60)]))
    }
    fn generate_stream(&self, messages: &[Message], params: &GenParams, on_token: &mut dyn FnMut(&str)) -> Result<(), LlmError> {
        let r = self.generate(messages, params)?;
        on_token(&r); Ok(())
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn which_llama_cli() -> String {
    let candidates = ["llama-cli", "llama.cpp/llama-cli", "sigma-llm", "/usr/bin/llama-cli"];
    for c in &candidates {
        if Command::new("which").arg(c).output().ok()
            .map(|o| o.status.success()).unwrap_or(false) { return c.to_string(); }
        if std::path::Path::new(c).exists() { return c.to_string(); }
    }
    String::new()
}

fn num_cpus() -> u32 {
    std::fs::read_to_string("/proc/cpuinfo")
        .ok().map(|s| s.lines().filter(|l| l.starts_with("processor")).count() as u32)
        .unwrap_or(4).max(1)
}

fn home_dir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/root".to_owned())
}
