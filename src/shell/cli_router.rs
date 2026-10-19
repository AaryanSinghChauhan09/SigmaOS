//! SigmaOS CLI Command Router
//!
//! Following principles: COMMAND_QUERY_SEPARATION, SINGLE_RESPONSIBILITY,
//! MINIMISE_COUPLING, MAXIMISE_COHESION, LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID,
//! SIMPLE_USER_INTERFACE, DISCOVERABILITY, DESIGN_PATTERN, COMMAND_PATTERN,
//! DEMETER_LAW, LEVEL_OF_ABSTRACTION, SIMPLE_API, USER_FRIENDLY.
//!
//! Competitive with: Omarchy omarchy command router, Mint mintupdate CLI.
//!
//! Command router pattern:
//! - Single entry point: `sigma` command
//! - Subcommands: sigma pkg, sigma hw, sigma theme, sigma menu, etc.
//! - Metadata-driven help generation
//! - Interactive command palette support

use core::fmt;

/// Command group categories
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandGroup {
    Pkg,
    Hw,
    Theme,
    Menu,
    Update,
    System,
    Network,
    User,
    Security,
    Help,
}

impl CommandGroup {
    pub fn description(&self) -> &str {
        match self {
            CommandGroup::Pkg => "Package management helpers",
            CommandGroup::Hw => "Hardware detection",
            CommandGroup::Theme => "Theme management",
            CommandGroup::Menu => "Sigma menu commands",
            CommandGroup::Update => "System updates and migrations",
            CommandGroup::System => "System administration",
            CommandGroup::Network => "Network configuration",
            CommandGroup::User => "User management",
            CommandGroup::Security => "Security and access control",
            CommandGroup::Help => "Help and information",
        }
    }
}

/// Individual command metadata
#[derive(Debug, Clone)]
pub struct CommandMetadata {
    pub name: String,
    pub group: CommandGroup,
    pub summary: String,
    pub args: Vec<String>,
    pub handler: CommandHandler,
}

/// Command handler type
pub type CommandHandler = fn(&[String]) -> CommandResult;

/// Command execution result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandResult {
    Success(String),
    Error(String),
    Help(String),
}

/// CLI router
pub struct Router {
    pub commands: Vec<CommandMetadata>,
}

impl Router {
    /// Create a new router with built-in commands
    pub fn new() -> Self {
        Self {
            commands: vec![
                CommandMetadata {
                    name: "pkg".to_string(),
                    group: CommandGroup::Pkg,
                    summary: "Package management helpers".to_string(),
                    args: vec!["install".to_string(), "remove".to_string(), "search".to_string()],
                    handler: pkg_handler,
                },
                CommandMetadata {
                    name: "hw".to_string(),
                    group: CommandGroup::Hw,
                    summary: "Hardware detection".to_string(),
                    args: vec!["detect".to_string(), "info".to_string()],
                    handler: hw_handler,
                },
                CommandMetadata {
                    name: "theme".to_string(),
                    group: CommandGroup::Theme,
                    summary: "Theme management".to_string(),
                    args: vec!["list".to_string(), "apply".to_string(), "create".to_string()],
                    handler: theme_handler,
                },
                CommandMetadata {
                    name: "menu".to_string(),
                    group: CommandGroup::Menu,
                    summary: "Sigma menu commands".to_string(),
                    args: vec!["show".to_string(), "search".to_string()],
                    handler: menu_handler,
                },
                CommandMetadata {
                    name: "update".to_string(),
                    group: CommandGroup::Update,
                    summary: "System updates and migrations".to_string(),
                    args: vec!["check".to_string(), "apply".to_string(), "rollback".to_string()],
                    handler: update_handler,
                },
                CommandMetadata {
                    name: "help".to_string(),
                    group: CommandGroup::Help,
                    summary: "Help and information".to_string(),
                    args: vec![],
                    handler: help_handler,
                },
            ],
        }
    }

    /// Route a command to its handler
    pub fn route(&self, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return self.show_help();
        }

        let subcommand = &args[0];
        let rest = &args[1..];

        for cmd in &self.commands {
            if cmd.name == subcommand {
                return (cmd.handler)(rest);
            }
        }

