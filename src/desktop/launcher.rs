//! SigmaOS Command Palette & Universal Launcher (Phase 4 / Omarchy Enhanced)
//!
//! Inspired by Omarchy's keyboard-first launcher workflow:
//! - In-memory query modes for apps, calculator, actions, clipboard, and windows
//! - Prefix/substring ranking with launch-count weighting
//! - Results are models; runtime action dispatch is not implemented here

#![allow(dead_code)]

use std::collections::VecDeque;
use std::string::String;
use std::vec::Vec;

/// Launcher operating mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherMode {
    Application,
    SystemAction,
    Calculator,
    Clipboard,
    WindowSwitcher,
}

/// System action executable from the command palette
#[derive(Debug, Clone)]
pub struct SystemAction {
    pub trigger: String,
    pub description: String,
    pub command: String,
    pub icon: String,
}

/// Open window entry for window switching
#[derive(Debug, Clone)]
pub struct WindowEntry {
    pub window_id: u64,
    pub title: String,
    pub app_class: String,
    pub workspace_id: u32,
    pub is_focused: bool,
}

/// Clipboard history entry
#[derive(Debug, Clone)]
pub struct ClipboardSnippet {
    pub snippet_id: u64,
    pub content: String,
    pub timestamp: u64,
}

/// Standard application entry
#[derive(Debug, Clone)]
pub struct LauncherEntry {
    pub name: String,
    pub exec_path: String,
    pub icon: String,
    pub category: String,
    pub keywords: Vec<String>,
    pub launch_count: u32,
}

/// Search result item returned to the UI
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchResultItem {
    pub title: String,
    pub subtitle: String,
    pub icon: String,
    pub action_payload: String,
    pub mode: LauncherMode,
    pub score: i32,
}

/// Enhanced Command Palette & Universal Launcher
pub struct CommandPalette {
    pub apps: Vec<LauncherEntry>,
    pub system_actions: Vec<SystemAction>,
    pub open_windows: Vec<WindowEntry>,
    pub clipboard_history: VecDeque<ClipboardSnippet>,
    pub max_clipboard_entries: usize,
    next_clipboard_id: u64,
    pub is_open: bool,
}

impl CommandPalette {
    pub fn new() -> Self {
        let mut palette = Self {
            apps: Vec::new(),
            system_actions: Vec::new(),
            open_windows: Vec::new(),
            clipboard_history: VecDeque::new(),
            max_clipboard_entries: 50,
            next_clipboard_id: 1,
            is_open: false,
        };
        palette.register_default_system_actions();
        palette
    }

    /// Register default Omarchy-style system power and desktop commands
    fn register_default_system_actions(&mut self) {
        self.register_action(
            ":lock",
            "Lock Screen (hyprlock / sigma-lock)",
            "sigma-lock",
            "system-lock-screen",
        );
        self.register_action(
            ":reboot",
            "Restart the Operating System",
            "systemctl reboot",
            "system-reboot",
        );
        self.register_action(
            ":poweroff",
            "Shut Down the Computer",
            "systemctl poweroff",
            "system-shutdown",
        );
        self.register_action(
            ":suspend",
            "Suspend to RAM (Sleep)",
            "systemctl suspend",
            "system-suspend",
        );
        self.register_action(
            ":screenshot",
            "Capture Screen or Selection",
            "grim -g $(slurp)",
            "camera-photo",
        );
        self.register_action(
            ":reload-theme",
            "Hot-reload Dynamic Wallpaper Theme",
            "wallust run",
            "preferences-desktop-theme",
        );
        self.register_action(
            ":terminal",
            "Spawn Scratchpad Terminal",
            "alacritty --class scratchpad",
            "utilities-terminal",
        );
        self.register_action(
            ":gaming",
            "Activate Gaming Governor & Boost GPU TDP",
            "sigma-gaming enable",
            "applications-games",
        );
        self.register_action(
            ":powersave",
            "Switch to Battery Powersave Governor",
            "sigma-power powersave",
            "battery-good",
        );
        self.register_action(
            ":snapshot",
            "Create Instant Timeshift Btrfs Snapshot",
            "sigma-timeshift create",
            "system-software-update",
        );
        self.register_action(
            ":backup",
            "Export Software Manifest & Backup",
            "sigma-backup export",
            "document-save",
        );
        self.register_action(
            ":matrix",
            "Verify Distro Launch Superiority Matrix",
            "sigma-launch-check",
            "security-high",
        );
    }

