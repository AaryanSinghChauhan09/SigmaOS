// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/agent/sigma_agent.rs — sigma-agent: AI-powered CLI agent
//
// Inspired by: Claude Code, Aider, Hermes IDE
// Language: Rust (std) — OOP via Agent struct + Tool trait
//
// Architecture:
//   User prompt → Intent parser → Tool selector → Tool executor → Response
//   (agentic loop: can call multiple tools, use output, continue)

use std::collections::{BTreeMap, VecDeque};
use std::io::{self, Write, BufRead};
use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;

// ── Tool Trait (every capability the agent can use) ───────────────────────────

pub trait Tool: Send + Sync {
    fn name(&self)        -> &'static str;
    fn description(&self) -> &'static str;
    fn aliases(&self)     -> &'static [&'static str] { &[] }
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult;
    /// Describe what args this tool accepts
    fn schema(&self)      -> Vec<ToolArg>;
}

#[derive(Debug, Clone)]
pub struct ToolArg {
    pub name:        &'static str,
    pub description: &'static str,
    pub required:    bool,
    pub example:     &'static str,
}

#[derive(Debug, Clone)]
pub struct ToolResult {
    pub success: bool,
    pub output:  String,
    pub error:   Option<String>,
    pub next:    Option<String>, // suggested follow-up action
}

impl ToolResult {
    pub fn ok(output: impl Into<String>)  -> Self { Self { success:true,  output:output.into(), error:None, next:None } }
    pub fn err(e: impl Into<String>)       -> Self { Self { success:false, output:String::new(), error:Some(e.into()), next:None } }
    pub fn with_next(mut self, n: &str)    -> Self { self.next = Some(n.to_owned()); self }
}

// ── Built-in Tools ────────────────────────────────────────────────────────────

// 1. File Read
pub struct ReadFileTool;
impl Tool for ReadFileTool {
    fn name(&self)        -> &'static str { "read_file" }
    fn description(&self) -> &'static str { "Read the content of a file" }
    fn aliases(&self)     -> &'static [&'static str] { &["cat", "show", "read"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"path", description:"File path to read", required:true, example:"/home/user/notes.md" },
        ToolArg { name:"lines", description:"Max lines to return (default all)", required:false, example:"50" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let path = match args.get("path") { Some(p) => p, None => return ToolResult::err("path required") };
        match fs::read_to_string(path) {
            Ok(content) => {
                let max_lines: usize = args.get("lines").and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);
                let out: String = content.lines().take(max_lines).collect::<Vec<_>>().join("\n");
                ToolResult::ok(out)
            }
            Err(e) => ToolResult::err(format!("Cannot read {}: {}", path, e))
        }
    }
}

// 2. File Write
pub struct WriteFileTool;
impl Tool for WriteFileTool {
    fn name(&self)        -> &'static str { "write_file" }
    fn description(&self) -> &'static str { "Write or overwrite a file with given content" }
    fn aliases(&self)     -> &'static [&'static str] { &["write", "save", "create_file"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"path",    description:"Destination file path", required:true,  example:"/tmp/out.txt" },
        ToolArg { name:"content", description:"Content to write",      required:true,  example:"hello world" },
        ToolArg { name:"append",  description:"Append instead of overwrite", required:false, example:"true" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let path    = match args.get("path")    { Some(p) => p, None => return ToolResult::err("path required") };
        let content = match args.get("content") { Some(c) => c, None => return ToolResult::err("content required") };
        let append  = args.get("append").map(|v| v == "true").unwrap_or(false);
        let result = if append {
            use std::io::Write;
            fs::OpenOptions::new().create(true).append(true).open(path)
                .and_then(|mut f| write!(f, "{}", content).map(|_| ()))
        } else {
            fs::write(path, content).map(|_| ())
        };
        match result {
            Ok(_)  => ToolResult::ok(format!("Written {} bytes to {}", content.len(), path)),
            Err(e) => ToolResult::err(format!("Write failed: {}", e))
        }
    }
}

