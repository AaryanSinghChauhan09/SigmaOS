//! VCS Integration - Git Version Control Engine for SigmaOS Toolchain
//! Zero external dependencies implementation using std::process::Command and native parsing

use std::path::{Path, PathBuf};
use std::process::Command;

/// Representation of a Git Commit
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitInfo {
    pub hash: String,
    pub author: String,
    pub email: String,
    pub date: String,
    pub message: String,
}

/// Representation of Git Status
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RepoStatus {
    pub current_branch: String,
    pub staged_files: Vec<String>,
    pub unstaged_files: Vec<String>,
    pub untracked_files: Vec<String>,
    pub is_clean: bool,
}

/// Representation of Git Diff Line or Chunk
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffEntry {
    pub file_path: String,
    pub status: String, // e.g. "modified", "added", "deleted"
    pub diff_content: String,
}

/// Result of a Merge Operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeResult {
    pub success: bool,
    pub message: String,
    pub conflicted_files: Vec<String>,
}

/// Main VCS Integration Manager
pub struct VcsManager {
    pub repo_path: PathBuf,
}

impl VcsManager {
    /// Create a new VCS Manager for the given repository path
    pub fn new(repo_path: impl AsRef<Path>) -> Self {
        Self {
            repo_path: repo_path.as_ref().to_path_buf(),
        }
    }

