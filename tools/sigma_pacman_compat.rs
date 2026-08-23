// SPDX-License-Identifier: MIT
// SigmaOS Arch Linux Pacman & AUR Helper Compatibility Utility (sigma_pacman_compat)
// Clean-room representation of Arch Linux's core package utility suite (pacman, yay, paru)

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ArchPackageRecord {
    pub name: String,
    pub version: String,
    pub repository: String, // "core", "extra", "community", or "aur"
    pub dependencies: Vec<String>,
    pub description: String,
}

pub struct PacmanSystem {
    pub sync_database: HashMap<String, ArchPackageRecord>,
    pub aur_database: HashMap<String, ArchPackageRecord>,
    pub installed_packages: HashMap<String, ArchPackageRecord>,
    pub db_synced: bool,
}

impl PacmanSystem {
    pub fn new() -> Self {
        let mut sys = Self {
            sync_database: HashMap::new(),
            aur_database: HashMap::new(),
            installed_packages: HashMap::new(),
            db_synced: false,
        };
        sys.load_default_arch_repos();
        sys
    }

    /// Preloads standard Arch Linux core/extra/community and AUR repository indices
    fn load_default_arch_repos(&mut self) {
        // Official Repositories
        self.sync_database.insert(
            "linux".to_string(),
            ArchPackageRecord {
                name: "linux".to_string(),
                version: "6.5.9.arch1-1".to_string(),
                repository: "core".to_string(),
                dependencies: Vec::new(),
                description: "The Linux kernel and modules".to_string(),
            },
        );

        self.sync_database.insert(
            "glibc".to_string(),
            ArchPackageRecord {
                name: "glibc".to_string(),
                version: "2.38-7".to_string(),
                repository: "core".to_string(),
                dependencies: Vec::new(),
                description: "GNU C Library".to_string(),
            },
        );

        self.sync_database.insert(
            "neovim".to_string(),
            ArchPackageRecord {
                name: "neovim".to_string(),
                version: "0.9.4-1".to_string(),
                repository: "extra".to_string(),
                dependencies: vec!["glibc".to_string(), "libuv".to_string()],
                description: "Vim-fork focused on extensibility and usability".to_string(),
            },
        );

        self.sync_database.insert(
            "libuv".to_string(),
            ArchPackageRecord {
                name: "libuv".to_string(),
                version: "1.46.0-1".to_string(),
                repository: "extra".to_string(),
                dependencies: vec!["glibc".to_string()],
                description: "Multi-platform support library with a focus on asynchronous I/O".to_string(),
            },
        );

        // Arch User Repository (AUR)
        self.aur_database.insert(
            "google-chrome".to_string(),
            ArchPackageRecord {
                name: "google-chrome".to_string(),
                version: "119.0.6045.105-1".to_string(),
                repository: "aur".to_string(),
                dependencies: vec!["glibc".to_string()],
                description: "The popular web browser from Google (AUR PKGBUILD)".to_string(),
            },
        );
    }

