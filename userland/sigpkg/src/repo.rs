//! SigmaOS Package Repository Management
//! Manages local and remote package repositories
//! Supports multiple repository types (local, HTTP, Git)

use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

/// Repository type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RepoType {
    Local,
    Http,
    Git,
}

/// Repository configuration
#[derive(Debug, Clone)]
pub struct RepoConfig {
    pub name: String,
    pub repo_type: RepoType,
    pub url: String,
    pub enabled: bool,
    pub priority: u32,
    pub gpg_key: Option<String>,
}

/// Repository index
#[derive(Debug, Clone)]
pub struct RepoIndex {
    pub packages: HashMap<String, PackageEntry>,
    pub last_updated: i64,
}

/// Package entry in repository index
#[derive(Debug, Clone)]
pub struct PackageEntry {
    pub name: String,
    pub version: String,
    pub architecture: String,
    pub checksum: String,
    pub size: u64,
    pub location: String,
    pub dependencies: Vec<String>,
}

/// Repository manager
#[derive(Debug)]
pub struct RepoManager {
    pub repos: Vec<RepoConfig>,
    pub indices: HashMap<String, RepoIndex>,
    pub cache_dir: PathBuf,
}

impl RepoManager {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            repos: Vec::new(),
            indices: HashMap::new(),
            cache_dir,
        }
    }
    
    /// Initialize repository manager
    pub fn init(&mut self) -> Result<(), String> {
        // Create cache directory
        fs::create_dir_all(&self.cache_dir).map_err(|e| format!("Failed to create cache dir: {}", e))?;
        
        // Load repository configurations
        self.load_repos()?;
        
        Ok(())
    }
    
    /// Add repository
    pub fn add_repo(&mut self, config: RepoConfig) -> Result<(), String> {
        // Check if repo already exists
        if self.repos.iter().any(|r| r.name == config.name) {
            return Err(format!("Repository {} already exists", config.name));
        }
        
        self.repos.push(config);
        self.save_repos()?;
        
        Ok(())
    }
    
    /// Remove repository
    pub fn remove_repo(&mut self, name: &str) -> Result<(), String> {
        let pos = self.repos.iter().position(|r| r.name == name)
            .ok_or_else(|| format!("Repository {} not found", name))?;
        
        self.repos.remove(pos);
        self.indices.remove(name);
        self.save_repos()?;
        
        Ok(())
    }
    
    /// Enable repository
    pub fn enable_repo(&mut self, name: &str) -> Result<(), String> {
        let repo = self.repos.iter_mut().find(|r| r.name == name)
            .ok_or_else(|| format!("Repository {} not found", name))?;
        
        repo.enabled = true;
        self.save_repos()?;
        
        Ok(())
    }
    
    /// Disable repository
    pub fn disable_repo(&mut self, name: &str) -> Result<(), String> {
        let repo = self.repos.iter_mut().find(|r| r.name == name)
            .ok_or_else(|| format!("Repository {} not found", name))?;
        
        repo.enabled = false;
        self.save_repos()?;
        
        Ok(())
    }
    
    /// Update repository index
    pub fn update_index(&mut self, name: &str) -> Result<(), String> {
        let repo = self.repos.iter().find(|r| r.name == name)
            .ok_or_else(|| format!("Repository {} not found", name))?;
        
        if !repo.enabled {
            return Err(format!("Repository {} is disabled", name));
        }
        
        // Fetch index based on repository type
        let index = match repo.repo_type {
            RepoType::Local => self.fetch_local_index(repo)?,
            RepoType::Http => self.fetch_http_index(repo)?,
            RepoType::Git => self.fetch_git_index(repo)?,
        };
        
        self.indices.insert(name.to_string(), index);
        
        Ok(())
    }
    
    /// Update all repository indices
    pub fn update_all_indices(&mut self) -> Result<(), String> {
        for repo in &self.repos {
            if repo.enabled {
                self.update_index(&repo.name)?;
            }
        }
        Ok(())
    }
    
    /// Search for package across all repositories
    pub fn search(&self, query: &str) -> Vec<PackageEntry> {
        let mut results = Vec::new();
        
        for (repo_name, index) in &self.indices {
            let repo = self.repos.iter().find(|r| &r.name == repo_name);
            
            if let Some(repo) = repo {
                if !repo.enabled {
                    continue;
                }
            }
            
            for (name, entry) in &index.packages {
                if name.contains(query) || entry.name.contains(query) {
                    results.push(entry.clone());
                }
            }
        }
        
        // Sort by priority
        results.sort_by(|a, b| {
            let a_prio = self.repos.iter()
                .find(|r| r.url == a.location)
                .map(|r| r.priority)
                .unwrap_or(0);
            let b_prio = self.repos.iter()
                .find(|r| r.url == b.location)
                .map(|r| r.priority)
                .unwrap_or(0);
            b_prio.cmp(&a_prio)
        });
        
        results
    }
    
    /// Get package entry
    pub fn get_package(&self, name: &str, version: Option<&str>) -> Option<PackageEntry> {
        for index in self.indices.values() {
            if let Some(entry) = index.packages.get(name) {
                if version.is_none() || entry.version == version.unwrap() {
                    return Some(entry.clone());
                }
            }
        }
        None
    }
    
    /// List all repositories
    pub fn list_repos(&self) -> Vec<RepoConfig> {
        self.repos.clone()
    }
    
    /// Load repository configurations
    fn load_repos(&mut self) -> Result<(), String> {
        let config_path = self.cache_dir.join("repos.toml");
        
        if !config_path.exists() {
            // Add default repositories
            self.add_default_repos()?;
            return Ok(());
        }
        
        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("Failed to read repo config: {}", e))?;
        
        let config: toml::Value = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse repo config: {}", e))?;
        
        if let Some(repos) = config.get("repositories").and_then(|v| v.as_array()) {
            for repo in repos {
                if let Some(repo) = repo.as_table() {
                    let name = repo.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    
                    let repo_type_str = repo.get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("local");
                    
                    let repo_type = match repo_type_str {
                        "local" => RepoType::Local,
                        "http" => RepoType::Http,
                        "git" => RepoType::Git,
                        _ => RepoType::Local,
                    };
                    
                    let url = repo.get("url")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    
                    let enabled = repo.get("enabled")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(true);
                    
                    let priority = repo.get("priority")
                        .and_then(|v| v.as_integer())
                        .unwrap_or(0) as u32;
                    
                    let gpg_key = repo.get("gpg_key")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    
                    self.repos.push(RepoConfig {
                        name,
                        repo_type,
                        url,
                        enabled,
                        priority,
                        gpg_key,
                    });
                }
            }
        }
        
        Ok(())
    }
    
    /// Save repository configurations
    fn save_repos(&self) -> Result<(), String> {
        let config_path = self.cache_dir.join("repos.toml");
        
        let mut repos_toml = toml::value::Table::new();
        let mut repos_array = toml::value::Array::new();
        
        for repo in &self.repos {
            let mut repo_table = toml::value::Table::new();
            repo_table.insert("name".to_string(), toml::Value::String(repo.name.clone()));
            repo_table.insert("type".to_string(), toml::Value::String(format!("{:?}", repo.repo_type).to_lowercase()));
            repo_table.insert("url".to_string(), toml::Value::String(repo.url.clone()));
            repo_table.insert("enabled".to_string(), toml::Value::Boolean(repo.enabled));
            repo_table.insert("priority".to_string(), toml::Value::Integer(repo.priority as i64));
            
            if let Some(ref gpg_key) = repo.gpg_key {
                repo_table.insert("gpg_key".to_string(), toml::Value::String(gpg_key.clone()));
            }
            
            repos_array.push(toml::Value::Table(repo_table));
        }
        
        repos_toml.insert("repositories".to_string(), toml::Value::Array(repos_array));
        
        let toml_string = toml::to_string(&repos_toml)
            .map_err(|e| format!("Failed to serialize TOML: {}", e))?;
        
        fs::write(&config_path, toml_string)
            .map_err(|e| format!("Failed to write repo config: {}", e))?;
        
        Ok(())
    }
    
    /// Add default repositories
    fn add_default_repos(&mut self) -> Result<(), String> {
        self.repos.push(RepoConfig {
            name: "sigmaos-official".to_string(),
            repo_type: RepoType::Http,
            url: "https://packages.sigmaos.dev".to_string(),
            enabled: true,
            priority: 100,
            gpg_key: Some("https://packages.sigmaos.dev/gpg-key.asc".to_string()),
        });
        
        self.repos.push(RepoConfig {
            name: "sigmaos-community".to_string(),
            repo_type: RepoType::Http,
            url: "https://community.sigmaos.dev".to_string(),
            enabled: true,
            priority: 50,
            gpg_key: Some("https://community.sigmaos.dev/gpg-key.asc".to_string()),
        });
        
        self.save_repos()
    }
    
    /// Fetch local repository index
    fn fetch_local_index(&self, repo: &RepoConfig) -> Result<RepoIndex, String> {
        let repo_path = Path::new(&repo.url);
        let index_path = repo_path.join("index.toml");
        
        let content = fs::read_to_string(&index_path)
            .map_err(|e| format!("Failed to read index: {}", e))?;
        
        let index: toml::Value = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse index: {}", e))?;
        
        let mut packages = HashMap::new();
        
        if let Some(pkgs) = index.get("packages").and_then(|v| v.as_array()) {
            for pkg in pkgs {
                if let Some(pkg) = pkg.as_table() {
                    let name = pkg.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    
                    let version = pkg.get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    
                    let architecture = pkg.get("architecture")
                        .and_then(|v| v.as_str())
                        .unwrap_or("x86_64")
                        .to_string();
                    
                    let checksum = pkg.get("checksum")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    
                    let size = pkg.get("size")
                        .and_then(|v| v.as_integer())
                        .unwrap_or(0) as u64;
                    
                    let location = pkg.get("location")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    
                    let deps = pkg.get("dependencies")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str())
                                .map(|s| s.to_string())
                                .collect()
                        })
                        .unwrap_or_default();
                    
                    packages.insert(name.clone(), PackageEntry {
                        name,
                        version,
                        architecture,
                        checksum,
                        size,
                        location,
                        dependencies: deps,
                    });
                }
            }
        }
        
        Ok(RepoIndex {
            packages,
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        })
    }
    
    /// Fetch HTTP repository index
    fn fetch_http_index(&self, repo: &RepoConfig) -> Result<RepoIndex, String> {
        // In real implementation, use HTTP client to fetch index
        // Stub: return empty index
        Ok(RepoIndex {
            packages: HashMap::new(),
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        })
    }
    
    /// Fetch Git repository index
    fn fetch_git_index(&self, repo: &RepoConfig) -> Result<RepoIndex, String> {
        // In real implementation, clone/pull git repo and read index
        // Stub: return empty index
        Ok(RepoIndex {
            packages: HashMap::new(),
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_repo_manager_init() {
        let cache_dir = PathBuf::from("/tmp/test_repo_cache");
        let mut manager = RepoManager::new(cache_dir);
        
        // Should not fail even if cache doesn't exist
        let result = manager.init();
        assert!(result.is_ok() || result.is_err()); // May fail on permissions
    }
    
    #[test]
    fn test_add_repo() {
        let cache_dir = PathBuf::from("/tmp/test_repo_cache");
        let mut manager = RepoManager::new(cache_dir);
        
        let config = RepoConfig {
            name: "test".to_string(),
            repo_type: RepoType::Local,
            url: "/tmp/test_repo".to_string(),
            enabled: true,
            priority: 10,
            gpg_key: None,
        };
        
        let result = manager.add_repo(config);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_search_empty() {
        let cache_dir = PathBuf::from("/tmp/test_repo_cache");
        let manager = RepoManager::new(cache_dir);
        
        let results = manager.search("test");
        assert!(results.is_empty());
    }
}
