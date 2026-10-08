//! SigmaOS Command Palette & Universal Launcher (Phase 4 / Omarchy Enhanced)
//!
//! Inspired by Omarchy's keyboard-first Walker / Rofi unified launcher:
//! - Multi-mode search: Applications, Inline Calculator, System Actions, Clipboard History, Window Switcher
//! - Fuzzy scoring algorithm with priority weighting
//! - Zero-allocation friendly parsing and fast lookup

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

/// Built-in action identifiers. These are dispatched to an OS service; they
/// are never interpreted as shell text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemActionKind {
    LockScreen,
    Restart,
    PowerOff,
    Suspend,
    Screenshot,
    ReloadTheme,
    OpenTerminal,
    GamingProfile,
    PowerSaveProfile,
    CreateSnapshot,
    ExportBackup,
    SystemInformation,
}

impl SystemActionKind {
    pub const fn id(self) -> &'static str {
        match self {
            Self::LockScreen => "lock-screen",
            Self::Restart => "restart",
            Self::PowerOff => "power-off",
            Self::Suspend => "suspend",
            Self::Screenshot => "screenshot",
            Self::ReloadTheme => "reload-theme",
            Self::OpenTerminal => "open-terminal",
            Self::GamingProfile => "gaming-profile",
            Self::PowerSaveProfile => "power-save-profile",
            Self::CreateSnapshot => "create-snapshot",
            Self::ExportBackup => "export-backup",
            Self::SystemInformation => "system-information",
        }
    }

    pub const fn requires_confirmation(self) -> bool {
        matches!(self, Self::Restart | Self::PowerOff)
    }
}

#[derive(Debug, Clone)]
pub struct SystemAction {
    pub kind: SystemActionKind,
    pub trigger: String,
    pub description: String,
    pub icon: String,
    /// Becomes true only after an actual service reports support.
    pub available: bool,
}

/// Platform backends must dispatch typed actions, never shell strings.
pub trait SystemActionHandler {
    fn is_available(&self, action: SystemActionKind) -> bool;
    fn execute(&mut self, action: SystemActionKind) -> Result<(), String>;
}

