// Sigma Config - Unified System Configuration Tool
// Inspired by YaST (Yet another Setup Tool) from openSUSE
// Provides centralized system configuration management

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigModule {
    pub name: String,
    pub description: String,
    pub category: ConfigCategory,
    pub settings: HashMap<String, ConfigValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConfigCategory {
    System,
    Network,
    Security,
    User,
    Software,
    Hardware,
    Services,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConfigValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    List(Vec<String>),
    Map(HashMap<String, String>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigProfile {
    pub name: String,
    pub description: String,
    pub modules: HashMap<String, HashMap<String, ConfigValue>>,
}

pub struct SigmaConfig {
    config_dir: PathBuf,
    modules: HashMap<String, ConfigModule>,
    profiles: HashMap<String, ConfigProfile>,
    active_profile: Option<String>,
}

impl SigmaConfig {
    pub fn new(config_dir: PathBuf) -> Result<Self, std::io::Error> {
        fs::create_dir_all(&config_dir)?;
        
        let modules = Self::load_modules(&config_dir)?;
        let profiles = Self::load_profiles(&config_dir)?;
        let active_profile = Self::load_active_profile(&config_dir)?;
        
        Ok(SigmaConfig {
            config_dir,
            modules,
            profiles,
            active_profile,
        })
    }

    /// Get a configuration module
    pub fn get_module(&self, name: &str) -> Option<&ConfigModule> {
        self.modules.get(name)
    }

    /// Set a configuration value
    pub fn set_value(&mut self, module: &str, key: &str, value: ConfigValue) -> Result<(), std::io::Error> {
        if let Some(mod_config) = self.modules.get_mut(module) {
            mod_config.settings.insert(key.to_string(), value);
            self.save_module(module)?;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Module {} not found", module),
            ))
        }
    }

    /// Get a configuration value
    pub fn get_value(&self, module: &str, key: &str) -> Option<&ConfigValue> {
        self.modules.get(module)?.settings.get(key)
    }

    /// Apply a configuration profile
    pub fn apply_profile(&mut self, profile_name: &str) -> Result<(), std::io::Error> {
        if let Some(profile) = self.profiles.get(profile_name) {
            for (module_name, settings) in &profile.modules {
                if let Some(module) = self.modules.get_mut(module_name) {
                    for (key, value) in settings {
                        module.settings.insert(key.clone(), value.clone());
                    }
                    self.save_module(module_name)?;
                }
            }
            
            self.active_profile = Some(profile_name.to_string());
            self.save_active_profile()?;
            
            println!("Applied profile: {}", profile_name);
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Profile {} not found", profile_name),
            ))
        }
    }

    /// Create a new configuration profile
    pub fn create_profile(&mut self, name: &str, description: &str) -> Result<(), std::io::Error> {
        let mut profile_settings = HashMap::new();
        
        for (module_name, module) in &self.modules {
            profile_settings.insert(module_name.clone(), module.settings.clone());
        }
        
        let profile = ConfigProfile {
            name: name.to_string(),
            description: description.to_string(),
            modules: profile_settings,
        };
        
        self.profiles.insert(name.to_string(), profile);
        self.save_profile(name)?;
        
        Ok(())
    }

    /// List all available modules
    pub fn list_modules(&self) -> Vec<&ConfigModule> {
        self.modules.values().collect()
    }

    /// List all available profiles
    pub fn list_profiles(&self) -> Vec<&ConfigProfile> {
        self.profiles.values().collect()
    }

    /// Validate current configuration
    pub fn validate(&self) -> Result<Vec<String>, std::io::Error> {
        let mut errors = Vec::new();
        
        for (name, module) in &self.modules {
            if module.settings.is_empty() {
                errors.push(format!("Module {} has no settings", name));
            }
        }
        
        Ok(errors)
    }

    /// Export configuration to file
    pub fn export(&self, path: &Path) -> Result<(), std::io::Error> {
        let export_data = serde_json::to_string_pretty(&self.modules)?;
        fs::write(path, export_data)?;
        Ok(())
    }

    /// Import configuration from file
    pub fn import(&mut self, path: &Path) -> Result<(), std::io::Error> {
        let import_data = fs::read_to_string(path)?;
        let imported_modules: HashMap<String, ConfigModule> = serde_json::from_str(&import_data)?;
        
        for (name, module) in imported_modules {
            self.modules.insert(name.clone(), module);
            self.save_module(&name)?;
        }
        
        Ok(())
    }

    /// Initialize default modules
    pub fn initialize_defaults(&mut self) -> Result<(), std::io::Error> {
        self.add_default_module("system", ConfigCategory::System)?;
        self.add_default_module("network", ConfigCategory::Network)?;
        self.add_default_module("security", ConfigCategory::Security)?;
        self.add_default_module("user", ConfigCategory::User)?;
        self.add_default_module("software", ConfigCategory::Software)?;
        self.add_default_module("hardware", ConfigCategory::Hardware)?;
        self.add_default_module("services", ConfigCategory::Services)?;
        Ok(())
    }

    fn add_default_module(&mut self, name: &str, category: ConfigCategory) -> Result<(), std::io::Error> {
        let module = ConfigModule {
            name: name.to_string(),
            description: format!("{} configuration module", name),
            category,
            settings: HashMap::new(),
        };
        
        self.modules.insert(name.to_string(), module);
        self.save_module(name)?;
        Ok(())
    }

    fn load_modules(config_dir: &Path) -> Result<HashMap<String, ConfigModule>, std::io::Error> {
        let modules_dir = config_dir.join("modules");
        let mut modules = HashMap::new();
        
        if modules_dir.exists() {
            for entry in fs::read_dir(&modules_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.extension().map(|e| e == "toml").unwrap_or(false) {
                    let content = fs::read_to_string(&path)?;
                    if let Ok(module) = toml::from_str::<ConfigModule>(&content) {
                        modules.insert(module.name.clone(), module);
                    }
                }
            }
        }
        
        Ok(modules)
    }

    fn load_profiles(config_dir: &Path) -> Result<HashMap<String, ConfigProfile>, std::io::Error> {
        let profiles_dir = config_dir.join("profiles");
        let mut profiles = HashMap::new();
        
        if profiles_dir.exists() {
            for entry in fs::read_dir(&profiles_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.extension().map(|e| e == "toml").unwrap_or(false) {
                    let content = fs::read_to_string(&path)?;
                    if let Ok(profile) = toml::from_str::<ConfigProfile>(&content) {
                        profiles.insert(profile.name.clone(), profile);
                    }
                }
            }
        }
        
        Ok(profiles)
    }

    fn load_active_profile(config_dir: &Path) -> Result<Option<String>, std::io::Error> {
        let active_file = config_dir.join("active_profile");
        
        if active_file.exists() {
            let content = fs::read_to_string(&active_file)?;
            Ok(Some(content.trim().to_string()))
        } else {
            Ok(None)
        }
    }

    fn save_module(&self, name: &str) -> Result<(), std::io::Error> {
        let modules_dir = self.config_dir.join("modules");
        fs::create_dir_all(&modules_dir)?;
        
        if let Some(module) = self.modules.get(name) {
            let module_path = modules_dir.join(format!("{}.toml", name));
            let content = toml::to_string_pretty(module)?;
            fs::write(&module_path, content)?;
        }
        
        Ok(())
    }

    fn save_profile(&self, name: &str) -> Result<(), std::io::Error> {
        let profiles_dir = self.config_dir.join("profiles");
        fs::create_dir_all(&profiles_dir)?;
        
        if let Some(profile) = self.profiles.get(name) {
            let profile_path = profiles_dir.join(format!("{}.toml", name));
            let content = toml::to_string_pretty(profile)?;
            fs::write(&profile_path, content)?;
        }
        
        Ok(())
    }

    fn save_active_profile(&self) -> Result<(), std::io::Error> {
        let active_file = self.config_dir.join("active_profile");
        
        if let Some(profile_name) = &self.active_profile {
            fs::write(&active_file, profile_name)?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_sigma_config_creation() {
        let temp_dir = tempdir().unwrap();
        let config_dir = temp_dir.path().to_path_buf();
        
        let config = SigmaConfig::new(config_dir).unwrap();
        assert_eq!(config.modules.len(), 0);
    }

    #[test]
    fn test_initialize_defaults() {
        let temp_dir = tempdir().unwrap();
        let config_dir = temp_dir.path().to_path_buf();
        
        let mut config = SigmaConfig::new(config_dir).unwrap();
        config.initialize_defaults().unwrap();
        
        assert_eq!(config.modules.len(), 7);
    }

    #[test]
    fn test_set_get_value() {
        let temp_dir = tempdir().unwrap();
        let config_dir = temp_dir.path().to_path_buf();
        
        let mut config = SigmaConfig::new(config_dir).unwrap();
        config.initialize_defaults().unwrap();
        
        config.set_value("system", "hostname", ConfigValue::String("sigmaos".to_string())).unwrap();
        
        if let Some(ConfigValue::String(value)) = config.get_value("system", "hostname") {
            assert_eq!(value, "sigmaos");
        } else {
            panic!("Expected string value");
        }
    }
}
