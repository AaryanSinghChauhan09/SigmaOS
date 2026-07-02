// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// userland/desktop/sigma_launcher.rs — App Launcher (fuzzy search)
// Language: Rust (std) — OOP via Launcher struct + AppEntry

use std::collections::BTreeMap;

// ── App Entry ─────────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct AppEntry {
    pub id:          String,
    pub name:        String,
    pub description: String,
    pub icon:        String,
    pub command:     String,
    pub category:    AppCategory,
    pub keywords:    Vec<String>,
    pub launch_count: u32,
    pub pinned:      bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppCategory {
    System, Development, Productivity, Media,
    Communication, Security, Education, Games, Utilities, Other,
}

impl AppEntry {
    pub fn new(id: &str, name: &str, cmd: &str, cat: AppCategory) -> Self {
        Self {
            id:          id.to_owned(),
            name:        name.to_owned(),
            description: String::new(),
            icon:        String::new(),
            command:     cmd.to_owned(),
            category:    cat,
            keywords:    Vec::new(),
            launch_count: 0,
            pinned:      false,
        }
    }
    pub fn with_desc(mut self, d: &str) -> Self { self.description = d.to_owned(); self }
    pub fn with_keywords(mut self, kw: &[&str]) -> Self {
        self.keywords = kw.iter().map(|s| s.to_string()).collect(); self
    }
    pub fn with_icon(mut self, i: &str) -> Self { self.icon = i.to_owned(); self }
}

// ── Fuzzy Score ───────────────────────────────────────────────────────────────
/// Returns 0..=100 match score; 0 = no match
fn fuzzy_score(query: &str, target: &str) -> u32 {
    if query.is_empty() { return 50; }
    let q = query.to_ascii_lowercase();
    let t = target.to_ascii_lowercase();
    // Exact match = 100
    if t == q { return 100; }
    // Prefix = 90
    if t.starts_with(&q) { return 90; }
    // Contains = 70
    if t.contains(&q) { return 70; }
    // All chars of query appear in order in target (subsequence) = 40
    let mut qi = 0;
    let qb: Vec<char> = q.chars().collect();
    for c in t.chars() {
        if qi < qb.len() && c == qb[qi] { qi += 1; }
    }
    if qi == qb.len() { return 40; }
    // Initials match: query chars match first letters of words
    let initials: String = t.split_whitespace()
        .filter_map(|w| w.chars().next()).collect();
    if initials.contains(&q) { return 30; }
    0
}

// ── Search Result ─────────────────────────────────────────────────────────────
#[derive(Clone, Debug)]
pub struct SearchResult {
    pub app:   AppEntry,
    pub score: u32,
}

// ── Launcher ──────────────────────────────────────────────────────────────────
pub struct Launcher {
    apps:    BTreeMap<String, AppEntry>,
    history: Vec<String>,         // recently launched app IDs
    pinned:  Vec<String>,
    max_results: usize,
}

impl Launcher {
    pub fn new() -> Self {
        let mut l = Self {
            apps: BTreeMap::new(), history: Vec::new(),
            pinned: Vec::new(), max_results: 10,
        };
        l.register_defaults();
        l
    }

    fn register_defaults(&mut self) {
        let defaults = vec![
            AppEntry::new("sigma-sh",      "Terminal",        "/bin/sigma-sh",       AppCategory::System)
                .with_desc("Sovereign Shell — terminal emulator")
                .with_keywords(&["terminal","shell","console"]),
            AppEntry::new("sigma-files",   "File Manager",    "/usr/bin/sigma-files", AppCategory::Utilities)
                .with_desc("Sovereign VFS file manager")
                .with_keywords(&["files","explorer","browser"]),
            AppEntry::new("sigma-browser", "Browser",         "/usr/bin/sigma-browser",AppCategory::Utilities)
                .with_desc("Sovereign web browser")
                .with_keywords(&["web","internet","chrome","browser"]),
            AppEntry::new("sigma-edit",    "Text Editor",     "/usr/bin/sigma-edit",  AppCategory::Development)
                .with_desc("Sovereign text and code editor")
                .with_keywords(&["editor","code","text"]),
            AppEntry::new("sigma-settings","Settings",        "/usr/bin/sigma-settings",AppCategory::System)
                .with_desc("Unified settings hub")
                .with_keywords(&["settings","preferences","config"]),
            AppEntry::new("sigma-pkg",     "Package Manager", "/usr/bin/sigma-pkg",   AppCategory::System)
                .with_desc("Install and manage sigma packages")
                .with_keywords(&["packages","install","apps"]),
            AppEntry::new("sigma-monitor", "System Monitor",  "/usr/bin/sigma-monitor",AppCategory::Utilities)
                .with_desc("CPU, memory, shard health monitor")
                .with_keywords(&["monitor","cpu","memory","performance"]),
        ];
        for app in defaults { self.apps.insert(app.id.clone(), app); }
    }

    pub fn register(&mut self, app: AppEntry) {
        self.apps.insert(app.id.clone(), app);
    }

    /// Search with fuzzy matching across name, description, keywords
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results: Vec<SearchResult> = self.apps.values().filter_map(|app| {
            let mut best = fuzzy_score(query, &app.name);
            best = best.max(fuzzy_score(query, &app.description));
            for kw in &app.keywords { best = best.max(fuzzy_score(query, kw)); }
            // Boost recently launched
            let recency_boost = self.history.iter().rev().enumerate()
                .find(|(_,id)| id.as_str() == app.id.as_str())
                .map(|(i,_)| (5 - i.min(4)) as u32 * 2).unwrap_or(0);
            // Boost pinned
            let pin_boost = if app.pinned { 5 } else { 0 };
            let score = best + recency_boost + pin_boost;
            if score > 0 { Some(SearchResult { app: app.clone(), score }) }
            else { None }
        }).collect();

        results.sort_by(|a, b| b.score.cmp(&a.score).then(a.app.name.cmp(&b.app.name)));
        results.truncate(self.max_results);
        results
    }

    /// Record a launch and return the command to execute
    pub fn launch(&mut self, id: &str) -> Option<String> {
        if let Some(app) = self.apps.get_mut(id) {
            app.launch_count += 1;
            let cmd = app.command.clone();
            self.history.retain(|h| h != id);
            self.history.insert(0, id.to_owned());
            if self.history.len() > 20 { self.history.pop(); }
            Some(cmd)
        } else { None }
    }

    pub fn pin(&mut self, id: &str) {
        if let Some(app) = self.apps.get_mut(id) { app.pinned = true; self.pinned.push(id.to_owned()); }
    }
    pub fn unpin(&mut self, id: &str) {
        if let Some(app) = self.apps.get_mut(id) { app.pinned = false; }
        self.pinned.retain(|p| p != id);
    }

    pub fn recent(&self, limit: usize) -> Vec<&AppEntry> {
        self.history.iter().take(limit)
            .filter_map(|id| self.apps.get(id)).collect()
    }

    pub fn pinned_apps(&self) -> Vec<&AppEntry> {
        self.pinned.iter().filter_map(|id| self.apps.get(id)).collect()
    }

    pub fn all_by_category(&self, cat: AppCategory) -> Vec<&AppEntry> {
        self.apps.values().filter(|a| a.category == cat).collect()
    }

    pub fn count(&self) -> usize { self.apps.len() }
}
