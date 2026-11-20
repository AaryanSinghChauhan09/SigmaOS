//! LSP client stub for SigmaOS-text-editor.

use crate::{EditorError, Result};

/// Connection state of the LSP server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LspState {
    Disconnected,
    Connecting,
    Ready,
}

/// A completion item from an LSP server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionItem {
    pub label: String,
    pub detail: Option<String>,
}

/// A diagnostic message from an LSP server.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub line: usize,
    pub message: String,
    pub severity: DiagnosticSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity { Error, Warning, Info, Hint }

/// LSP client (stub — real impl uses JSON-RPC over stdio/TCP).
#[derive(Debug)]
pub struct LspClient {
    pub state: LspState,
    pub server_name: String,
    pub mock_completions: Vec<CompletionItem>,
    pub mock_diagnostics: Vec<Diagnostic>,
}

impl LspClient {
    pub fn new(server_name: impl Into<String>) -> Self {
        Self {
            state: LspState::Disconnected,
            server_name: server_name.into(),
            mock_completions: Vec::new(),
            mock_diagnostics: Vec::new(),
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        self.state = LspState::Ready;
        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.state = LspState::Disconnected;
    }

    pub fn is_ready(&self) -> bool {
        self.state == LspState::Ready
    }

    pub fn completions(&self, _file: &str, _line: usize, _col: usize) -> Vec<CompletionItem> {
        if self.state == LspState::Ready {
            self.mock_completions.clone()
        } else {
            Vec::new()
        }
    }

    pub fn diagnostics(&self, _file: &str) -> Vec<Diagnostic> {
        if self.state == LspState::Ready {
            self.mock_diagnostics.clone()
        } else {
            Vec::new()
        }
    }
}
