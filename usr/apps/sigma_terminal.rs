// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/apps/sigma_terminal.rs — Sigma-Terminal Terminal Emulator
//
// Implements a terminal emulator with shell integration, command history,
// and basic terminal capabilities.
//
// Language: Rust (std for userland applications)

use std::io::{self, Write};
use std::process::Command;

// ─── Terminal State ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Terminal {
    pub prompt: String,
    pub history: Vec<String>,
    pub history_index: usize,
    pub current_input: String,
    pub cursor_pos: usize,
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            prompt: "sigma-sh $ ".to_string(),
            history: Vec::new(),
            history_index: 0,
            current_input: String::new(),
            cursor_pos: 0,
        }
    }

    /// Set prompt string
    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }

    /// Add command to history
    pub fn add_to_history(&mut self, cmd: String) {
        if !cmd.trim().is_empty() {
            self.history.push(cmd);
            self.history_index = self.history.len();
        }
    }

    /// Navigate history up
    pub fn history_up(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            if let Some(cmd) = self.history.get(self.history_index) {
                self.current_input = cmd.clone();
                self.cursor_pos = self.current_input.len();
            }
        }
    }

    /// Navigate history down
    pub fn history_down(&mut self) {
        if self.history_index < self.history.len() {
            self.history_index += 1;
            if self.history_index < self.history.len() {
                if let Some(cmd) = self.history.get(self.history_index) {
                    self.current_input = cmd.clone();
                    self.cursor_pos = self.current_input.len();
                }
            } else {
                self.current_input.clear();
                self.cursor_pos = 0;
            }
        }
    }

    /// Insert character at cursor position
    pub fn insert_char(&mut self, c: char) {
        self.current_input.insert(self.cursor_pos, c);
        self.cursor_pos += 1;
    }

    /// Delete character at cursor position
    pub fn delete_char(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.current_input.remove(self.cursor_pos);
        }
    }

    /// Move cursor left
    pub fn cursor_left(&mut self) {
        self.cursor_pos = self.cursor_pos.saturating_sub(1);
    }

    /// Move cursor right
    pub fn cursor_right(&mut self) {
        self.cursor_pos = (self.cursor_pos + 1).min(self.current_input.len());
    }

    /// Clear current input
    pub fn clear_input(&mut self) {
        self.current_input.clear();
        self.cursor_pos = 0;
    }

    /// Execute current command
    pub fn execute(&mut self) -> String {
        let cmd = self.current_input.clone();
        self.add_to_history(cmd.clone());
        self.clear_input();
        
        // Simple command execution
        let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
        if parts.is_empty() {
            return String::new();
        }
        
        let program = parts[0];
        let args = &parts[1..];
        
        match Command::new(program).args(args).output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                format!("{}\n{}", stdout, stderr)
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    /// Get current input
    pub fn get_input(&self) -> &str {
        &self.current_input
    }

    /// Render prompt and input
    pub fn render(&self) -> String {
        format!("{}{}", self.prompt, self.current_input)
    }
}

// ─── CLI Interface ───────────────────────────────────────────────────────────

fn main() {
    let mut terminal = Terminal::new();
    
    println!("Sigma-Terminal v0.1 - Terminal Emulator");
    println!("Type 'exit' to quit\n");
    
    loop {
        print!("{}", terminal.render());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        match input {
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush().unwrap();
            }
            "history" => {
                for (i, cmd) in terminal.history.iter().enumerate() {
                    println!("{} {}", i + 1, cmd);
                }
            }
            cmd if cmd.starts_with("prompt ") => {
                let new_prompt = &cmd[7..];
                terminal.set_prompt(new_prompt.to_string());
            }
            _ => {
                terminal.current_input = input.to_string();
                let output = terminal.execute();
                if !output.is_empty() {
                    println!("{}", output);
                }
            }
        }
    }
}
