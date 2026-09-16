

use std::format;
use std::string::String;
use std::vec;
use std::vec::Vec;

/// MintSystem-inspired system utilities wrapper
/// Provides enhanced command-line tools similar to Linux Mint's mintsystem
/// which wraps apt, dpkg, and other package management utilities

#[derive(Debug, Clone, PartialEq)]
pub enum AptCommand {
    Install,
    Remove,
    Update,
    Upgrade,
    DistUpgrade,
    Autoremove,
    Autoclean,
    Search,
    Show,
    List,
    Purge,
    Hold,
    Unhold,
    AddRepository,
    EditSources,
    Depends,
    Rdepends,
    Policy,
    Download,
    Source,
    BuildDep,
    Changelog,
    Check,
    MarkAuto,
    UnmarkAuto,
}

#[derive(Debug, Clone)]
pub struct AptResult {
    pub success: bool,
    pub output: String,
    pub error: String,
    pub packages_affected: Vec<String>,
}

/// MintSystem - Linux Mint system utilities wrapper
pub struct MintSystem {
    pub apt_command_history: Vec<String>,
    pub package_cache: Vec<String>,
    pub held_packages: Vec<String>,
    pub auto_installed_packages: Vec<String>,
}

impl MintSystem {
    pub fn new() -> Self {
        Self {
            apt_command_history: Vec::new(),
            package_cache: Vec::new(),
            held_packages: Vec::new(),
            auto_installed_packages: Vec::new(),
        }
    }

    /// Execute apt command
    pub fn execute_apt(&mut self, command: AptCommand, packages: &[String]) -> AptResult {
        let command_str = self.apt_command_to_string(&command);
        let full_command = if packages.is_empty() {
            format!("apt {}", command_str)
        } else {
            format!("apt {} {}", command_str, packages.join(" "))
        };
        
        self.apt_command_history.push(full_command.clone());
        
        let result = match command {
            AptCommand::Install => self.apt_install(packages),
            AptCommand::Remove => self.apt_remove(packages),
            AptCommand::Update => self.apt_update(),
            AptCommand::Upgrade => self.apt_upgrade(),
            AptCommand::DistUpgrade => self.apt_dist_upgrade(),
            AptCommand::Autoremove => self.apt_autoremove(),
            AptCommand::Autoclean => self.apt_autoclean(),
            AptCommand::Search => self.apt_search(packages),
            AptCommand::Show => self.apt_show(packages),
            AptCommand::List => self.apt_list(packages),
            AptCommand::Purge => self.apt_purge(packages),
            AptCommand::Hold => self.apt_hold(packages),
            AptCommand::Unhold => self.apt_unhold(packages),
            AptCommand::AddRepository => self.apt_add_repository(packages),
            AptCommand::EditSources => self.apt_edit_sources(),
            AptCommand::Depends => self.apt_depends(packages),
            AptCommand::Rdepends => self.apt_rdepends(packages),
            AptCommand::Policy => self.apt_policy(packages),
            AptCommand::Download => self.apt_download(packages),
            AptCommand::Source => self.apt_source(packages),
            AptCommand::BuildDep => self.apt_build_dep(packages),
            AptCommand::Changelog => self.apt_changelog(packages),
            AptCommand::Check => self.apt_check(),
            AptCommand::MarkAuto => self.apt_mark_auto(packages),
            AptCommand::UnmarkAuto => self.apt_unmark_auto(packages),
        };
        
        result
    }

    /// Get command history
    pub fn get_command_history(&self) -> Vec<&String> {
        self.apt_command_history.iter().collect()
    }

    /// Clear command history
    pub fn clear_command_history(&mut self) {
        self.apt_command_history.clear();
    }

    /// Get held packages
    pub fn get_held_packages(&self) -> Vec<&String> {
        self.held_packages.iter().collect()
    }

    /// Get auto-installed packages
    pub fn get_auto_installed_packages(&self) -> Vec<&String> {
        self.auto_installed_packages.iter().collect()
    }

    /// Display system information
    pub fn display_system_info(&self) -> String {
        let mut output = String::from("=== MintSystem Information ===\n\n");
        output.push_str(&format!("Command History Size: {}\n", self.apt_command_history.len()));
        output.push_str(&format!("Held Packages: {}\n", self.held_packages.len()));
        output.push_str(&format!("Auto-Installed Packages: {}\n", self.auto_installed_packages.len()));
        output.push_str(&format!("Package Cache Size: {}\n", self.package_cache.len()));
        output
    }

