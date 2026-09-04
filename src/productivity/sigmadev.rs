// SigmaDev IDE: Zero-dependency, lightweight code editor optimized for
// Rust, Zig, and Nim with sandboxed AI code completion assistants for SigmaOS.

use std::string::String;
use std::string::ToString;
use std::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedLanguage {
    Rust,
    Zig,
    Nim,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: String,
    pub language: SupportedLanguage,
    pub content: String,
    pub cursor_line: usize,
}

pub struct SigmaDevIde {
    pub open_files: Vec<SourceFile>,
    pub active_file_idx: Option<usize>,
    pub is_ai_completion_enabled: bool,
}

impl SigmaDevIde {
    pub fn new() -> Self {
        Self {
            open_files: Vec::new(),
            active_file_idx: None,
            is_ai_completion_enabled: true,
        }
    }

    pub fn open_file(&mut self, path: &str, content: &str) {
        let lang = if path.ends_with(".rs") {
            SupportedLanguage::Rust
        } else if path.ends_with(".zig") {
            SupportedLanguage::Zig
        } else if path.ends_with(".nim") {
            SupportedLanguage::Nim
        } else {
            SupportedLanguage::Unknown
        };

        let file = SourceFile {
            path: path.to_string(),
            language: lang,
            content: content.to_string(),
            cursor_line: 0,
        };

        self.open_files.push(file);
        self.active_file_idx = Some(self.open_files.len() - 1);
    }

    pub fn trigger_ai_completion(&self) -> Result<String, &'static str> {
        if !self.is_ai_completion_enabled {
            return Err("AI completion assistant is disabled");
        }
        let idx = self.active_file_idx.ok_or("No file currently open")?;
        let active_file = &self.open_files[idx];

        match active_file.language {
            SupportedLanguage::Rust => Ok("fn main() {\n    println!(\"Sovereign SigmaOS!\");\n}".to_string()),
            SupportedLanguage::Zig => Ok("pub fn main() !void {\n    const stdout = std.io.getStdOut().writer();\n}".to_string()),
            SupportedLanguage::Nim => Ok("proc main() =\n  echo \"Sovereign Nim!\"".to_string()),
            SupportedLanguage::Unknown => Ok("// Sovereign code suggestion".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sigmadev_ide_workspace() {
        let mut ide = SigmaDevIde::new();
        ide.open_file("main.rs", "fn main() {}");
        assert_eq!(ide.open_files.len(), 1);
        assert_eq!(ide.open_files[0].language, SupportedLanguage::Rust);

        let completion = ide.trigger_ai_completion().unwrap();
        assert!(completion.contains("Sovereign SigmaOS!"));

        ide.open_file("build.zig", "pub fn build() {}");
        assert_eq!(ide.open_files[1].language, SupportedLanguage::Zig);
        let zig_completion = ide.trigger_ai_completion().unwrap();
        assert!(zig_completion.contains("std.io"));
    }
}
