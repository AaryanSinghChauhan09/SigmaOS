// SigmaOS Open-Source Tool Innovations Suite
// Clean-room, zero-dependency safe Rust implementations inspired by htop, ripgrep, bat, fzf, lazygit, tldr, and tmux.

use std::collections::HashMap;
use std::format;
use std::string::String;
use std::vec::Vec;

/// Process information for htop/btop style visualization
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtopProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage_percent: u8,
    pub mem_usage_mb: u32,
    pub state: String,
}

/// Interactive htop/btop inspired process tree and memory monitor
#[derive(Debug, Clone)]
pub struct HtopProcessVisualizer {
    pub processes: Vec<HtopProcessInfo>,
    pub total_ram_mb: u32,
    pub used_ram_mb: u32,
}

impl HtopProcessVisualizer {
    pub fn new(total_ram: u32) -> Self {
        Self {
            processes: Vec::new(),
            total_ram_mb: total_ram,
            used_ram_mb: 0,
        }
    }

    pub fn update_process(&mut self, pid: u32, name: &str, cpu: u8, mem: u32, state: &str) {
        if let Some(p) = self.processes.iter_mut().find(|proc| proc.pid == pid) {
            p.cpu_usage_percent = cpu;
            p.mem_usage_mb = mem;
            p.state = state.to_string();
        } else {
            self.processes.push(HtopProcessInfo {
                pid,
                name: name.to_string(),
                cpu_usage_percent: cpu,
                mem_usage_mb: mem,
                state: state.to_string(),
            });
        }
        self.used_ram_mb = self.processes.iter().map(|p| p.mem_usage_mb).sum();
    }

    pub fn generate_summary_bar(&self) -> String {
        let ram_pct = if self.total_ram_mb > 0 {
            (self.used_ram_mb * 100) / self.total_ram_mb
        } else {
            0
        };
        format!(
            "MEM [{}MB/{}MB ({}%)] | Tasks: {}",
            self.used_ram_mb,
            self.total_ram_mb,
            ram_pct,
            self.processes.len()
        )
    }
}

/// Fast search result entry inspired by ripgrep
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RipgrepMatch {
    pub filepath: String,
    pub line_number: usize,
    pub line_text: String,
}

/// Ripgrep inspired ultra-fast pattern search engine
#[derive(Debug, Clone, Default)]
pub struct RipgrepFastSearch {
    pub case_sensitive: bool,
}

impl RipgrepFastSearch {
    pub fn new(case_sensitive: bool) -> Self {
        Self { case_sensitive }
    }

    pub fn search_content(
        &self,
        filepath: &str,
        content: &str,
        query: &str,
    ) -> Vec<RipgrepMatch> {
        let mut matches = Vec::new();
        let target_query = if self.case_sensitive {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        for (idx, line) in content.lines().enumerate() {
            let line_cmp = if self.case_sensitive {
                line.to_string()
            } else {
                line.to_lowercase()
            };

            if line_cmp.contains(&target_query) {
                matches.push(RipgrepMatch {
                    filepath: filepath.to_string(),
                    line_number: idx + 1,
                    line_text: line.trim().to_string(),
                });
            }
        }
        matches
    }
}

/// Bat inspired syntax-highlighted file viewer
#[derive(Debug, Clone)]
pub struct BatSyntaxPrinter {
    pub show_line_numbers: bool,
    pub theme: String,
}

impl BatSyntaxPrinter {
    pub fn new(theme: &str) -> Self {
        Self {
            show_line_numbers: true,
            theme: theme.to_string(),
        }
    }

    pub fn render_file(&self, filename: &str, content: &str) -> String {
        let mut output = format!("─── File: {} ({}) ───\n", filename, self.theme);
        for (idx, line) in content.lines().enumerate() {
            if self.show_line_numbers {
                output.push_str(&format!("{:4} │ {}\n", idx + 1, line));
            } else {
                output.push_str(&format!("{}\n", line));
            }
        }
        output
    }
}

/// Fzf inspired fuzzy path & text matcher
#[derive(Debug, Clone, Default)]
pub struct FzfFuzzyMatcher;

impl FzfFuzzyMatcher {
    pub fn new() -> Self {
        Self
    }

    pub fn fuzzy_match(&self, candidates: &[&str], query: &str) -> Vec<String> {
        let q_lower = query.to_lowercase();
        let mut results: Vec<(&str, usize)> = Vec::new();

        for &cand in candidates {
            let c_lower = cand.to_lowercase();
            if let Some(pos) = c_lower.find(&q_lower) {
                results.push((cand, pos));
            }
        }

        results.sort_by_key(|&(_, pos)| pos);
        results.into_iter().map(|(cand, _)| cand.to_string()).collect()
    }
}

/// Lazygit inspired terminal git workflow manager
#[derive(Debug, Clone)]
pub struct LazygitRepositoryManager {
    pub staged_files: Vec<String>,
    pub unstaged_files: Vec<String>,
    pub commit_history: Vec<String>,
}

impl LazygitRepositoryManager {
    pub fn new() -> Self {
        Self {
            staged_files: Vec::new(),
            unstaged_files: Vec::new(),
            commit_history: Vec::new(),
        }
    }

    pub fn stage_file(&mut self, file: &str) {
        self.unstaged_files.retain(|f| f != file);
        if !self.staged_files.contains(&file.to_string()) {
            self.staged_files.push(file.to_string());
        }
    }