/// Applications are launched through a process service, never through shell
/// interpolation of catalog strings.
pub trait ApplicationLaunchHandler {
    fn is_installed(&self, executable: &str) -> bool;
    fn launch(&mut self, executable: &str) -> Result<(), String>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemActionError {
    UnknownAction,
    Unavailable,
    ConfirmationRequired,
    Backend(String),
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
    /// True only when verified installed-application inventory confirms it.
    pub is_installed: bool,
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
    pub enabled: bool,
    pub requires_confirmation: bool,
}

/// Enhanced Command Palette & Universal Launcher
pub struct CommandPalette {
    pub apps: Vec<LauncherEntry>,
    pub system_actions: Vec<SystemAction>,
    pub open_windows: Vec<WindowEntry>,
    pub clipboard_history: VecDeque<ClipboardSnippet>,
    pub max_clipboard_entries: usize,
    pub clipboard_history_enabled: bool,
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
            clipboard_history_enabled: false,
            next_clipboard_id: 1,
            is_open: false,
        };
        palette.register_default_system_actions();
        palette
    }

    /// Register default Omarchy-style system power and desktop commands
    fn register_default_system_actions(&mut self) {
        self.register_action(
            SystemActionKind::LockScreen,
            ":lock",
            "Lock the session when a supported lock service is available",
            "system-lock-screen",
        );
        self.register_action(
            SystemActionKind::Restart,
            ":reboot",
            "Restart the system (requires confirmation and a supported power service)",
            "system-reboot",
        );
        self.register_action(
            SystemActionKind::PowerOff,
            ":poweroff",
            "Shut down the system (requires confirmation and a supported power service)",
            "system-shutdown",
        );
        self.register_action(
            SystemActionKind::Suspend,
            ":suspend",
            "Suspend the system when a supported power service is available",
            "system-suspend",
        );
        self.register_action(
            SystemActionKind::Screenshot,
            ":screenshot",
            "Capture the screen when a supported capture service is available",
            "camera-photo",
        );
        self.register_action(
            SystemActionKind::ReloadTheme,
            ":reload-theme",
            "Reload the theme when a supported settings service is available",
            "preferences-desktop-theme",
        );
        self.register_action(
            SystemActionKind::OpenTerminal,
            ":terminal",
            "Open a terminal when an installed terminal is available",
            "utilities-terminal",
        );
        self.register_action(
            SystemActionKind::GamingProfile,
            ":gaming",
            "Request a gaming power profile when supported",
            "applications-games",
        );
        self.register_action(
            SystemActionKind::PowerSaveProfile,
            ":powersave",
            "Request a power-saving profile when supported",
            "battery-good",
        );
        self.register_action(
            SystemActionKind::CreateSnapshot,
            ":snapshot",
            "Create a restore snapshot when a supported storage service is available",
            "system-software-update",
        );
        self.register_action(
            SystemActionKind::ExportBackup,
            ":backup",
            "Export a backup when a supported backup service is available",
            "document-save",
        );
        self.register_action(
            SystemActionKind::SystemInformation,
            ":system-info",
            "Show system information when a supported report service is available",
            "computer",
        );
    }

    /// Register an application entry
    pub fn register(&mut self, entry: LauncherEntry) {
        self.apps.push(entry);
    }

    /// Register a system action
    pub fn register_action(
        &mut self,
        kind: SystemActionKind,
        trigger: &str,
        desc: &str,
        icon: &str,
    ) {
        self.system_actions.push(SystemAction {
            kind,
            trigger: trigger.to_string(),
            description: desc.to_string(),
            icon: icon.to_string(),
            available: false,
        });
    }

    pub fn set_system_action_available(&mut self, kind: SystemActionKind, available: bool) {
        if let Some(action) = self
            .system_actions
            .iter_mut()
            .find(|action| action.kind == kind)
        {
            action.available = available;
        }
    }

    /// Execute only through an explicit service handler. The handler must
    /// independently report availability before it can receive the action.
    pub fn execute_system_action<H: SystemActionHandler>(
        &self,
        kind: SystemActionKind,
        confirmed: bool,
        handler: &mut H,
    ) -> Result<(), SystemActionError> {
        let action = self
            .system_actions
            .iter()
            .find(|action| action.kind == kind)
            .ok_or(SystemActionError::UnknownAction)?;
        if !action.available || !handler.is_available(kind) {
            return Err(SystemActionError::Unavailable);
        }
        if kind.requires_confirmation() && !confirmed {
            return Err(SystemActionError::ConfirmationRequired);
        }
        handler.execute(kind).map_err(SystemActionError::Backend)
    }

    pub fn launch_application<H: ApplicationLaunchHandler>(
        &self,
        executable: &str,
        handler: &mut H,
    ) -> Result<(), SystemActionError> {
        let app = self
            .apps
            .iter()
            .find(|app| app.exec_path == executable)
            .ok_or(SystemActionError::UnknownAction)?;
        if !app.is_installed || !handler.is_installed(&app.exec_path) {
            return Err(SystemActionError::Unavailable);
        }
        handler
            .launch(&app.exec_path)
            .map_err(SystemActionError::Backend)
    }

    /// Update running window list from compositor
    pub fn sync_open_windows(&mut self, windows: Vec<WindowEntry>) {
        self.open_windows = windows;
    }

    /// Push text to clipboard history
    pub fn push_clipboard(&mut self, content: &str, timestamp: u64) {
        if !self.clipboard_history_enabled
            || self.max_clipboard_entries == 0
            || content.trim().is_empty()
        {
            return;
        }
        while self.clipboard_history.len() >= self.max_clipboard_entries {
            self.clipboard_history.pop_back();
        }
        let snippet_id = self.next_clipboard_id;
        let Some(next_id) = self.next_clipboard_id.checked_add(1) else {
            return;
        };
        self.next_clipboard_id = next_id;
        self.clipboard_history.push_front(ClipboardSnippet {
            snippet_id,
            content: content.to_string(),
            timestamp,
        });
    }

    pub fn clear_clipboard_history(&mut self) {
        self.clipboard_history.clear();
    }

    /// Clipboard history is opt-in because snippets can contain private text.
    /// Disabling it also clears previously collected data.
    pub fn set_clipboard_history_enabled(&mut self, enabled: bool) {
        self.clipboard_history_enabled = enabled;
        if !enabled {
            self.clear_clipboard_history();
        }
    }

    /// Unified fuzzy search across all active modes
    pub fn query(&self, input: &str) -> Vec<SearchResultItem> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            // Return top frequent apps if no query
            let mut apps: Vec<&LauncherEntry> = self.apps.iter().collect();
            apps.sort_by(|left, right| {
                right
                    .launch_count
                    .cmp(&left.launch_count)
                    .then_with(|| left.name.cmp(&right.name))
            });
            return apps
                .into_iter()
                .take(8)
                .map(|app| SearchResultItem {
                    title: app.name.clone(),
                    subtitle: app.category.clone(),
                    icon: app.icon.clone(),
                    action_payload: app.exec_path.clone(),
                    mode: LauncherMode::Application,
                    score: 100_i32.saturating_add(app.launch_count.min(i32::MAX as u32) as i32),
                    enabled: app.is_installed,
                    requires_confirmation: false,
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
                    enabled: true,
                    requires_confirmation: false,
                });
            }
        }

        // 2. System Actions: triggered if starts with ":" or matches action triggers
        if trimmed.starts_with(':') {
            let action_q = trimmed.to_lowercase();
            for act in &self.system_actions {
                // Performance: zero-allocation matching eliminates transient String heap allocations on every keypress
                if contains_ignore_case(&act.trigger, &action_q)
                    || contains_ignore_case(&act.description, &action_q)
                {
                    results.push(SearchResultItem {
                        title: act.trigger.clone(),
                        subtitle: act.description.clone(),
                        icon: act.icon.clone(),
                        action_payload: act.kind.id().to_string(),
                        mode: LauncherMode::SystemAction,
                        score: 5_000,
                        enabled: act.available,
                        requires_confirmation: act.kind.requires_confirmation(),
                    });
                }
            }
        }

        // 3. Window Switcher: search open running windows
        let q_lower = trimmed.to_lowercase();
        for win in &self.open_windows {
            // Performance: zero-allocation matching avoids lowercasing title and app_class in loops
            if contains_ignore_case(&win.title, &q_lower)
                || contains_ignore_case(&win.app_class, &q_lower)
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
                    enabled: true,
                    requires_confirmation: false,
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
                // Performance: zero-allocation matching avoids lowercasing content in loops
                if contains_ignore_case(&snippet.content, &cb_q) {
                    let preview = clipboard_preview(&snippet.content, 60);
                    results.push(SearchResultItem {
                        title: preview,
                        subtitle: "Paste from Clipboard History".to_string(),
                        icon: "edit-paste".to_string(),
                        action_payload: snippet.content.clone(),
                        mode: LauncherMode::Clipboard,
                        score: 1_500,
                        enabled: true,
                        requires_confirmation: false,
                    });
                }
            }
        }

        // 5. Application search
        for app in &self.apps {
            let mut score: i32 = -1;

            // Performance: zero-allocation matching avoids lowercasing app name and keywords in loops
            if app.name.eq_ignore_ascii_case(&q_lower) {
                score = 1000;
            } else if starts_with_ignore_case(&app.name, &q_lower) {
                score = 800;
            } else if contains_ignore_case(&app.name, &q_lower) {
                score = 500;
            } else if app
                .keywords
                .iter()
                .any(|k| contains_ignore_case(k, &q_lower))
            {
                score = 300;
            }

            if score > 0 {
                score = score.saturating_add(
                    (app.launch_count.min(i32::MAX as u32) as i32).saturating_mul(5),
                );
                results.push(SearchResultItem {
                    title: app.name.clone(),
                    subtitle: app.category.clone(),
                    icon: app.icon.clone(),
                    action_payload: app.exec_path.clone(),
                    mode: LauncherMode::Application,
                    score,
                    enabled: app.is_installed,
                    requires_confirmation: false,
                });
            }
        }

        // Sort descending by score
        results.sort_by(|a, b| b.score.cmp(&a.score));
        results
    }

    /// Legacy fuzzy search compatibility wrapper
    pub fn fuzzy_search(&self, query: &str) -> Vec<&LauncherEntry> {
        let q = query.to_lowercase();
        let mut matched: Vec<(&LauncherEntry, usize)> = self
            .apps
            .iter()
            .filter_map(|e| {
                if contains_ignore_case(&e.name, &q) {
                    let pos = e
                        .name
                        .as_bytes()
                        .windows(q.len())
                        .position(|w| {
                            w.iter()
                                .zip(q.as_bytes())
                                .all(|(&b1, &b2)| b1.to_ascii_lowercase() == b2)
                        })
                        .unwrap_or(usize::MAX);
                    Some((e, pos))
                } else if e.keywords.iter().any(|k| contains_ignore_case(k, &q)) {
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
        let trimmed = expr.trim();
        if trimmed.is_empty() {
            return None;
        }
        if trimmed.as_bytes().iter().any(|b| b.is_ascii_whitespace()) {
            let clean: String = trimmed.chars().filter(|c| !c.is_whitespace()).collect();
            return Self::evaluate_math_expr_clean(&clean);
        }
        Self::evaluate_math_expr_clean(trimmed)
    }

    /// Internal helper that operates on whitespace-free expressions without allocations during recursion
    fn evaluate_math_expr_clean(clean: &str) -> Option<f64> {
        if clean.is_empty() {
            return None;
        }

        // Simple binary operation parser (+, -, *, /)
        if let Some(pos) = clean.rfind('+') {
            let left = Self::evaluate_math_expr_clean(&clean[..pos])?;
            let right = Self::evaluate_math_expr_clean(&clean[pos + 1..])?;
            return Some(left + right);
        }
        if let Some(pos) = clean.rfind('-') {
            if pos > 0 {
                // Avoid unary minus
                let left = Self::evaluate_math_expr_clean(&clean[..pos])?;
                let right = Self::evaluate_math_expr_clean(&clean[pos + 1..])?;
                return Some(left - right);
            }
        }
        if let Some(pos) = clean.rfind('*') {
            let left = Self::evaluate_math_expr_clean(&clean[..pos])?;
            let right = Self::evaluate_math_expr_clean(&clean[pos + 1..])?;
            return Some(left * right);
        }
        if let Some(pos) = clean.rfind('/') {
            let left = Self::evaluate_math_expr_clean(&clean[..pos])?;
            let right = Self::evaluate_math_expr_clean(&clean[pos + 1..])?;
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

/// Perform zero-allocation ASCII case-insensitive substring search.
/// `needle_lower` is expected to be lowercased by the caller.
#[inline]
pub fn contains_ignore_case(haystack: &str, needle_lower: &str) -> bool {
    if needle_lower.is_empty() {
        return true;
    }
    if needle_lower.len() > haystack.len() {
        return false;
    }
    haystack.as_bytes().windows(needle_lower.len()).any(|window| {
        window
            .iter()
            .zip(needle_lower.as_bytes())
            .all(|(&b1, &b2)| b1.to_ascii_lowercase() == b2)
    })
}

/// Perform zero-allocation ASCII case-insensitive prefix match.
/// `needle_lower` is expected to be lowercased by the caller.
#[inline]
pub fn starts_with_ignore_case(haystack: &str, needle_lower: &str) -> bool {
    if needle_lower.len() > haystack.len() {
        return false;
    }
    haystack.as_bytes()[..needle_lower.len()]
        .iter()
        .zip(needle_lower.as_bytes())
        .all(|(&b1, &b2)| b1.to_ascii_lowercase() == b2)
}

fn clipboard_preview(content: &str, max_bytes: usize) -> String {
    if content.len() <= max_bytes {
        return content.to_string();
    }
    let mut end = max_bytes.min(content.len());
    while !content.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…", &content[..end])
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

    struct TestActionHandler {
        available: bool,
        calls: Vec<SystemActionKind>,
    }

    impl SystemActionHandler for TestActionHandler {
        fn is_available(&self, _action: SystemActionKind) -> bool {
            self.available
        }

        fn execute(&mut self, action: SystemActionKind) -> Result<(), String> {
            self.calls.push(action);
            Ok(())
        }
    }

    struct TestApplicationHandler {
        installed: bool,
        launches: Vec<String>,
    }

    impl ApplicationLaunchHandler for TestApplicationHandler {
        fn is_installed(&self, _executable: &str) -> bool {
            self.installed
        }

        fn launch(&mut self, executable: &str) -> Result<(), String> {
            self.launches.push(executable.to_string());
            Ok(())
        }
    }

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
            is_installed: true,
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
        palette.set_clipboard_history_enabled(true);
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
    fn system_actions_are_typed_unavailable_and_require_confirmation() {
        let mut palette = CommandPalette::new();
        let results = palette.query(":poweroff");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].action_payload, "power-off");
        assert!(!results[0].enabled);
        assert!(results[0].requires_confirmation);

        let mut handler = TestActionHandler {
            available: true,
            calls: Vec::new(),
        };
        assert_eq!(
            palette.execute_system_action(SystemActionKind::PowerOff, true, &mut handler),
            Err(SystemActionError::Unavailable)
        );
        palette.set_system_action_available(SystemActionKind::PowerOff, true);
        assert_eq!(
            palette.execute_system_action(SystemActionKind::PowerOff, false, &mut handler),
            Err(SystemActionError::ConfirmationRequired)
        );
        assert!(handler.calls.is_empty());
        assert_eq!(
            palette.execute_system_action(SystemActionKind::PowerOff, true, &mut handler),
            Ok(())
        );
        assert_eq!(handler.calls, vec![SystemActionKind::PowerOff]);
    }

    #[test]
    fn clipboard_ids_stay_unique_after_eviction_and_clear() {
        let mut palette = CommandPalette::new();
        palette.set_clipboard_history_enabled(true);
        palette.max_clipboard_entries = 1;
        palette.push_clipboard("first", 1);
        palette.push_clipboard("second", 2);
        let second_id = palette.clipboard_history.front().unwrap().snippet_id;
        palette.clear_clipboard_history();
        palette.push_clipboard("third", 3);
        assert!(palette.clipboard_history.front().unwrap().snippet_id > second_id);

        palette.max_clipboard_entries = 0;
        palette.push_clipboard("ignored", 4);
        assert_eq!(palette.clipboard_history.len(), 1);
    }

    #[test]
    fn clipboard_preview_respects_utf8_boundaries() {
        let preview = clipboard_preview("ééé", 3);
        assert_eq!(preview, "é…");
    }

    #[test]
    fn empty_query_orders_frequent_apps_with_stable_ties() {
        let mut palette = CommandPalette::new();
        for (name, launches) in [("Zulu", 3), ("Alpha", 3), ("Often", 9)] {
            palette.register(LauncherEntry {
                name: name.into(),
                exec_path: format!("/apps/{name}"),
                icon: "app".into(),
                category: "Test".into(),
                keywords: Vec::new(),
                launch_count: launches,
                is_installed: true,
            });
        }
        let results = palette.query("");
        let names: Vec<&str> = results.iter().map(|item| item.title.as_str()).collect();
        assert_eq!(names, vec!["Often", "Alpha", "Zulu"]);
    }

    #[test]
    fn app_launch_uses_verified_inventory_and_process_backend() {
        let mut palette = CommandPalette::new();
        palette.register(LauncherEntry {
            name: "Editor".into(),
            exec_path: "/apps/editor".into(),
            icon: "editor".into(),
            category: "Development".into(),
            keywords: Vec::new(),
            launch_count: 0,
            is_installed: false,
        });
        let mut backend = TestApplicationHandler {
            installed: true,
            launches: Vec::new(),
        };
        assert_eq!(
            palette.launch_application("/apps/editor", &mut backend),
            Err(SystemActionError::Unavailable)
        );
        assert!(backend.launches.is_empty());

        palette.apps[0].is_installed = true;
        assert_eq!(
            palette.launch_application("/apps/editor", &mut backend),
            Ok(())
        );
        assert_eq!(backend.launches, vec!["/apps/editor"]);
    }

    #[test]
    fn clipboard_history_is_opt_in_and_disable_clears_private_text() {
        let mut palette = CommandPalette::new();
        palette.push_clipboard("secret", 1);
        assert!(palette.clipboard_history.is_empty());
        palette.set_clipboard_history_enabled(true);
        palette.push_clipboard("secret", 2);
        assert_eq!(palette.clipboard_history.len(), 1);
        palette.set_clipboard_history_enabled(false);
        assert!(palette.clipboard_history.is_empty());
    }

    #[test]
    fn test_zero_allocation_case_insensitive_matching() {
        assert!(contains_ignore_case("Firefox Web Browser", "firefox"));
        assert!(contains_ignore_case("SigmaOS Terminal", "term"));
        assert!(!contains_ignore_case("SigmaOS", "nonexistent"));

        assert!(starts_with_ignore_case("Calculator", "calc"));
        assert!(!starts_with_ignore_case("Calculator", "lator"));
    }
}
