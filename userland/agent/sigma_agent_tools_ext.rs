// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/agent/sigma_agent_tools_ext.rs — Extended Tools (AI, code, GUI mirror)
// Language: Rust (std) — additional tool implementations

use std::collections::BTreeMap;
use std::fs;
use std::process::Command;
use std::path::PathBuf;
use super::sigma_agent::{Tool, ToolResult, ToolArg};

fn home_dir() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_owned()))
}

// ── 11. AI Explain ────────────────────────────────────────────────────────────
pub struct ExplainTool;
impl Tool for ExplainTool {
    fn name(&self)        -> &'static str { "explain" }
    fn description(&self) -> &'static str { "Ask sigma-ai to explain a topic, command, or file" }
    fn aliases(&self)     -> &'static [&'static str] { &["what", "how", "why", "what does", "explain"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"topic", description:"Topic or command to explain", required:true, example:"what does sigma-pkg do" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let topic = match args.get("topic") { Some(t) => t, None => return ToolResult::err("topic required") };
        // Try sigma-ai daemon first
        let out = Command::new("sigma-ai").arg("explain").arg(topic).output();
        match out {
            Ok(o) if o.status.success() => ToolResult::ok(String::from_utf8_lossy(&o.stdout).to_string()),
            _ => {
                // Fallback: built-in explanations for common SigmaOS topics
                let lower = topic.to_ascii_lowercase();
                let explanation = if lower.contains("sigma-pkg") {
                    "sigma-pkg is the SigmaOS sovereign package manager. It installs, removes, and updates .sigpkg packages. All packages are signed with Dilithium-5 (post-quantum). Usage: sigma-pkg install <name> | remove <name> | list | search <query>"
                } else if lower.contains("sigma-sh") {
                    "sigma-sh is the Sovereign Shell — a POSIX-compatible shell written in Nim. It supports pipes, redirects, variables, and scripting. It is the default shell in SigmaOS."
                } else if lower.contains("sigma_pledge") || lower.contains("pledge") {
                    "sigma_pledge is a security syscall that restricts what a process can do after calling it. A process calls pledge(capabilities) and can never gain more capabilities — only restrict further. Inspired by OpenBSD pledge()."
                } else if lower.contains("shard") {
                    "A shard is an atomic, independently-deployable capability module in SigmaOS. The OS has 600+ shards. Each shard can be loaded/unloaded at runtime. Shards communicate via sigma-bus typed IPC."
                } else if lower.contains("sigmafs") {
                    "SigmaFS is the native CoW (Copy-on-Write) journaling filesystem of SigmaOS. It supports snapshots, transparent compression (zstd), per-volume encryption, and atomic updates via OSTree A/B."
                } else {
                    "sigma-ai is not available locally. Install it with: sigma-pkg install sigma-ai"
                };
                ToolResult::ok(explanation.to_owned())
            }
        }
    }
}

// ── 12. Code Edit (Aider-inspired) ────────────────────────────────────────────
pub struct CodeEditTool;
impl Tool for CodeEditTool {
    fn name(&self)        -> &'static str { "code_edit" }
    fn description(&self) -> &'static str { "Use sigma-ai to suggest and apply code changes to a file" }
    fn aliases(&self)     -> &'static [&'static str] { &["edit", "fix", "refactor", "improve code"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"file",        description:"File to edit",             required:true,  example:"src/main.rs" },
        ToolArg { name:"instruction", description:"What to change",           required:true,  example:"add error handling" },
        ToolArg { name:"dry_run",     description:"Preview only (no write)",  required:false, example:"true" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let file        = match args.get("file")        { Some(f) => f, None => return ToolResult::err("file required") };
        let instruction = match args.get("instruction") { Some(i) => i, None => return ToolResult::err("instruction required") };
        let dry_run     = args.get("dry_run").map(|v| v == "true").unwrap_or(false);

        let content = match fs::read_to_string(file) {
            Ok(c) => c,
            Err(e) => return ToolResult::err(format!("Cannot read {}: {}", file, e)),
        };

        // Try sigma-ai for actual editing
        let ai_out = Command::new("sigma-ai")
            .args(["edit", "--file", file, "--instruction", instruction])
            .output();

        match ai_out {
            Ok(o) if o.status.success() => {
                let new_content = String::from_utf8_lossy(&o.stdout).to_string();
                if dry_run {
                    ToolResult::ok(format!("Proposed changes:\n{}", new_content))
                } else {
                    match fs::write(file, &new_content) {
                        Ok(_)  => ToolResult::ok(format!("✓ Applied changes to {}", file)),
                        Err(e) => ToolResult::err(format!("Write failed: {}", e)),
                    }
                }
            }
            _ => {
                // Fallback: show the file with instruction as comment
                let preview = format!(
                    "# sigma-agent would apply: {}\n# File: {}\n\n{}",
                    instruction, file, &content[..content.len().min(500)]
                );
                ToolResult::ok(format!("(sigma-ai not available)\n{}", preview))
                    .with_next("Install sigma-ai with: sigma-pkg install sigma-ai")
            }
        }
    }
}