    pub fn commit(&mut self, message: &str) -> Result<String, &'static str> {
        if self.staged_files.is_empty() {
            Err("Lazygit: No staged files to commit")
        } else {
            let commit_entry = format!("commit: {} ({})", message, self.staged_files.join(", "));
            self.commit_history.push(commit_entry.clone());
            self.staged_files.clear();
            Ok(commit_entry)
        }
    }
}

impl Default for LazygitRepositoryManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Tldr inspired simplified command help summarizer
#[derive(Debug, Clone)]
pub struct TldrPageSummarizer {
    pub pages: HashMap<String, String>,
}

impl TldrPageSummarizer {
    pub fn new() -> Self {
        let mut pages = HashMap::new();
        pages.insert(
            "tar".to_string(),
            "tar -xzf archive.tar.gz # Extract gzipped archive\ntar -czf archive.tar.gz dir # Create gzipped archive".to_string(),
        );
        pages.insert(
            "curl".to_string(),
            "curl -O https://example.com/file # Download file\ncurl -X POST -d 'data' url # Send POST request".to_string(),
        );
        Self { pages }
    }

    pub fn get_summary(&self, cmd: &str) -> Option<&String> {
        self.pages.get(cmd)
    }
}

impl Default for TldrPageSummarizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Tmux inspired terminal session multiplexer
#[derive(Debug, Clone)]
pub struct TmuxPane {
    pub pane_id: u32,
    pub active_cmd: String,
}

#[derive(Debug, Clone)]
pub struct TmuxWindow {
    pub window_id: u32,
    pub name: String,
    pub panes: Vec<TmuxPane>,
}

pub struct TmuxSessionMultiplexer {
    pub session_name: String,
    pub windows: Vec<TmuxWindow>,
    pub active_window_id: u32,
}

impl TmuxSessionMultiplexer {
    pub fn new(session_name: &str) -> Self {
        let initial_win = TmuxWindow {
            window_id: 0,
            name: "bash".to_string(),
            panes: vec![TmuxPane {
                pane_id: 0,
                active_cmd: "bash".to_string(),
            }],
        };
        Self {
            session_name: session_name.to_string(),
            windows: vec![initial_win],
            active_window_id: 0,
        }
    }

    pub fn create_window(&mut self, name: &str) -> u32 {
        let win_id = self.windows.len() as u32;
        self.windows.push(TmuxWindow {
            window_id: win_id,
            name: name.to_string(),
            panes: vec![TmuxPane {
                pane_id: 0,
                active_cmd: "bash".to_string(),
            }],
        });
        self.active_window_id = win_id;
        win_id
    }

    pub fn split_pane(&mut self, window_id: u32, cmd: &str) -> bool {
        if let Some(win) = self.windows.iter_mut().find(|w| w.window_id == window_id) {
            let pane_id = win.panes.len() as u32;
            win.panes.push(TmuxPane {
                pane_id,
                active_cmd: cmd.to_string(),
            });
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_htop_process_visualizer() {
        let mut htop = HtopProcessVisualizer::new(16384);
        htop.update_process(101, "kernel_task", 5, 256, "running");
        htop.update_process(102, "zenith_wm", 12, 512, "running");

        assert_eq!(htop.processes.len(), 2);
        assert_eq!(htop.used_ram_mb, 768);
        assert!(htop.generate_summary_bar().contains("768MB/16384MB"));
    }

    #[test]
    fn test_ripgrep_fast_search() {
        let search = RipgrepFastSearch::new(false);
        let sample = "SigmaOS Kernel v1.0\nHigh performance Rust OS\nZero-dependency architecture";
        let matches = search.search_content("kernel.rs", sample, "rust");

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line_number, 2);
        assert_eq!(matches[0].line_text, "High performance Rust OS");
    }

    #[test]
    fn test_bat_syntax_printer() {
        let printer = BatSyntaxPrinter::new("nord");
        let rendered = printer.render_file("main.rs", "fn main() {\n    println!(\"Hello\");\n}");
        assert!(rendered.contains("File: main.rs"));
        assert!(rendered.contains("   1 │ fn main() {"));
    }

    #[test]
    fn test_fzf_fuzzy_matcher() {
        let fzf = FzfFuzzyMatcher::new();
        let candidates = vec!["/bin/bash", "/usr/bin/python", "/usr/local/bin/cargo"];
        let matched = fzf.fuzzy_match(&candidates, "python");
        assert_eq!(matched.len(), 1);
        assert_eq!(matched[0], "/usr/bin/python");
    }

    #[test]
    fn test_lazygit_repository_manager() {
        let mut git = LazygitRepositoryManager::new();
        git.unstaged_files.push("src/lib.rs".to_string());
        git.stage_file("src/lib.rs");

        let commit_res = git.commit("feat: initial commit").unwrap();
        assert!(commit_res.contains("feat: initial commit"));
        assert_eq!(git.commit_history.len(), 1);
    }

    #[test]
    fn test_tldr_page_summarizer() {
        let tldr = TldrPageSummarizer::new();
        let tar_summary = tldr.get_summary("tar").unwrap();
        assert!(tar_summary.contains("Extract gzipped archive"));
    }

    #[test]
    fn test_tmux_session_multiplexer() {
        let mut tmux = TmuxSessionMultiplexer::new("dev_session");
        assert_eq!(tmux.windows.len(), 1);

        let win2 = tmux.create_window("editor");
        assert_eq!(win2, 1);
        assert!(tmux.split_pane(1, "htop"));
        assert_eq!(tmux.windows[1].panes.len(), 2);
    }
}
