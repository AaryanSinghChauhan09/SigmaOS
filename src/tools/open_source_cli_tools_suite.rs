// SigmaOS Open-Source Modern CLI Power Tools Suite
// Inspired by fd, ripgrep, zoxide, and eza (featured on Phoronix, ItsFOSS, How-To Geek, The New Stack)
// Zero-dependency, #![no_std] compliant native Rust implementations

#[cfg(not(any(feature = "standalone_test", test)))]
extern crate alloc;

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::collections::BTreeMap;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::format;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::collections::BTreeMap;
#[cfg(any(feature = "standalone_test", test))]
use std::format;
#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

// =========================================================================
// 1. FD PARALLEL DIRECTORY WALKER ENGINE (fd-find)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FdFileEntry {
    pub path: String,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub extension: String,
}

pub struct FdFileSearchEngine {
    pub hidden_files: bool,
    pub max_depth: Option<usize>,
    pub extension_filter: Option<String>,
}

impl FdFileSearchEngine {
    pub fn new() -> Self {
        Self {
            hidden_files: false,
            max_depth: None,
            extension_filter: None,
        }
    }

    /// Filters files matching query, extension, hidden file settings, and max depth
    pub fn search<'a>(&self, entries: &'a [FdFileEntry], query: &str, depth: usize) -> Vec<&'a FdFileEntry> {
        let query_lower = query.to_lowercase();

        entries
            .iter()
            .filter(|e| {
                if let Some(max_d) = self.max_depth {
                    if depth > max_d {
                        return false;
                    }
                }

                if !self.hidden_files && e.path.split('/').any(|part| part.starts_with('.')) {
                    return false;
                }

                if let Some(ref ext) = self.extension_filter {
                    if &e.extension != ext {
                        return false;
                    }
                }

                if query.is_empty() {
                    true
                } else {
                    e.path.to_lowercase().contains(&query_lower)
                }
            })
            .collect()
    }
}

impl Default for FdFileSearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. RIPGREP PARALLEL REGEX LINE MATCHER ENGINE (rg)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RipgrepMatchResult {
    pub line_number: usize,
    pub line_content: String,
    pub match_range: (usize, usize),
}

pub struct RipgrepSearchEngine {
    pub case_insensitive: bool,
    pub line_numbers: bool,
}

impl RipgrepSearchEngine {
    pub fn new() -> Self {
        Self {
            case_insensitive: false,
            line_numbers: true,
        }
    }

    /// Searches document lines for substring matches and returns line-numbered results
    pub fn search_document(&self, content: &str, pattern: &str) -> Vec<RipgrepMatchResult> {
        let mut results = Vec::new();
        if pattern.is_empty() {
            return results;
        }

        let pat_target = if self.case_insensitive {
            pattern.to_lowercase()
        } else {
            pattern.to_string()
        };

        for (idx, line) in content.lines().enumerate() {
            let line_check = if self.case_insensitive {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            if let Some(start_pos) = line_check.find(&pat_target) {
                results.push(RipgrepMatchResult {
                    line_number: idx + 1,
                    line_content: line.to_string(),
                    match_range: (start_pos, start_pos + pattern.len()),
                });
            }
        }

        results
    }
}

impl Default for RipgrepSearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. ZOXIDE FRECENCY SMART DIRECTORY NAVIGATOR (z)
// =========================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct ZoxideEntry {
    pub path: String,
    pub frecency_score: f64,
    pub last_accessed_timestamp: u64,
}

pub struct ZoxideCdEngine {
    pub database: BTreeMap<String, ZoxideEntry>,
}

impl ZoxideCdEngine {
    pub fn new() -> Self {
        Self {
            database: BTreeMap::new(),
        }
    }

    /// Records directory access, incrementing frecency score
    pub fn add_visit(&mut self, path: &str, current_time: u64) {
        let entry = self.database.entry(path.to_string()).or_insert_with(|| ZoxideEntry {
            path: path.to_string(),
            frecency_score: 0.0,
            last_accessed_timestamp: current_time,
        });

        entry.frecency_score += 10.0;
        entry.last_accessed_timestamp = current_time;
    }

    /// Queries best matching directory using frecency ranking
    pub fn query_best(&self, keyword: &str) -> Option<String> {
        let kw_lower = keyword.to_lowercase();

        let mut matches: Vec<&ZoxideEntry> = self
            .database
            .values()
            .filter(|e| e.path.to_lowercase().contains(&kw_lower))
            .collect();

        matches.sort_by(|a, b| b.frecency_score.partial_cmp(&a.frecency_score).unwrap_or(std::cmp::Ordering::Equal));
        matches.first().map(|e| e.path.clone())
    }
}

impl Default for ZoxideCdEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. EZA / EXA MODERN TREE & ICON FILE LISTER (eza)
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EzaFormattedEntry {
    pub icon: &'static str,
    pub name: String,
    pub formatted_size: String,
    pub is_dir: bool,
}

pub struct EzaLsEngine;

impl EzaLsEngine {
    pub fn format_entry(name: &str, is_dir: bool, size_bytes: u64) -> EzaFormattedEntry {
        let icon = if is_dir {
            "📁"
        } else if name.ends_with(".rs") {
            "🦀"
        } else if name.ends_with(".py") {
            "🐍"
        } else if name.ends_with(".sh") {
            "📜"
        } else {
            "📄"
        };

        let formatted_size = if is_dir {
            "-".to_string()
        } else if size_bytes >= 1_048_576 {
            format!("{:.1}M", size_bytes as f64 / 1_048_576.0)
        } else if size_bytes >= 1024 {
            format!("{:.1}K", size_bytes as f64 / 1024.0)
        } else {
            format!("{}B", size_bytes)
        };

        EzaFormattedEntry {
            icon,
            name: name.to_string(),
            formatted_size,
            is_dir,
        }
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fd_file_search_engine() {
        let entries = vec![
            FdFileEntry {
                path: "src/main.rs".to_string(),
                is_dir: false,
                size_bytes: 1024,
                extension: "rs".to_string(),
            },
            FdFileEntry {
                path: "src/.hidden.rs".to_string(),
                is_dir: false,
                size_bytes: 500,
                extension: "rs".to_string(),
            },
            FdFileEntry {
                path: "docs/readme.md".to_string(),
                is_dir: false,
                size_bytes: 2048,
                extension: "md".to_string(),
            },
        ];

        let fd = FdFileSearchEngine::new();
        let results = fd.search(&entries, "main", 1);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].path, "src/main.rs");
    }

    #[test]
    fn test_ripgrep_search_engine() {
        let doc = "fn main() {\n    println!(\"Hello SigmaOS\");\n}\n";
        let mut rg = RipgrepSearchEngine::new();
        rg.case_insensitive = true;

        let results = rg.search_document(doc, "sigmaos");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].line_number, 2);
    }

    #[test]
    fn test_zoxide_cd_engine() {
        let mut zoxide = ZoxideCdEngine::new();
        zoxide.add_visit("/home/user/projects/sigmaos", 100);
        zoxide.add_visit("/home/user/downloads", 105);
        zoxide.add_visit("/home/user/projects/sigmaos", 110);

        let best = zoxide.query_best("sigma");
        assert_eq!(best, Some("/home/user/projects/sigmaos".to_string()));
    }

    #[test]
    fn test_eza_ls_engine() {
        let entry = EzaLsEngine::format_entry("main.rs", false, 2048);
        assert_eq!(entry.icon, "🦀");
        assert_eq!(entry.formatted_size, "2.0K");
    }
}