// ── 13. Summarise File ────────────────────────────────────────────────────────
pub struct SummariseTool;
impl Tool for SummariseTool {
    fn name(&self)        -> &'static str { "summarise" }
    fn description(&self) -> &'static str { "Summarise a file or directory using sigma-ai" }
    fn aliases(&self)     -> &'static [&'static str] { &["summary", "describe", "overview", "tldr"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"path",   description:"File or dir to summarise", required:true,  example:"README.md" },
        ToolArg { name:"length", description:"short | medium | long",    required:false, example:"short" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let path   = match args.get("path") { Some(p) => p, None => return ToolResult::err("path required") };
        let length = args.get("length").map(|s| s.as_str()).unwrap_or("medium");

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return ToolResult::err(format!("Cannot read {}: {}", path, e)),
        };

        let max_chars: usize = match length { "short" => 200, "long" => 2000, _ => 500 };

        // Try sigma-ai
        let ai_out = Command::new("sigma-ai")
            .args(["summarise", "--path", path, "--length", length])
            .output();

        match ai_out {
            Ok(o) if o.status.success() => ToolResult::ok(String::from_utf8_lossy(&o.stdout).to_string()),
            _ => {
                // Fallback: extract first N chars + word count
                let words  = content.split_whitespace().count();
                let lines  = content.lines().count();
                let preview = &content[..content.len().min(max_chars)];
                let summary = format!(
                    "File: {}\nLines: {} | Words: {}\n\nPreview:\n{}\n{}",
                    path, lines, words, preview,
                    if content.len() > max_chars { "... (truncated)" } else { "" }
                );
                ToolResult::ok(summary)
            }
        }
    }
}

