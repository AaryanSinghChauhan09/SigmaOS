// SPDX-License-Identifier: MIT
// Elementary OS GitHub Inspired Components Suite
// (`src/desktop/elementary_os_github_inspired_components.rs`)
//
// Implements missing desktop components inspired by official elementary GitHub repositories
// including elementary/granite, elementary/pantheon, elementary/code, elementary/files,
// elementary/tasks, and elementary/stylesheet.

use std::collections::BTreeMap;
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. Granite HeaderBar & Dynamic Style Suite (`elementary/granite`)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraniteAccentColor {
    Strawberry,
    Orange,
    Banana,
    Lime,
    Mint,
    Grape,
    Bubblegum,
    Slate,
    Cocoa,
}

impl GraniteAccentColor {
    pub fn to_hex(&self) -> &'static str {
        match self {
            GraniteAccentColor::Strawberry => "#e63333",
            GraniteAccentColor::Orange => "#f37329",
            GraniteAccentColor::Banana => "#f8c21a",
            GraniteAccentColor::Lime => "#68b723",
            GraniteAccentColor::Mint => "#2bb274",
            GraniteAccentColor::Grape => "#824bc8",
            GraniteAccentColor::Bubblegum => "#e85292",
            GraniteAccentColor::Slate => "#64788a",
            GraniteAccentColor::Cocoa => "#7d5d4d",
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeaderBarWidgetSpec {
    pub widget_id: String,
    pub title: String,
    pub show_back_button: bool,
    pub action_buttons_count: usize,
}

pub struct GraniteHeaderBarAndStyleSuite {
    pub current_accent_color: GraniteAccentColor,
    pub is_dark_style: bool,
    pub header_bars: BTreeMap<String, HeaderBarWidgetSpec>,
    pub css_rules_count: usize,
}

impl GraniteHeaderBarAndStyleSuite {
    pub fn new() -> Self {
        let mut suite = Self {
            current_accent_color: GraniteAccentColor::Mint,
            is_dark_style: false,
            header_bars: BTreeMap::new(),
            css_rules_count: 120,
        };
        suite.register_header_bar("app-main", "Elementary App", false, 3);
        suite
    }

    pub fn set_accent_color(&mut self, color: GraniteAccentColor) {
        self.current_accent_color = color;
    }

    pub fn toggle_dark_style(&mut self) -> bool {
        self.is_dark_style = !self.is_dark_style;
        self.is_dark_style
    }

    pub fn register_header_bar(&mut self, id: &str, title: &str, back_btn: bool, actions: usize) {
        self.header_bars.insert(
            id.to_string(),
            HeaderBarWidgetSpec {
                widget_id: id.to_string(),
                title: title.to_string(),
                show_back_button: back_btn,
                action_buttons_count: actions,
            },
        );
    }
}

impl Default for GraniteHeaderBarAndStyleSuite {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 2. Pantheon Files Tabbed Navigation & Cloud Sync (`elementary/files`)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileColorTag {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Gray,
}

#[derive(Debug, Clone)]
pub struct FileTabState {
    pub tab_id: u32,
    pub path: String,
    pub active_view_mode: String, // "grid", "list", "miller-columns"
}

pub struct PantheonFilesTabAndCloudSyncEngine {
    pub tabs: BTreeMap<u32, FileTabState>,
    pub active_tab_id: u32,
    pub cloud_shares_connected: Vec<String>,
    pub color_tags: BTreeMap<String, FileColorTag>,
    pub next_tab_id: u32,
}

impl PantheonFilesTabAndCloudSyncEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            tabs: BTreeMap::new(),
            active_tab_id: 1,
            cloud_shares_connected: vec!["Nextcloud-Personal".to_string()],
            color_tags: BTreeMap::new(),
            next_tab_id: 1,
        };
        engine.open_tab("/home/user/Documents", "grid");
        engine
    }

    pub fn open_tab(&mut self, path: &str, view_mode: &str) -> u32 {
        let id = self.next_tab_id;
        self.next_tab_id += 1;
        self.tabs.insert(
            id,
            FileTabState {
                tab_id: id,
                path: path.to_string(),
                active_view_mode: view_mode.to_string(),
            },
        );
        self.active_tab_id = id;
        id
    }

    pub fn tag_file(&mut self, filepath: &str, tag: FileColorTag) {
        self.color_tags.insert(filepath.to_string(), tag);
    }

    pub fn connect_cloud_share(&mut self, provider_name: &str) {
        if !self
            .cloud_shares_connected
            .contains(&provider_name.to_string())
        {
            self.cloud_shares_connected.push(provider_name.to_string());
        }
    }
}

impl Default for PantheonFilesTabAndCloudSyncEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. Elementary Code LSP & Syntax Engine (`elementary/code`)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitGutterState {
    Unchanged,
    Added,
    Modified,
    Deleted,
}