// 3. List Directory
pub struct ListDirTool;
impl Tool for ListDirTool {
    fn name(&self)        -> &'static str { "list_dir" }
    fn description(&self) -> &'static str { "List files and directories at a path" }
    fn aliases(&self)     -> &'static [&'static str] { &["ls", "dir", "list"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"path",      description:"Directory to list", required:false, example:"/home/user" },
        ToolArg { name:"recursive", description:"List recursively",  required:false, example:"true" },
        ToolArg { name:"hidden",    description:"Show hidden files",  required:false, example:"true" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let path  = args.get("path").map(|s| s.as_str()).unwrap_or(".");
        let hidden = args.get("hidden").map(|v| v=="true").unwrap_or(false);
        match fs::read_dir(path) {
            Ok(entries) => {
                let mut lines = Vec::new();
                for e in entries.flatten() {
                    let name = e.file_name().to_string_lossy().to_string();
                    if !hidden && name.starts_with('.') { continue; }
                    let meta = e.metadata().ok();
                    let kind = if meta.as_ref().map(|m| m.is_dir()).unwrap_or(false) { "/" } else { "" };
                    let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                    lines.push(format!("{}{:>10}  {}{}", kind, size, name, kind));
                }
                lines.sort();
                ToolResult::ok(lines.join("\n"))
            }
            Err(e) => ToolResult::err(format!("Cannot list {}: {}", path, e))
        }
    }
}

// 4. Run Shell Command
pub struct ShellTool;
impl Tool for ShellTool {
    fn name(&self)        -> &'static str { "shell" }
    fn description(&self) -> &'static str { "Execute a shell command and return stdout+stderr (with optional interactive input simulation)" }
    fn aliases(&self)     -> &'static [&'static str] { &["run", "exec", "bash", "cmd"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"command", description:"Shell command to execute", required:true, example:"sigma-pkg list" },
        ToolArg { name:"cwd",     description:"Working directory",        required:false, example:"/home/user" },
        ToolArg { name:"input",   description:"Interactive input string to feed into stdin", required:false, example:"y\n" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let cmd   = match args.get("command") { Some(c) => c, None => return ToolResult::err("command required") };
        let cwd   = args.get("cwd").map(|s| s.as_str()).unwrap_or(".");
        let input = args.get("input");
        
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .current_dir(cwd)
            .stdin(if input.is_some() { std::process::Stdio::piped() } else { std::process::Stdio::null() })
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn();

        let mut child = match child {
            Ok(c) => c,
            Err(e) => return ToolResult::err(format!("Spawn failed: {}", e)),
        };

        if let Some(in_str) = input {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = write!(stdin, "{}", in_str);
                let _ = stdin.flush();
            }
        }

        let output = child.wait_with_output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let combined = format!("{}{}", stdout, stderr).trim().to_owned();
                if out.status.success() { ToolResult::ok(combined) }
                else { ToolResult::err(combined) }
            }
            Err(e) => ToolResult::err(format!("Exec failed: {}", e))
        }
    }
}

// 5. Install Package
pub struct InstallPkgTool;
impl Tool for InstallPkgTool {
    fn name(&self)        -> &'static str { "install_package" }
    fn description(&self) -> &'static str { "Install a sigpkg package via sigma-pkg" }
    fn aliases(&self)     -> &'static [&'static str] { &["install", "pkg-install", "add"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"name", description:"Package name to install", required:true, example:"sigma-edit" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let name = match args.get("name") { Some(n) => n, None => return ToolResult::err("name required") };
        let out = Command::new("sigma-pkg").arg("install").arg(name).output();
        match out {
            Ok(o) if o.status.success() => ToolResult::ok(format!("✓ Installed {}", name)),
            Ok(o) => ToolResult::err(String::from_utf8_lossy(&o.stderr).to_string()),
            Err(_) => ToolResult::err("sigma-pkg not found — ensure it is in PATH".to_owned()),
        }
    }
}

// 6. Open App
pub struct OpenAppTool;
impl Tool for OpenAppTool {
    fn name(&self)        -> &'static str { "open_app" }
    fn description(&self) -> &'static str { "Launch a SigmaOS application by name" }
    fn aliases(&self)     -> &'static [&'static str] { &["open", "launch", "start"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"app",  description:"App name or path",  required:true,  example:"sigma-edit" },
        ToolArg { name:"args", description:"Arguments to pass", required:false, example:"myfile.rs" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let app  = match args.get("app") { Some(a) => a, None => return ToolResult::err("app required") };
        let xargs = args.get("args").map(|s| s.split(' ').collect::<Vec<_>>()).unwrap_or_default();
        match Command::new(app).args(&xargs).spawn() {
            Ok(_)  => ToolResult::ok(format!("Launched {}", app)),
            Err(e) => ToolResult::err(format!("Cannot launch {}: {}", app, e)),
        }
    }
}