    /// Simulates 'pacman -Sy' or 'pacman -Syu' database sync
    pub fn pacman_sync_db(&mut self) -> Result<usize, &'static str> {
        println!(":: Synchronizing package databases...");
        println!(" downloading core.db...");
        println!(" downloading extra.db...");
        println!(" downloading community.db...");
        self.db_synced = true;
        Ok(self.sync_database.len())
    }

    /// Simulates 'pacman -Ss <query>' search
    pub fn pacman_search(&self, query: &str) -> Vec<ArchPackageRecord> {
        let mut results = Vec::new();
        for pkg in self.sync_database.values() {
            if pkg.name.contains(query) || pkg.description.contains(query) {
                results.push(pkg.clone());
            }
        }
        for pkg in self.aur_database.values() {
            if pkg.name.contains(query) || pkg.description.contains(query) {
                results.push(pkg.clone());
            }
        }
        results.sort_by(|a, b| a.name.cmp(&b.name));
        results
    }

    /// Simulates 'pacman -S <pkg>' installation from official repos
    pub fn pacman_install(&mut self, package_name: &str) -> Result<Vec<String>, &'static str> {
        if !self.db_synced {
            return Err("pacman Error: Databases not synchronized (run pacman -Sy).");
        }

        if !self.sync_database.contains_key(package_name) {
            return Err("pacman Error: Target not found in official repositories.");
        }

        let mut install_order = Vec::new();
        let mut visited = HashMap::new();

        self.resolve_deps(package_name, &mut install_order, &mut visited)?;

        println!("resolving dependencies...");
        println!("looking for conflicting packages...");
        println!("Packages ({}) {}", install_order.len(), install_order.join(" "));

        for pkg_name in &install_order {
            if let Some(record) = self.sync_database.get(pkg_name) {
                self.installed_packages.insert(pkg_name.clone(), record.clone());
                println!(":: Processing package changes...");
                println!("({}) installing {}...", pkg_name, record.version);
            }
        }

        Ok(install_order)
    }

    /// Helper dependency resolver
    fn resolve_deps(
        &self,
        name: &str,
        order: &mut Vec<String>,
        visited: &mut HashMap<String, bool>,
    ) -> Result<(), &'static str> {
        if let Some(&in_progress) = visited.get(name) {
            if in_progress {
                return Err("pacman Error: Circular dependency detected.");
            }
            return Ok(());
        }

        if self.installed_packages.contains_key(name) {
            return Ok(());
        }

        visited.insert(name.to_string(), true);

        if let Some(record) = self.sync_database.get(name) {
            for dep in &record.dependencies {
                self.resolve_deps(dep, order, visited)?;
            }
        }

        visited.insert(name.to_string(), false);
        if !order.contains(&name.to_string()) {
            order.push(name.to_string());
        }

        Ok(())
    }

    /// Simulates 'yay -S <aur_pkg>' or 'paru -S <aur_pkg>' AUR helper execution
    pub fn yay_aur_install(&mut self, aur_package: &str) -> Result<(), &'static str> {
        if !self.aur_database.contains_key(aur_package) {
            return Err("yay Error: Package not found in AUR database.");
        }

        let record = self.aur_database.get(aur_package).unwrap().clone();
        println!(":: Fetching PKGBUILD from AUR for {}...", aur_package);
        println!(":: Parsing PKGBUILD recipe... OK");
        println!(":: Compiling {} in sandboxed Ring 3 workspace...", aur_package);
        self.installed_packages.insert(aur_package.to_string(), record);
        println!(":: Package {} installed successfully via yay helper.", aur_package);
        Ok(())
    }

    /// Simulates 'pacman -R <pkg>' package removal
    pub fn pacman_remove(&mut self, package_name: &str) -> Result<(), &'static str> {
        if !self.installed_packages.contains_key(package_name) {
            return Err("pacman Error: Target not found in installed database.");
        }

        self.installed_packages.remove(package_name);
        println!("checking dependencies...");
        println!(":: Removing {}...", package_name);
        Ok(())
    }
}

impl Default for PacmanSystem {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    println!("Σ SIGMAOS ARCH LINUX PACMAN & YAY PARITY SUITE");
    let mut sys = PacmanSystem::new();
    sys.pacman_sync_db().unwrap();
    sys.pacman_install("neovim").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pacman_sync_and_search() {
        let mut sys = PacmanSystem::new();
        assert!(!sys.db_synced);

        let count = sys.pacman_sync_db().unwrap();
        assert_eq!(count, 4);
        assert!(sys.db_synced);

        let search_res = sys.pacman_search("kernel");
        assert_eq!(search_res.len(), 1);
        assert_eq!(search_res[0].name, "linux");
    }

    #[test]
    fn test_pacman_install_and_remove() {
        let mut sys = PacmanSystem::new();
        assert!(sys.pacman_install("neovim").is_err()); // Needs sync first

        sys.pacman_sync_db().unwrap();
        let order = sys.pacman_install("neovim").unwrap();

        // Dependencies: neovim -> [glibc, libuv] (libuv -> glibc)
        assert_eq!(order, vec!["glibc".to_string(), "libuv".to_string(), "neovim".to_string()]);
        assert!(sys.installed_packages.contains_key("neovim"));

        // Removal
        assert!(sys.pacman_remove("neovim").is_ok());
        assert!(!sys.installed_packages.contains_key("neovim"));
    }

    #[test]
    fn test_yay_aur_helper() {
        let mut sys = PacmanSystem::new();
        assert!(sys.yay_aur_install("google-chrome").is_ok());
        assert!(sys.installed_packages.contains_key("google-chrome"));
        assert_eq!(sys.installed_packages.get("google-chrome").unwrap().repository, "aur");
    }
}
