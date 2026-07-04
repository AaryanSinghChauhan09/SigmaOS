// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/integration/sigma_git.rs — Sigma Git Integration
//
// Implements Git version control integration with visual diff,
// branch management, and commit history visualization.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Git Types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GitOperation {
    Init,
    Clone,
    Add,
    Commit,
    Push,
    Pull,
    Branch,
    Merge,
    Checkout,
    Status,
    Log,
    Diff,
}

#[derive(Debug, Clone)]
pub struct GitCommit {
    pub hash: String,
    pub author: String,
    pub date: String,
    pub message: String,
    pub files_changed: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct GitBranch {
    pub name: String,
    pub is_head: bool,
    pub is_remote: bool,
    pub last_commit: String,
}

#[derive(Debug, Clone)]
pub struct GitRepository {
    pub name: String,
    pub path: String,
    pub current_branch: String,
    pub branches: Vec<GitBranch>,
    pub commits: Vec<GitCommit>,
    pub status: String,
}

// ─── Git Integration Manager ─────────────────────────────────────────────────

pub struct GitManager {
    pub repositories: HashMap<String, GitRepository>,
    pub current_repo: Option<String>,
}

impl GitManager {
    pub fn new() -> Self {
        let mut manager = GitManager {
            repositories: HashMap::new(),
            current_repo: None,
        };
        
        manager.init_sample_repo();
        manager
    }

    /// Initialize sample repository
    fn init_sample_repo(&mut self) {
        let repo = GitRepository {
            name: "sigmaos".to_string(),
            path: "/home/user/sigmaos".to_string(),
            current_branch: "main".to_string(),
            branches: vec![
                GitBranch {
                    name: "main".to_string(),
                    is_head: true,
                    is_remote: false,
                    last_commit: "abc123".to_string(),
                },
                GitBranch {
                    name: "feature/new-tools".to_string(),
                    is_head: false,
                    is_remote: false,
                    last_commit: "def456".to_string(),
                },
                GitBranch {
                    name: "origin/main".to_string(),
                    is_head: false,
                    is_remote: true,
                    last_commit: "abc123".to_string(),
                },
            ],
            commits: vec![
                GitCommit {
                    hash: "abc123".to_string(),
                    author: "Aaryan Singh".to_string(),
                    date: "2024-01-15".to_string(),
                    message: "Add Security & Law features".to_string(),
                    files_changed: vec!["usr/security/".to_string(), "usr/law/".to_string()],
                },
                GitCommit {
                    hash: "def456".to_string(),
                    author: "Aaryan Singh".to_string(),
                    date: "2024-01-14".to_string(),
                    message: "Add Education extensions".to_string(),
                    files_changed: vec!["usr/education/".to_string()],
                },
                GitCommit {
                    hash: "ghi789".to_string(),
                    author: "Aaryan Singh".to_string(),
                    date: "2024-01-13".to_string(),
                    message: "Initial commit".to_string(),
                    files_changed: vec!["README.md".to_string(), "Cargo.toml".to_string()],
                },
            ],
            status: "On branch main, working tree clean".to_string(),
        };
        
        self.repositories.insert(repo.name.clone(), repo);
    }

    /// Set current repository
    pub fn set_current_repo(&mut self, name: &str) -> Result<(), String> {
        if self.repositories.contains_key(name) {
            self.current_repo = Some(name.to_string());
            Ok(())
        } else {
            Err("Repository not found".to_string())
        }
    }

    /// Get current repository
    pub fn get_current_repo(&self) -> Option<&GitRepository> {
        self.current_repo.as_ref()
            .and_then(|name| self.repositories.get(name))
    }

