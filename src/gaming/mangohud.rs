// SPDX-License-Identifier: MIT
// SigmaOS - MangoHUD Config Parser
// Inspired by MangoHUD (https://github.com/flightlessmango/MangoHud)
// Provides config parser for MangoHUD configuration

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// MangoHUD config option
#[derive(Debug, Clone, PartialEq)]
pub enum MangoHudOption {
    Bool(bool),
    Int(i32),
    Float(f32),
    String(String),
    Position(MangoHudPosition),
}

/// MangoHUD position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MangoHudPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    TopCenter,
    BottomCenter,
}

/// MangoHUD config
#[derive(Debug, Clone)]
pub struct MangoHudConfig {
    pub options: BTreeMap<String, MangoHudOption>,
}

impl MangoHudConfig {
    pub fn new() -> Self {
        Self {
            options: BTreeMap::new(),
        }
    }
    
    /// Parse config string
    pub fn parse(&mut self, config_str: &str) -> Result<(), &'static str> {
        for line in config_str.lines() {
            let line = line.trim();
            
            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Parse key=value
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();
                
                let option = self.parse_value(value)?;
                self.options.insert(key.to_string(), option);
            }
        }
        
        Ok(())
    }
    
    /// Parse value
    fn parse_value(&self, value: &str) -> Result<MangoHudOption, &'static str> {
        // Try bool
        if value == "true" || value == "false" {
            return Ok(MangoHudOption::Bool(value == "true"));
        }
        
        // Try int
        if let Ok(int_val) = value.parse::<i32>() {
            return Ok(MangoHudOption::Int(int_val));
        }
        
        // Try float
        if let Ok(float_val) = value.parse::<f32>() {
            return Ok(MangoHudOption::Float(float_val));
        }
        
        // Try position
        if let Ok(position) = self.parse_position(value) {
            return Ok(MangoHudOption::Position(position));
        }
        
        // Default to string
        Ok(MangoHudOption::String(value.to_string()))
    }
    
    /// Parse position
    fn parse_position(&self, value: &str) -> Result<MangoHudPosition, &'static str> {
        match value.to_lowercase().as_str() {
            "top-left" => Ok(MangoHudPosition::TopLeft),
            "top-right" => Ok(MangoHudPosition::TopRight),
            "bottom-left" => Ok(MangoHudPosition::BottomLeft),
            "bottom-right" => Ok(MangoHudPosition::BottomRight),
            "top-center" => Ok(MangoHudPosition::TopCenter),
            "bottom-center" => Ok(MangoHudPosition::BottomCenter),
            _ => Err("Invalid position"),
        }
    }
    
    /// Get option
    pub fn get_option(&self, key: &str) -> Option<&MangoHudOption> {
        self.options.get(key)
    }
    
    /// Get bool option
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.options.get(key).and_then(|opt| {
            if let MangoHudOption::Bool(val) = opt {
                Some(*val)
            } else {
                None
            }
        })
    }
    
    /// Get int option
    pub fn get_int(&self, key: &str) -> Option<i32> {
        self.options.get(key).and_then(|opt| {
            if let MangoHudOption::Int(val) = opt {
                Some(*val)
            } else {
                None
            }
        })
    }
    
    /// Get float option
    pub fn get_float(&self, key: &str) -> Option<f32> {
        self.options.get(key).and_then(|opt| {
            if let MangoHudOption::Float(val) = opt {
                Some(*val)
            } else {
                None
            }
        })
    }
    
    /// Set option
    pub fn set_option(&mut self, key: String, option: MangoHudOption) {
        self.options.insert(key, option);
    }
    
    /// Load default config
    pub fn load_default(&mut self) {
        self.options.insert("fps".to_string(), MangoHudOption::Bool(true));
        self.options.insert("fps_limit".to_string(), MangoHudOption::Int(0));
        self.options.insert("fps_sampling_period".to_string(), MangoHudOption::Float(500.0));
        self.options.insert("gpu_stats".to_string(), MangoHudOption::Bool(true));
        self.options.insert("cpu_stats".to_string(), MangoHudOption::Bool(true));
        self.options.insert("vram".to_string(), MangoHudOption::Bool(true));
        self.options.insert("ram".to_string(), MangoHudOption::Bool(true));
        self.options.insert("gpu_temp".to_string(), MangoHudOption::Bool(true));
        self.options.insert("cpu_temp".to_string(), MangoHudOption::Bool(true));
        self.options.insert("position".to_string(), MangoHudOption::Position(MangoHudPosition::TopRight));
        self.options.insert("background_alpha".to_string(), MangoHudOption::Float(0.5));
        self.options.insert("font_size".to_string(), MangoHudOption::Int(24));
    }
}

impl Default for MangoHudConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Global MangoHUD config
static mut MANGOHUD_CONFIG: Option<MangoHudConfig> = None;

/// Initialize MangoHUD config
pub fn init_mangohud_config() {
    unsafe {
        MANGOHUD_CONFIG = Some(MangoHudConfig::new());
    }
}

/// Get global MangoHUD config
pub fn get_mangohud_config() -> Option<&'static mut MangoHudConfig> {
    unsafe {
        MANGOHUD_CONFIG.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_bool() {
        let mut config = MangoHudConfig::new();
        config.parse("fps=true").unwrap();
        assert_eq!(config.get_bool("fps"), Some(true));
    }
    
    #[test]
    fn test_parse_int() {
        let mut config = MangoHudConfig::new();
        config.parse("fps_limit=60").unwrap();
        assert_eq!(config.get_int("fps_limit"), Some(60));
    }
    
    #[test]
    fn test_parse_float() {
        let mut config = MangoHudConfig::new();
        config.parse("background_alpha=0.5").unwrap();
        assert_eq!(config.get_float("background_alpha"), Some(0.5));
    }
    
    #[test]
    fn test_parse_position() {
        let mut config = MangoHudConfig::new();
        config.parse("position=top-right").unwrap();
        if let Some(MangoHudOption::Position(pos)) = config.get_option("position") {
            assert_eq!(*pos, MangoHudPosition::TopRight);
        } else {
            panic!("Position not parsed correctly");
        }
    }
    
    #[test]
    fn test_parse_comment() {
        let mut config = MangoHudConfig::new();
        config.parse("# This is a comment\nfps=true").unwrap();
        assert_eq!(config.get_bool("fps"), Some(true));
    }
    
    #[test]
    fn test_load_default() {
        let mut config = MangoHudConfig::new();
        config.load_default();
        assert_eq!(config.get_bool("fps"), Some(true));
        assert_eq!(config.get_bool("gpu_stats"), Some(true));
    }
    
    #[test]
    fn test_parse_config_string() {
        let mut config = MangoHudConfig::new();
        let config_str = r#"
fps=true
fps_limit=60
background_alpha=0.5
position=top-right
"#;
        config.parse(config_str).unwrap();
        assert_eq!(config.get_bool("fps"), Some(true));
        assert_eq!(config.get_int("fps_limit"), Some(60));
    }
}
