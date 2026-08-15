//! Arch Linux-Inspired User Repository (AUR-like) System
//! 
//! Community-driven package repository with user-submitted build scripts

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AurPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub maintainer: String,
    pub pkgbuild: String,
    pub dependencies: Vec<String>,
    pub votes: u32,
    pub popularity: f64,
}

#[derive(Debug)]
pub struct SigmaAur {
    packages: HashMap<String, AurPackage>,
}

impl SigmaAur {
    pub fn new() -> Self {
        Self {
            packages: HashMap::new(),
        }
    }

    /// Submit a new package to the repository
    pub fn submit_package(&mut self, package: AurPackage) -> Result<(), String> {
        if package.name.is_empty() {
            return Err("Package name cannot be empty".to_string());
        }

        // Security validation of PKGBUILD
        if self.validate_pkgbuild(&package.pkgbuild)? {
            self.packages.insert(package.name.clone(), package);
            Ok(())
        } else {
            Err("Invalid PKGBUILD script".to_string())
        }
    }

    /// Search for packages
    pub fn search(&self, query: &str) -> Vec<&AurPackage> {
        self.packages
            .values()
            .filter(|pkg| {
                pkg.name.contains(query) || 
                pkg.description.to_lowercase().contains(&query.to_lowercase())
            })
            .collect()
    }

    /// Vote for a package
    pub fn vote_package(&mut self, name: &str) -> Result<(), String> {
        if let Some(package) = self.packages.get_mut(name) {
            package.votes += 1;
            Ok(())
        } else {
            Err("Package not found".to_string())
        }
    }

    /// Build and install package from AUR
    pub fn build_package(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(package) = self.packages.get(name) {
            println!("Building {} from AUR...", name);

            // Create temporary build directory
            let build_dir = std::env::temp_dir().join(format!("sigmaur-{}", name));
            std::fs::create_dir_all(&build_dir)?;

            // Write PKGBUILD
            let pkgbuild_path = build_dir.join("PKGBUILD");
            std::fs::write(&pkgbuild_path, &package.pkgbuild)?;

            // Execute makepkg equivalent
            let output = Command::new("sh")
                .arg("-c")
                .arg("source PKGBUILD && build && package")
                .current_dir(&build_dir)
                .output()?;

            if !output.status.success() {
                return Err(format!("Build failed: {}", 
                    String::from_utf8_lossy(&output.stderr)).into());
            }

            println!("Successfully built {}", name);
            Ok(())
        } else {
            Err("Package not found in AUR".into())
        }
    }

    fn validate_pkgbuild(&self, pkgbuild: &str) -> Result<bool, String> {
        // Basic security checks
        let forbidden_commands = ["rm -rf /", "dd if=", "mkfs", "format"];
        
        for cmd in forbidden_commands {
            if pkgbuild.contains(cmd) {
                return Err(format!("Forbidden command detected: {}", cmd));
            }
        }

        // Check for required fields
        let required_fields = ["pkgname=", "pkgver=", "build()"];
        for field in required_fields {
            if !pkgbuild.contains(field) {
                return Err(format!("Missing required field: {}", field));
            }
        }

        Ok(true)
    }

    /// Get most popular packages
    pub fn get_popular(&self, limit: usize) -> Vec<&AurPackage> {
        let mut packages: Vec<&AurPackage> = self.packages.values().collect();
        packages.sort_by(|a, b| b.votes.cmp(&a.votes));
        packages.into_iter().take(limit).collect()
    }
}