    /// Execute Git operation (simulated)
    pub fn execute_operation(&mut self, operation: GitOperation, args: &[String]) -> Result<String, String> {
        match operation {
            GitOperation::Status => self.get_status(),
            GitOperation::Log => self.get_log(),
            GitOperation::Branch => self.list_branches(),
            GitOperation::Diff => self.get_diff(args),
            GitOperation::Add => self.add_files(args),
            GitOperation::Commit => self.create_commit(args),
            GitOperation::Push => self.push_changes(),
            GitOperation::Pull => self.pull_changes(),
            GitOperation::Checkout => self.checkout_branch(args),
            GitOperation::Merge => self.merge_branch(args),
            GitOperation::Init => self.init_repo(args),
            GitOperation::Clone => self.clone_repo(args),
        }
    }

    /// Get repository status
    fn get_status(&self) -> Result<String, String> {
        if let Some(repo) = self.get_current_repo() {
            Ok(format!("On branch {}\n{}", repo.current_branch, repo.status))
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// Get commit log
    fn get_log(&self) -> Result<String, String> {
        if let Some(repo) = self.get_current_repo() {
            let mut log = String::new();
            for commit in &repo.commits {
                log.push_str(&format!("commit {}\n", commit.hash));
                log.push_str(&format!("Author: {}\n", commit.author));
                log.push_str(&format!("Date: {}\n", commit.date));
                log.push_str(&format!("    {}\n\n", commit.message));
            }
            Ok(log)
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// List branches
    fn list_branches(&self) -> Result<String, String> {
        if let Some(repo) = self.get_current_repo() {
            let mut branches = String::new();
            for branch in &repo.branches {
                let marker = if branch.is_head { "* " } else { "  " };
                let remote = if branch.is_remote { " (remote)" } else { "" };
                branches.push_str(&format!("{}{}{}\n", marker, branch.name, remote));
            }
            Ok(branches)
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// Get diff
    fn get_diff(&self, _args: &[String]) -> Result<String, String> {
        Ok("diff --git a/usr/example.rs b/usr/example.rs\nindex abc123..def456 100644\n--- a/usr/example.rs\n+++ b/usr/example.rs\n@@ -1,5 +1,6 @@\n fn main() {\n-    println!(\"Hello\");\n+    println!(\"Hello, World!\");\n }\n".to_string())
    }

    /// Add files
    fn add_files(&mut self, _args: &[String]) -> Result<String, String> {
        Ok("Files staged for commit".to_string())
    }

    /// Create commit
    fn create_commit(&mut self, args: &[String]) -> Result<String, String> {
        let message = args.get(0).cloned().unwrap_or_else(|| "Update".to_string());
        
        if let Some(repo_name) = &self.current_repo {
            if let Some(repo) = self.repositories.get_mut(repo_name) {
                let new_commit = GitCommit {
                    hash: format!("{:x}", md5_compute(&repo.commits.len().to_string())),
                    author: "User".to_string(),
                    date: "now".to_string(),
                    message,
                    files_changed: vec!["usr/".to_string()],
                };
                repo.commits.insert(0, new_commit);
                return Ok(format!("Commit created: {}", repo.commits[0].hash));
            }
        }
        
        Err("No repository selected".to_string())
    }

    /// Push changes
    fn push_changes(&self) -> Result<String, String> {
        Ok("Changes pushed to remote".to_string())
    }

    /// Pull changes
    fn pull_changes(&self) -> Result<String, String> {
        Ok("Changes pulled from remote".to_string())
    }

    /// Checkout branch
    fn checkout_branch(&mut self, args: &[String]) -> Result<String, String> {
        if let Some(branch_name) = args.get(0) {
            if let Some(repo_name) = &self.current_repo {
                if let Some(repo) = self.repositories.get_mut(repo_name) {
                    if repo.branches.iter().any(|b| &b.name == branch_name) {
                        repo.current_branch = branch_name.clone();
                        for branch in &mut repo.branches {
                            branch.is_head = &branch.name == branch_name;
                        }
                        return Ok(format!("Switched to branch '{}'", branch_name));
                    }
                }
            }
            Err("Branch not found".to_string())
        } else {
            Err("Branch name required".to_string())
        }
    }

    /// Merge branch
    fn merge_branch(&self, _args: &[String]) -> Result<String, String> {
        Ok("Branch merged successfully".to_string())
    }

    /// Initialize repository
    fn init_repo(&mut self, args: &[String]) -> Result<String, String> {
        let name = args.get(0).cloned().unwrap_or_else(|| "new-repo".to_string());
        
        let repo = GitRepository {
            name: name.clone(),
            path: format!("/home/user/{}", name),
            current_branch: "main".to_string(),
            branches: vec![
                GitBranch {
                    name: "main".to_string(),
                    is_head: true,
                    is_remote: false,
                    last_commit: "initial".to_string(),
                },
            ],
            commits: Vec::new(),
            status: "Initialized empty Git repository".to_string(),
        };
        
        self.repositories.insert(name.clone(), repo);
        self.current_repo = Some(name);
        Ok("Repository initialized".to_string())
    }

    /// Clone repository
    fn clone_repo(&mut self, args: &[String]) -> Result<String, String> {
        let url = args.get(0).cloned().unwrap_or_else(|| "https://github.com/user/repo".to_string());
        let name = args.get(1).cloned().unwrap_or_else(|| "repo".to_string());
        
        let repo = GitRepository {
            name: name.clone(),
            path: format!("/home/user/{}", name),
            current_branch: "main".to_string(),
            branches: vec![
                GitBranch {
                    name: "main".to_string(),
                    is_head: true,
                    is_remote: false,
                    last_commit: "cloned".to_string(),
                },
                GitBranch {
                    name: "origin/main".to_string(),
                    is_head: false,
                    is_remote: true,
                    last_commit: "cloned".to_string(),
                },
            ],
            commits: Vec::new(),
            status: format!("Cloned from {}", url),
        };
        
        self.repositories.insert(name.clone(), repo);
        self.current_repo = Some(name);
        Ok(format!("Repository cloned from {}", url))
    }

    /// Get all repositories
    pub fn get_all_repos(&self) -> Vec<&GitRepository> {
        self.repositories.values().collect()
    }
}

// Simple MD5-like hash function for demo
fn md5_compute(input: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in input.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
    }
    hash
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut manager = GitManager::new();
    
    println!("Sigma Git Integration v0.1 - Version Control");
    
    loop {
        println!("\n--- Current Repository ---");
        if let Some(repo) = manager.get_current_repo() {
            println!("Name: {}", repo.name);
            println!("Branch: {}", repo.current_branch);
            println!("Status: {}", repo.status);
        } else {
            println!("No repository selected");
        }
        
        println!("\nCommands: repo <name>, init <name>, clone <url> [name], status, log, branch, diff, add <files>, commit <message>, push, pull, checkout <branch>, merge <branch>, repos, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "repo" => {
                if let Some(arg) = parts.get(1) {
                    match manager.set_current_repo(arg) {
                        Ok(_) => println!("Repository selected"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "init" => {
                let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                match manager.execute_operation(GitOperation::Init, &args) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "clone" => {
                let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                match manager.execute_operation(GitOperation::Clone, &args) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "status" => {
                match manager.execute_operation(GitOperation::Status, &[]) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "log" => {
                match manager.execute_operation(GitOperation::Log, &[]) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "branch" => {
                match manager.execute_operation(GitOperation::Branch, &[]) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "diff" => {
                let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                match manager.execute_operation(GitOperation::Diff, &args) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "add" => {
                let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                match manager.execute_operation(GitOperation::Add, &args) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "commit" => {
                let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                match manager.execute_operation(GitOperation::Commit, &args) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "push" => {
                match manager.execute_operation(GitOperation::Push, &[]) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "pull" => {
                match manager.execute_operation(GitOperation::Pull, &[]) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "checkout" => {
                let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                match manager.execute_operation(GitOperation::Checkout, &args) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "merge" => {
                let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                match manager.execute_operation(GitOperation::Merge, &args) {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "repos" => {
                println!("--- All Repositories ---");
                for repo in manager.get_all_repos() {
                    println!("{} - {} ({})", repo.name, repo.path, repo.current_branch);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
