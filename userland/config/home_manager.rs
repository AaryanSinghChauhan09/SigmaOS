// Home Manager for SigmaOS
// Implements user-specific configuration management
// Inspired by Nix Home Manager for declarative user environment management

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserConfig {
    pub username: String,
    pub home_dir: PathBuf,
    pub shell: String,
    pub editor: String,
    pub terminal: String,
    pub environment: HashMap<String, String>,
    pub aliases: HashMap<String, String>,
    pub packages: Vec<String>,
    pub services: Vec<UserService>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserService {
    pub name: String,
    pub enabled: bool,
    pub command: String,
    pub autostart: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HomeManagerConfig {
    pub users: HashMap<String, UserConfig>,
    pub global_settings: GlobalSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalSettings {
    pub default_shell: String,
    pub default_editor: String,
    pub default_terminal: String,
    pub enable_auto_update: bool,
}

pub struct HomeManager {
    config_path: PathBuf,
    config: HomeManagerConfig,
}

impl HomeManager {
    pub fn new(config_path: PathBuf) -> Result<Self, std::io::Error> {
        let config = Self::load_config(&config_path)?;
        
        Ok(HomeManager {
            config_path,
            config,
        })
    }

    /// Apply user configuration for a specific user
    pub fn apply_user_config(&self, username: &str) -> Result<(), std::io::Error> {
        if let Some(user_config) = self.config.users.get(username) {
            self.apply_shell_config(user_config)?;
            self.apply_environment_variables(user_config)?;
            self.apply_aliases(user_config)?;
            self.install_user_packages(user_config)?;
            self.enable_user_services(user_config)?;
            
            println!("Applied configuration for user: {}", username);
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("User {} not found in configuration", username),
            ))
        }
    }

    /// Apply configuration for all users
    pub fn apply_all_configs(&self) -> Result<(), std::io::Error> {
        for username in self.config.users.keys() {
            self.apply_user_config(username)?;
        }
        Ok(())
    }

    /// Add a new user configuration
    pub fn add_user(&mut self, user_config: UserConfig) -> Result<(), std::io::Error> {
        self.config.users.insert(user_config.username.clone(), user_config);
        self.save_config()?;
        Ok(())
    }

    /// Remove a user configuration
    pub fn remove_user(&mut self, username: &str) -> Result<(), std::io::Error> {
        if self.config.users.remove(username).is_some() {
            self.save_config()?;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("User {} not found", username),
            ))
        }
    }

    /// Update user configuration
    pub fn update_user(&mut self, username: &str, updates: UserConfig) -> Result<(), std::io::Error> {
        if self.config.users.contains_key(username) {
            self.config.users.insert(username.to_string(), updates);
            self.save_config()?;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("User {} not found", username),
            ))
        }
    }

    /// Generate user configuration from template
    pub fn generate_user_template(&self, username: &str, home_dir: &str) -> UserConfig {
        UserConfig {
            username: username.to_string(),
            home_dir: PathBuf::from(home_dir),
            shell: self.config.global_settings.default_shell.clone(),
            editor: self.config.global_settings.default_editor.clone(),
            terminal: self.config.global_settings.default_terminal.clone(),
            environment: HashMap::new(),
            aliases: HashMap::new(),
            packages: Vec::new(),
            services: Vec::new(),
        }
    }

    fn apply_shell_config(&self, user_config: &UserConfig) -> Result<(), std::io::Error> {
        let shellrc_path = user_config.home_dir.join(format!(".{}rc", 
            user_config.shell.rsplit('/').next().unwrap_or(user_config.shell.as_str())));
        
        let mut shellrc_content = format!("# SigmaOS Home Manager Configuration for {}\n", user_config.username);
        shellrc_content.push_str(&format!("export SHELL={}\n", user_config.shell));
        shellrc_content.push_str(&format!("export EDITOR={}\n", user_config.editor));
        
        fs::write(&shellrc_path, shellrc_content)?;
        Ok(())
    }

    fn apply_environment_variables(&self, user_config: &UserConfig) -> Result<(), std::io::Error> {
        let profile_path = user_config.home_dir.join(".profile");
        
        let mut profile_content = "# SigmaOS Environment Variables\n".to_string();
        for (key, value) in &user_config.environment {
            profile_content.push_str(&format!("export {}=\"{}\"\n", key, value));
        }
        
        fs::write(&profile_path, profile_content)?;
        Ok(())
    }

    fn apply_aliases(&self, user_config: &UserConfig) -> Result<(), std::io::Error> {
        let aliases_path = user_config.home_dir.join(".aliases");
        
        let mut aliases_content = "# SigmaOS Aliases\n".to_string();
        for (alias, command) in &user_config.aliases {
            aliases_content.push_str(&format!("alias {}='{}'\n", alias, command));
        }
        
        fs::write(&aliases_path, aliases_content)?;
        Ok(())
    }

    fn install_user_packages(&self, user_config: &UserConfig) -> Result<(), std::io::Error> {
        // In a real implementation, this would call the package manager
        for package in &user_config.packages {
            println!("Installing user package: {}", package);
        }
        Ok(())
    }

    fn enable_user_services(&self, user_config: &UserConfig) -> Result<(), std::io::Error> {
        for service in &user_config.services {
            if service.enabled {
                println!("Enabling user service: {}", service.name);
                if service.autostart {
                    println!("Setting {} to autostart", service.name);
                }
            }
        }
        Ok(())
    }

    fn load_config(config_path: &Path) -> Result<HomeManagerConfig, std::io::Error> {
        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            let config: HomeManagerConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Return default configuration if file doesn't exist
            Ok(HomeManagerConfig::default())
        }
    }

    fn save_config(&self) -> Result<(), std::io::Error> {
        let content = toml::to_string_pretty(&self.config)?;
        fs::write(&self.config_path, content)?;
        Ok(())
    }
}

impl Default for HomeManagerConfig {
    fn default() -> Self {
        HomeManagerConfig {
            users: HashMap::new(),
            global_settings: GlobalSettings {
                default_shell: "/bin/sigma-sh".to_string(),
                default_editor: "sigma-editor".to_string(),
                default_terminal: "sigma-terminal".to_string(),
                enable_auto_update: false,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_home_manager_creation() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("home.toml");
        
        let manager = HomeManager::new(config_path).unwrap();
        assert_eq!(manager.config.users.len(), 0);
    }

    #[test]
    fn test_add_user() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("home.toml");
        
        let mut manager = HomeManager::new(config_path).unwrap();
        let user_config = manager.generate_user_template("testuser", "/home/testuser");
        
        manager.add_user(user_config).unwrap();
        assert_eq!(manager.config.users.len(), 1);
    }

    #[test]
    fn test_generate_template() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("home.toml");
        
        let manager = HomeManager::new(config_path).unwrap();
        let template = manager.generate_user_template("testuser", "/home/testuser");
        
        assert_eq!(template.username, "testuser");
        assert_eq!(template.home_dir, PathBuf::from("/home/testuser"));
    }
}
