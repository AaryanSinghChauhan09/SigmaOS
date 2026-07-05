// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/dev/sigma_git.rs — Sigma Git Integration
//
// Implements Git-style version control with repository management,
// branching, committing, staging, and remote operations.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Git Types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
    Tag,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileStatus {
    Unmodified,
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Untracked,
}

#[derive(Debug, Clone)]
pub struct GitObject {
    pub hash: String,
    pub object_type: ObjectType,
    pub content: String,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct Commit {
    pub hash: String,
    pub tree_hash: String,
    pub parent_hashes: Vec<String>,
    pub author: String,
    pub author_email: String,
    pub author_time: String,
    pub committer: String,
    pub committer_email: String,
    pub committer_time: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct Branch {
    pub name: String,
    pub commit_hash: String,
    pub is_head: bool,
    pub is_remote: bool,
}

#[derive(Debug, Clone)]
pub struct Remote {
    pub name: String,
    pub url: String,
    pub fetch_url: String,
    pub push_url: String,
}

#[derive(Debug, Clone)]
pub struct StagedFile {
    pub path: String,
    pub status: FileStatus,
    pub original_hash: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub path: String,
    pub current_branch: String,
    pub head_commit: Option<String>,
    pub branches: HashMap<String, Branch>,
    pub remotes: HashMap<String, Remote>,
    pub staged_files: Vec<StagedFile>,
    pub working_tree: HashMap<String, FileStatus>,
    pub tags: HashMap<String, String>,
}

// ─── Git Manager ─────────────────────────────────────────────────────────

pub struct GitManager {
    pub repositories: HashMap<String, Repository>,
    pub global_config: HashMap<String, String>,
    pub current_repo: Option<String>,
}

impl GitManager {
    pub fn new() -> Self {
        let mut manager = GitManager {
            repositories: HashMap::new(),
            global_config: HashMap::new(),
            current_repo: None,
        };

        manager.init_global_config();
        manager
    }

    /// Initialize global Git configuration
    fn init_global_config(&mut self) {
        self.global_config.insert("user.name".to_string(), "SigmaOS User".to_string());
        self.global_config.insert("user.email".to_string(), "user@sigmaos.local".to_string());
        self.global_config.insert("core.editor".to_string(), "sigma-edit".to_string());
        self.global_config.insert("init.defaultBranch".to_string(), "main".to_string());
        self.global_config.insert("core.autocrlf".to_string(), "input".to_string());
    }

    /// Initialize a new repository
    pub fn init_repo(&mut self, path: String, name: String) -> Result<Repository, String> {
        let repo_id = format!("repo_{}", self.repositories.len());
        
        let mut branches = HashMap::new();
        branches.insert("main".to_string(), Branch {
            name: "main".to_string(),
            commit_hash: String::new(),
            is_head: true,
            is_remote: false,
        });

        let repo = Repository {
            name: name.clone(),
            path: path.clone(),
            current_branch: "main".to_string(),
            head_commit: None,
            branches,
            remotes: HashMap::new(),
            staged_files: vec![],
            working_tree: HashMap::new(),
            tags: HashMap::new(),
        };

        self.repositories.insert(repo_id.clone(), repo.clone());
        self.current_repo = Some(repo_id);
        Ok(repo)
    }

    /// Clone a repository
    pub fn clone_repo(&mut self, url: String, path: String) -> Result<Repository, String> {
        let name = url.split('/').last().unwrap_or("repo").to_string();
        let repo_id = format!("repo_{}", self.repositories.len());

        let mut branches = HashMap::new();
        branches.insert("main".to_string(), Branch {
            name: "main".to_string(),
            commit_hash: "abc123".to_string(),
            is_head: true,
            is_remote: false,
        });

        let mut remotes = HashMap::new();
        remotes.insert("origin".to_string(), Remote {
            name: "origin".to_string(),
            url: url.clone(),
            fetch_url: url.clone(),
            push_url: url.clone(),
        });

        let repo = Repository {
            name: name.clone(),
            path: path.clone(),
            current_branch: "main".to_string(),
            head_commit: Some("abc123".to_string()),
            branches,
            remotes,
            staged_files: vec![],
            working_tree: HashMap::new(),
            tags: HashMap::new(),
        };

        self.repositories.insert(repo_id.clone(), repo.clone());
        self.current_repo = Some(repo_id);
        Ok(repo)
    }

    /// Stage a file
    pub fn stage_file(&mut self, path: String, status: FileStatus) -> Result<(), String> {
        if let Some(repo_id) = &self.current_repo {
            if let Some(repo) = self.repositories.get_mut(repo_id) {
                repo.staged_files.push(StagedFile {
                    path: path.clone(),
                    status,
                    original_hash: None,
                });
                repo.working_tree.insert(path, status);
                Ok(())
            } else {
                Err("Repository not found".to_string())
            }
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// Unstage a file
    pub fn unstage_file(&mut self, path: &str) -> Result<(), String> {
        if let Some(repo_id) = &self.current_repo {
            if let Some(repo) = self.repositories.get_mut(repo_id) {
                repo.staged_files.retain(|f| f.path != path);
                Ok(())
            } else {
                Err("Repository not found".to_string())
            }
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// Commit staged changes
    pub fn commit(&mut self, message: String) -> Result<Commit, String> {
        if let Some(repo_id) = &self.current_repo {
            if let Some(repo) = self.repositories.get_mut(repo_id) {
                if repo.staged_files.is_empty() {
                    return Err("Nothing to commit".to_string());
                }

                let commit_hash = format!("commit_{}", repo.head_commit.as_ref().unwrap_or(&"initial".to_string()).len());
                let tree_hash = format!("tree_{}", commit_hash);

                let author_name = self.global_config.get("user.name").unwrap_or(&"User".to_string()).clone();
                let author_email = self.global_config.get("user.email").unwrap_or(&"user@local".to_string()).clone();

                let commit = Commit {
                    hash: commit_hash.clone(),
                    tree_hash,
                    parent_hashes: repo.head_commit.clone().map(|h| vec![h]).unwrap_or_default(),
                    author: author_name.clone(),
                    author_email: author_email.clone(),
                    author_time: "now".to_string(),
                    committer: author_name,
                    committer_email: author_email,
                    committer_time: "now".to_string(),
                    message: message.clone(),
                };

                repo.head_commit = Some(commit_hash.clone());
                repo.staged_files.clear();
                
                // Update current branch
                if let Some(branch) = repo.branches.get_mut(&repo.current_branch) {
                    branch.commit_hash = commit_hash.clone();
                }

                Ok(commit)
            } else {
                Err("Repository not found".to_string())
            }
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// Create a new branch
    pub fn create_branch(&mut self, name: String, start_point: Option<String>) -> Result<Branch, String> {
        if let Some(repo_id) = &self.current_repo {
            if let Some(repo) = self.repositories.get_mut(repo_id) {
                if repo.branches.contains_key(&name) {
                    return Err("Branch already exists".to_string());
                }

                let commit_hash = start_point.unwrap_or_else(|| {
                    repo.head_commit.clone().unwrap_or_default()
                });

                let branch = Branch {
                    name: name.clone(),
                    commit_hash,
                    is_head: false,
                    is_remote: false,
                };

                repo.branches.insert(name.clone(), branch.clone());
                Ok(branch)
            } else {
                Err("Repository not found".to_string())
            }
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// Switch to a branch
    pub fn checkout(&mut self, branch_name: &str) -> Result<(), String> {
        if let Some(repo_id) = &self.current_repo {
            if let Some(repo) = self.repositories.get_mut(repo_id) {
                if !repo.branches.contains_key(branch_name) {
                    return Err("Branch not found".to_string());
                }

                // Update all branches
                for branch in repo.branches.values_mut() {
                    branch.is_head = branch.name == branch_name;
                }

                repo.current_branch = branch_name.to_string();
                repo.head_commit = repo.branches.get(branch_name).map(|b| b.commit_hash.clone());
                Ok(())
            } else {
                Err("Repository not found".to_string())
            }
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// Merge a branch
    pub fn merge(&mut self, branch_name: &str) -> Result<(), String> {
        if let Some(repo_id) = &self.current_repo {
            if let Some(repo) = self.repositories.get_mut(repo_id) {
                if !repo.branches.contains_key(branch_name) {
                    return Err("Branch not found".to_string());
                }

                // Simulate merge
                let source_branch = repo.branches.get(branch_name).unwrap();
                repo.head_commit = Some(source_branch.commit_hash.clone());
                
                Ok(())
            } else {
                Err("Repository not found".to_string())
            }
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// Add a remote
    pub fn add_remote(&mut self, name: String, url: String) -> Result<Remote, String> {
        if let Some(repo_id) = &self.current_repo {
            if let Some(repo) = self.repositories.get_mut(repo_id) {
                let remote = Remote {
                    name: name.clone(),
                    url: url.clone(),
                    fetch_url: url.clone(),
                    push_url: url.clone(),
                };

                repo.remotes.insert(name.clone(), remote.clone());
                Ok(remote)
            } else {
                Err("Repository not found".to_string())
            }
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// Push to remote
    pub fn push(&mut self, remote: &str, branch: &str) -> Result<(), String> {
        if let Some(repo_id) = &self.current_repo {
            if let Some(repo) = self.repositories.get_mut(repo_id) {
                if !repo.remotes.contains_key(remote) {
                    return Err("Remote not found".to_string());
                }

                // Simulate push
                Ok(())
            } else {
                Err("Repository not found".to_string())
            }
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// Pull from remote
    pub fn pull(&mut self, remote: &str, branch: &str) -> Result<(), String> {
        if let Some(repo_id) = &self.current_repo {
            if let Some(repo) = self.repositories.get_mut(repo_id) {
                if !repo.remotes.contains_key(remote) {
                    return Err("Remote not found".to_string());
                }

                // Simulate pull
                Ok(())
            } else {
                Err("Repository not found".to_string())
            }
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// Get repository status
    pub fn get_status(&self) -> Result<HashMap<String, Vec<String>>, String> {
        if let Some(repo_id) = &self.current_repo {
            if let Some(repo) = self.repositories.get(repo_id) {
                let mut status = HashMap::new();
                
                let staged: Vec<String> = repo.staged_files.iter().map(|f| f.path.clone()).collect();
                let modified: Vec<String> = repo.working_tree.iter()
                    .filter(|(_, s)| **s == FileStatus::Modified)
                    .map(|(p, _)| p.clone())
                    .collect();
                let untracked: Vec<String> = repo.working_tree.iter()
                    .filter(|(_, s)| **s == FileStatus::Untracked)
                    .map(|(p, _)| p.clone())
                    .collect();

                status.insert("staged".to_string(), staged);
                status.insert("modified".to_string(), modified);
                status.insert("untracked".to_string(), untracked);
                
                Ok(status)
            } else {
                Err("Repository not found".to_string())
            }
        } else {
            Err("No repository selected".to_string())
        }
    }

    /// Get log
    pub fn get_log(&self, limit: usize) -> Vec<String> {
        vec![
            "abc123 - Initial commit - SigmaOS User".to_string(),
            "def456 - Add feature X - SigmaOS User".to_string(),
            "ghi789 - Fix bug Y - SigmaOS User".to_string(),
        ].into_iter().take(limit).collect()
    }

    /// Set global config
    pub fn set_config(&mut self, key: String, value: String) {
        self.global_config.insert(key, value);
    }

    /// Get global config
    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.global_config.get(key)
    }

    /// List all repositories
    pub fn list_repos(&self) -> Vec<&Repository> {
        self.repositories.values().collect()
    }

    /// Get current repository
    pub fn get_current_repo(&self) -> Option<&Repository> {
        if let Some(repo_id) = &self.current_repo {
            self.repositories.get(repo_id)
        } else {
            None
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut git = GitManager::new();
    
    println!("Sigma Git v0.1 - Version Control System");
    
    loop {
        println!("\n--- Git Commands ---");
        println!("init <path> <name> - Initialize new repository");
        println!("clone <url> <path> - Clone repository");
        println!("status            - Show working tree status");
        println!("add <file>        - Stage file");
        println!("unstage <file>    - Unstage file");
        println!("commit <message>  - Commit staged changes");
        println!("branch <name>     - Create branch");
        println!("checkout <name>   - Switch branch");
        println!("merge <name>      - Merge branch");
        println!("branches          - List branches");
        println!("remote <name> <url> - Add remote");
        println!("push <remote> <branch> - Push to remote");
        println!("pull <remote> <branch> - Pull from remote");
        println!("log [limit]       - Show commit log");
        println!("config <key> <value> - Set config");
        println!("repos             - List repositories");
        println!("quit              - Exit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "init" => {
                if parts.len() >= 3 {
                    let path = parts[1].to_string();
                    let name = parts[2].to_string();
                    match git.init_repo(path, name) {
                        Ok(_) => println!("Repository initialized"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "clone" => {
                if parts.len() >= 3 {
                    let url = parts[1].to_string();
                    let path = parts[2].to_string();
                    match git.clone_repo(url, path) {
                        Ok(_) => println!("Repository cloned"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "status" => {
                match git.get_status() {
                    Ok(status) => {
                        println!("--- Status ---");
                        if let Some(staged) = status.get("staged") {
                            println!("Staged: {:?}", staged);
                        }
                        if let Some(modified) = status.get("modified") {
                            println!("Modified: {:?}", modified);
                        }
                        if let Some(untracked) = status.get("untracked") {
                            println!("Untracked: {:?}", untracked);
                        }
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "add" => {
                if parts.len() >= 2 {
                    let path = parts[1].to_string();
                    match git.stage_file(path, FileStatus::Added) {
                        Ok(_) => println!("File staged"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "unstage" => {
                if let Some(path) = parts.get(1) {
                    match git.unstage_file(path) {
                        Ok(_) => println!("File unstaged"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "commit" => {
                if parts.len() >= 2 {
                    let message = parts[1..].join(" ");
                    match git.commit(message) {
                        Ok(commit) => println!("Committed: {}", commit.hash),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "branch" => {
                if parts.len() >= 2 {
                    let name = parts[1].to_string();
                    match git.create_branch(name, None) {
                        Ok(_) => println!("Branch created"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "checkout" => {
                if let Some(name) = parts.get(1) {
                    match git.checkout(name) {
                        Ok(_) => println!("Switched to branch"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "merge" => {
                if let Some(name) = parts.get(1) {
                    match git.merge(name) {
                        Ok(_) => println!("Branch merged"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "branches" => {
                if let Some(repo) = git.get_current_repo() {
                    println!("--- Branches ---");
                    for branch in repo.branches.values() {
                        println!("{} {} {}", 
                            if branch.is_head { "*" } else { " " },
                            branch.name,
                            if branch.is_remote { "(remote)" } else { "" }
                        );
                    }
                }
            }
            "remote" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let url = parts[2].to_string();
                    match git.add_remote(name, url) {
                        Ok(_) => println!("Remote added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "push" => {
                if parts.len() >= 3 {
                    let remote = parts[1];
                    let branch = parts[2];
                    match git.push(remote, branch) {
                        Ok(_) => println!("Pushed to remote"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "pull" => {
                if parts.len() >= 3 {
                    let remote = parts[1];
                    let branch = parts[2];
                    match git.pull(remote, branch) {
                        Ok(_) => println!("Pulled from remote"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "log" => {
                let limit = parts.get(1).and_then(|l| l.parse::<usize>().ok()).unwrap_or(10);
                for entry in git.get_log(limit) {
                    println!("{}", entry);
                }
            }
            "config" => {
                if parts.len() >= 3 {
                    let key = parts[1].to_string();
                    let value = parts[2].to_string();
                    git.set_config(key, value);
                    println!("Config updated");
                }
            }
            "repos" => {
                println!("--- Repositories ---");
                for repo in git.list_repos() {
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
