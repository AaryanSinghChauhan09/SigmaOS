//! LSP client stub for SigmaOS-text-editor.

use lsp_types::{CompletionItem, Diagnostic, Position, Url};
use crate::{Result, EditorError};

/// Connection state of the LSP server.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LspState {
    Disconnected,
    Connecting,
    Ready,
}

/// LSP client (stub — real impl uses JSON-RPC over stdio/TCP).
#[derive(Debug)]
pub struct LspClient {
    pub state: LspState,
    pub server_name: String,
    /// Stubbed completions returned by the mock server.
    pub mock_completions: Vec<CompletionItem>,
    /// Stubbed diagnostics returned by the mock server.
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

    /// Request completions at a position (stub).
    pub fn completions(&self, _uri: &Url, _pos: Position) -> Vec<CompletionItem> {
        if self.state == LspState::Ready {
            self.mock_completions.clone()
        } else {
            Vec::new()
        }
    }

    /// Get cached diagnostics for a file (stub).
    pub fn diagnostics(&self, _uri: &Url) -> Vec<Diagnostic> {
        if self.state == LspState::Ready {
            self.mock_diagnostics.clone()
        } else {
            Vec::new()
        }
    }
}