        CommandResult::Error(format!("Unknown command: {}", subcommand))
    }

    /// Show general help
    fn show_help(&self) -> CommandResult {
        let mut help = String::from("SigmaOS CLI Router\n\nAvailable commands:\n");
        for cmd in &self.commands {
            help.push_str(&format!(
                "  {:<10} - {}\n",
                cmd.name,
                cmd.summary
            ));
        }
        help.push_str("\nUse 'sigma <command> --help' for command-specific help.");
        CommandResult::Help(help)
    }

    /// List all command groups
    pub fn list_groups(&self) -> Vec<&CommandGroup> {
        let mut groups: Vec<&CommandGroup> = self.commands.iter().map(|c| &c.group).collect();
        groups.sort();
        groups.dedup();
        groups
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

// Command handlers

fn pkg_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma pkg <install|remove|search> <package>".to_string());
    }
    match args[0].as_str() {
        "install" => CommandResult::Success("Installing packages...".to_string()),
        "remove" => CommandResult::Success("Removing packages...".to_string()),
        "search" => CommandResult::Success("Searching packages...".to_string()),
        _ => CommandResult::Error(format!("Unknown pkg subcommand: {}", args[0])),
    }
}

fn hw_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma hw <detect|info>".to_string());
    }
    match args[0].as_str() {
        "detect" => CommandResult::Success("Detecting hardware...".to_string()),
        "info" => CommandResult::Success("Hardware info displayed.".to_string()),
        _ => CommandResult::Error(format!("Unknown hw subcommand: {}", args[0])),
    }
}

fn theme_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma theme <list|apply|create>".to_string());
    }
    match args[0].as_str() {
        "list" => CommandResult::Success("Available themes: default, dark, light, solarized".to_string()),
        "apply" => CommandResult::Success("Theme applied.".to_string()),
        "create" => CommandResult::Success("Theme created.".to_string()),
        _ => CommandResult::Error(format!("Unknown theme subcommand: {}", args[0])),
    }
}

fn menu_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma menu <show|search>".to_string());
    }
    match args[0].as_str() {
        "show" => CommandResult::Success("Sigma menu displayed.".to_string()),
        "search" => CommandResult::Success("Menu search activated.".to_string()),
        _ => CommandResult::Error(format!("Unknown menu subcommand: {}", args[0])),
    }
}

fn update_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma update <check|apply|rollback>".to_string());
    }
    match args[0].as_str() {
        "check" => CommandResult::Success("Checking for updates...".to_string()),
        "apply" => CommandResult::Success("Applying updates...".to_string()),
        "rollback" => CommandResult::Success("Rolling back updates...".to_string()),
        _ => CommandResult::Error(format!("Unknown update subcommand: {}", args[0])),
    }
}

fn help_handler(_args: &[String]) -> CommandResult {
    CommandResult::Help("SigmaOS Help System".to_string())
}

impl fmt::Display for CommandResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandResult::Success(msg) => write!(f, "✓ {}", msg),
            CommandResult::Error(msg) => write!(f, "✗ {}", msg),
            CommandResult::Help(msg) => write!(f, "{}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_unknown_command() {
        let router = Router::new();
        let result = router.route(&["unknown".to_string()]);
        assert!(matches!(result, CommandResult::Error(_)));
    }

    #[test]
    fn test_route_pkg_install() {
        let router = Router::new();
        let result = router.route(&["pkg".to_string(), "install".to_string(), "vim".to_string()]);
        assert!(matches!(result, CommandResult::Success(_)));
    }

    #[test]
    fn test_route_empty() {
        let router = Router::new();
        let result = router.route(&[]);
        assert!(matches!(result, CommandResult::Help(_)));
    }

    #[test]
    fn test_command_groups() {
        let router = Router::new();
        let groups = router.list_groups();
        assert!(!groups.is_empty());
        assert!(groups.contains(&&CommandGroup::Pkg));
        assert!(groups.contains(&&CommandGroup::Help));
    }

    #[test]
    fn test_group_descriptions() {
        assert_eq!(CommandGroup::Pkg.description(), "Package management helpers");
        assert_eq!(CommandGroup::Hw.description(), "Hardware detection");
    }
}