// 7. Settings Get/Set
pub struct SettingsTool;
impl Tool for SettingsTool {
    fn name(&self)        -> &'static str { "settings" }
    fn description(&self) -> &'static str { "Get or set SigmaOS settings (theme, accessibility, network, etc.)" }
    fn aliases(&self)     -> &'static [&'static str] { &["setting", "config", "configure", "set", "get"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"action", description:"'get' or 'set'",    required:true,  example:"set" },
        ToolArg { name:"panel",  description:"Settings panel ID",  required:true,  example:"appearance" },
        ToolArg { name:"key",    description:"Setting key",        required:true,  example:"theme" },
        ToolArg { name:"value",  description:"New value (for set)", required:false, example:"zenith-dark" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let action = args.get("action").map(|s| s.as_str()).unwrap_or("get");
        let panel  = args.get("panel").unwrap_or(&"appearance".to_owned()).clone();
        let key    = match args.get("key") { Some(k) => k, None => return ToolResult::err("key required") };
        let cfg_dir = dirs_home().join(".config/sigma/settings");
        let _ = fs::create_dir_all(&cfg_dir);
        let file = cfg_dir.join(format!("{}.toml", panel));
        if action == "set" {
            let value = match args.get("value") { Some(v) => v, None => return ToolResult::err("value required for set") };
            let existing = fs::read_to_string(&file).unwrap_or_default();
            let mut entries: BTreeMap<String,String> = toml_parse_simple(&existing);
            entries.insert(key.clone(), value.clone());
            let content = entries.iter().map(|(k,v)| format!("{} = \"{}\"\n", k, v)).collect::<String>();
            match fs::write(&file, content) {
                Ok(_)  => ToolResult::ok(format!("✓ {}.{} = {}", panel, key, value)),
                Err(e) => ToolResult::err(format!("Cannot write settings: {}", e)),
            }
        } else {
            let existing = fs::read_to_string(&file).unwrap_or_default();
            let entries = toml_parse_simple(&existing);
            match entries.get(key.as_str()) {
                Some(v) => ToolResult::ok(format!("{}.{} = {}", panel, key, v)),
                None    => ToolResult::ok(format!("{}.{} = (not set)", panel, key)),
            }
        }
    }
}

fn dirs_home() -> PathBuf { PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_owned())) }

fn toml_parse_simple(s: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for line in s.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() { continue; }
        if let Some(eq) = line.find('=') {
            let k = line[..eq].trim().to_owned();
            let v = line[eq+1..].trim().trim_matches('"').to_owned();
            map.insert(k, v);
        }
    }
    map
}

// 8. System Info
pub struct SystemInfoTool;
impl Tool for SystemInfoTool {
    fn name(&self)        -> &'static str { "system_info" }
    fn description(&self) -> &'static str { "Show system information: CPU, memory, disk, network, OS version" }
    fn aliases(&self)     -> &'static [&'static str] { &["sysinfo", "info", "status", "neofetch"] }
    fn schema(&self) -> Vec<ToolArg> { vec![] }
    fn execute(&self, _: &BTreeMap<String, String>) -> ToolResult {
        let mut lines = Vec::new();
        lines.push("Σ SigmaOS v15.0 Zenith".to_owned());
        lines.push("─".repeat(40));
        // OS
        let hostname = fs::read_to_string("/etc/hostname").unwrap_or_else(|_| "sigmaos".to_owned()).trim().to_owned();
        lines.push(format!("Host:    {}", hostname));
        // Kernel
        if let Ok(ver) = fs::read_to_string("/proc/version") {
            let first = ver.split_whitespace().take(3).collect::<Vec<_>>().join(" ");
            lines.push(format!("Kernel:  {}", first));
        }
        // CPU
        if let Ok(info) = fs::read_to_string("/proc/cpuinfo") {
            if let Some(model) = info.lines().find(|l| l.starts_with("model name")) {
                if let Some(v) = model.split(':').nth(1) { lines.push(format!("CPU:     {}", v.trim())); }
            }
        }
        // Memory
        if let Ok(mem) = fs::read_to_string("/proc/meminfo") {
            let mut total = 0u64; let mut avail = 0u64;
            for line in mem.lines() {
                let p: Vec<&str> = line.split_whitespace().collect();
                if p.len() >= 2 {
                    match p[0] {
                        "MemTotal:"     => { total = p[1].parse().unwrap_or(0); }
                        "MemAvailable:" => { avail = p[1].parse().unwrap_or(0); }
                        _ => {}
                    }
                }
            }
            lines.push(format!("Memory:  {} MB / {} MB", (total-avail)/1024, total/1024));
        }
        // Uptime
        if let Ok(up) = fs::read_to_string("/proc/uptime") {
            let secs: f64 = up.split_whitespace().next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
            let h = secs as u64 / 3600; let m = (secs as u64 % 3600) / 60;
            lines.push(format!("Uptime:  {}h {}m", h, m));
        }
        ToolResult::ok(lines.join("\n"))
    }
}

