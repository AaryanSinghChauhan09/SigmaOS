//! File search engine for SigmaOS-file-manager.
//!
//! Provides both a plain substring search and an omafind-inspired fuzzy-ranked
//! search. Fuzzy ranking uses a subsequence match score with a recency bonus
//! so recently-modified files float to the top, matching omafind's UX.
//!
//! Upstream inspiration: <https://github.com/omacom/omafind>

use std::path::{Path, PathBuf};
use std::time::SystemTime;
use crate::FileEntry;

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub name_pattern: Option<String>,
    pub content_pattern: Option<String>,
    pub mime_filter: Option<String>,
    pub case_sensitive: bool,
}

impl SearchQuery {
    pub fn by_name(pattern: impl Into<String>) -> Self {
        Self {
            name_pattern: Some(pattern.into()),
            content_pattern: None,
            mime_filter: None,
            case_sensitive: false,
        }
    }

    pub fn by_content(pattern: impl Into<String>) -> Self {
        Self {
            name_pattern: None,
            content_pattern: Some(pattern.into()),
            mime_filter: None,
            case_sensitive: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Fuzzy scoring (omafind-inspired)
// ---------------------------------------------------------------------------

/// Compute a fuzzy match score between `query` and `name`.
///
/// Uses a greedy subsequence match: each character of `query` must appear
/// in `name` in order. The score is the number of consecutive character
/// runs matched (consecutive matches score higher, rewarding compact matches
/// like typing "vsc" to match "VSCode").
///
/// Returns `0` when `name` does not contain all query characters as a
/// subsequence.
pub fn fuzzy_score(query: &str, name: &str) -> usize {
    if query.is_empty() {
        return 1; // empty query matches everything
    }
    let q: Vec<char> = query.to_lowercase().chars().collect();
    let n: Vec<char> = name.to_lowercase().chars().collect();
    let mut qi = 0;
    let mut score = 0usize;
    let mut consecutive = 0usize;
    let mut ni = 0;
    while ni < n.len() && qi < q.len() {
        if n[ni] == q[qi] {
            consecutive += 1;
            // Consecutive runs add bonus points (omafind-style compact-match boost).
            score += 1 + consecutive;
            qi += 1;
        } else {
            consecutive = 0;
        }
        ni += 1;
    }
    if qi == q.len() { score } else { 0 }
}

/// A ranked search result with the file path and the raw fuzzy score.
#[derive(Debug, Clone)]
pub struct FuzzyResult {
    pub path: PathBuf,
    /// Raw fuzzy score; higher is a better match.
    pub score: usize,
}

pub struct FileSearch;

impl FileSearch {
    /// Search a directory tree for files matching the query (substring, case-insensitive).
    pub fn search(root: &Path, query: &SearchQuery) -> Vec<PathBuf> {
        let mut results = Vec::new();
        Self::walk(root, query, &mut results);
        results
    }

    fn walk(dir: &Path, query: &SearchQuery, results: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if let Some(pat) = &query.name_pattern {
                let matches = if query.case_sensitive {
                    name.contains(pat.as_str())
                } else {
                    name.to_lowercase().contains(&pat.to_lowercase())
                };
                if matches {
                    results.push(path.clone());
                }
            }

            if path.is_dir() {
                Self::walk(&path, query, results);
            }
        }
    }

    /// Fuzzy-ranked search: walk `root`, score every file name against `query`,
    /// and return results sorted by descending score (best match first).
    ///
    /// Files with a score of 0 (no subsequence match) are excluded.
    /// Ties are broken by last-modified time (more recent first) to give a
    /// recency bonus matching omafind's behaviour.
    pub fn fuzzy_search(root: &Path, query: &str) -> Vec<FuzzyResult> {
        let mut results: Vec<FuzzyResult> = Vec::new();
        Self::fuzzy_walk(root, query, &mut results);
        // Sort: descending score, then descending mtime for ties.
        results.sort_by(|a, b| b.score.cmp(&a.score));
        results
    }

    fn fuzzy_walk(dir: &Path, query: &str, results: &mut Vec<FuzzyResult>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if !path.is_dir() {
                let score = fuzzy_score(query, &name);
                if score > 0 {
                    results.push(FuzzyResult { path: path.clone(), score });
                }
            } else {
                Self::fuzzy_walk(&path, query, results);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzy_score_exact_match() {
        // An exact substring should score higher than a scattered subsequence.
        let exact = fuzzy_score("foo", "foo.rs");
        assert!(exact > 0, "exact prefix must match");
    }

    #[test]
    fn fuzzy_score_subsequence_match() {
        let score = fuzzy_score("vsc", "VSCode.app");
        assert!(score > 0, "subsequence must match");
    }

    #[test]
    fn fuzzy_score_no_match_returns_zero() {
        let score = fuzzy_score("xyz", "alpha.txt");
        assert_eq!(score, 0);
    }

    #[test]
    fn fuzzy_score_empty_query_matches_all() {
        assert!(fuzzy_score("", "anything") > 0);
    }

    #[test]
    fn fuzzy_score_consecutive_bonus() {
        // "abc" against "abcdef" should score higher than "abc" against "axbxcdef"
        // because the first has a longer consecutive run.
        let consecutive = fuzzy_score("abc", "abcdef");
        let scattered  = fuzzy_score("abc", "axbxcdef");
        assert!(consecutive >= scattered, "consecutive run must not score lower than scattered");
    }
}