#[derive(Debug, Clone)]
pub struct CodeDocumentBuffer {
    pub doc_id: u32,
    pub filename: String,
    pub language_id: String, // "vala", "rust", "c", "lisp"
    pub total_lines: usize,
    pub git_gutter_status: Vec<GitGutterState>,
}

pub struct ElementaryCodeLspAndSyntaxEngine {
    pub buffers: BTreeMap<u32, CodeDocumentBuffer>,
    pub lsp_servers_running: Vec<String>,
    pub next_doc_id: u32,
}

impl ElementaryCodeLspAndSyntaxEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            buffers: BTreeMap::new(),
            lsp_servers_running: vec!["vala-language-server".to_string(), "rust-analyzer".to_string()],
            next_doc_id: 1,
        };
        engine.open_buffer("main.rs", "rust", 100);
        engine
    }

    pub fn open_buffer(&mut self, filename: &str, lang: &str, lines: usize) -> u32 {
        let id = self.next_doc_id;
        self.next_doc_id += 1;
        let mut gutter = vec![GitGutterState::Unchanged; lines];
        if lines > 0 {
            gutter[0] = GitGutterState::Modified;
        }
        self.buffers.insert(
            id,
            CodeDocumentBuffer {
                doc_id: id,
                filename: filename.to_string(),
                language_id: lang.to_string(),
                total_lines: lines,
                git_gutter_status: gutter,
            },
        );
        id
    }

    pub fn trigger_lsp_completion(&self, doc_id: u32, _line: usize, _col: usize) -> Vec<String> {
        if let Some(buf) = self.buffers.get(&doc_id) {
            match buf.language_id.as_str() {
                "rust" => vec!["fn".to_string(), "struct".to_string(), "let mut".to_string()],
                "vala" => vec!["public class".to_string(), "signal".to_string(), "Object".to_string()],
                _ => vec!["item".to_string()],
            }
        } else {
            Vec::new()
        }
    }
}

impl Default for ElementaryCodeLspAndSyntaxEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 4. Elementary Tasks & Calendar Sync (`elementary/tasks` & `elementary/calendar`)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone)]
pub struct TaskItem {
    pub task_id: u32,
    pub title: String,
    pub priority: TaskPriority,
    pub completed: bool,
    pub due_timestamp: Option<u64>,
}

pub struct ElementaryTasksAndCalendarSyncEngine {
    pub tasks: BTreeMap<u32, TaskItem>,
    pub caldav_sync_sources: Vec<String>,
    pub next_task_id: u32,
}

impl ElementaryTasksAndCalendarSyncEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            tasks: BTreeMap::new(),
            caldav_sync_sources: vec!["https://caldav.example.com/tasks/".to_string()],
            next_task_id: 1,
        };
        engine.add_task("Review SigmaOS Pull Requests", TaskPriority::Urgent, Some(1700000000));
        engine.add_task("Test Elementary Pantheon Components", TaskPriority::High, None);
        engine
    }

    pub fn add_task(&mut self, title: &str, priority: TaskPriority, due: Option<u64>) -> u32 {
        let id = self.next_task_id;
        self.next_task_id += 1;
        self.tasks.insert(
            id,
            TaskItem {
                task_id: id,
                title: title.to_string(),
                priority,
                completed: false,
                due_timestamp: due,
            },
        );
        id
    }

    pub fn complete_task(&mut self, task_id: u32) -> bool {
        if let Some(task) = self.tasks.get_mut(&task_id) {
            task.completed = true;
            true
        } else {
            false
        }
    }
}

impl Default for ElementaryTasksAndCalendarSyncEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 5. Pantheon Touchpad Gesture & Shortcut Engine (`elementary/shortcut-overlay`)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GestureType {
    SwipeThreeFingersLeft,
    SwipeThreeFingersRight,
    SwipeFourFingersUp, // Multitasking view
    PinchFourFingersIn,
}

pub struct PantheonShortcutAndGestureEngine {
    pub gesture_bindings: BTreeMap<String, String>, // gesture_name -> action
    pub keyboard_shortcuts: BTreeMap<String, String>, // "Super+A" -> "applications_menu"
    pub gesture_events_recognized: u64,
}

impl PantheonShortcutAndGestureEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            gesture_bindings: BTreeMap::new(),
            keyboard_shortcuts: BTreeMap::new(),
            gesture_events_recognized: 0,
        };
        engine.register_gesture(GestureType::SwipeFourFingersUp, "toggle_multitasking_view");
        engine.register_gesture(GestureType::SwipeThreeFingersLeft, "switch_workspace_left");
        engine.register_gesture(GestureType::SwipeThreeFingersRight, "switch_workspace_right");

        engine.keyboard_shortcuts.insert("Super+A".to_string(), "open_app_launcher".to_string());
        engine.keyboard_shortcuts.insert("Super+Tab".to_string(), "switch_windows".to_string());
        engine.keyboard_shortcuts.insert("Super+Space".to_string(), "shortcut_overlay".to_string());
        engine
    }

    pub fn register_gesture(&mut self, gesture: GestureType, action: &str) {
        let key = format!("{:?}", gesture);
        self.gesture_bindings.insert(key, action.to_string());
    }

    pub fn process_gesture_event(&mut self, gesture: GestureType) -> Option<String> {
        self.gesture_events_recognized += 1;
        let key = format!("{:?}", gesture);
        self.gesture_bindings.get(&key).cloned()
    }
}

impl Default for PantheonShortcutAndGestureEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MASTER PR SUITE FOR ELEMENTARY OS GITHUB INSPIRATIONS
// ============================================================================

pub struct ElementaryOsGithubInspirationsPrSuite {
    pub granite_style: GraniteHeaderBarAndStyleSuite,
    pub pantheon_files: PantheonFilesTabAndCloudSyncEngine,
    pub code_lsp: ElementaryCodeLspAndSyntaxEngine,
    pub tasks_cal: ElementaryTasksAndCalendarSyncEngine,
    pub gestures: PantheonShortcutAndGestureEngine,
}

impl ElementaryOsGithubInspirationsPrSuite {
    pub fn new() -> Self {
        Self {
            granite_style: GraniteHeaderBarAndStyleSuite::new(),
            pantheon_files: PantheonFilesTabAndCloudSyncEngine::new(),
            code_lsp: ElementaryCodeLspAndSyntaxEngine::new(),
            tasks_cal: ElementaryTasksAndCalendarSyncEngine::new(),
            gestures: PantheonShortcutAndGestureEngine::new(),
        }
    }

    pub fn run_all_elementary_checks(&mut self) -> bool {
        self.granite_style.set_accent_color(GraniteAccentColor::Strawberry);
        assert_eq!(self.granite_style.current_accent_color.to_hex(), "#e63333");
        assert!(self.granite_style.toggle_dark_style());

        let tab_id = self.pantheon_files.open_tab("/home/user/Pictures", "miller-columns");
        self.pantheon_files.tag_file("/home/user/Pictures/photo.jpg", FileColorTag::Blue);
        self.pantheon_files.connect_cloud_share("Dropbox-Personal");
        assert_eq!(self.pantheon_files.active_tab_id, tab_id);

        let doc_id = self.code_lsp.open_buffer("app.vala", "vala", 50);
        let completions = self.code_lsp.trigger_lsp_completion(doc_id, 10, 5);
        assert!(!completions.is_empty());

        let task_id = self.tasks_cal.add_task("Deploy Elementary Suite to SigmaOS", TaskPriority::Urgent, None);
        assert!(self.tasks_cal.complete_task(task_id));

        let action = self.gestures.process_gesture_event(GestureType::SwipeFourFingersUp);
        assert_eq!(action, Some("toggle_multitasking_view".to_string()));

        true
    }
}

impl Default for ElementaryOsGithubInspirationsPrSuite {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// STANDALONE UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_granite_style_suite() {
        let mut suite = GraniteHeaderBarAndStyleSuite::new();
        suite.set_accent_color(GraniteAccentColor::Lime);
        assert_eq!(suite.current_accent_color.to_hex(), "#68b723");
        assert_eq!(suite.header_bars.len(), 1);
        assert!(suite.toggle_dark_style());
    }

    #[test]
    fn test_pantheon_files_sync() {
        let mut files = PantheonFilesTabAndCloudSyncEngine::new();
        let tab2 = files.open_tab("/tmp", "list");
        assert_eq!(tab2, 2);
        files.tag_file("/tmp/file.txt", FileColorTag::Orange);
        assert_eq!(files.color_tags.len(), 1);
        files.connect_cloud_share("GoogleDrive");
        assert_eq!(files.cloud_shares_connected.len(), 2);
    }

    #[test]
    fn test_elementary_code_lsp() {
        let mut code = ElementaryCodeLspAndSyntaxEngine::new();
        let doc = code.open_buffer("lib.rs", "rust", 200);
        let completions = code.trigger_lsp_completion(doc, 1, 1);
        assert!(completions.contains(&"fn".to_string()));
    }

    #[test]
    fn test_elementary_tasks_calendar() {
        let mut tasks = ElementaryTasksAndCalendarSyncEngine::new();
        let id = tasks.add_task("Buy groceries", TaskPriority::Medium, None);
        assert!(tasks.complete_task(id));
        assert!(tasks.tasks.get(&id).unwrap().completed);
    }

    #[test]
    fn test_pantheon_gestures() {
        let mut gestures = PantheonShortcutAndGestureEngine::new();
        let act = gestures.process_gesture_event(GestureType::SwipeThreeFingersLeft);
        assert_eq!(act, Some("switch_workspace_left".to_string()));
    }

    #[test]
    fn test_master_elementary_pr_suite() {
        let mut suite = ElementaryOsGithubInspirationsPrSuite::new();
        assert!(suite.run_all_elementary_checks());
    }
}