    /// Register an application entry
    pub fn register(&mut self, entry: LauncherEntry) {
        self.apps.push(entry);
    }

    /// Register a system action
    pub fn register_action(&mut self, trigger: &str, desc: &str, cmd: &str, icon: &str) {
        self.system_actions.push(SystemAction {
            trigger: trigger.to_string(),
            description: desc.to_string(),
            command: cmd.to_string(),
            icon: icon.to_string(),
        });
    }

    /// Update running window list from compositor
    pub fn sync_open_windows(&mut self, windows: Vec<WindowEntry>) {
        self.open_windows = windows;
    }

    /// Push text to clipboard history
    pub fn push_clipboard(&mut self, content: &str, timestamp: u64) {
        if content.trim().is_empty() || self.max_clipboard_entries == 0 {
            return;
        }
        let Some(next_id) = self.next_clipboard_id.checked_add(1) else {
            return;
        };
        if self.clipboard_history.len() >= self.max_clipboard_entries {
            self.clipboard_history.pop_back();
        }
        let snippet_id = self.next_clipboard_id;
        self.next_clipboard_id = next_id;
        self.clipboard_history.push_front(ClipboardSnippet {
            snippet_id,
            content: content.to_string(),
            timestamp,
        });
    }

    /// Clear sensitive clipboard history while leaving the monotonic ID sequence intact.
    pub fn clear_clipboard_history(&mut self) {
        self.clipboard_history.clear();
    }

