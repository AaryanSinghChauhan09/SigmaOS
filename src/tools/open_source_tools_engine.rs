// SigmaOS Open-Source Tool Integration Parity Suite
// Implements native zero-dependency Rust parity engines for:
// - rsync (rolling checksum delta sync)
// - htop/btop (process tree telemetry & metric sorting)
// - bat (syntax-highlighted file viewer with line numbering)
// - fzf (fuzzy finder matching & scoring engine)

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// ============================================================================
// 1. rsync Delta Sync Engine (Adler-32 Rolling Checksum & Block Delta)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsyncBlockChecksum {
    pub block_index: usize,
    pub weak_checksum: u32,  // Adler-32 inspired rolling checksum
    pub strong_checksum: u64, // FNV-1a strong hash
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RsyncDeltaOp {
    BlockMatch { block_index: usize },
    LiteralData { bytes: Vec<u8> },
}

pub struct RsyncDeltaSyncEngine {
    pub block_size: usize,
}

impl RsyncDeltaSyncEngine {
    pub fn new(block_size: usize) -> Self {
        Self {
            block_size: block_size.max(16),
        }
    }

    /// Compute Adler-32 weak rolling checksum for a byte slice
    pub fn compute_weak_checksum(data: &[u8]) -> u32 {
        let mut a: u32 = 1;
        let mut b: u32 = 0;
        for &byte in data {
            a = (a + byte as u32) % 65521;
            b = (b + a) % 65521;
        }
        (b << 16) | a
    }

    /// Compute FNV-1a 64-bit strong checksum for a byte slice
    pub fn compute_strong_checksum(data: &[u8]) -> u64 {
        let mut hash: u64 = 0xcbf29ce484222325;
        for &byte in data {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    pub fn generate_signature(&self, basis_data: &[u8]) -> Vec<RsyncBlockChecksum> {
        let mut sigs = Vec::new();
        let mut index = 0;
        for chunk in basis_data.chunks(self.block_size) {
            sigs.push(RsyncBlockChecksum {
                block_index: index,
                weak_checksum: Self::compute_weak_checksum(chunk),
                strong_checksum: Self::compute_strong_checksum(chunk),
            });
            index += 1;
        }
        sigs
    }

    pub fn compute_delta(&self, target_data: &[u8], signature: &[RsyncBlockChecksum]) -> Vec<RsyncDeltaOp> {
        let mut delta = Vec::new();
        let mut cursor = 0;
        let mut pending_literal = Vec::new();

        while cursor < target_data.len() {
            let chunk_end = (cursor + self.block_size).min(target_data.len());
            let chunk = &target_data[cursor..chunk_end];

            if chunk.len() == self.block_size {
                let weak = Self::compute_weak_checksum(chunk);
                let strong = Self::compute_strong_checksum(chunk);

                if let Some(matched) = signature.iter().find(|s| s.weak_checksum == weak && s.strong_checksum == strong) {
                    if !pending_literal.is_empty() {
                        delta.push(RsyncDeltaOp::LiteralData {
                            bytes: pending_literal.clone(),
                        });
                        pending_literal.clear();
                    }
                    delta.push(RsyncDeltaOp::BlockMatch {
                        block_index: matched.block_index,
                    });
                    cursor += self.block_size;
                    continue;
                }
            }

            pending_literal.push(target_data[cursor]);
            cursor += 1;
        }

        if !pending_literal.is_empty() {
            delta.push(RsyncDeltaOp::LiteralData {
                bytes: pending_literal,
            });
        }

        delta
    }
}

impl Default for RsyncDeltaSyncEngine {
    fn default() -> Self {
        Self::new(64)
    }
}

// ============================================================================
// 2. htop / btop Process Telemetry & Sort Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtopSortField {
    ByPid,
    ByCpu,
    ByMemory,
    ByName,
}

#[derive(Debug, Clone)]
pub struct HtopProcessEntry {
    pub pid: u32,
    pub ppid: u32,
    pub user: String,
    pub name: String,
    pub cpu_percent: f32,
    pub mem_percent: f32,
    pub state: String,
}

pub struct HtopProcessMonitorEngine {
    pub processes: Vec<HtopProcessEntry>,
}

impl HtopProcessMonitorEngine {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
        }
    }

    pub fn add_process(&mut self, process: HtopProcessEntry) {
        self.processes.push(process);
    }

    pub fn sort_processes(&mut self, sort_by: HtopSortField) {
        match sort_by {
            HtopSortField::ByPid => self.processes.sort_by_key(|p| p.pid),
            HtopSortField::ByCpu => self.processes.sort_by(|a, b| b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap_or(core::cmp::Ordering::Equal)),
            HtopSortField::ByMemory => self.processes.sort_by(|a, b| b.mem_percent.partial_cmp(&a.mem_percent).unwrap_or(core::cmp::Ordering::Equal)),
            HtopSortField::ByName => self.processes.sort_by(|a, b| a.name.cmp(&b.name)),
        }
    }

    pub fn render_tree_view(&self) -> Vec<String> {
        let mut lines = Vec::new();
        for p in &self.processes {
            if p.ppid == 0 || p.ppid == 1 {
                lines.push(format!("[PID {:>5}] {} ({:.1}% CPU, {:.1}% RAM)", p.pid, p.name, p.cpu_percent, p.mem_percent));
                for child in &self.processes {
                    if child.ppid == p.pid {
                        lines.push(format!("  └── [PID {:>5}] {} ({:.1}% CPU)", child.pid, child.name, child.cpu_percent));
                    }
                }
            }
        }
        lines
    }
}

