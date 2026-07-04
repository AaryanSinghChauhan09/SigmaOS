// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/tools/sigma_logs.rs — sigma-logs: Structured Log Viewer
// Language: Rust (std) — OOP via LogViewer struct + LogEntry + Filter

use std::collections::VecDeque;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

// ── Log Level ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level { Trace, Debug, Info, Warn, Error, Critical }

impl Level {
    pub fn from_str(s: &str) -> Self {
        match s.to_ascii_uppercase().as_str() {
            "TRACE"    => Self::Trace,
            "DEBUG"    => Self::Debug,
            "INFO"     => Self::Info,
            "WARN" | "WARNING" => Self::Warn,
            "ERROR"    => Self::Error,
            "CRITICAL" | "FATAL" | "CRIT" => Self::Critical,
            _ => Self::Info,
        }
    }
    pub fn label(&self) -> &'static str {
        match self {
            Self::Trace    => "TRACE",
            Self::Debug    => "DEBUG",
            Self::Info     => "INFO ",
            Self::Warn     => "WARN ",
            Self::Error    => "ERROR",
            Self::Critical => "CRIT ",
        }
    }
    pub fn color_prefix(&self) -> &'static str {
        match self {
            Self::Trace    => "\x1b[90m",  // dark gray
            Self::Debug    => "\x1b[36m",  // cyan
            Self::Info     => "\x1b[32m",  // green
            Self::Warn     => "\x1b[33m",  // yellow
            Self::Error    => "\x1b[31m",  // red
            Self::Critical => "\x1b[35m",  // magenta
        }
    }
}

// ── Log Entry ─────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub level:     Level,
    pub source:    String,
    pub message:   String,
    pub pid:       Option<u32>,
    pub seq:       u64,
}

impl LogEntry {
    /// Parse a structured log line: "YYYY-MM-DD HH:MM:SS [LEVEL] source: message"
    pub fn parse(line: &str, seq: u64) -> Option<Self> {
        if line.trim().is_empty() { return None; }
        // Try structured format first
        let parts: Vec<&str> = line.splitn(4, ' ').collect();
        if parts.len() >= 4 {
            let ts = format!("{} {}", parts[0], parts[1]);
            let level_str = parts[2].trim_matches(|c| c == '[' || c == ']');
            let rest = parts[3];
            let (source, message) = if let Some(colon) = rest.find(':') {
                (rest[..colon].trim().to_owned(), rest[colon+1..].trim().to_owned())
            } else {
                ("system".to_owned(), rest.trim().to_owned())
            };
            return Some(Self {
                timestamp: ts,
                level:     Level::from_str(level_str),
                source, message,
                pid:       None,
                seq,
            });
        }
        // Fallback: treat whole line as INFO message
        Some(Self {
            timestamp: String::new(),
            level:     Level::Info,
            source:    "system".to_owned(),
            message:   line.trim().to_owned(),
            pid:       None,
            seq,
        })
    }

    pub fn format_colored(&self, use_color: bool) -> String {
        let reset = if use_color { "\x1b[0m" } else { "" };
        let col   = if use_color { self.level.color_prefix() } else { "" };
        if self.timestamp.is_empty() {
            format!("{}{} [{}] {}: {}{}", col, self.seq, self.level.label(),
                    self.source, self.message, reset)
        } else {
            format!("{}{} {} [{}] {}: {}{}", col, self.timestamp, self.seq,
                    self.level.label(), self.source, self.message, reset)
        }
    }
}

// ── Filter ────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default)]
pub struct LogFilter {
    pub min_level:  Option<Level>,
    pub source:     Option<String>,
    pub search:     Option<String>,
    pub pid:        Option<u32>,
}

impl LogFilter {
    pub fn matches(&self, e: &LogEntry) -> bool {
        if let Some(min) = self.min_level { if e.level < min { return false; } }
        if let Some(ref src) = self.source {
            if !e.source.to_ascii_lowercase().contains(&src.to_ascii_lowercase()) { return false; }
        }
        if let Some(ref q) = self.search {
            let q_l = q.to_ascii_lowercase();
            if !e.message.to_ascii_lowercase().contains(&q_l)
               && !e.source.to_ascii_lowercase().contains(&q_l) { return false; }
        }
        if let Some(pid) = self.pid { if e.pid != Some(pid) { return false; } }
        true
    }
}

// ── Log Source ────────────────────────────────────────────────────────────────

pub trait LogSource: Send {
    fn name(&self) -> &str;
    fn read_new(&mut self) -> Vec<LogEntry>;
}

pub struct FileLogSource {
    path:     PathBuf,
    position: u64,
    seq:      u64,
}

impl FileLogSource {
    pub fn new(path: &str) -> Self {
        Self { path: PathBuf::from(path), position: 0, seq: 0 }
    }
}

impl LogSource for FileLogSource {
    fn name(&self) -> &str { self.path.to_str().unwrap_or("") }

    fn read_new(&mut self) -> Vec<LogEntry> {
        let mut entries = Vec::new();
        if let Ok(f) = fs::File::open(&self.path) {
            use std::io::Seek;
            let mut reader = BufReader::new(f);
            if let Ok(_) = reader.seek(std::io::SeekFrom::Start(self.position)) {
                for line in reader.lines().flatten() {
                    self.seq += 1;
                    if let Some(e) = LogEntry::parse(&line, self.seq) {
                        entries.push(e);
                    }
                }
                self.position = reader.stream_position().unwrap_or(self.position);
            }
        }
        entries
    }
}

// ── Log Viewer ────────────────────────────────────────────────────────────────

pub struct LogViewer {
    sources:  Vec<Box<dyn LogSource>>,
    buffer:   VecDeque<LogEntry>,
    filter:   LogFilter,
    max_buf:  usize,
    use_color: bool,
}

impl LogViewer {
    pub fn new() -> Self {
        Self {
            sources:   Vec::new(),
            buffer:    VecDeque::with_capacity(10000),
            filter:    LogFilter::default(),
            max_buf:   10000,
            use_color: true,
        }
    }

    pub fn add_source(&mut self, src: Box<dyn LogSource>) { self.sources.push(src); }

    pub fn set_filter(&mut self, f: LogFilter) { self.filter = f; }
    pub fn set_color(&mut self, v: bool) { self.use_color = v; }

    /// Poll all sources for new entries
    pub fn poll(&mut self) -> usize {
        let mut count = 0;
        let mut new_entries = Vec::new();
        for src in &mut self.sources { new_entries.extend(src.read_new()); }
        for e in new_entries {
            if self.buffer.len() >= self.max_buf { self.buffer.pop_front(); }
            self.buffer.push_back(e);
            count += 1;
        }
        count
    }

    /// Matching entries from buffer
    pub fn visible(&self) -> impl Iterator<Item = &LogEntry> {
        self.buffer.iter().filter(|e| self.filter.matches(e))
    }

    /// Print all visible entries
    pub fn print_all(&self) {
        for e in self.visible() {
            println!("{}", e.format_colored(self.use_color));
        }
    }

    /// Follow mode: print new entries as they arrive
    pub fn follow(&mut self) {
        loop {
            let n = self.poll();
            if n > 0 {
                for e in self.buffer.iter().rev().take(n).collect::<Vec<_>>().iter().rev() {
                    if self.filter.matches(e) {
                        println!("{}", e.format_colored(self.use_color));
                    }
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }

    pub fn count(&self) -> usize { self.buffer.len() }
    pub fn filtered_count(&self) -> usize { self.visible().count() }
}
