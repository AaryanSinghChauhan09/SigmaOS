// SigmaOS Open-Source Tool Parity Suite
// Implements rsync delta algorithm, htop/btop process monitor,
// bat syntax highlighter, and fzf fuzzy finder engines.

use std::collections::HashMap;
use std::format;
use std::string::String;
use std::string::ToString;
use std::vec::Vec;

// ============================================================================
// 1. Rsync Rolling Checksum Delta Sync Engine (librsync / rsync parity)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RsyncChunkSignature {
    pub chunk_index: usize,
    pub weak_checksum: u32,
    pub strong_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RsyncDeltaOp {
    MatchChunk(usize),
    LiteralData(Vec<u8>),
}

pub struct RsyncDeltaSyncEngine {
    pub chunk_size: usize,
}

impl RsyncDeltaSyncEngine {
    pub fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size: if chunk_size == 0 { 64 } else { chunk_size },
        }
    }

    /// Computes Adler-32 style weak checksum for rolling window
    pub fn compute_weak_checksum(data: &[u8]) -> u32 {
        let mut a: u32 = 0;
        let mut b: u32 = 0;
        for &byte in data {
            a = (a + byte as u32) % 65521;
            b = (b + a) % 65521;
        }
        (b << 16) | a
    }

    /// Generates file chunk signatures for target file
    pub fn generate_signatures(&self, file_data: &[u8]) -> Vec<RsyncChunkSignature> {
        let mut sigs = Vec::new();
        for (i, chunk) in file_data.chunks(self.chunk_size).enumerate() {
            let weak = Self::compute_weak_checksum(chunk);
            let mut strong_sim = String::from("HASH-");
            for b in chunk.iter().take(8) {
                strong_sim.push_str(&format!("{:02x}", b));
            }
            sigs.push(RsyncChunkSignature {
                chunk_index: i,
                weak_checksum: weak,
                strong_hash: strong_sim,
            });
        }
        sigs
    }

    /// Computes delta operations from basis signatures and modified file
    pub fn compute_delta(&self, modified_data: &[u8], basis_sigs: &[RsyncChunkSignature]) -> Vec<RsyncDeltaOp> {
        let mut delta = Vec::new();
        let mut sig_map: HashMap<u32, usize> = HashMap::new();
        for sig in basis_sigs {
            sig_map.insert(sig.weak_checksum, sig.chunk_index);
        }

        let mut offset = 0;
        let mut literal_buffer = Vec::new();

        while offset < modified_data.len() {
            let remaining = modified_data.len() - offset;
            let current_chunk_len = if remaining < self.chunk_size { remaining } else { self.chunk_size };
            let window = &modified_data[offset..offset + current_chunk_len];
            let weak = Self::compute_weak_checksum(window);

            if let Some(&matched_chunk_idx) = sig_map.get(&weak) {
                if !literal_buffer.is_empty() {
                    delta.push(RsyncDeltaOp::LiteralData(literal_buffer.clone()));
                    literal_buffer.clear();
                }
                delta.push(RsyncDeltaOp::MatchChunk(matched_chunk_idx));
                offset += current_chunk_len;
            } else {
                literal_buffer.push(modified_data[offset]);
                offset += 1;
            }
        }

        if !literal_buffer.is_empty() {
            delta.push(RsyncDeltaOp::LiteralData(literal_buffer));
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
// 2. Htop / Btop Process Monitor Engine
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessSortOrder {
    Pid,
    CpuPercent,
    MemoryBytes,
    Command,
}

#[derive(Debug, Clone)]
pub struct ProcessMetrics {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub state: char,
}

pub struct HtopProcessMonitorEngine {
    pub processes: Vec<ProcessMetrics>,
}

impl HtopProcessMonitorEngine {
    pub fn new() -> Self {
        Self { processes: Vec::new() }
    }

    pub fn add_process(&mut self, proc: ProcessMetrics) {
        self.processes.push(proc);
    }

    pub fn sort_processes(&mut self, order: ProcessSortOrder) {
        match order {
            ProcessSortOrder::Pid => self.processes.sort_by_key(|p| p.pid),
            ProcessSortOrder::CpuPercent => self.processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
            ProcessSortOrder::MemoryBytes => self.processes.sort_by_key(|p| std::cmp::Reverse(p.memory_bytes)),
            ProcessSortOrder::Command => self.processes.sort_by(|a, b| a.name.cmp(&b.name)),
        }
    }

    pub fn filter_by_name(&self, query: &str) -> Vec<ProcessMetrics> {
        self.processes.iter().filter(|p| p.name.contains(query)).cloned().collect()
    }
}

impl Default for HtopProcessMonitorEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 3. Bat Syntax Highlighter Engine (bat / cat parity)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxTheme {
    DarkNord,
    LightSolarized,
    Monokai,
}

pub struct BatSyntaxHighlighterEngine {
    pub theme: SyntaxTheme,
    pub show_line_numbers: bool,
}

impl BatSyntaxHighlighterEngine {
    pub fn new(theme: SyntaxTheme) -> Self {
        Self {
            theme,
            show_line_numbers: true,
        }
    }

    pub fn highlight_code(&self, language: &str, code: &str) -> String {
        let mut output = String::new();
        output.push_str(&format!("// Syntax Highlighted [{}] Theme: {:?}\n", language, self.theme));

        for (line_idx, line) in code.lines().enumerate() {
            if self.show_line_numbers {
                output.push_str(&format!("{:4} | ", line_idx + 1));
            }

            let mut highlighted = line.to_string();
            if language == "rust" {
                highlighted = highlighted.replace("fn ", "\x1b[32mfn\x1b[0m ");
                highlighted = highlighted.replace("let ", "\x1b[34mlet\x1b[0m ");
                highlighted = highlighted.replace("pub ", "\x1b[35mpub\x1b[0m ");
            }

            output.push_str(&highlighted);
            output.push('\n');
        }

        output
    }
}

impl Default for BatSyntaxHighlighterEngine {
    fn default() -> Self {
        Self::new(SyntaxTheme::DarkNord)
    }
}

// ============================================================================
// 4. Fzf Fuzzy Finder Engine (fzf parity)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzyMatch {
    pub candidate: String,
    pub score: i32,
    pub matched_indices: Vec<usize>,
}

pub struct FzfFuzzyFinderEngine;

impl FzfFuzzyFinderEngine {
    pub fn new() -> Self {
        Self
    }

    /// Computes fuzzy match score for a single string query against candidate
    pub fn fuzzy_score(candidate: &str, query: &str) -> Option<FuzzyMatch> {
        if query.is_empty() {
            return Some(FuzzyMatch {
                candidate: candidate.to_string(),
                score: 0,
                matched_indices: Vec::new(),
            });
        }

        let cand_chars: Vec<char> = candidate.chars().collect();
        let query_chars: Vec<char> = query.to_lowercase().chars().collect();

        let mut matched_indices = Vec::new();
        let mut query_idx = 0;
        let mut score = 0;

        for (i, &ch) in cand_chars.iter().enumerate() {
            if query_idx < query_chars.len() && ch.to_lowercase().next() == Some(query_chars[query_idx]) {
                matched_indices.push(i);
                query_idx += 1;
                score += 10;
                if i > 0 && matched_indices.contains(&(i - 1)) {
                    score += 5; // Consecutive bonus
                }
            }
        }

        if query_idx == query_chars.len() {
            Some(FuzzyMatch {
                candidate: candidate.to_string(),
                score,
                matched_indices,
            })
        } else {
            None
        }
    }

    pub fn search(&self, candidates: &[String], query: &str) -> Vec<FuzzyMatch> {
        let mut results = Vec::new();
        for cand in candidates {
            if let Some(m) = Self::fuzzy_score(cand, query) {
                results.push(m);
            }
        }
        results.sort_by_key(|m| std::cmp::Reverse(m.score));
        results
    }
}

impl Default for FzfFuzzyFinderEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsync_delta_sync() {
        let engine = RsyncDeltaSyncEngine::new(8);
        let basis = b"Hello, World! This is basis payload.";
        let sigs = engine.generate_signatures(basis);
        assert!(!sigs.is_empty());

        let modified = b"Hello, World! This is MODIFIED payload.";
        let delta = engine.compute_delta(modified, &sigs);
        assert!(!delta.is_empty());
    }

    #[test]
    fn test_htop_process_monitor() {
        let mut monitor = HtopProcessMonitorEngine::new();
        monitor.add_process(ProcessMetrics {
            pid: 100,
            ppid: 1,
            name: "systemd".to_string(),
            cpu_usage: 1.5,
            memory_bytes: 10240,
            state: 'S',
        });
        monitor.add_process(ProcessMetrics {
            pid: 200,
            ppid: 100,
            name: "chrome".to_string(),
            cpu_usage: 25.0,
            memory_bytes: 1048576,
            state: 'R',
        });

        monitor.sort_processes(ProcessSortOrder::CpuPercent);
        assert_eq!(monitor.processes[0].pid, 200);
    }

    #[test]
    fn test_bat_syntax_highlighter() {
        let bat = BatSyntaxHighlighterEngine::new(SyntaxTheme::Monokai);
        let code = "pub fn main() {\n    let x = 42;\n}";
        let res = bat.highlight_code("rust", code);
        assert!(res.contains("\x1b[35mpub\x1b[0m"));
    }

    #[test]
    fn test_fzf_fuzzy_finder() {
        let fzf = FzfFuzzyFinderEngine::new();
        let candidates = vec![
            "src/main.rs".to_string(),
            "src/tools/mod.rs".to_string(),
            "docs/AGENTS.md".to_string(),
        ];
        let matches = fzf.search(&candidates, "main");
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].candidate, "src/main.rs");
    }
}
