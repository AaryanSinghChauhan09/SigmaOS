// Omarchy-Inspired CLI Router
// Provides command routing and dispatch for CLI tools
// Inspired by Omarchy's command-line interface architecture

use std::collections::HashMap;
use std::path::PathBuf;

/// Command metadata
#[derive(Debug, Clone)]
pub struct CommandMetadata {
    pub name: String,
    pub group: String,
    pub description: String,
    pub hidden: bool,
    pub requires_confirmation: bool,
    pub route_override: Option<String>,
}

impl CommandMetadata {
    pub fn new(name: String, group: String) -> Self {
        Self {
            name,
            group,
            description: String::new(),
            hidden: false,
            requires_confirmation: false,
            route_override: None,
        }
    }

    pub fn with_description(mut self, desc: String) -> Self {
        self.description = desc;
        self
    }

    pub fn with_hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    pub fn with_confirmation(mut self, requires: bool) -> Self {
        self.requires_confirmation = requires;
        self
    }

    pub fn with_route_override(mut self, route: String) -> Self {
        self.route_override = Some(route);
        self
    }
}

/// Command registration
#[derive(Debug, Clone)]
pub struct CommandRegistration {
    pub executable_path: PathBuf,
    pub metadata: CommandMetadata,
    pub canonical_route: String,
    pub filename_route: String,
}

impl CommandRegistration {
    pub fn new(executable_path: PathBuf, metadata: CommandMetadata) -> Self {
        let canonical_route = Self::build_canonical_route(&metadata);
        let filename_route = Self::build_filename_route(&executable_path);
        
        Self {
            executable_path,
            metadata,
            canonical_route,
            filename_route,
        }
    }

    fn build_canonical_route(metadata: &CommandMetadata) -> String {
        if let Some(override_route) = &metadata.route_override {
            return override_route.clone();
        }
        
        if metadata.name.is_empty() {
            return metadata.group.clone();
        }
        
        format!("{} {}", metadata.group, metadata.name)
    }

    fn build_filename_route(path: &PathBuf) -> String {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .replace("omarchy-", "")
            .replace('-', " ")
            .to_string()
    }
}

/// Command resolution result
#[derive(Debug, Clone)]
pub struct CommandResolution {
    pub registration: CommandRegistration,
    pub route_used: String,
    pub remaining_args: Vec<String>,
}

/// CLI router
#[derive(Debug, Clone)]
pub struct CliRouter {
    registrations: Vec<CommandRegistration>,
    route_conflicts: Vec<String>,
}

impl CliRouter {
    pub fn new() -> Self {
        Self {
            registrations: Vec::new(),
            route_conflicts: Vec::new(),
        }
    }

    /// Register a command
    pub fn register(&mut self, registration: CommandRegistration) {
        // Check for conflicts
        let canonical = registration.canonical_route.clone();
        let filename = registration.filename_route.clone();
        
        for existing in &self.registrations {
            if existing.canonical_route == canonical {
                self.route_conflicts.push(format!(
                    "Conflict: '{}' claimed by both {} and {}",
                    canonical,
                    existing.executable_path.display(),
                    registration.executable_path.display()
                ));
            }
        }
        
        self.registrations.push(registration);
    }

    /// Resolve a command from arguments
    pub fn resolve(&self, args: &[String]) -> Option<CommandResolution> {
        if args.is_empty() {
            return None;
        }

        // Try longest-prefix resolution
        for i in (1..=args.len()).rev() {
            let prefix = args[..i].join(" ");
            
            // Check canonical routes
            for registration in &self.registrations {
                if registration.canonical_route == prefix {
                    let remaining = args[i..].to_vec();
                    return Some(CommandResolution {
                        registration: registration.clone(),
                        route_used: registration.canonical_route.clone(),
                        remaining_args: remaining,
                    });
                }
            }
            
            // Check filename routes
            for registration in &self.registrations {
                if registration.filename_route == prefix {
                    let remaining = args[i..].to_vec();
                    return Some(CommandResolution {
                        registration: registration.clone(),
                        route_used: registration.filename_route.clone(),
                        remaining_args: remaining,
                    });
                }
            }
        }

        None
    }

    /// List all commands
    pub fn list_commands(&self, include_hidden: bool) -> Vec<&CommandRegistration> {
        self.registrations
            .iter()
            .filter(|r| include_hidden || !r.metadata.hidden)
            .collect()
    }

    /// List commands by group
    pub fn list_commands_by_group(&self, group: &str, include_hidden: bool) -> Vec<&CommandRegistration> {
        self.registrations
            .iter()
            .filter(|r| r.metadata.group == group && (include_hidden || !r.metadata.hidden))
            .collect()
    }

    /// Get all groups
    pub fn get_groups(&self) -> Vec<String> {
        let mut groups: Vec<String> = self.registrations
            .iter()
            .map(|r| r.metadata.group.clone())
            .collect();
        groups.sort();
        groups.dedup();
        groups
    }

    /// Get route conflicts
    pub fn get_conflicts(&self) -> &[String] {
        &self.route_conflicts
    }