// 9. Network Control
pub struct NetworkTool;
impl Tool for NetworkTool {
    fn name(&self)        -> &'static str { "network" }
    fn description(&self) -> &'static str { "Manage network interfaces: list, connect Wi-Fi, set DNS, check status" }
    fn aliases(&self)     -> &'static [&'static str] { &["net", "wifi", "internet", "netctl"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"action", description:"list | status | wifi-connect | dns | vpn", required:true, example:"list" },
        ToolArg { name:"iface",  description:"Interface name",   required:false, example:"wlan0" },
        ToolArg { name:"ssid",   description:"Wi-Fi SSID",       required:false, example:"MyNetwork" },
        ToolArg { name:"pass",   description:"Wi-Fi password",   required:false, example:"secret" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let action = args.get("action").map(|s| s.as_str()).unwrap_or("list");
        let mut cmd_args = vec!["sigma-netctl", action];
        if let Some(i) = args.get("iface") { cmd_args.push(i); }
        if action == "wifi-connect" {
            if let Some(s) = args.get("ssid")  { cmd_args.push(s); }
            if let Some(p) = args.get("pass")  { cmd_args.push(p); }
        }
        let out = Command::new(cmd_args[0]).args(&cmd_args[1..]).output();
        match out {
            Ok(o) => ToolResult::ok(String::from_utf8_lossy(&o.stdout).to_string()),
            Err(_) => ToolResult::err("sigma-netctl not found".to_owned()),
        }
    }
}

// 10. Process Control
pub struct ProcessTool;
impl Tool for ProcessTool {
    fn name(&self)        -> &'static str { "process" }
    fn description(&self) -> &'static str { "List, kill, or monitor processes" }
    fn aliases(&self)     -> &'static [&'static str] { &["ps", "kill", "top", "processes"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"action", description:"list | kill | top",  required:true,  example:"list" },
        ToolArg { name:"pid",    description:"Process ID to kill", required:false, example:"1234" },
        ToolArg { name:"name",   description:"Process name filter",required:false, example:"sigma-sh" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let action = args.get("action").map(|s| s.as_str()).unwrap_or("list");
        match action {
            "list" => {
                let filter = args.get("name").map(|s| s.as_str()).unwrap_or("");
                let out = Command::new("ps").args(&["aux"]).output();
                match out {
                    Ok(o) => {
                        let text = String::from_utf8_lossy(&o.stdout).to_string();
                        let filtered: String = text.lines()
                            .filter(|l| filter.is_empty() || l.contains(filter))
                            .collect::<Vec<_>>().join("\n");
                        ToolResult::ok(filtered)
                    }
                    Err(_) => {
                        // Fallback: read /proc
                        let mut out_lines = vec!["PID   NAME".to_owned()];
                        if let Ok(entries) = fs::read_dir("/proc") {
                            for e in entries.flatten() {
                                let name = e.file_name().to_string_lossy().to_string();
                                if name.chars().all(|c| c.is_ascii_digit()) {
                                    let comm = fs::read_to_string(format!("/proc/{}/comm", name))
                                        .unwrap_or_default().trim().to_owned();
                                    if filter.is_empty() || comm.contains(filter) {
                                        out_lines.push(format!("{:<6} {}", name, comm));
                                    }
                                }
                            }
                        }
                        ToolResult::ok(out_lines.join("\n"))
                    }
                }
            }
            "kill" => {
                let pid = match args.get("pid") { Some(p) => p, None => return ToolResult::err("pid required") };
                let _ = Command::new("kill").arg(pid).status();
                ToolResult::ok(format!("Sent SIGTERM to PID {}", pid))
            }
            _ => ToolResult::err(format!("Unknown action: {}", action)),
        }
    }
}