    /// Unified fuzzy search across all active modes
    pub fn query(&self, input: &str) -> Vec<SearchResultItem> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            // Return top frequent apps if no query
            return self
                .apps
                .iter()
                .take(8)
                .map(|app| SearchResultItem {
                    title: app.name.clone(),
                    subtitle: app.category.clone(),
                    icon: app.icon.clone(),
                    action_payload: app.exec_path.clone(),
                    mode: LauncherMode::Application,
                    score: 100 + app.launch_count as i32,
                })
                .collect();
        }

        let mut results = Vec::new();

        // 1. Inline Calculator mode: triggered if starts with "=" or starts with a digit/math operator
        if trimmed.starts_with('=')
            || (trimmed.chars().next().map_or(false, |c| c.is_ascii_digit())
                && trimmed.contains(['+', '-', '*', '/']))
        {
            let expr = trimmed.trim_start_matches('=').trim();
            if let Some(val) = Self::evaluate_math_expr(expr) {
                results.push(SearchResultItem {
                    title: format!("= {}", val),
                    subtitle: format!("Calculation result for '{}'", expr),
                    icon: "accessories-calculator".to_string(),
                    action_payload: format!("{}", val),
                    mode: LauncherMode::Calculator,
                    score: 10_000, // Top priority
                });
            }
        }

        // 2. System Actions: triggered if starts with ":" or matches action triggers
        if trimmed.starts_with(':') {
            let action_q = trimmed.to_lowercase();
            for act in &self.system_actions {
                if act.trigger.to_lowercase().contains(&action_q)
                    || act.description.to_lowercase().contains(&action_q)
                {
                    results.push(SearchResultItem {
                        title: act.trigger.clone(),
                        subtitle: act.description.clone(),
                        icon: act.icon.clone(),
                        action_payload: act.command.clone(),
                        mode: LauncherMode::SystemAction,
                        score: 5_000,
                    });
                }
            }
        }

        // 3. Window Switcher: search open running windows
        let q_lower = trimmed.to_lowercase();
        for win in &self.open_windows {
            if win.title.to_lowercase().contains(&q_lower)
                || win.app_class.to_lowercase().contains(&q_lower)
            {
                results.push(SearchResultItem {
                    title: win.title.clone(),
                    subtitle: format!(
                        "Switch to [{}] on Workspace {}",
                        win.app_class, win.workspace_id
                    ),
                    icon: "window".to_string(),
                    action_payload: format!("focus:{}", win.window_id),
                    mode: LauncherMode::WindowSwitcher,
                    score: 2_000,
                });
            }
        }

        // 4. Clipboard History: if query starts with "cb " or contains clipboard snippets
        if trimmed.starts_with("cb ") || trimmed.starts_with("clip ") {
            let cb_q = trimmed
                .trim_start_matches("cb ")
                .trim_start_matches("clip ")
                .to_lowercase();
            for snippet in &self.clipboard_history {
                if snippet.content.to_lowercase().contains(&cb_q) {
                    let preview = Self::clipboard_preview(&snippet.content, 60);
                    results.push(SearchResultItem {
                        title: preview,
                        subtitle: "Paste from Clipboard History".to_string(),
                        icon: "edit-paste".to_string(),
                        action_payload: snippet.content.clone(),
                        mode: LauncherMode::Clipboard,
                        score: 1_500,
                    });
                }
            }
        }

        // 5. Application search
        for app in &self.apps {
            let name_lower = app.name.to_lowercase();
            let mut score = -1;

            if name_lower == q_lower {
                score = 1000;
            } else if name_lower.starts_with(&q_lower) {
                score = 800;
            } else if name_lower.contains(&q_lower) {
                score = 500;
            } else if app
                .keywords
                .iter()
                .any(|k| k.to_lowercase().contains(&q_lower))
            {
                score = 300;
            }

            if score > 0 {
                score += (app.launch_count as i32) * 5;
                results.push(SearchResultItem {
                    title: app.name.clone(),
                    subtitle: app.category.clone(),
                    icon: app.icon.clone(),
                    action_payload: app.exec_path.clone(),
                    mode: LauncherMode::Application,
                    score,
                });
            }
        }

        // Sort descending by score
        results.sort_by(|a, b| b.score.cmp(&a.score));
        results
    }

    fn clipboard_preview(content: &str, max_bytes: usize) -> String {
        let mut boundary = 0;
        for (index, character) in content.char_indices() {
            let end = index + character.len_utf8();
            if end > max_bytes {
                break;
            }
            boundary = end;
        }
        if boundary < content.len() {
            format!("{}...", &content[..boundary])
        } else {
            content.to_string()
        }
    }

    /// Legacy fuzzy search compatibility wrapper
    pub fn fuzzy_search(&self, query: &str) -> Vec<&LauncherEntry> {
        let q = query.to_lowercase();
        let mut matched: Vec<(&LauncherEntry, usize)> = self
            .apps
            .iter()
            .filter_map(|e| {
                let name_lower = e.name.to_lowercase();
                if name_lower.contains(&q) {
                    Some((e, name_lower.find(&q).unwrap_or(usize::MAX)))
                } else if e.keywords.iter().any(|k| k.to_lowercase().contains(&q)) {
                    Some((e, 1000))
                } else {
                    None
                }
            })
            .collect();
        matched.sort_by_key(|(_, score)| *score);
        matched.into_iter().map(|(e, _)| e).collect()
    }

    /// Simple safe arithmetic expression parser for the calculator mode
    fn evaluate_math_expr(expr: &str) -> Option<f64> {
        let clean: String = expr.chars().filter(|c| !c.is_whitespace()).collect();
        if clean.is_empty() {
            return None;
        }

        // Simple binary operation parser (+, -, *, /)
        if let Some(pos) = clean.rfind('+') {
            let left = Self::evaluate_math_expr(&clean[..pos])?;
            let right = Self::evaluate_math_expr(&clean[pos + 1..])?;
            return Some(left + right);
        }
        if let Some(pos) = clean.rfind('-') {
            if pos > 0 {
                // Avoid unary minus
                let left = Self::evaluate_math_expr(&clean[..pos])?;
                let right = Self::evaluate_math_expr(&clean[pos + 1..])?;
                return Some(left - right);
            }
        }
        if let Some(pos) = clean.rfind('*') {
            let left = Self::evaluate_math_expr(&clean[..pos])?;
            let right = Self::evaluate_math_expr(&clean[pos + 1..])?;
            return Some(left * right);
        }
        if let Some(pos) = clean.rfind('/') {
            let left = Self::evaluate_math_expr(&clean[..pos])?;
            let right = Self::evaluate_math_expr(&clean[pos + 1..])?;
            if right == 0.0 {
                return None;
            }
            return Some(left / right);
        }

        clean.parse::<f64>().ok()
    }

    pub fn toggle(&mut self) -> bool {
        self.is_open = !self.is_open;
        self.is_open
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_evaluation() {
        assert_eq!(CommandPalette::evaluate_math_expr("12 + 8"), Some(20.0));
        assert_eq!(CommandPalette::evaluate_math_expr("100 - 25"), Some(75.0));
        assert_eq!(CommandPalette::evaluate_math_expr("6 * 7"), Some(42.0));
        assert_eq!(CommandPalette::evaluate_math_expr("100 / 4"), Some(25.0));
        assert_eq!(CommandPalette::evaluate_math_expr("5 / 0"), None);
    }

    #[test]
    fn test_multi_mode_query() {
        let mut palette = CommandPalette::new();
        palette.register(LauncherEntry {
            name: "Terminal".into(),
            exec_path: "/usr/bin/sigma-term".into(),
            icon: "terminal".into(),
            category: "System".into(),
            keywords: vec!["console".into(), "shell".into()],
            launch_count: 5,
        });

        // Test math expression
        let calc_results = palette.query("= 50 * 2");
        assert_eq!(calc_results.len(), 1);
        assert_eq!(calc_results[0].mode, LauncherMode::Calculator);
        assert_eq!(calc_results[0].title, "= 100");

        // Test system action
        let action_results = palette.query(":lock");
        assert_eq!(action_results.len(), 1);
        assert_eq!(action_results[0].mode, LauncherMode::SystemAction);

        // Test app search
        let app_results = palette.query("term");
        assert_eq!(app_results.len(), 1);
        assert_eq!(app_results[0].title, "Terminal");
    }

    #[test]
    fn test_window_switcher_and_clipboard() {
        let mut palette = CommandPalette::new();
        palette.sync_open_windows(vec![WindowEntry {
            window_id: 101,
            title: "Firefox — GitHub".into(),
            app_class: "firefox".into(),
            workspace_id: 2,
            is_focused: false,
        }]);
        palette.push_clipboard(
            "https://github.com/AaryanSinghChauhan09/SigmaOS",
            1700000000,
        );

        let win_results = palette.query("Firefox");
        assert_eq!(win_results.len(), 1);
        assert_eq!(win_results[0].mode, LauncherMode::WindowSwitcher);

        let cb_results = palette.query("cb SigmaOS");
        assert_eq!(cb_results.len(), 1);
        assert_eq!(cb_results[0].mode, LauncherMode::Clipboard);
    }

    #[test]
    fn clipboard_previews_preserve_utf8_boundaries() {
        let mut palette = CommandPalette::new();
        let content = "é".repeat(40);
        palette.push_clipboard(&content, 1);
        let results = palette.query("cb é");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, format!("{}...", "é".repeat(30)));
    }

    #[test]
    fn clipboard_history_ids_stay_unique_after_eviction_and_clear() {
        let mut palette = CommandPalette::new();
        palette.max_clipboard_entries = 2;
        palette.push_clipboard("first", 1);
        palette.push_clipboard("second", 2);
        palette.push_clipboard("third", 3);
        let ids: Vec<u64> = palette
            .clipboard_history
            .iter()
            .map(|entry| entry.snippet_id)
            .collect();
        assert_eq!(ids, [3, 2]);

        palette.clear_clipboard_history();
        palette.push_clipboard("after clear", 4);
        assert_eq!(palette.clipboard_history[0].snippet_id, 4);
    }

    #[test]
    fn zero_capacity_disables_clipboard_history() {
        let mut palette = CommandPalette::new();
        palette.max_clipboard_entries = 0;
        palette.push_clipboard("private text", 1);
        assert!(palette.clipboard_history.is_empty());
    }
}