    /// Auto-register from directory
    pub fn auto_register_from_dir(&mut self, dir: PathBuf) -> Result<usize, String> {
        if !dir.exists() {
            return Err(format!("Directory {} does not exist", dir.display()));
        }

        let mut count = 0;
        
        // In a real implementation, this would scan the directory for executables
        // For now, we'll simulate it
        count = 0;
        
        Ok(count)
    }
}

impl Default for CliRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Command group
#[derive(Debug, Clone)]
pub struct CommandGroup {
    pub name: String,
    pub description: String,
    pub commands: Vec<String>,
}

impl CommandGroup {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            commands: Vec::new(),
        }
    }

    pub fn with_description(mut self, desc: String) -> Self {
        self.description = desc;
        self
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let router = CliRouter::new();
        assert_eq!(router.list_commands(true).len(), 0);
    }

    #[test]
    fn test_command_registration() {
        let mut router = CliRouter::new();
        let metadata = CommandMetadata::new("set".to_string(), "theme".to_string());
        let registration = CommandRegistration::new(
            PathBuf::from("/bin/omarchy-theme-set"),
            metadata,
        );
        router.register(registration);
        assert_eq!(router.list_commands(true).len(), 1);
    }

    #[test]
    fn test_command_resolution() {
        let mut router = CliRouter::new();
        let metadata = CommandMetadata::new("set".to_string(), "theme".to_string());
        let registration = CommandRegistration::new(
            PathBuf::from("/bin/omarchy-theme-set"),
            metadata,
        );
        router.register(registration);

        let args = vec!["theme".to_string(), "set".to_string(), "dark".to_string()];
        let resolution = router.resolve(&args);
        assert!(resolution.is_some());
        
        let res = resolution.unwrap();
        assert_eq!(res.remaining_args, vec!["dark"]);
    }

    #[test]
    fn test_filename_route() {
        let metadata = CommandMetadata::new("set".to_string(), "theme".to_string());
        let registration = CommandRegistration::new(
            PathBuf::from("/bin/omarchy-theme-set"),
            metadata,
        );
        assert_eq!(registration.filename_route, "theme set");
    }

    #[test]
    fn test_canonical_route() {
        let metadata = CommandMetadata::new("set".to_string(), "theme".to_string());
        let registration = CommandRegistration::new(
            PathBuf::from("/bin/omarchy-theme-set"),
            metadata,
        );
        assert_eq!(registration.canonical_route, "theme set");
    }

    #[test]
    fn test_route_override() {
        let metadata = CommandMetadata::new("set".to_string(), "theme".to_string())
            .with_route_override("apply theme".to_string());
        let registration = CommandRegistration::new(
            PathBuf::from("/bin/omarchy-theme-set"),
            metadata,
        );
        assert_eq!(registration.canonical_route, "apply theme");
    }

    #[test]
    fn test_hidden_commands() {
        let mut router = CliRouter::new();
        let metadata = CommandMetadata::new("set".to_string(), "theme".to_string())
            .with_hidden(true);
        let registration = CommandRegistration::new(
            PathBuf::from("/bin/omarchy-theme-set"),
            metadata,
        );
        router.register(registration);

        assert_eq!(router.list_commands(false).len(), 0);
        assert_eq!(router.list_commands(true).len(), 1);
    }

    #[test]
    fn test_commands_by_group() {
        let mut router = CliRouter::new();
        let metadata1 = CommandMetadata::new("set".to_string(), "theme".to_string());
        let reg1 = CommandRegistration::new(
            PathBuf::from("/bin/omarchy-theme-set"),
            metadata1,
        );
        router.register(reg1);

        let metadata2 = CommandMetadata::new("list".to_string(), "theme".to_string());
        let reg2 = CommandRegistration::new(
            PathBuf::from("/bin/omarchy-theme-list"),
            metadata2,
        );
        router.register(reg2);

        let metadata3 = CommandMetadata::new("set".to_string(), "font".to_string());
        let reg3 = CommandRegistration::new(
            PathBuf::from("/bin/omarchy-font-set"),
            metadata3,
        );
        router.register(reg3);

        let theme_commands = router.list_commands_by_group("theme", true);
        assert_eq!(theme_commands.len(), 2);
    }

    #[test]
    fn test_get_groups() {
        let mut router = CliRouter::new();
        let metadata1 = CommandMetadata::new("set".to_string(), "theme".to_string());
        let reg1 = CommandRegistration::new(
            PathBuf::from("/bin/omarchy-theme-set"),
            metadata1,
        );
        router.register(reg1);

        let metadata2 = CommandMetadata::new("set".to_string(), "font".to_string());
        let reg2 = CommandRegistration::new(
            PathBuf::from("/bin/omarchy-font-set"),
            metadata2,
        );
        router.register(reg2);

        let groups = router.get_groups();
        assert_eq!(groups.len(), 2);
        assert!(groups.contains(&"theme".to_string()));
        assert!(groups.contains(&"font".to_string()));
    }

    #[test]
    fn test_command_metadata() {
        let metadata = CommandMetadata::new("set".to_string(), "theme".to_string())
            .with_description("Set a theme".to_string())
            .with_confirmation(true)
            .with_hidden(false);
        
        assert_eq!(metadata.description, "Set a theme");
        assert!(metadata.requires_confirmation);
        assert!(!metadata.hidden);
    }
}