    // Helper methods for apt commands
    fn apt_install(&mut self, packages: &[String]) -> AptResult {
        for pkg in packages {
            if !self.package_cache.contains(pkg) {
                self.package_cache.push(pkg.clone());
            }
        }
        AptResult {
            success: true,
            output: format!("Installed packages: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_remove(&mut self, packages: &[String]) -> AptResult {
        for pkg in packages {
            self.package_cache.retain(|p| p != pkg);
        }
        AptResult {
            success: true,
            output: format!("Removed packages: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_update(&self) -> AptResult {
        AptResult {
            success: true,
            output: String::from("Package lists updated"),
            error: String::new(),
            packages_affected: Vec::new(),
        }
    }

    fn apt_upgrade(&self) -> AptResult {
        AptResult {
            success: true,
            output: String::from("Packages upgraded"),
            error: String::new(),
            packages_affected: self.package_cache.clone(),
        }
    }

    fn apt_dist_upgrade(&self) -> AptResult {
        AptResult {
            success: true,
            output: String::from("Distribution upgraded"),
            error: String::new(),
            packages_affected: self.package_cache.clone(),
        }
    }

    fn apt_autoremove(&mut self) -> AptResult {
        let removed_count = self.auto_installed_packages.len();
        self.auto_installed_packages.clear();
        AptResult {
            success: true,
            output: format!("Auto-removed {} packages", removed_count),
            error: String::new(),
            packages_affected: Vec::new(),
        }
    }

    fn apt_autoclean(&mut self) -> AptResult {
        self.package_cache.clear();
        AptResult {
            success: true,
            output: String::from("Package cache cleaned"),
            error: String::new(),
            packages_affected: Vec::new(),
        }
    }

    fn apt_search(&self, packages: &[String]) -> AptResult {
        let results: Vec<String> = packages.iter()
            .filter(|pkg| self.package_cache.contains(pkg))
            .cloned()
            .collect();
        
        AptResult {
            success: true,
            output: format!("Found {} packages", results.len()),
            error: String::new(),
            packages_affected: results,
        }
    }

    fn apt_show(&self, packages: &[String]) -> AptResult {
        AptResult {
            success: true,
            output: format!("Package information for: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_list(&self, packages: &[String]) -> AptResult {
        let filtered: Vec<String> = if packages.is_empty() {
            self.package_cache.clone()
        } else {
            self.package_cache.iter()
                .filter(|pkg| packages.iter().any(|p| pkg.contains(p)))
                .cloned()
                .collect()
        };
        
        AptResult {
            success: true,
            output: format!("Listed {} packages", filtered.len()),
            error: String::new(),
            packages_affected: filtered,
        }
    }

    fn apt_purge(&mut self, packages: &[String]) -> AptResult {
        for pkg in packages {
            self.package_cache.retain(|p| p != pkg);
            self.held_packages.retain(|p| p != pkg);
            self.auto_installed_packages.retain(|p| p != pkg);
        }
        AptResult {
            success: true,
            output: format!("Purged packages: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_hold(&mut self, packages: &[String]) -> AptResult {
        for pkg in packages {
            if !self.held_packages.contains(pkg) {
                self.held_packages.push(pkg.clone());
            }
        }
        AptResult {
            success: true,
            output: format!("Held packages: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_unhold(&mut self, packages: &[String]) -> AptResult {
        for pkg in packages {
            self.held_packages.retain(|p| p != pkg);
        }
        AptResult {
            success: true,
            output: format!("Unheld packages: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_add_repository(&self, packages: &[String]) -> AptResult {
        AptResult {
            success: true,
            output: format!("Added repository: {}", packages.join(" ")),
            error: String::new(),
            packages_affected: Vec::new(),
        }
    }

    fn apt_edit_sources(&self) -> AptResult {
        AptResult {
            success: true,
            output: String::from("Sources list editor opened"),
            error: String::new(),
            packages_affected: Vec::new(),
        }
    }

    fn apt_depends(&self, packages: &[String]) -> AptResult {
        AptResult {
            success: true,
            output: format!("Dependencies for: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_rdepends(&self, packages: &[String]) -> AptResult {
        AptResult {
            success: true,
            output: format!("Reverse dependencies for: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_policy(&self, packages: &[String]) -> AptResult {
        AptResult {
            success: true,
            output: format!("Policy for: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_download(&self, packages: &[String]) -> AptResult {
        AptResult {
            success: true,
            output: format!("Downloaded packages: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_source(&self, packages: &[String]) -> AptResult {
        AptResult {
            success: true,
            output: format!("Source packages for: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_build_dep(&self, packages: &[String]) -> AptResult {
        AptResult {
            success: true,
            output: format!("Build dependencies for: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_changelog(&self, packages: &[String]) -> AptResult {
        AptResult {
            success: true,
            output: format!("Changelog for: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_check(&self) -> AptResult {
        AptResult {
            success: true,
            output: String::from("Dependency check passed"),
            error: String::new(),
            packages_affected: Vec::new(),
        }
    }

    fn apt_mark_auto(&mut self, packages: &[String]) -> AptResult {
        for pkg in packages {
            if !self.auto_installed_packages.contains(pkg) {
                self.auto_installed_packages.push(pkg.clone());
            }
        }
        AptResult {
            success: true,
            output: format!("Marked as auto-installed: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_unmark_auto(&mut self, packages: &[String]) -> AptResult {
        for pkg in packages {
            self.auto_installed_packages.retain(|p| p != pkg);
        }
        AptResult {
            success: true,
            output: format!("Unmarked as auto-installed: {}", packages.join(", ")),
            error: String::new(),
            packages_affected: packages.to_vec(),
        }
    }

    fn apt_command_to_string(&self, command: &AptCommand) -> String {
        match command {
            AptCommand::Install => String::from("install"),
            AptCommand::Remove => String::from("remove"),
            AptCommand::Update => String::from("update"),
            AptCommand::Upgrade => String::from("upgrade"),
            AptCommand::DistUpgrade => String::from("dist-upgrade"),
            AptCommand::Autoremove => String::from("autoremove"),
            AptCommand::Autoclean => String::from("autoclean"),
            AptCommand::Search => String::from("search"),
            AptCommand::Show => String::from("show"),
            AptCommand::List => String::from("list"),
            AptCommand::Purge => String::from("purge"),
            AptCommand::Hold => String::from("hold"),
            AptCommand::Unhold => String::from("unhold"),
            AptCommand::AddRepository => String::from("add-repository"),
            AptCommand::EditSources => String::from("edit-sources"),
            AptCommand::Depends => String::from("depends"),
            AptCommand::Rdepends => String::from("rdepends"),
            AptCommand::Policy => String::from("policy"),
            AptCommand::Download => String::from("download"),
            AptCommand::Source => String::from("source"),
            AptCommand::BuildDep => String::from("build-dep"),
            AptCommand::Changelog => String::from("changelog"),
            AptCommand::Check => String::from("check"),
            AptCommand::MarkAuto => String::from("markauto"),
            AptCommand::UnmarkAuto => String::from("unmarkauto"),
        }
    }
}

impl Default for MintSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_system_creation() {
        let mint_system = MintSystem::new();
        assert!(mint_system.apt_command_history.is_empty());
        assert!(mint_system.package_cache.is_empty());
    }

    #[test]
    fn test_apt_install() {
        let mut mint_system = MintSystem::new();
        let result = mint_system.execute_apt(AptCommand::Install, &vec![String::from("vim"), String::from("git")]);
        
        assert!(result.success);
        assert_eq!(mint_system.package_cache.len(), 2);
        assert!(mint_system.package_cache.contains(&String::from("vim")));
    }

    #[test]
    fn test_apt_remove() {
        let mut mint_system = MintSystem::new();
        mint_system.execute_apt(AptCommand::Install, &vec![String::from("vim")]);
        
        let result = mint_system.execute_apt(AptCommand::Remove, &vec![String::from("vim")]);
        assert!(result.success);
        assert!(!mint_system.package_cache.contains(&String::from("vim")));
    }

    #[test]
    fn test_apt_hold() {
        let mut mint_system = MintSystem::new();
        let result = mint_system.execute_apt(AptCommand::Hold, &vec![String::from("vim")]);
        
        assert!(result.success);
        assert!(mint_system.held_packages.contains(&String::from("vim")));
    }

    #[test]
    fn test_apt_unhold() {
        let mut mint_system = MintSystem::new();
        mint_system.execute_apt(AptCommand::Hold, &vec![String::from("vim")]);
        
        let result = mint_system.execute_apt(AptCommand::Unhold, &vec![String::from("vim")]);
        assert!(result.success);
        assert!(!mint_system.held_packages.contains(&String::from("vim")));
    }

    #[test]
    fn test_apt_mark_auto() {
        let mut mint_system = MintSystem::new();
        let result = mint_system.execute_apt(AptCommand::MarkAuto, &vec![String::from("vim")]);
        
        assert!(result.success);
        assert!(mint_system.auto_installed_packages.contains(&String::from("vim")));
    }

    #[test]
    fn test_command_history() {
        let mut mint_system = MintSystem::new();
        mint_system.execute_apt(AptCommand::Update, &vec![]);
        mint_system.execute_apt(AptCommand::Upgrade, &vec![]);
        
        let history = mint_system.get_command_history();
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn test_display_system_info() {
        let mint_system = MintSystem::new();
        let info = mint_system.display_system_info();
        
        assert!(info.contains("MintSystem Information"));
        assert!(info.contains("Command History Size"));
    }

    #[test]
    fn test_apt_search() {
        let mut mint_system = MintSystem::new();
        mint_system.execute_apt(AptCommand::Install, &vec![String::from("vim"), String::from("git")]);
        
        let result = mint_system.execute_apt(AptCommand::Search, &vec![String::from("vim")]);
        assert!(result.success);
        assert_eq!(result.packages_affected.len(), 1);
    }
}
