//! SigmaOS Distro-Inspired Command Alias System
//!
//! Provides a highly advanced alias manager with support for:
//! 1. **User-named aliases** (custom commands registered and manipulated by users)
//! 2. **Fixed-named aliases** (standards-based package manager and utility mappings from Linux & BSD)
//! 3. **Automatic aliases** (typo auto-correction and zsh-style suffix-based file extensions)
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Represents the classification/origin of a SigmaOS alias.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AliasType {
    /// Custom aliases created, modified, and deleted by users.
    UserNamed,
    /// Canonical system-wide or platform-wide standard commands from other distros.
    FixedNamed,
    /// Context-aware, typo-correcting, or suffix-based automatic aliases.
    Automatic,
}

/// A structured command alias in SigmaOS.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigmaAlias {
    pub name: String,
    pub value: String,
    pub alias_type: AliasType,
    pub description: String,
}

impl SigmaAlias {
    pub fn new(name: &str, value: &str, alias_type: AliasType, description: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            alias_type,
            description: description.to_string(),
        }
    }
}

/// The main Alias Manager for SigmaOS, inspired by Arch, Alpine, Debian, Gentoo, and FreeBSD.
#[derive(Debug, Clone)]
pub struct AliasManager {
    /// User-defined and fixed-named command aliases.
    pub aliases: BTreeMap<String, SigmaAlias>,
    /// Common command typo auto-correction mapping (Automatic).
    pub typo_corrections: BTreeMap<String, String>,
    /// File extension to command/application mapping (Automatic Suffix).
    pub suffix_aliases: BTreeMap<String, String>,
}

impl AliasManager {
    /// Creates a new Alias Manager pre-loaded with Linux/BSD distro compatibility and automatic mappings.
    pub fn new() -> Self {
        let mut manager = Self {
            aliases: BTreeMap::new(),
            typo_corrections: BTreeMap::new(),
            suffix_aliases: BTreeMap::new(),
        };

        // 1. Load default Fixed-Named compatibility aliases
        // Inspired by BSD, Alpine, Debian, Arch, and RedHat distros.
        manager.register_alias(
            "ll",
            "ls -la",
            AliasType::FixedNamed,
            "List directory contents with details (GNU/BSD common)",
        );
        manager.register_alias(
            "la",
            "ls -A",
            AliasType::FixedNamed,
            "List almost all files (GNU/BSD common)",
        );
        manager.register_alias(
            "grep",
            "sigmagrep",
            AliasType::FixedNamed,
            "Competitive ripgrep-inspired utility",
        );
        manager.register_alias(
            "find",
            "sigmafind",
            AliasType::FixedNamed,
            "Competitive fd/find alternative",
        );
        manager.register_alias(
            "diff",
            "sigmadiff",
            AliasType::FixedNamed,
            "Competitive git-diff alternative",
        );

        // 2. Load default Automatic typo corrections
        manager.register_typo_correction("gti", "git");
        manager.register_typo_correction("sl", "ls");
        manager.register_typo_correction("co", "cd");
        manager.register_typo_correction("gerp", "grep");
        manager.register_typo_correction("exi", "exit");

        // 3. Load default Automatic Suffix/Extension-based mappings (zsh-inspired)
        manager.register_suffix_alias("txt", "editor");
        manager.register_suffix_alias("rs", "rustc");
        manager.register_suffix_alias("sh", "sigma-sh");
        manager.register_suffix_alias("json", "textproc");

        manager
    }

    /// Register a user, fixed, or automatic alias.
    pub fn register_alias(
        &mut self,
        name: &str,
        value: &str,
        alias_type: AliasType,
        description: &str,
    ) {
        let alias = SigmaAlias::new(name, value, alias_type, description);
        self.aliases.insert(name.to_string(), alias);
    }

    /// Remove an alias. Returns true if it existed and was removed.
    pub fn remove_alias(&mut self, name: &str) -> bool {
        self.aliases.remove(name).is_some()
    }

    /// Retrieve a reference to a registered alias by name.
    pub fn get_alias(&self, name: &str) -> Option<&SigmaAlias> {
        self.aliases.get(name)
    }

    /// Get all registered aliases.
    pub fn list_aliases(&self) -> Vec<SigmaAlias> {
        self.aliases.values().cloned().collect()
    }

    /// Register a typo correction mapping.
    pub fn register_typo_correction(&mut self, typo: &str, correction: &str) {
        self.typo_corrections
            .insert(typo.to_string(), correction.to_string());
    }

