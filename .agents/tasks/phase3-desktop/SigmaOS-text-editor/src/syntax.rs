//! Tree-sitter syntax highlighting for SigmaOS-text-editor.

use std::collections::HashMap;

/// A syntax highlight span.
#[derive(Debug, Clone)]
pub struct HighlightSpan {
    pub start: usize,
    pub end: usize,
    pub scope: String, // e.g. "keyword", "string", "comment"
}

/// Language grammar identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LanguageId(pub String);

impl LanguageId {
    pub fn rust() -> Self { Self("rust".into()) }
    pub fn python() -> Self { Self("python".into()) }
    pub fn typescript() -> Self { Self("typescript".into()) }
}

/// Stub highlighter — real impl loads tree-sitter grammars and queries.
#[derive(Debug, Default)]
pub struct SyntaxHighlighter {
    language: Option<LanguageId>,
    pub last_highlights: Vec<HighlightSpan>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self { Self::default() }

    pub fn set_language(&mut self, lang: LanguageId) {
        self.language = Some(lang);
    }

    pub fn language(&self) -> Option<&LanguageId> {
        self.language.as_ref()
    }

    /// Compute highlights for the given source text.
    pub fn highlight(&mut self, source: &str) -> Vec<HighlightSpan> {
        // Stub: mark Rust keywords for demonstration.
        let keywords = ["fn", "let", "mut", "pub", "use", "struct", "enum", "impl"];
        let mut spans = Vec::new();
        for kw in keywords {
            let mut start = 0;
            while let Some(pos) = source[start..].find(kw) {
                let abs = start + pos;
                // Verify word boundary.
                let before_ok = abs == 0 || !source.as_bytes()[abs - 1].is_ascii_alphanumeric();
                let after_ok = abs + kw.len() >= source.len()
                    || !source.as_bytes()[abs + kw.len()].is_ascii_alphanumeric();
                if before_ok && after_ok {
                    spans.push(HighlightSpan { start: abs, end: abs + kw.len(), scope: "keyword".into() });
                }
                start += pos + kw.len();
            }
        }
        self.last_highlights = spans.clone();
        spans
    }
}