    /// Run a git command in the repository path
    fn run_git(&self, args: &[&str]) -> Result<String, String> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute git process: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let err_msg = String::from_utf8_lossy(&output.stderr).trim().to_string();
            if err_msg.is_empty() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err(err_msg)
            }
        }
    }

    /// Initialize a new Git repository
    pub fn init_repo(&self) -> Result<String, String> {
        self.run_git(&["init"])
    }

    /// Get current repository status
    pub fn get_status(&self) -> Result<RepoStatus, String> {
        let status_raw = self.run_git(&["status", "--porcelain=v1", "-b"])?;
        let mut status = RepoStatus::default();

        let mut lines = status_raw.lines();
        if let Some(header) = lines.next() {
            if header.starts_with("## ") {
                let branch_part = &header[3..];
                if let Some(dot_idx) = branch_part.find("...") {
                    status.current_branch = branch_part[..dot_idx].to_string();
                } else {
                    status.current_branch = branch_part.to_string();
                }
            }
        }

        for line in lines {
            if line.len() < 3 {
                continue;
            }
            let x = line.chars().next().unwrap_or(' ');
            let y = line.chars().nth(1).unwrap_or(' ');
            let file_path = line[3..].trim().to_string();

            if x == '?' && y == '?' {
                status.untracked_files.push(file_path);
            } else {
                if x != ' ' && x != '?' {
                    status.staged_files.push(file_path.clone());
                }
                if y != ' ' && y != '?' {
                    status.unstaged_files.push(file_path);
                }
            }
        }

        status.is_clean = status.staged_files.is_empty()
            && status.unstaged_files.is_empty()
            && status.untracked_files.is_empty();

        Ok(status)
    }

    /// Stage files for commit
    pub fn stage_file(&self, path: &str) -> Result<String, String> {
        self.run_git(&["add", path])
    }

    /// Stage all files
    pub fn stage_all(&self) -> Result<String, String> {
        self.run_git(&["add", "-A"])
    }

    /// Create a commit with a message
    pub fn commit(&self, message: &str) -> Result<String, String> {
        self.run_git(&["commit", "-m", message])
    }

    /// List recent commits
    pub fn get_log(&self, max_count: usize) -> Result<Vec<CommitInfo>, String> {
        let count_str = max_count.to_string();
        let log_raw = self.run_git(&[
            "log",
            &format!("-n{}", count_str),
            "--pretty=format:%H%x00%an%x00%ae%x00%ad%x00%s",
        ])?;

        if log_raw.is_empty() {
            return Ok(Vec::new());
        }

        let mut commits = Vec::new();
        for line in log_raw.lines() {
            let parts: Vec<&str> = line.split('\0').collect();
            if parts.len() >= 5 {
                commits.push(CommitInfo {
                    hash: parts[0].to_string(),
                    author: parts[1].to_string(),
                    email: parts[2].to_string(),
                    date: parts[3].to_string(),
                    message: parts[4].to_string(),
                });
            }
        }

        Ok(commits)
    }

    /// List all local branches
    pub fn list_branches(&self) -> Result<Vec<String>, String> {
        let branches_raw = self.run_git(&["branch", "--format=%(refname:short)"])?;
        if branches_raw.is_empty() {
            return Ok(Vec::new());
        }
        Ok(branches_raw.lines().map(|s| s.to_string()).collect())
    }

    /// Create a new branch
    pub fn create_branch(&self, branch_name: &str) -> Result<String, String> {
        self.run_git(&["branch", branch_name])
    }

    /// Switch to a branch
    pub fn switch_branch(&self, branch_name: &str) -> Result<String, String> {
        self.run_git(&["checkout", branch_name])
    }

    /// Delete a branch
    pub fn delete_branch(&self, branch_name: &str) -> Result<String, String> {
        self.run_git(&["branch", "-d", branch_name])
    }

    /// Merge a branch into current branch
    pub fn merge_branch(&self, branch_name: &str) -> Result<MergeResult, String> {
        match self.run_git(&["merge", branch_name]) {
            Ok(output) => Ok(MergeResult {
                success: true,
                message: output,
                conflicted_files: Vec::new(),
            }),
            Err(err_msg) => {
                let status = self.get_status().unwrap_or_default();
                let conflicted: Vec<String> = status
                    .unstaged_files
                    .into_iter()
                    .filter(|f| status.staged_files.contains(f))
                    .collect();

                Ok(MergeResult {
                    success: false,
                    message: err_msg,
                    conflicted_files: conflicted,
                })
            }
        }
    }

    /// View diff of unstaged changes or between targets
    pub fn get_diff(&self, target_a: Option<&str>, target_b: Option<&str>) -> Result<Vec<DiffEntry>, String> {
        let mut args = vec!["diff", "--stat"];
        if let Some(a) = target_a {
            args.push(a);
        }
        if let Some(b) = target_b {
            args.push(b);
        }

        let diff_raw = self.run_git(&args)?;
        if diff_raw.is_empty() {
            return Ok(Vec::new());
        }

        let mut entries = Vec::new();
        for line in diff_raw.lines() {
            if line.contains('|') {
                let parts: Vec<&str> = line.split('|').collect();
                if parts.len() == 2 {
                    let file_path = parts[0].trim().to_string();
                    let stat = parts[1].trim().to_string();
                    entries.push(DiffEntry {
                        file_path,
                        status: stat.clone(),
                        diff_content: format!("Diff summary: {}", stat),
                    });
                }
            }
        }

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_vcs_git_workflows() {
        let temp_dir = std::env::temp_dir().join(format!("test_vcs_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let _ = fs::create_dir_all(&temp_dir);

        let vcs = VcsManager::new(&temp_dir);

        // 1. Git init
        assert!(vcs.init_repo().is_ok());

        // Configure git user for test commit
        let _ = vcs.run_git(&["config", "user.name", "SigmaOS Tester"]);
        let _ = vcs.run_git(&["config", "user.email", "tester@sigmaos.org"]);

        // 2. Initial status
        let status1 = vcs.get_status().unwrap();
        assert!(status1.is_clean);

        // 3. Create a test file
        let file_path = temp_dir.join("README.md");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "# SigmaOS Repository").unwrap();

        let status2 = vcs.get_status().unwrap();
        assert!(!status2.is_clean);
        assert_eq!(status2.untracked_files, vec!["README.md".to_string()]);

        // 4. Stage and commit
        assert!(vcs.stage_file("README.md").is_ok());
        let status3 = vcs.get_status().unwrap();
        assert_eq!(status3.staged_files, vec!["README.md".to_string()]);

        assert!(vcs.commit("Initial commit").is_ok());
        let status4 = vcs.get_status().unwrap();
        assert!(status4.is_clean);

        // 5. Log
        let log = vcs.get_log(5).unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].message, "Initial commit");
        assert_eq!(log[0].author, "SigmaOS Tester");

        // 6. Branch management
        assert!(vcs.create_branch("feature-branch").is_ok());
        let branches = vcs.list_branches().unwrap();
        assert!(branches.contains(&"feature-branch".to_string()));

        assert!(vcs.switch_branch("feature-branch").is_ok());
        let status5 = vcs.get_status().unwrap();
        assert_eq!(status5.current_branch, "feature-branch");

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