impl Default for HtopProcessMonitorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. bat Syntax Highlighter & Viewer Engine
// ============================================================================

pub struct BatSyntaxHighlighterEngine {
    pub theme: String,
    pub show_line_numbers: bool,
}

impl BatSyntaxHighlighterEngine {
    pub fn new(theme: &str, show_line_numbers: bool) -> Self {
        Self {
            theme: theme.to_string(),
            show_line_numbers,
        }
    }

    pub fn highlight_code(&self, filename: &str, code: &str) -> String {
        let mut output = String::new();
        output.push_str(&format!("─── File: {} ─── Theme: {} ───\n", filename, self.theme));

        for (idx, line) in code.lines().enumerate() {
            let line_num = idx + 1;
            let mut highlighted = line.to_string();

            // Highlight Rust keywords
            for kw in &["fn", "let", "mut", "pub", "struct", "enum", "impl", "use", "return"] {
                if highlighted.contains(kw) {
                    highlighted = highlighted.replace(kw, &format!("\x1b[35m{}\x1b[0m", kw));
                }
            }

            if self.show_line_numbers {
                output.push_str(&format!("{:>4} │ {}\n", line_num, highlighted));
            } else {
                output.push_str(&format!("{}\n", highlighted));
            }
        }

        output
    }
}

impl Default for BatSyntaxHighlighterEngine {
    fn default() -> Self {
        Self::new("Nord", true)
    }
}

// ============================================================================
// 4. fzf Fuzzy Finder & Scoring Engine
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FzfSearchResult {
    pub candidate: String,
    pub score: i32,
}

pub struct FzfFuzzyFinderEngine;

impl FzfFuzzyFinderEngine {
    pub fn new() -> Self {
        Self
    }

    /// Smith-Waterman inspired fuzzy match score
    pub fn fuzzy_score(candidate: &str, query: &str) -> Option<i32> {
        if query.is_empty() {
            return Some(0);
        }

        let cand_lower = candidate.to_lowercase();
        let query_lower = query.to_lowercase();

        let mut query_idx = 0;
        let query_chars: Vec<char> = query_lower.chars().collect();
        let cand_chars: Vec<char> = cand_lower.chars().collect();

        let mut score = 0;
        let mut consecutive_bonus = 0;

        for (c_idx, &c) in cand_chars.iter().enumerate() {
            if query_idx < query_chars.len() && c == query_chars[query_idx] {
                score += 10 + consecutive_bonus;
                if c_idx == 0 || cand_chars.get(c_idx - 1) == Some(&'/') || cand_chars.get(c_idx - 1) == Some(&'_') {
                    score += 15; // Prefix / boundary bonus
                }
                consecutive_bonus += 5;
                query_idx += 1;
            } else {
                consecutive_bonus = 0;
            }
        }

        if query_idx == query_chars.len() {
            Some(score - (candidate.len() as i32)) // Penalty for longer strings
        } else {
            None // Query not matched fully
        }
    }

    pub fn filter_and_rank(&self, candidates: &[&str], query: &str) -> Vec<FzfSearchResult> {
        let mut results = Vec::new();
        for &cand in candidates {
            if let Some(score) = Self::fuzzy_score(cand, query) {
                results.push(FzfSearchResult {
                    candidate: cand.to_string(),
                    score,
                });
            }
        }

        results.sort_by(|a, b| b.score.cmp(&a.score));
        results
    }
}

impl Default for FzfFuzzyFinderEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsync_delta_sync() {
        let engine = RsyncDeltaSyncEngine::new(16);
        let basis = b"Hello, SigmaOS Operating System World!";
        let sig = engine.generate_signature(basis);
        assert!(!sig.is_empty());

        let target = b"Hello, SigmaOS Operating System World! Extra payload appended.";
        let delta = engine.compute_delta(target, &sig);
        assert!(!delta.is_empty());

        assert!(matches!(delta[0], RsyncDeltaOp::BlockMatch { .. }));
    }

    #[test]
    fn test_htop_process_monitor() {
        let mut htop = HtopProcessMonitorEngine::new();
        htop.add_process(HtopProcessEntry {
            pid: 100,
            ppid: 1,
            user: "root".to_string(),
            name: "systemd".to_string(),
            cpu_percent: 0.5,
            mem_percent: 1.2,
            state: "S".to_string(),
        });
        htop.add_process(HtopProcessEntry {
            pid: 101,
            ppid: 1,
            user: "root".to_string(),
            name: "kernel_task".to_string(),
            cpu_percent: 45.0,
            mem_percent: 8.5,
            state: "R".to_string(),
        });

        htop.sort_processes(HtopSortField::ByCpu);
        assert_eq!(htop.processes[0].pid, 101);

        let tree = htop.render_tree_view();
        assert!(!tree.is_empty());
    }

    #[test]
    fn test_bat_syntax_highlighter() {
        let bat = BatSyntaxHighlighterEngine::new("Monokai", true);
        let highlighted = bat.highlight_code("main.rs", "fn main() {\n    let mut x = 42;\n}");
        assert!(highlighted.contains("File: main.rs"));
        assert!(highlighted.contains("1 │"));
    }

    #[test]
    fn test_fzf_fuzzy_finder() {
        let fzf = FzfFuzzyFinderEngine::new();
        let candidates = ["src/kernel/main.rs", "src/security/landlock.rs", "src/filesystem/erofs.rs"];
        let results = fzf.filter_and_rank(&candidates, "landlock");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].candidate, "src/security/landlock.rs");
    }
}
