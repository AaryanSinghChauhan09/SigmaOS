// src/shell/interactive/mod.rs
// Interactive shell baseline implementation for SigmaOS shell

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
use std::string::{String, ToString};
#[cfg(feature = "std")]
use std::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::string::{String, ToString};
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalEvent {
    None,
    Interrupt, // Ctrl-C
    Eof,       // Ctrl-D
}

#[derive(Debug, Clone)]
pub struct PromptConfig {
    pub ps1: String,
    pub ps2: String,
    pub username: String,
    pub hostname: String,
    pub current_dir: String,
    pub is_root: bool,
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            ps1: String::from("\\u@\\h:\\w\\$ "),
            ps2: String::from("> "),
            username: String::from("sigma"),
            hostname: String::from("sigmaos"),
            current_dir: String::from("~"),
            is_root: false,
        }
    }
}

pub struct InteractiveShellEngine {
    pub prompt_config: PromptConfig,
    pub history: Vec<String>,
    pub max_history_size: usize,
    pub line_buffer: String,
    pub cursor_position: usize,
    pub multiline_buffer: String,
    pub in_multiline: bool,
}

impl InteractiveShellEngine {
    pub fn new() -> Self {
        Self {
            prompt_config: PromptConfig::default(),
            history: Vec::new(),
            max_history_size: 1000,
            line_buffer: String::new(),
            cursor_position: 0,
            multiline_buffer: String::new(),
            in_multiline: false,
        }
    }

    pub fn render_prompt(&self) -> String {
        if self.in_multiline {
            return self.prompt_config.ps2.clone();
        }

        let mut rendered = self.prompt_config.ps1.clone();
        rendered = rendered.replace("\\u", &self.prompt_config.username);
        rendered = rendered.replace("\\h", &self.prompt_config.hostname);
        rendered = rendered.replace("\\w", &self.prompt_config.current_dir);
        let symbol = if self.prompt_config.is_root { "#" } else { "$" };
        rendered = rendered.replace("\\$", symbol);
        rendered
    }

    pub fn handle_key_input(&mut self, input: &str) -> SignalEvent {
        if input == "\x03" {
            // Ctrl-C
            self.line_buffer.clear();
            self.multiline_buffer.clear();
            self.cursor_position = 0;
            self.in_multiline = false;
            return SignalEvent::Interrupt;
        }

        if input == "\x04" {
            // Ctrl-D
            if self.line_buffer.is_empty() && !self.in_multiline {
                return SignalEvent::Eof;
            }
        }

        for ch in input.chars() {
            match ch {
                '\x01' => self.cursor_position = 0,                      // Ctrl-A
                '\x05' => self.cursor_position = self.line_buffer.len(), // Ctrl-E
                '\x15' => {
                    // Ctrl-U
                    self.line_buffer.clear();
                    self.cursor_position = 0;
                }
                '\n' | '\r' => {
                    if self.is_incomplete_input(&self.line_buffer) {
                        self.multiline_buffer.push_str(&self.line_buffer);
                        self.multiline_buffer.push('\n');
                        self.line_buffer.clear();
                        self.cursor_position = 0;
                        self.in_multiline = true;
                    } else {
                        let full_line = if self.in_multiline {
                            self.multiline_buffer.push_str(&self.line_buffer);
                            let res = self.multiline_buffer.clone();
                            self.multiline_buffer.clear();
                            self.in_multiline = false;
                            res
                        } else {
                            self.line_buffer.clone()
                        };

                        if !full_line.trim().is_empty() {
                            self.add_history(&full_line);
                        }
                        self.line_buffer.clear();
                        self.cursor_position = 0;
                    }
                }
                _ => {
                    if ch >= ' ' {
                        self.line_buffer.insert(self.cursor_position, ch);
                        self.cursor_position += 1;
                    }
                }
            }
        }

        SignalEvent::None
    }

    pub fn is_incomplete_input(&self, input: &str) -> bool {
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut escaped = false;

        for ch in input.chars() {
            if escaped {
                escaped = false;
                continue;
            }

            match ch {
                '\\' if !in_single_quote => escaped = true,
                '\'' if !in_double_quote => in_single_quote = !in_single_quote,
                '"' if !in_single_quote => in_double_quote = !in_double_quote,
                _ => {}
            }
        }

        in_single_quote || in_double_quote
    }

    pub fn add_history(&mut self, cmd: &str) {
        if self.history.last().map(|s: &String| s.as_str()) != Some(cmd) {
            self.history.push(cmd.to_string());
            if self.history.len() > self.max_history_size {
                self.history.remove(0);
            }
        }
    }

    pub fn complete_path_or_command(&self, prefix: &str, candidates: &[&str]) -> Vec<String> {
        candidates
            .iter()
            .filter(|item| item.starts_with(prefix))
            .map(|item| item.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prompt_rendering() {
        let engine = InteractiveShellEngine::new();
        let prompt = engine.render_prompt();
        assert!(prompt.contains("sigma@sigmaos:~"));
        assert!(prompt.contains("$"));
    }

    #[test]
    fn test_ctrl_c_and_ctrl_d() {
        let mut engine = InteractiveShellEngine::new();
        engine.line_buffer = String::from("some command");
        assert_eq!(engine.handle_key_input("\x03"), SignalEvent::Interrupt);
        assert!(engine.line_buffer.is_empty());

        assert_eq!(engine.handle_key_input("\x04"), SignalEvent::Eof);
    }

    #[test]
    fn test_multiline_quote_detection() {
        let engine = InteractiveShellEngine::new();
        assert!(engine.is_incomplete_input("echo \"unclosed quote"));
        assert!(!engine.is_incomplete_input("echo \"closed quote\""));
    }
}
