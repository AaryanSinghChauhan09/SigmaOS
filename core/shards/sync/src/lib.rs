//! Sigma Sync Shard — GitHub integration and local state cache
//! Exposes a clean API usable from both sigmactl (CLI) and the GUI backend.

use std::process::{Command, Output};

#[derive(Debug)]
pub struct SyncResult {
    pub success: bool,
    pub message: String,
    pub commits_pushed: u32,
}

pub struct SigmaSync {
    pub repo_root: String,
}

impl SigmaSync {
    pub fn new(root: impl Into<String>) -> Self {
        Self { repo_root: root.into() }
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new("git")
            .args(args)
            .current_dir(&self.repo_root)
            .output()
            .unwrap_or_else(|_| panic!("git not found"))
    }

    pub fn fetch(&self) -> SyncResult {
        let out = self.run(&["fetch", "origin"]);
        SyncResult {
            success: out.status.success(),
            message: String::from_utf8_lossy(&out.stderr).trim().to_string(),
            commits_pushed: 0,
        }
    }

    pub fn push(&self) -> SyncResult {
        let ahead = self.run(&["rev-list", "HEAD...origin/main", "--count"]);
        let count: u32 = String::from_utf8_lossy(&ahead.stdout)
            .trim().parse().unwrap_or(0);
        let out = self.run(&["push"]);
        SyncResult {
            success: out.status.success(),
            message: String::from_utf8_lossy(&out.stderr).trim().to_string(),
            commits_pushed: count,
        }
    }

    pub fn pull_rebase(&self) -> SyncResult {
        let out = self.run(&["pull", "--rebase"]);
        let msg = String::from_utf8_lossy(&out.stdout).trim().to_string();
        SyncResult {
            success: out.status.success(),
            message: msg.clone(),
            commits_pushed: 0,
        }
    }

    pub fn status(&self) -> String {
        let branch = self.run(&["branch", "--show-current"]);
        let ahead  = self.run(&["rev-list", "HEAD...origin/main", "--count"]);
        format!(
            "branch={} ahead={}",
            String::from_utf8_lossy(&branch.stdout).trim(),
            String::from_utf8_lossy(&ahead.stdout).trim()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sync_struct_init() {
        let s = SigmaSync::new("/tmp");
        assert_eq!(s.repo_root, "/tmp");
    }
}