    /// Register a suffix-based auto-alias.
    pub fn register_suffix_alias(&mut self, extension: &str, app: &str) {
        self.suffix_aliases
            .insert(extension.to_string(), app.to_string());
    }

    /// Recursively expands user-named and fixed-named aliases up to a specific recursion limit to prevent cycles.
    pub fn expand_aliases(&self, command: &str) -> String {
        let trimmed = command.trim();
        if trimmed.is_empty() {
            return String::new();
        }

        let mut current = trimmed.to_string();
        let mut depth = 0;
        let max_depth = 8;

        while depth < max_depth {
            // Split the command line to check the first word
            let parts: Vec<&str> = current.split_whitespace().collect();
            if parts.is_empty() {
                break;
            }
            let first_token = parts[0];

            if let Some(alias) = self.get_alias(first_token) {
                let rest = if current.len() > first_token.len() {
                    &current[first_token.len()..]
                } else {
                    ""
                };
                current = format!("{}{}", alias.value, rest).trim().to_string();
                depth += 1;
            } else {
                break;
            }
        }
        current
    }

    /// Translates package management commands from main Linux/BSD guest distros to native SigmaOS package managers.
    /// Supports Alpine (apk), Debian/Ubuntu (apt, apt-get), Arch (pacman), Fedora/RHEL (dnf, yum), Gentoo (emerge), and FreeBSD (pkg).
    pub fn interpret_fixed_name_distro(&self, command: &str) -> String {
        let trimmed = command.trim();
        if trimmed.is_empty() {
            return String::new();
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let cmd_name = parts[0];
        let args = &parts[1..];

        match cmd_name {
            // Alpine APK Translation
            "apk" => {
                if args.first() == Some(&"add") {
                    let pkg_args = &args[1..];
                    format!("sigpkg install {}", pkg_args.join(" "))
                } else if args.first() == Some(&"del") {
                    let pkg_args = &args[1..];
                    format!("sigpkg remove {}", pkg_args.join(" "))
                } else if args.first() == Some(&"update") {
                    "sigpkg update".to_string()
                } else {
                    trimmed.to_string()
                }
            }
            // Debian/Ubuntu APT Translation
            "apt" | "apt-get" => {
                if args.first() == Some(&"install") {
                    let pkg_args = &args[1..];
                    format!("sigpkg install {}", pkg_args.join(" "))
                } else if args.first() == Some(&"remove") || args.first() == Some(&"purge") {
                    let pkg_args = &args[1..];
                    format!("sigpkg remove {}", pkg_args.join(" "))
                } else if args.first() == Some(&"update") {
                    "sigpkg update".to_string()
                } else {
                    trimmed.to_string()
                }
            }
            // Arch Linux Pacman Translation
            "pacman" => {
                if args.len() >= 1 && args[0].starts_with("-S") {
                    // Check flags like -S, -Syu, -Sy
                    let flag = args[0];
                    let pkg_args = &args[1..];
                    if flag.contains('y') && flag.contains('u') {
                        if pkg_args.is_empty() {
                            "sigpkg update".to_string()
                        } else {
                            format!("sigpkg update && sigpkg install {}", pkg_args.join(" "))
                        }
                    } else if flag == "-S" {
                        format!("sigpkg install {}", pkg_args.join(" "))
                    } else {
                        trimmed.to_string()
                    }
                } else if args.len() >= 1 && args[0] == "-R" {
                    let pkg_args = &args[1..];
                    if pkg_args.is_empty() {
                        trimmed.to_string()
                    } else {
                        format!("sigpkg remove {}", pkg_args.join(" "))
                    }
                } else {
                    trimmed.to_string()
                }
            }
            // RedHat/Fedora DNF/YUM Translation
            "dnf" | "yum" => {
                if args.first() == Some(&"install") {
                    let pkg_args = &args[1..];
                    format!("sigpkg install {}", pkg_args.join(" "))
                } else if args.first() == Some(&"remove") || args.first() == Some(&"erase") {
                    let pkg_args = &args[1..];
                    format!("sigpkg remove {}", pkg_args.join(" "))
                } else if args.first() == Some(&"check-update") || args.first() == Some(&"update") {
                    "sigpkg update".to_string()
                } else {
                    trimmed.to_string()
                }
            }
            // Gentoo Emerge Translation
            "emerge" => {
                if args.contains(&"--unmerge") || args.contains(&"-C") {
                    let pkg_args: Vec<&str> = args
                        .iter()
                        .filter(|&&a| a != "--unmerge" && a != "-C")
                        .cloned()
                        .collect();
                    format!("sigpkg remove {}", pkg_args.join(" "))
                } else {
                    // Filtering options to get package name
                    let pkg_args: Vec<&str> = args
                        .iter()
                        .filter(|&&a| !a.starts_with('-'))
                        .cloned()
                        .collect();
                    if pkg_args.is_empty() {
                        "sigpkg update".to_string()
                    } else {
                        format!("sigpkg install {}", pkg_args.join(" "))
                    }
                }
            }
            // FreeBSD PKG Translation
            "pkg" => {
                if args.first() == Some(&"install") {
                    let pkg_args = &args[1..];
                    format!("sigpkg install {}", pkg_args.join(" "))
                } else if args.first() == Some(&"delete") || args.first() == Some(&"remove") {
                    let pkg_args = &args[1..];
                    format!("sigpkg remove {}", pkg_args.join(" "))
                } else if args.first() == Some(&"update") {
                    "sigpkg update".to_string()
                } else {
                    trimmed.to_string()
                }
            }
            _ => trimmed.to_string(),
        }
    }

    /// Automatically corrects common user typos or applies zsh-style suffix aliases based on file extension.
    pub fn interpret_automatic_alias(&self, command: &str) -> String {
        let trimmed = command.trim();
        if trimmed.is_empty() {
            return String::new();
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let mut first_word = parts[0].to_string();
        let rest_of_cmd = if trimmed.len() > parts[0].len() {
            &trimmed[parts[0].len()..]
        } else {
            ""
        };

        // 1. Attempt Suffix Alias evaluation (zsh style).
        // If the first token looks like a file name (e.g. contains dot and has a registered suffix),
        // we automatically prepend the associated app command.
        if let Some(dot_idx) = first_word.rfind('.') {
            let ext = &first_word[dot_idx + 1..];
            if let Some(app) = self.suffix_aliases.get(ext) {
                return format!("{} {}{}", app, first_word, rest_of_cmd)
                    .trim()
                    .to_string();
            }
        }

        // 2. Attempt Typo Auto-Correction.
        if let Some(correction) = self.typo_corrections.get(&first_word) {
            first_word = correction.clone();
        }

        format!("{}{}", first_word, rest_of_cmd).trim().to_string()
    }

    /// Fully resolves and interprets a command line by applying typo corrections, suffix-mappings,
    /// recursive user-named aliases, and distro compatibility translation layers.
    pub fn resolve_command(&self, input_line: &str) -> String {
        let step1 = self.interpret_automatic_alias(input_line);
        let step2 = self.expand_aliases(&step1);
        self.interpret_fixed_name_distro(&step2)
    }
}

impl Default for AliasManager {
    fn default() -> Self {
        Self::new()
    }
}

// UNIT TESTS FOR ALIAS SYSTEM
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_named_aliases_creation_manipulation() {
        let mut manager = AliasManager::new();

        // Register custom user aliases
        manager.register_alias(
            "gs",
            "git status",
            AliasType::UserNamed,
            "Shortcut for Git status",
        );
        manager.register_alias(
            "dc",
            "docker-compose",
            AliasType::UserNamed,
            "Shortcut for compose",
        );

        assert_eq!(manager.get_alias("gs").unwrap().value, "git status");
        assert_eq!(
            manager.get_alias("dc").unwrap().alias_type,
            AliasType::UserNamed
        );

        // Verify manipulation: remove and assert
        assert!(manager.remove_alias("dc"));
        assert!(manager.get_alias("dc").is_none());
        assert!(!manager.remove_alias("dc")); // non-existent now
    }

    #[test]
    fn test_recursive_alias_expansion() {
        let mut manager = AliasManager::new();
        manager.register_alias("ll", "ls -la", AliasType::UserNamed, "ls verbose");
        manager.register_alias("la", "ll -h", AliasType::UserNamed, "recursive with flags");

        // Single expansion
        assert_eq!(manager.expand_aliases("ll /tmp"), "ls -la /tmp");

        // Recursive expansion (la -> ll -h -> ls -la -h)
        assert_eq!(manager.expand_aliases("la /usr"), "ls -la -h /usr");

        // Verify non-alias command line remains untouched
        assert_eq!(manager.expand_aliases("mkdir -p /app"), "mkdir -p /app");
    }

    #[test]
    fn test_circular_alias_expansion_protection() {
        let mut manager = AliasManager::new();
        // Infinite recursion cycle: a -> b -> a
        manager.register_alias("a", "b", AliasType::UserNamed, "");
        manager.register_alias("b", "a", AliasType::UserNamed, "");

        // Should exit gracefully within max expansion depth limit instead of crashing/hanging
        let result = manager.expand_aliases("a test");
        assert!(result == "a test" || result == "b test");
    }

    #[test]
    fn test_fixed_name_distro_translations() {
        let manager = AliasManager::new();

        // Alpine APK Translation
        assert_eq!(
            manager.interpret_fixed_name_distro("apk add curl git"),
            "sigpkg install curl git"
        );
        assert_eq!(
            manager.interpret_fixed_name_distro("apk del vim"),
            "sigpkg remove vim"
        );
        assert_eq!(
            manager.interpret_fixed_name_distro("apk update"),
            "sigpkg update"
        );

        // Debian/Ubuntu APT Translation
        assert_eq!(
            manager.interpret_fixed_name_distro("apt install htop"),
            "sigpkg install htop"
        );
        assert_eq!(
            manager.interpret_fixed_name_distro("apt-get purge apache2"),
            "sigpkg remove apache2"
        );

        // Arch Linux Pacman Translation
        assert_eq!(
            manager.interpret_fixed_name_distro("pacman -S nmap"),
            "sigpkg install nmap"
        );
        assert_eq!(
            manager.interpret_fixed_name_distro("pacman -Syu"),
            "sigpkg update"
        );
        assert_eq!(
            manager.interpret_fixed_name_distro("pacman -R tree"),
            "sigpkg remove tree"
        );

        // RedHat/Fedora DNF/YUM Translation
        assert_eq!(
            manager.interpret_fixed_name_distro("dnf install python3"),
            "sigpkg install python3"
        );
        assert_eq!(
            manager.interpret_fixed_name_distro("yum update"),
            "sigpkg update"
        );

        // Gentoo Emerge Translation
        assert_eq!(
            manager.interpret_fixed_name_distro("emerge --unmerge libpng"),
            "sigpkg remove libpng"
        );
        assert_eq!(
            manager.interpret_fixed_name_distro("emerge -av sys-kernel/gentoo-sources"),
            "sigpkg install sys-kernel/gentoo-sources"
        );

        // FreeBSD PKG Translation
        assert_eq!(
            manager.interpret_fixed_name_distro("pkg install bash"),
            "sigpkg install bash"
        );
        assert_eq!(
            manager.interpret_fixed_name_distro("pkg remove nano"),
            "sigpkg remove nano"
        );
    }

    #[test]
    fn test_automatic_typo_correction() {
        let manager = AliasManager::new();

        // sl -> ls
        assert_eq!(manager.interpret_automatic_alias("sl -lh"), "ls -lh");
        // gti -> git
        assert_eq!(
            manager.interpret_automatic_alias("gti clone url"),
            "git clone url"
        );
        // non-typo remains unchanged
        assert_eq!(manager.interpret_automatic_alias("pwd"), "pwd");
    }

    #[test]
    fn test_automatic_suffix_aliases() {
        let manager = AliasManager::new();

        // txt -> editor
        assert_eq!(
            manager.interpret_automatic_alias("notes.txt"),
            "editor notes.txt"
        );
        // rs -> rustc
        assert_eq!(
            manager.interpret_automatic_alias("main.rs --bin"),
            "rustc main.rs --bin"
        );
        // non-matching file extension remains untouched
        assert_eq!(
            manager.interpret_automatic_alias("archive.tar.gz"),
            "archive.tar.gz"
        );
    }

    #[test]
    fn test_complete_resolve_pipeline() {
        let mut manager = AliasManager::new();
        // Setup complex pipeline:
        // Suffix: main.rs -> rustc main.rs
        // Custom alias: rustc -> compiler run
        manager.register_alias("rustc", "compiler run", AliasType::UserNamed, "");

        let resolved = manager.resolve_command("main.rs -O");
        // 1. Suffix alias converts "main.rs -O" to "rustc main.rs -O"
        // 2. Custom alias expands "rustc" to "compiler run main.rs -O"
        // 3. No distro package manager keywords matched, so returned as is.
        assert_eq!(resolved, "compiler run main.rs -O");
    }
}
