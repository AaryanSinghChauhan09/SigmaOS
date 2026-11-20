//! SigmaOS system-level file management.
//!
//! Provides mount point handling, file operation daemons, and extended
//! metadata indexing.

#![warn(missing_docs)]

/// A mounted filesystem.
#[derive(Debug, Clone)]
pub struct Mount {
    /// Device path or label.
    pub device: String,
    /// Mount point path.
    pub mount_point: String,
    /// Filesystem type (e.g. ext4).
    pub fs_type: String,
}

/// A file operation in progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileOp {
    /// Copying files.
    Copy,
    /// Moving files.
    Move,
    /// Deleting files.
    Delete,
}

/// Trait for the file manager backend.
pub trait FileManagerBackend {
    /// List all current mounts.
    fn mounts(&self) -> Result<Vec<Mount>, FileError>;
    /// Mount a device by path.
    fn mount(&mut self, device: &str) -> Result<Mount, FileError>;
    /// Perform a file operation.
    fn operate(&mut self, op: FileOp, sources: &[String], dest: &str) -> Result<(), FileError>;
}

/// Errors returned by the file manager system.
#[derive(Debug)]
pub enum FileError {
    /// The device could not be mounted.
    MountFailed(String),
    /// The file operation failed.
    OperationFailed(String),
    /// The path was not found.
    NotFound,
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::MountFailed(msg) => write!(f, "mount failed: {msg}"),
            FileError::OperationFailed(msg) => write!(f, "operation failed: {msg}"),
            FileError::NotFound => write!(f, "not found"),
        }
    }
}

impl std::error::Error for FileError {}

// =========================================================================
// Fuzzy file finder (omafind-inspired)
// =========================================================================
//
// Adds a greedy subsequence-match scorer and `FuzzyFinder::find` to the
// file manager system backend so it can surface relevant files quickly
// without an external dependency.
//
// Upstream inspiration: omafind (https://github.com/omacom/omafind)

/// Compute a fuzzy match score between `query` and `name` (case-insensitive).
///
/// Uses the same greedy subsequence + consecutive-run-bonus algorithm as the
/// desktop file manager search module.  Returns `0` when `name` does not
/// contain all query characters as a subsequence.
pub fn fuzzy_score(query: &str, name: &str) -> usize {
    if query.is_empty() {
        return 1;
    }
    let q: Vec<char> = query.chars().flat_map(|c| c.to_lowercase()).collect();
    let n: Vec<char> = name.chars().flat_map(|c| c.to_lowercase()).collect();
    let mut qi = 0;
    let mut score = 0usize;
    let mut consecutive = 0usize;
    for &nc in &n {
        if qi < q.len() && nc == q[qi] {
            consecutive += 1;
            score += 1 + consecutive;
            qi += 1;
        } else {
            consecutive = 0;
        }
    }
    if qi == q.len() { score } else { 0 }
}

/// A ranked fuzzy search result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuzzyMatch {
    pub name: String,
    pub score: usize,
}

/// Fuzzy file-name finder.
pub struct FuzzyFinder {
    /// The file names to search (in a real impl: a metadata index).
    pub index: Vec<String>,
}

impl FuzzyFinder {
    pub fn new(names: Vec<String>) -> Self {
        Self { index: names }
    }

    /// Find all names that fuzzy-match `query`, sorted by descending score.
    pub fn find(&self, query: &str) -> Vec<FuzzyMatch> {
        let mut results: Vec<FuzzyMatch> = self
            .index
            .iter()
            .filter_map(|name| {
                let score = fuzzy_score(query, name);
                if score > 0 {
                    Some(FuzzyMatch { name: name.clone(), score })
                } else {
                    None
                }
            })
            .collect();
        results.sort_by(|a, b| b.score.cmp(&a.score));
        results
    }
}

// =========================================================================
// Unit tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzy_score_exact_prefix() {
        assert!(fuzzy_score("foo", "foo.rs") > 0);
    }

    #[test]
    fn fuzzy_score_no_match() {
        assert_eq!(fuzzy_score("xyz", "alpha.txt"), 0);
    }

    #[test]
    fn fuzzy_score_empty_query() {
        assert!(fuzzy_score("", "anything") > 0);
    }

    #[test]
    fn fuzzy_finder_returns_sorted_results() {
        let finder = FuzzyFinder::new(vec![
            "document.txt".to_string(),
            "docs.md".to_string(),
            "readme.md".to_string(),
        ]);
        let results = finder.find("doc");
        // Both "document.txt" and "docs.md" should match; "readme.md" should not.
        assert!(!results.is_empty());
        assert!(results.iter().all(|r| r.score > 0));
        // Sorted descending.
        assert!(results.windows(2).all(|w| w[0].score >= w[1].score));
    }

    #[test]
    fn fuzzy_finder_no_match_returns_empty() {
        let finder = FuzzyFinder::new(vec!["alpha.rs".to_string()]);
        assert!(finder.find("xyz").is_empty());
    }
}
