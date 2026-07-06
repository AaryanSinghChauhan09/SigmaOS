// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/code/sigma_code.rs — Code Editor Suite
// VS Code OSS/Neovim-inspired development environment
//
// Features:
//   - Modern code editor with LSP support
//   - Integrated terminal
//   - Git integration
//   - Extension marketplace
//   - India context: Support for Indian programming languages and scripts
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Editor Configuration ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    pub font_family: String,
    pub font_size: u32,
    pub tab_size: u32,
    pub insert_spaces: bool,
    pub word_wrap: bool,
    pub line_numbers: bool,
    pub minimap: bool,
    pub auto_save: bool,
    pub auto_save_delay_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    HighContrast,
    Custom(String),
}

// ── Language Server Protocol (LSP) Support ───────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LSPClient {
    pub language_id: String,
    pub server_command: String,
    pub server_args: Vec<String>,
    pub enabled: bool,
    pub capabilities: LSPCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LSPCapabilities {
    pub completion: bool,
    pub hover: bool,
    pub signature_help: bool,
    pub definition: bool,
    pub type_definition: bool,
    pub implementation: bool,
    pub references: bool,
    pub document_highlight: bool,
    pub document_symbol: bool,
    pub workspace_symbol: bool,
    pub code_action: bool,
    pub code_lens: bool,
    pub formatting: bool,
    pub range_formatting: bool,
    pub on_type_formatting: bool,
    pub rename: bool,
    pub publish_diagnostics: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub sort_text: Option<String>,
    pub filter_text: Option<String>,
    pub insert_text: String,
    pub insert_text_format: InsertTextFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionItemKind {
    Text,
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Unit,
    Value,
    Enum,
    Keyword,
    Snippet,
    Color,
    File,
    Reference,
    Folder,
    EnumMember,
    Constant,
    Struct,
    Event,
    Operator,
    TypeParameter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InsertTextFormat {
    PlainText,
    Snippet,
}

// ── Integrated Terminal ─────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSession {
    pub id: String,
    pub shell: String,
    pub working_directory: String,
    pub env_vars: HashMap<String, String>,
    pub rows: u32,
    pub cols: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerminalShell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Cmd,
    Custom(String),
}

// ── Git Integration ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitStatus {
    pub branch: String,
    pub remote: Option<String>,
    pub ahead: u32,
    pub behind: u32,
    pub staged: Vec<GitFileStatus>,
    pub unstaged: Vec<GitFileStatus>,
    pub untracked: Vec<String>,
    pub conflicted: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitFileStatus {
    pub path: String,
    pub status: GitFileState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GitFileState {
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Unmerged,
}

// ── Extension System ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extension {
    pub id: String,
    pub name: String,
    pub publisher: String,
    pub version: String,
    pub description: String,
    pub categories: Vec<String>,
    pub installed: bool,
    pub enabled: bool,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionMarketplace {
    pub extensions: Vec<Extension>,
    pub featured: Vec<String>,
    pub search_index: HashMap<String, Vec<String>>,
}

// ── Indian Language Support ───────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndianLanguageSupport {
    pub language: String,
    pub script: String,
    pub ime_enabled: bool,
    pub spell_check_enabled: bool,
    pub comment_templates: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgrammingLanguage {
    pub name: String,
    pub file_extensions: Vec<String>,
    pub lsp_client: Option<LSPClient>,
    pub indian_support: Option<IndianLanguageSupport>,
}

// ── Editor State ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub path: String,
    pub language: String,
    pub content: String,
    pub version: u32,
    pub dirty: bool,
    pub encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub path: String,
    pub documents: Vec<Document>,
    pub open_documents: Vec<String>,
    pub active_document: Option<String>,
}

// ── Code Editor Engine ───────────────────────────────────────────────────

pub struct CodeEditorEngine {
    config: EditorConfig,
    theme: Theme,
    lsp_clients: HashMap<String, LSPClient>,
    terminal_sessions: Vec<TerminalSession>,
    git_status: Option<GitStatus>,
    extensions: Vec<Extension>,
    marketplace: ExtensionMarketplace,
    workspace: Option<Workspace>,
    programming_languages: HashMap<String, ProgrammingLanguage>,
}

impl CodeEditorEngine {
    pub fn new() -> Self {
        Self {
            config: EditorConfig {
                font_family: "Fira Code".to_string(),
                font_size: 14,
                tab_size: 4,
                insert_spaces: true,
                word_wrap: false,
                line_numbers: true,
                minimap: true,
                auto_save: true,
                auto_save_delay_ms: 1000,
            },
            theme: Theme::Dark,
            lsp_clients: HashMap::new(),
            terminal_sessions: Vec::new(),
            git_status: None,
            extensions: Vec::new(),
            marketplace: ExtensionMarketplace {
                extensions: Vec::new(),
                featured: Vec::new(),
                search_index: HashMap::new(),
            },
            workspace: None,
            programming_languages: HashMap::new(),
        }
    }

    /// Set editor configuration
    pub fn set_config(&mut self, config: EditorConfig) {
        self.config = config;
    }

    /// Get editor configuration
    pub fn get_config(&self) -> &EditorConfig {
        &self.config
    }

    /// Set theme
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    /// Get theme
    pub fn get_theme(&self) -> &Theme {
        &self.theme
    }

    /// Register LSP client
    pub fn register_lsp_client(&mut self, language_id: String, client: LSPClient) {
        self.lsp_clients.insert(language_id, client);
    }

    /// Get LSP client for language
    pub fn get_lsp_client(&self, language_id: &str) -> Option<&LSPClient> {
        self.lsp_clients.get(language_id)
    }

    /// Create terminal session
    pub fn create_terminal(&mut self, shell: TerminalShell, working_dir: String) -> Result<String, String> {
        let session_id = format!("term_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        
        let session = TerminalSession {
            id: session_id.clone(),
            shell: match shell {
                TerminalShell::Bash => "/bin/bash".to_string(),
                TerminalShell::Zsh => "/bin/zsh".to_string(),
                TerminalShell::Fish => "/usr/bin/fish".to_string(),
                TerminalShell::PowerShell => "pwsh".to_string(),
                TerminalShell::Cmd => "cmd.exe".to_string(),
                TerminalShell::Custom(s) => s,
            },
            working_directory: working_dir,
            env_vars: HashMap::new(),
            rows: 24,
            cols: 80,
        };
        
        self.terminal_sessions.push(session);
        Ok(session_id)
    }

    /// Get terminal sessions
    pub fn get_terminals(&self) -> &[TerminalSession] {
        &self.terminal_sessions
    }

    /// Update Git status
    pub fn update_git_status(&mut self, workspace_path: &str) -> Result<(), String> {
        // In production: Run git status command
        // For now: Set mock status
        self.git_status = Some(GitStatus {
            branch: "main".to_string(),
            remote: Some("origin/main".to_string()),
            ahead: 0,
            behind: 0,
            staged: Vec::new(),
            unstaged: Vec::new(),
            untracked: Vec::new(),
            conflicted: Vec::new(),
        });
        Ok(())
    }

    /// Get Git status
    pub fn get_git_status(&self) -> Option<&GitStatus> {
        self.git_status.as_ref()
    }

    /// Install extension
    pub fn install_extension(&mut self, extension_id: &str) -> Result<(), String> {
        // In production: Download and install extension
        // For now: Mark as installed
        if let Some(ext) = self.marketplace.extensions.iter_mut().find(|e| e.id == extension_id) {
            ext.installed = true;
            ext.enabled = true;
            self.extensions.push(ext.clone());
            Ok(())
        } else {
            Err("Extension not found".to_string())
        }
    }

    /// Uninstall extension
    pub fn uninstall_extension(&mut self, extension_id: &str) -> Result<(), String> {
        self.extensions.retain(|e| e.id != extension_id);
        if let Some(ext) = self.marketplace.extensions.iter_mut().find(|e| e.id == extension_id) {
            ext.installed = false;
            ext.enabled = false;
        }
        Ok(())
    }

    /// Search extensions
    pub fn search_extensions(&self, query: &str) -> Vec<&Extension> {
        self.marketplace.extensions.iter()
            .filter(|e| {
                e.name.to_lowercase().contains(&query.to_lowercase()) ||
                e.description.to_lowercase().contains(&query.to_lowercase()) ||
                e.categories.iter().any(|c| c.to_lowercase().contains(&query.to_lowercase()))
            })
            .collect()
    }

    /// Open workspace
    pub fn open_workspace(&mut self, path: String) -> Result<(), String> {
        // In production: Scan directory for files
        self.workspace = Some(Workspace {
            path,
            documents: Vec::new(),
            open_documents: Vec::new(),
            active_document: None,
        });
        Ok(())
    }

    /// Get workspace
    pub fn get_workspace(&self) -> Option<&Workspace> {
        self.workspace.as_ref()
    }

    /// Register programming language
    pub fn register_language(&mut self, language: ProgrammingLanguage) {
        self.programming_languages.insert(language.name.clone(), language);
    }

    /// Get programming language
    pub fn get_language(&self, name: &str) -> Option<&ProgrammingLanguage> {
        self.programming_languages.get(name)
    }

    /// Get completion items
    pub fn get_completions(&self, document_id: &str, position: (u32, u32)) -> Vec<CompletionItem> {
        // In production: Request completions from LSP
        // For now: Return empty
        Vec::new()
    }
}

impl Default for CodeEditorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn code_editor_create() -> *mut CodeEditorEngine {
    Box::into_raw(Box::new(CodeEditorEngine::new()))
}

#[no_mangle]
pub extern "C" fn code_editor_destroy(editor: *mut CodeEditorEngine) {
    unsafe {
        if !editor.is_null() {
            let _ = Box::from_raw(editor);
        }
    }
}

#[no_mangle]
pub extern "C" fn code_editor_open_workspace(editor: *mut CodeEditorEngine,
                                            path: *const u8, path_len: usize) -> i32 {
    unsafe {
        if editor.is_null() || path.is_null() { return -1; }
        let path = String::from_utf8_unchecked(
            std::slice::from_raw_parts(path, path_len));
        match (*editor).open_workspace(path) {
            Ok(_) => 0,
            Err(_) => -1,
        }
    }
}

#[no_mangle]
pub extern "C" fn code_editor_create_terminal(editor: *mut CodeEditorEngine,
                                             shell: i32,
                                             working_dir: *const u8, dir_len: usize,
                                             out_id: *mut u8, id_len: usize) -> i32 {
    unsafe {
        if editor.is_null() || working_dir.is_null() { return -1; }
        let working_dir = String::from_utf8_unchecked(
            std::slice::from_raw_parts(working_dir, dir_len));
        let shell = match shell {
            0 => TerminalShell::Bash,
            1 => TerminalShell::Zsh,
            2 => TerminalShell::Fish,
            3 => TerminalShell::PowerShell,
            _ => TerminalShell::Bash,
        };
        match (*editor).create_terminal(shell, working_dir) {
            Ok(id) => {
                let bytes = id.as_bytes();
                let copy_len = std::cmp::min(bytes.len(), id_len);
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out_id, copy_len);
                copy_len as i32
            }
            Err(_) => -1,
        }
    }
}
