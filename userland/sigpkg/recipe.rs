// Package Recipe System for SigmaOS
// Implements declarative package build recipes
// Inspired by Guix and Portage package management systems

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub name: String,
    pub version: String,
    pub description: String,
    pub license: String,
    pub homepage: String,
    pub source: Source,
    pub build: BuildConfig,
    pub dependencies: Dependencies,
    pub install: InstallConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Source {
    pub url: String,
    pub hash: String,
    pub method: SourceMethod,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SourceMethod {
    Tarball,
    Git { branch: Option<String>, commit: Option<String> },
    Hg,
    Svn,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildConfig {
    pub system: BuildSystem,
    pub configure_args: Vec<String>,
    pub make_args: Vec<String>,
    pub environment: HashMap<String, String>,
    pub patches: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BuildSystem {
    Autotools,
    CMake,
    Meson,
    Cargo,
    Go,
    Python,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dependencies {
    pub build: Vec<String>,
    pub runtime: Vec<String>,
    pub test: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstallConfig {
    pub files: Vec<InstallFile>,
    pub directories: Vec<String>,
    pub post_install: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstallFile {
    pub source: String,
    pub destination: String,
    pub mode: Option<String>,
}

pub struct RecipeManager {
    recipes_dir: PathBuf,
    recipes: HashMap<String, Recipe>,
}

impl RecipeManager {
    pub fn new(recipes_dir: PathBuf) -> Result<Self, std::io::Error> {
        fs::create_dir_all(&recipes_dir)?;
        
        let recipes = Self::load_recipes(&recipes_dir)?;
        
        Ok(RecipeManager {
            recipes_dir,
            recipes,
        })
    }

    /// Load a recipe from file
    pub fn load_recipe(&mut self, name: &str) -> Result<Recipe, std::io::Error> {
        let recipe_path = self.recipes_dir.join(format!("{}.toml", name));
        
        if recipe_path.exists() {
            let content = fs::read_to_string(&recipe_path)?;
            let recipe: Recipe = toml::from_str(&content)?;
            self.recipes.insert(name.to_string(), recipe.clone());
            Ok(recipe)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Recipe {} not found", name),
            ))
        }
    }

    /// Save a recipe to file
    pub fn save_recipe(&self, recipe: &Recipe) -> Result<(), std::io::Error> {
        let recipe_path = self.recipes_dir.join(format!("{}.toml", recipe.name));
        let content = toml::to_string_pretty(recipe)?;
        fs::write(&recipe_path, content)?;
        Ok(())
    }

    /// Build a package from recipe
    pub fn build_package(&self, recipe: &Recipe) -> Result<PathBuf, std::io::Error> {
        println!("Building {}-{}", recipe.name, recipe.version);
        
        // Download source
        let source_dir = self.download_source(&recipe.source)?;
        
        // Apply patches
        self.apply_patches(&source_dir, &recipe.build.patches)?;
        
        // Configure
        self.configure_build(&source_dir, recipe)?;
        
        // Build
        self.compile(&source_dir, recipe)?;
        
        // Install to temporary directory
        let install_dir = self.install_package(&source_dir, recipe)?;
        
        Ok(install_dir)
    }

    /// Create a new recipe template
    pub fn create_template(&self, name: &str) -> Recipe {
        Recipe {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            description: format!("Package: {}", name),
            license: "MIT".to_string(),
            homepage: format!("https://example.com/{}", name),
            source: Source {
                url: format!("https://example.com/{}-{}.tar.gz", name, "1.0.0"),
                hash: "".to_string(),
                method: SourceMethod::Tarball,
            },
            build: BuildConfig {
                system: BuildSystem::Autotools,
                configure_args: vec!["--prefix=/usr".to_string()],
                make_args: vec!["-j$(nproc)".to_string()],
                environment: HashMap::new(),
                patches: vec![],
            },
            dependencies: Dependencies {
                build: vec![],
                runtime: vec![],
                test: vec![],
            },
            install: InstallConfig {
                files: vec![],
                directories: vec!["/usr/bin".to_string(), "/usr/lib".to_string()],
                post_install: None,
            },
        }
    }

    /// Validate recipe
    pub fn validate_recipe(&self, recipe: &Recipe) -> Result<(), std::io::Error> {
        if recipe.name.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Recipe name cannot be empty",
            ));
        }
        
        if recipe.source.url.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Source URL cannot be empty",
            ));
        }
        
        Ok(())
    }

    fn download_source(&self, source: &Source) -> Result<PathBuf, std::io::Error> {
        let cache_dir = self.recipes_dir.join("cache");
        fs::create_dir_all(&cache_dir)?;
        
        let filename = source.url.split('/').last().unwrap_or("source.tar.gz");
        let source_path = cache_dir.join(filename);
        
        if !source_path.exists() {
            println!("Downloading from {}", source.url);
            // In a real implementation, this would download the file
            // For now, create a placeholder
            fs::write(&source_path, "placeholder source")?;
        }
        
        let extract_dir = cache_dir.join("extracted");
        fs::create_dir_all(&extract_dir)?;
        
        Ok(extract_dir)
    }

    fn apply_patches(&self, source_dir: &Path, patches: &[String]) -> Result<(), std::io::Error> {
        for patch in patches {
            println!("Applying patch: {}", patch);
            // In a real implementation, this would apply the patch
        }
        Ok(())
    }

    fn configure_build(&self, source_dir: &Path, recipe: &Recipe) -> Result<(), std::io::Error> {
        match recipe.build.system {
            BuildSystem::Autotools => {
                println!("Running ./configure with args: {:?}", recipe.build.configure_args);
                // In a real implementation, this would run ./configure
            }
            BuildSystem::CMake => {
                println!("Running cmake with args: {:?}", recipe.build.configure_args);
                // In a real implementation, this would run cmake
            }
            BuildSystem::Meson => {
                println!("Running meson with args: {:?}", recipe.build.configure_args);
                // In a real implementation, this would run meson
            }
            BuildSystem::Cargo => {
                println!("Building with cargo");
                // In a real implementation, this would run cargo build
            }
            _ => {
                println!("Using custom build system");
            }
        }
        Ok(())
    }

    fn compile(&self, source_dir: &Path, recipe: &Recipe) -> Result<(), std::io::Error> {
        println!("Running make with args: {:?}", recipe.build.make_args);
        // In a real implementation, this would run make
        Ok(())
    }

    fn install_package(&self, source_dir: &Path, recipe: &Recipe) -> Result<PathBuf, std::io::Error> {
        let install_dir = self.recipes_dir.join("install").join(&recipe.name);
        fs::create_dir_all(&install_dir)?;
        
        println!("Installing to {}", install_dir.display());
        // In a real implementation, this would run make install
        
        Ok(install_dir)
    }

    fn load_recipes(recipes_dir: &Path) -> Result<HashMap<String, Recipe>, std::io::Error> {
        let mut recipes = HashMap::new();
        
        if recipes_dir.exists() {
            for entry in fs::read_dir(recipes_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.extension().map(|e| e == "toml").unwrap_or(false) {
                    let content = fs::read_to_string(&path)?;
                    if let Ok(recipe) = toml::from_str::<Recipe>(&content) {
                        recipes.insert(recipe.name.clone(), recipe);
                    }
                }
            }
        }
        
        Ok(recipes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_recipe_manager_creation() {
        let temp_dir = tempdir().unwrap();
        let recipes_dir = temp_dir.path().to_path_buf();
        
        let manager = RecipeManager::new(recipes_dir).unwrap();
        assert_eq!(manager.recipes.len(), 0);
    }

    #[test]
    fn test_recipe_template() {
        let temp_dir = tempdir().unwrap();
        let recipes_dir = temp_dir.path().to_path_buf();
        
        let manager = RecipeManager::new(recipes_dir).unwrap();
        let template = manager.create_template("test-package");
        
        assert_eq!(template.name, "test-package");
        assert_eq!(template.version, "1.0.0");
    }

    #[test]
    fn test_recipe_validation() {
        let temp_dir = tempdir().unwrap();
        let recipes_dir = temp_dir.path().to_path_buf();
        
        let manager = RecipeManager::new(recipes_dir).unwrap();
        let template = manager.create_template("test-package");
        
        assert!(manager.validate_recipe(&template).is_ok());
        
        let mut invalid_recipe = template;
        invalid_recipe.name = "".to_string();
        assert!(manager.validate_recipe(&invalid_recipe).is_err());
    }
}
