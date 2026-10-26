// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS - Warp GPU-accelerated terminal
// Principles: MODULARITY, COMMAND-QUERY SEPARATION, DISCOVERABILITY
use alloc::{collections::BTreeMap, string::String, vec::Vec};
#[derive(Debug, Clone, PartialEq)]
pub enum BlockState {
    Running,
    Completed { exit_code: i32 },
    Failed { exit_code: i32 },
}
#[derive(Debug, Clone)]
pub struct TermBlock {
    pub id: u32,
    pub command: String,
    pub output: Vec<u8>,
    pub state: BlockState,
    pub timestamp_ms: u64,
    pub working_dir: String,
}
impl TermBlock {
    pub fn new(id: u32, cmd: &str, dir: &str) -> Self {
        Self {
            id,
            command: cmd.into(),
            output: Vec::new(),
            state: BlockState::Running,
            timestamp_ms: 0,
            working_dir: dir.into(),
        }
    }
    pub fn finish(&mut self, exit_code: i32, output: Vec<u8>) {
        self.output = output;
        self.state = if exit_code == 0 {
            BlockState::Completed { exit_code }
        } else {
            BlockState::Failed { exit_code }
        };
    }
    pub fn append_output(&mut self, data: &[u8]) {
        self.output.extend_from_slice(data);
    }
    pub fn is_success(&self) -> bool {
        matches!(self.state, BlockState::Completed { exit_code: 0 })
    }
}
#[derive(Debug, Clone)]
pub struct AiSuggestion {
    pub command: String,
    pub explanation: String,
    pub confidence: f32,
}
#[derive(Debug, Clone)]
pub struct ShellHistoryEntry {
    pub command: String,
    pub timestamp_ms: u64,
    pub directory: String,
    pub exit_code: i32,
}
pub struct WarpSession {
    pub blocks: Vec<TermBlock>,
    pub history: Vec<ShellHistoryEntry>,
    pub current_dir: String,
    pub next_block_id: u32,
    pub env: BTreeMap<String, String>,
}
impl WarpSession {
    pub fn new(start_dir: &str) -> Self {
        Self {
            blocks: Vec::new(),
            history: Vec::new(),
            current_dir: start_dir.into(),
            next_block_id: 1,
            env: BTreeMap::new(),
        }
    }
    pub fn execute_command(&mut self, cmd: &str) -> u32 {
        let id = self.next_block_id;
        self.next_block_id += 1;
        let block = TermBlock::new(id, cmd, &self.current_dir);
        self.blocks.push(block);
        self.history.push(ShellHistoryEntry {
            command: cmd.into(),
            timestamp_ms: 0,
            directory: self.current_dir.clone(),
            exit_code: 0,
        });
        id
    }
    pub fn complete_block(&mut self, id: u32, exit_code: i32, output: Vec<u8>) {
        if let Some(b) = self.blocks.iter_mut().find(|b| b.id == id) {
            b.finish(exit_code, output);
        }
        if let Some(h) = self.history.iter_mut().find(|h| {
            h.command
                == self
                    .blocks
                    .iter()
                    .find(|b| b.id == id)
                    .map(|b| b.command.as_str())
                    .unwrap_or("")
        }) {
            h.exit_code = exit_code;
        }
    }
    pub fn get_block(&self, id: u32) -> Option<&TermBlock> {
        self.blocks.iter().find(|b| b.id == id)
    }
    pub fn search_history(&self, query: &str) -> Vec<&ShellHistoryEntry> {
        self.history
            .iter()
            .filter(|h| h.command.contains(query))
            .collect()
    }
    pub fn get_ai_suggestions(&self, prefix: &str) -> Vec<AiSuggestion> {
        if prefix.starts_with("git") {
            alloc::vec![AiSuggestion {
                command: alloc::format!("{} status", prefix),
                explanation: String::from("Show working tree status"),
                confidence: 0.9
            }]
        } else {
            Vec::new()
        }
    }
    pub fn share_workflow(&self, block_ids: &[u32]) -> String {
        alloc::format!("https://app.warp.dev/shared/{}", block_ids.len())
    }
    pub fn gpu_render_frame(&self) -> Vec<u8> {
        Vec::new()
    }
    pub fn set_env(&mut self, k: &str, v: &str) {
        self.env.insert(k.into(), v.into());
    }
    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_execute_creates_block() {
        let mut s = WarpSession::new("/home/sigma");
        let id = s.execute_command("ls -la");
        assert_eq!(s.block_count(), 1);
        let b = s.get_block(id).unwrap();
        assert_eq!(b.command, "ls -la");
        assert!(matches!(b.state, BlockState::Running));
    }
    #[test]
    fn test_complete_block() {
        let mut s = WarpSession::new("/");
        let id = s.execute_command("echo hello");
        s.complete_block(
            id,
            0,
            b"hello
"
            .to_vec(),
        );
        let b = s.get_block(id).unwrap();
        assert!(matches!(b.state, BlockState::Completed { exit_code: 0 }));
        assert!(b.is_success());
    }
    #[test]
    fn test_history_search() {
        let mut s = WarpSession::new("/");
        s.execute_command("git status");
        s.execute_command("git log");
        s.execute_command("ls");
        let git_cmds = s.search_history("git");
        assert_eq!(git_cmds.len(), 2);
    }
    #[test]
    fn test_ai_suggestions() {
        let s = WarpSession::new("/");
        let sug = s.get_ai_suggestions("git");
        assert!(!sug.is_empty());
        assert!(sug[0].command.starts_with("git"));
    }
}