// ── 14. Window Manager Control ────────────────────────────────────────────────
pub struct WmControlTool;
impl Tool for WmControlTool {
    fn name(&self)        -> &'static str { "wm_control" }
    fn description(&self) -> &'static str { "Control the Zenith window manager: tile, float, move, resize, workspace" }
    fn aliases(&self)     -> &'static [&'static str] { &["window", "tile", "workspace", "wm", "desktop"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"action",    description:"switch-workspace | tile | float | close | fullscreen", required:true,  example:"switch-workspace" },
        ToolArg { name:"workspace", description:"Workspace number (1–9)",  required:false, example:"2" },
        ToolArg { name:"layout",    description:"master-stack | grid | bsp", required:false, example:"grid" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let action = match args.get("action") { Some(a) => a, None => return ToolResult::err("action required") };
        // Send command to Zenith compositor via sigma-bus IPC socket
        let socket = "/run/sigma/compositor.sock";
        let cmd_json = match action.as_str() {
            "switch-workspace" => {
                let ws = args.get("workspace").map(|s| s.as_str()).unwrap_or("1");
                format!(r#"{{"cmd":"switch_workspace","workspace":{}}}"#, ws)
            }
            "tile"  => r#"{"cmd":"tile"}"#.to_owned(),
            "float" => r#"{"cmd":"float"}"#.to_owned(),
            "close" => r#"{"cmd":"close_focused"}"#.to_owned(),
            "fullscreen" => r#"{"cmd":"fullscreen"}"#.to_owned(),
            "layout" => {
                let layout = args.get("layout").map(|s| s.as_str()).unwrap_or("master-stack");
                format!(r#"{{"cmd":"set_layout","layout":"{}"}}"#, layout)
            }
            _ => return ToolResult::err(format!("Unknown action: {}", action)),
        };
        // Try unix socket IPC
        let out = Command::new("sigma-ipc-send")
            .args(["--socket", socket, "--msg", &cmd_json])
            .output();
        match out {
            Ok(o) if o.status.success() => ToolResult::ok(format!("✓ WM: {}", action)),
            _ => ToolResult::ok(format!("(compositor IPC not available) Would send: {}", cmd_json))
                    .with_next("Start Zenith compositor to enable WM control"),
        }
    }
}

// ── 15. Notification Send ─────────────────────────────────────────────────────
pub struct NotifyTool;
impl Tool for NotifyTool {
    fn name(&self)        -> &'static str { "notify" }
    fn description(&self) -> &'static str { "Send a desktop notification" }
    fn aliases(&self)     -> &'static [&'static str] { &["notification", "alert", "toast", "popup"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"title",   description:"Notification title",   required:true,  example:"Build complete" },
        ToolArg { name:"body",    description:"Notification body",    required:false, example:"sigma-edit compiled successfully" },
        ToolArg { name:"urgency", description:"low | normal | critical", required:false, example:"normal" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let title   = match args.get("title") { Some(t) => t, None => return ToolResult::err("title required") };
        let body    = args.get("body").map(|s| s.as_str()).unwrap_or("");
        let urgency = args.get("urgency").map(|s| s.as_str()).unwrap_or("normal");
        // sigma-notify daemon socket
        let out = Command::new("sigma-notify")
            .args(["--title", title, "--body", body, "--urgency", urgency])
            .output();
        match out {
            Ok(o) if o.status.success() => ToolResult::ok(format!("✓ Notification sent: {}", title)),
            _ => {
                // Fallback: print to terminal in notification style
                let urgency_icon = match urgency { "critical" => "🔴", "low" => "ℹ", _ => "🔔" };
                ToolResult::ok(format!("{} {}: {}", urgency_icon, title, body))
            }
        }
    }
}

// ── 16. Clipboard Control ─────────────────────────────────────────────────────
pub struct ClipboardTool;
impl Tool for ClipboardTool {
    fn name(&self)        -> &'static str { "clipboard" }
    fn description(&self) -> &'static str { "Read from or write to the system clipboard" }
    fn aliases(&self)     -> &'static [&'static str] { &["copy", "paste", "clip"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"action",  description:"read | write | clear",  required:true,  example:"read" },
        ToolArg { name:"content", description:"Content to write",      required:false, example:"Hello world" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let action = match args.get("action") { Some(a) => a, None => return ToolResult::err("action required") };
        let clip_file = home_dir().join(".cache/sigma/clipboard.txt");
        match action.as_str() {
            "read"  => match fs::read_to_string(&clip_file) {
                Ok(c)  => ToolResult::ok(c),
                Err(_) => ToolResult::ok("(clipboard empty)".to_owned()),
            },
            "write" => {
                let content = match args.get("content") { Some(c) => c, None => return ToolResult::err("content required") };
                let _ = fs::create_dir_all(clip_file.parent().unwrap());
                match fs::write(&clip_file, content) {
                    Ok(_)  => ToolResult::ok(format!("✓ Copied {} bytes to clipboard", content.len())),
                    Err(e) => ToolResult::err(format!("Write failed: {}", e)),
                }
            },
            "clear" => { let _ = fs::remove_file(&clip_file); ToolResult::ok("✓ Clipboard cleared".to_owned()) },
            _ => ToolResult::err(format!("Unknown action: {}", action)),
        }
    }
}

// ── 17. Search Files ──────────────────────────────────────────────────────────
pub struct FindFilesTool;
impl Tool for FindFilesTool {
    fn name(&self)        -> &'static str { "find_files" }
    fn description(&self) -> &'static str { "Search for files by name or content" }
    fn aliases(&self)     -> &'static [&'static str] { &["find", "search", "grep", "locate"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"query",   description:"Filename or content to search for", required:true,  example:"sigma_net" },
        ToolArg { name:"path",    description:"Directory to search in",            required:false, example:"/home/user" },
        ToolArg { name:"type",    description:"name | content | both",             required:false, example:"name" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let query = match args.get("query") { Some(q) => q, None => return ToolResult::err("query required") };
        let path  = args.get("path").map(|s| s.as_str()).unwrap_or(".");
        let stype = args.get("type").map(|s| s.as_str()).unwrap_or("name");
        let cmd = if stype == "content" {
            Command::new("grep").args(["-r", "-l", "--include=*.rs", "--include=*.zig",
                                       "--include=*.nim", "-I", query, path]).output()
        } else {
            Command::new("find").args([path, "-name", &format!("*{}*", query)]).output()
        };
        match cmd {
            Ok(o) => {
                let text = String::from_utf8_lossy(&o.stdout).to_string();
                let lines: Vec<&str> = text.lines().take(30).collect();
                ToolResult::ok(if lines.is_empty() { format!("No files matching '{}'", query) }
                               else { lines.join("\n") })
            }
            Err(e) => ToolResult::err(format!("Search failed: {}", e)),
        }
    }
}

// ── 18. Accessibility Control ─────────────────────────────────────────────────
pub struct AccessibilityTool;
impl Tool for AccessibilityTool {
    fn name(&self)        -> &'static str { "accessibility" }
    fn description(&self) -> &'static str { "Toggle accessibility features: high contrast, large text, screen reader, reduce motion" }
    fn aliases(&self)     -> &'static [&'static str] { &["a11y", "access", "accessibility"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"feature", description:"high-contrast | large-text | screen-reader | reduce-motion | colour-blind", required:true,  example:"high-contrast" },
        ToolArg { name:"state",   description:"on | off | toggle",      required:false, example:"on" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let feature = match args.get("feature") { Some(f) => f, None => return ToolResult::err("feature required") };
        let state   = args.get("state").map(|s| s.as_str()).unwrap_or("toggle");
        let cfg     = home_dir().join(".config/sigma/settings/accessibility.toml");
        let _ = std::fs::create_dir_all(cfg.parent().unwrap());
        let existing = fs::read_to_string(&cfg).unwrap_or_default();
        let mut entries: std::collections::BTreeMap<String,String> = existing.lines()
            .filter_map(|l| { let p: Vec<&str> = l.splitn(2,'=').collect(); if p.len()==2 { Some((p[0].trim().to_owned(), p[1].trim().trim_matches('"').to_owned())) } else { None } })
            .collect();
        let key = feature.replace('-', "_");
        let new_val = match state {
            "on"  => "true",
            "off" => "false",
            _     => if entries.get(&key).map(|v| v=="true").unwrap_or(false) { "false" } else { "true" },
        };
        entries.insert(key.clone(), new_val.to_owned());
        let content: String = entries.iter().map(|(k,v)| format!("{} = \"{}\"\n", k, v)).collect();
        match fs::write(&cfg, content) {
            Ok(_)  => ToolResult::ok(format!("✓ Accessibility: {} = {}", feature, new_val)),
            Err(e) => ToolResult::err(format!("Cannot save: {}", e)),
        }
    }
}

// ── 19. VPN Control ──────────────────────────────────────────────────────────
pub struct VpnTool;
impl Tool for VpnTool {
    fn name(&self)        -> &'static str { "vpn" }
    fn description(&self) -> &'static str { "Connect, disconnect, or check WireGuard VPN status" }
    fn aliases(&self)     -> &'static [&'static str] { &["wireguard", "vpn-connect", "vpn-disconnect"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"action",  description:"connect | disconnect | status | list", required:true,  example:"connect" },
        ToolArg { name:"profile", description:"VPN profile name",                    required:false, example:"work-vpn" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let action  = match args.get("action") { Some(a) => a, None => return ToolResult::err("action required") };
        let profile = args.get("profile").map(|s| s.as_str()).unwrap_or("");
        let mut cmd_args = vec!["sigma-vpn", action.as_str()];
        if !profile.is_empty() { cmd_args.push(profile); }
        let out = Command::new(cmd_args[0]).args(&cmd_args[1..]).output();
        match out {
            Ok(o) => ToolResult::ok(String::from_utf8_lossy(&o.stdout).to_string()),
            Err(_) => ToolResult::err("sigma-vpn not found".to_owned()),
        }
    }
}

// ── 20. Disk Management ───────────────────────────────────────────────────────
pub struct DiskTool;
impl Tool for DiskTool {
    fn name(&self)        -> &'static str { "disk" }
    fn description(&self) -> &'static str { "Show disk usage, list disks, format partitions" }
    fn aliases(&self)     -> &'static [&'static str] { &["df", "du", "disk-usage", "storage", "disks"] }
    fn schema(&self) -> Vec<ToolArg> { vec![
        ToolArg { name:"action", description:"usage | list | info <dev>", required:true, example:"usage" },
        ToolArg { name:"path",   description:"Path or device",            required:false, example:"/home" },
    ]}
    fn execute(&self, args: &BTreeMap<String, String>) -> ToolResult {
        let action = match args.get("action") { Some(a) => a, None => return ToolResult::err("action required") };
        let path   = args.get("path").map(|s| s.as_str()).unwrap_or("/");
        match action.as_str() {
            "usage" => {
                let out = Command::new("df").args(["-h", path]).output();
                match out {
                    Ok(o) => ToolResult::ok(String::from_utf8_lossy(&o.stdout).to_string()),
                    Err(_) => {
                        // Fallback: check /proc/mounts
                        let mounts = fs::read_to_string("/proc/mounts").unwrap_or_default();
                        ToolResult::ok(format!("Mounted filesystems:\n{}", mounts.lines().take(10).collect::<Vec<_>>().join("\n")))
                    }
                }
            }
            "list" => {
                let out = Command::new("sigma-disks").arg("list").output();
                match out {
                    Ok(o) => ToolResult::ok(String::from_utf8_lossy(&o.stdout).to_string()),
                    Err(_) => {
                        let out2 = Command::new("lsblk").args(["-o","NAME,SIZE,TYPE,MOUNTPOINT"]).output();
                        match out2 {
                            Ok(o) => ToolResult::ok(String::from_utf8_lossy(&o.stdout).to_string()),
                            Err(_) => ToolResult::ok("(sigma-disks not found — install with: sigma-pkg install sigma-disks)".to_owned()),
                        }
                    }
                }
            }
            _ => ToolResult::err(format!("Unknown action: {}", action)),
        }
    }
}
