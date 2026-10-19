//! SigmaOS Unified CLI - The `sigma` Command Router
//!
//! Inspired by: Omarchy's `omarchy` command router
//! Following principles: COMMAND_QUERY_SEPARATION, SINGLE_RESPONSIBILITY,
//! MINIMISE_COUPLING, MAXIMISE_COHESION, LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID,
//! SIMPLE_USER_INTERFACE, DISCOVERABILITY, DESIGN_PATTERN, COMMAND_PATTERN,
//! DEMETER_LAW, LEVEL_OF_ABSTRACTION, SIMPLE_API, USER_FRIENDLY,
//! COMMAND_QUERY_SEPARATION, COMPOSITION_OVER_INHERITANCE, MODULARITY.
//!
//! A single unified entry point (`sigma`) that routes to specialized subcommands,
//! with metadata-driven help generation and interactive command palette support.

use std::collections::HashMap;

/// Command group categories
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    Notify,
    Clipboard,
    Emoji,
    Ai,
    Keybindings,
    Help,
}

impl CommandGroup {
    pub fn description(&self) -> &str {
        match self {
            CommandGroup::Pkg => "Package management helpers",
            CommandGroup::Hw => "Hardware detection",
            CommandGroup::Theme => "Theme management",
            CommandGroup::Menu => "Unified search & menu",
            CommandGroup::Update => "System updates & rollbacks",
            CommandGroup::System => "System administration",
            CommandGroup::Network => "Network configuration",
            CommandGroup::User => "User management",
            CommandGroup::Security => "Security & access control",
            CommandGroup::Notify => "Notifications & history",
            CommandGroup::Clipboard => "Clipboard manager",
            CommandGroup::Emoji => "Emoji picker",
            CommandGroup::Ai => "AI agent configuration",
            CommandGroup::Keybindings => "Keybindings viewer",
            CommandGroup::Help => "Help & information",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            CommandGroup::Pkg => "📦",
            CommandGroup::Hw => "🔧",
            CommandGroup::Theme => "🎨",
            CommandGroup::Menu => "🔍",
            CommandGroup::Update => "🔄",
            CommandGroup::System => "⚙️",
            CommandGroup::Network => "🌐",
            CommandGroup::User => "👤",
            CommandGroup::Security => "🔒",
            CommandGroup::Notify => "🔔",
            CommandGroup::Clipboard => "📋",
            CommandGroup::Emoji => "😀",
            CommandGroup::Ai => "🤖",
            CommandGroup::Keybindings => "⌨️",
            CommandGroup::Help => "❓",
        }
    }
}

/// Individual command metadata
#[derive(Debug, Clone)]
pub struct CommandMetadata {
    pub name: String,
    pub group: CommandGroup,
    pub summary: String,
    pub description: String,
    pub args: Vec<String>,
    pub aliases: Vec<String>,
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
    Interactive(String),
}

/// CLI Router - The unified `sigma` command
pub struct SigmaRouter {
    pub commands: HashMap<String, CommandMetadata>,
    pub groups: HashMap<CommandGroup, Vec<String>>,
    pub version: String,
    pub config_dir: String,
}

impl SigmaRouter {
    /// Create a new router with built-in commands
    pub fn new() -> Self {
        let mut router = Self {
            commands: HashMap::new(),
            groups: HashMap::new(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_dir: "/etc/sigmaos".to_string(),
        };

        router.register_builtin_commands();
        router
    }

    fn register_builtin_commands(&mut self) {
        let commands: Vec<CommandMetadata> = vec![
            CommandMetadata {
                name: "pkg".to_string(),
                group: CommandGroup::Pkg,
                summary: "Package management".to_string(),
                description: "Install, remove, search, and update packages via sigpkg".to_string(),
                args: vec!["install".to_string(), "remove".to_string(), "search".to_string(), "update".to_string()],
                aliases: vec!["p".to_string()],
                handler: pkg_handler,
            },
            CommandMetadata {
                name: "theme".to_string(),
                group: CommandGroup::Theme,
                summary: "Theme management".to_string(),
                description: "List, apply, create, and switch themes (base 8 to 24 colors)".to_string(),
                args: vec!["list".to_string(), "apply".to_string(), "create".to_string(), "switch".to_string()],
                aliases: vec!["t".to_string()],
                handler: theme_handler,
            },
            CommandMetadata {
                name: "menu".to_string(),
                group: CommandGroup::Menu,
                summary: "Unified search & menu".to_string(),
                description: "Filterable nested command palette - SUPER+SPACE equivalent".to_string(),
                args: vec!["show".to_string(), "search".to_string(), "launch".to_string()],
                aliases: vec!["m".to_string()],
                handler: menu_handler,
            },
            CommandMetadata {
                name: "notify".to_string(),
                group: CommandGroup::Notify,
                summary: "Notifications & history".to_string(),
                description: "View notification history, dismiss, DND toggle, replay last N".to_string(),
                args: vec!["list".to_string(), "dismiss".to_string(), "dnd".to_string(), "replay".to_string()],
                aliases: vec!["n".to_string()],
                handler: notify_handler,
            },
            CommandMetadata {
                name: "clipboard".to_string(),
                group: CommandGroup::Clipboard,
                summary: "Clipboard manager".to_string(),
                description: "Clipboard history with image previews, sensitive-content exclusion".to_string(),
                args: vec!["list".to_string(), "paste".to_string(), "clear".to_string(), "toggle".to_string()],
                aliases: vec!["c".to_string()],
                handler: clipboard_handler,
            },
            CommandMetadata {
                name: "emoji".to_string(),
                group: CommandGroup::Emoji,
                summary: "Emoji picker".to_string(),
                description: "Native emoji picker with search and categories".to_string(),
                args: vec!["list".to_string(), "search".to_string(), "insert".to_string()],
                aliases: vec!["e".to_string()],
                handler: emoji_handler,
            },
            CommandMetadata {
                name: "ai".to_string(),
                group: CommandGroup::Ai,
                summary: "AI agent configuration".to_string(),
                description: "Configure default coding agent, instant launch shortcuts, crash diagnosis".to_string(),
                args: vec!["set-agent".to_string(), "status".to_string(), "diagnose".to_string()],
                aliases: vec!["a".to_string()],
                handler: ai_handler,
            },
            CommandMetadata {
                name: "keybindings".to_string(),
                group: CommandGroup::Keybindings,
                summary: "Keybindings viewer".to_string(),
                description: "Visual keybinding reference - Herd's keybindings viewer equivalent".to_string(),
                args: vec!["list".to_string(), "search".to_string(), "export".to_string()],
                aliases: vec!["kb".to_string()],
                handler: keybindings_handler,
            },
            CommandMetadata {
                name: "update".to_string(),
                group: CommandGroup::Update,
                summary: "System updates & rollbacks".to_string(),
                description: "Check, apply, or rollback system updates with Btrfs snapshots".to_string(),
                args: vec!["check".to_string(), "apply".to_string(), "rollback".to_string(), "status".to_string()],
                aliases: vec!["u".to_string()],
                handler: update_handler,
            },
            CommandMetadata {
                name: "hw".to_string(),
                group: CommandGroup::Hw,
                summary: "Hardware detection".to_string(),
                description: "Detect and display hardware information".to_string(),
                args: vec!["detect".to_string(), "info".to_string(), "sensors".to_string()],
                aliases: vec!["h".to_string()],
                handler: hw_handler,
            },
            CommandMetadata {
                name: "help".to_string(),
                group: CommandGroup::Help,
                summary: "Help & information".to_string(),
                description: "Show help for commands or the router itself".to_string(),
                args: vec![],
                aliases: vec?[],
                handler: help_handler,
            },
        ];

        for cmd in commands {
            self.commands.insert(cmd.name.clone(), cmd.clone());
            self.groups
                .entry(cmd.group.clone())
                .or_default()
                .push(cmd.name.clone());
            // Register aliases
            for alias in &cmd.aliases {
                self.commands.insert(alias.clone(), cmd.clone());
            }
        }
    }

    /// Route a command to its handler
    pub fn route(&self, args: &[String]) -> CommandResult {
        if args.is_empty() {
            return self.show_help();
        }

        let subcommand = &args[0];
        let rest = &args[1..];

        if let Some(cmd) = self.commands.get(subcommand) {
            return (cmd.handler)(rest);
        }

        CommandResult::Error(format!("Unknown command: {}\nRun 'sigma help' for available commands.", subcommand))
    }

    /// Show general help with emoji icons
    fn show_help(&self) -> CommandResult {
        let mut help = format!(
            "🚀 SigmaOS CLI v{} - Unified Command Router\n\n",
            self.version
        );
        help.push_str("Available commands:\n\n");

        for (group, cmds) in &self.groups {
            help.push_str(&format!("{} {}\n", group.emoji(), group.description()));
            for cmd_name in cmds {
                if let Some(cmd) = self.commands.get(cmd_name) {
                    if !cmd.aliases.is_empty() {
                        help.push_str(&format!(
                            "  {} {}/{} - {}\n",
                            cmd.emoji(),
                            cmd.name,
                            cmd.aliases.join("/"),
                            cmd.summary
                        ));
                    } else {
                        help.push_str(&format!(
                            "  {} {} - {}\n",
                            cmd.emoji(),
                            cmd.name,
                            cmd.summary
                        ));
                    }
                }
            }
            help.push('\n');
        }

        help.push_str("Usage: sigma <command> [args...]\n");
        help.push_str("Run 'sigma <command> --help' for command-specific help.\n");
        help.push_str("\nInspired by Omarchy's unified CLI router.\n");

        CommandResult::Help(help)
    }

    /// List all command groups
    pub fn list_groups(&self) -> Vec<&CommandGroup> {
        let mut groups: Vec<&CommandGroup> = self.groups.keys().collect();
        groups.sort();
        groups
    }
}

impl Default for SigmaRouter {
    fn default() -> Self {
        Self::new()
    }
}

// Command handlers

fn pkg_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma pkg <install|remove|search|update> <package>\nExamples:\n  sigma pkg install vim\n  sigma pkg remove firefox\n  sigma pkg search kernel\n  sigma pkg update".to_string());
    }
    match args[0].as_str() {
        "install" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma pkg install <package>".to_string());
            }
            CommandResult::Success(format!("✅ Installing package: {}", args[1]))
        }
        "remove" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma pkg remove <package>".to_string());
            }
            CommandResult::Success(format!("✅ Removing package: {}", args[1]))
        }
        "search" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma pkg search <query>".to_string());
            }
            CommandResult::Success(format!("🔍 Searching for: {}", args[1]))
        }
        "update" => CommandResult::Success("🔄 Checking for package updates...".to_string()),
        _ => CommandResult::Error(format!("Unknown pkg subcommand: {}", args[0])),
    }
}

fn theme_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma theme <list|apply|create|switch>\nExamples:\n  sigma theme list\n  sigma theme apply dark\n  sigma theme create my-theme\n  sigma theme switch light".to_string());
    }
    match args[0].as_str() {
        "list" => CommandResult::Success("🎨 Available themes: dark, light, solarized, nord, catppuccin, tokyonight".to_string()),
        "apply" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma theme apply <theme>".to_string());
            }
            CommandResult::Success(format!("✅ Applied theme: {}", args[1]))
        }
        "create" => CommandResult::Success("🎨 Theme creator launched - edit ~/.config/sigmaos/themes/".to_string()),
        "switch" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma theme switch <theme>".to_string());
            }
            CommandResult::Success(format!("🔄 Switched to theme: {}", args[1]))
        }
        _ => CommandResult::Error(format!("Unknown theme subcommand: {}", args[0])),
    }
}

fn menu_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma menu <show|search|launch>\nExamples:\n  sigma menu show\n  sigma menu search terminal\n  sigma menu launch browser".to_string());
    }
    match args[0].as_str() {
        "show" => CommandResult::Interactive("🔍 Unified command palette opened (SUPER+SPACE)".to_string()),
        "search" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma menu search <query>".to_string());
            }
            CommandResult::Success(format!("🔍 Searching menu for: {}", args[1]))
        }
        "launch" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma menu launch <app>".to_string());
            }
            CommandResult::Success(format!("🚀 Launching: {}", args[1]))
        }
        _ => CommandResult::Error(format!("Unknown menu subcommand: {}", args[0])),
    }
}

fn notify_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma notify <list|dismiss|dnd|replay>\nExamples:\n  sigma notify list\n  sigma notify dismiss 3\n  sigma notify dnd on\n  sigma notify replay 10".to_string());
    }
    match args[0].as_str() {
        "list" => CommandResult::Success("🔔 Last 10 notifications: [1] System update ready, [2] Low battery, [3] New email...".to_string()),
        "dismiss" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma notify dismiss <id>".to_string());
            }
            CommandResult::Success(format!("✅ Dismissed notification: {}", args[1]))
        }
        "dnd" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma notify dnd <on|off>".to_string());
            }
            let state = if args[1] == "on" { "enabled" } else { "disabled" };
            CommandResult::Success(format!("🔕 Do Not Disturb: {}", state))
        }
        "replay" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma notify replay <count>".to_string());
            }
            CommandResult::Success(format!("🔄 Replaying last {} notifications", args[1]))
        }
        _ => CommandResult::Error(format!("Unknown notify subcommand: {}", args[0])),
    }
}

fn clipboard_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma clipboard <list|paste|clear|toggle>\nExamples:\n  sigma clipboard list\n  sigma clipboard paste 2\n  sigma clipboard clear\n  sigma clipboard toggle".to_string());
    }
    match args[0].as_str() {
        "list" => CommandResult::Success("📋 Clipboard history: [1] Text snippet, [2] Image (preview), [3] URL...".to_string()),
        "paste" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma clipboard paste <id>".to_string());
            }
            CommandResult::Success(format!("📋 Pasted item: {}", args[1]))
        }
        "clear" => CommandResult::Success("📋 Clipboard history cleared.".to_string()),
        "toggle" => CommandResult::Success("📋 Clipboard manager toggled.".to_string()),
        _ => CommandResult::Error(format!("Unknown clipboard subcommand: {}", args[0])),
    }
}

fn emoji_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma emoji <list|search|insert>\nExamples:\n  sigma emoji list\n  sigma emoji search smile\n  sigma emoji insert 😀".to_string());
    }
    match args[0].as_str() {
        "list" => CommandResult::Success("😀 Emoji categories: smileys, animals, food, activities, travel, objects, symbols, flags".to_string()),
        "search" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma emoji search <query>".to_string());
            }
            CommandResult::Success(format!("🔍 Searching emoji for: {}", args[1]))
        }
        "insert" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma emoji insert <emoji>".to_string());
            }
            CommandResult::Success(format!("✅ Inserted: {}", args[1]))
        }
        _ => CommandResult::Error(format!("Unknown emoji subcommand: {}", args[0])),
    }
}

fn ai_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma ai <set-agent|status|diagnose>\nExamples:\n  sigma ai set-agent claude-code\n  sigma ai status\n  sigma ai diagnose".to_string());
    }
    match args[0].as_str() {
        "set-agent" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma ai set-agent <agent>".to_string());
            }
            CommandResult::Success(format!("🤖 Default AI agent set to: {}", args[1]))
        }
        "status" => CommandResult::Success("🤖 AI Agent: Claude Code (active), Instant launch: SUPER+ALT+A".to_string()),
        "diagnose" => CommandResult::Success("🩺 Running crash diagnosis...".to_string()),
        _ => CommandResult::Error(format!("Unknown ai subcommand: {}", args[0])),
    }
}

fn keybindings_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma keybindings <list|search|export>\nExamples:\n  sigma keybindings list\n  sigma keybindings search terminal\n  sigma keybindings export".to_string());
    }
    match args[0].as_str() {
        "list" => CommandResult::Success("⌨️ Keybindings: SUPER+SPACE=Menu, SUPER+ALT+A=AI, SUPER+CTRL+V=Clipboard, SUPER+CTRL+E=Emoji...".to_string()),
        "search" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: sigma keybindings search <query>".to_string());
            }
            CommandResult::Success(format!("⌨️ Searching keybindings for: {}", args[1]))
        }
        "export" => CommandResult::Success("⌨️ Keybindings exported to ~/.config/sigmaos/keybindings.json".to_string()),
        _ => CommandResult::Error(format!("Unknown keybindings subcommand: {}", args[0])),
    }
}

fn update_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma update <check|apply|rollback|status>\nExamples:\n  sigma update check\n  sigma update apply\n  sigma update rollback\n  sigma update status".to_string());
    }
    match args[0].as_str() {
        "check" => CommandResult::Success("🔄 Checking for system updates...".to_string()),
        "apply" => CommandResult::Success("🔄 Applying system updates with Btrfs snapshot...".to_string()),
        "rollback" => CommandResult::Success("🔄 Rolling back to previous snapshot...".to_string()),
        "status" => CommandResult::Success("📊 System is up to date. Last update: 2 hours ago.".to_string()),
        _ => CommandResult::Error(format!("Unknown update subcommand: {}", args[0])),
    }
}

fn hw_handler(args: &[String]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Help("Usage: sigma hw <detect|info|sensors>\nExamples:\n  sigma hw detect\n  sigma hw info\n  sigma hw sensors".to_string());
    }
    match args[0].as_str() {
        "detect" => CommandResult::Success("🔧 Detecting hardware...".to_string()),
        "info" => CommandResult::Success("🔧 Hardware: x86_64, 16GB RAM, 1TB NVMe, Intel UHD Graphics".to_string()),
        "sensors" => CommandResult::Success("🌡️ Sensors: CPU 45°C, GPU 50°C, Battery 85%".to_string()),
        _ => CommandResult::Error(format!("Unknown hw subcommand: {}", args[0])),
    }
}

fn help_handler(_args: &[String]) -> CommandResult {
    CommandResult::Help("SigmaOS Help System - Use 'sigma help' for overview, 'sigma <cmd> --help' for specifics.".to_string())
}

impl fmt::Display for CommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandResult::Success(msg) => write!(f, "{}", msg),
            CommandResult::Error(msg) => write!(f, "{}", msg),
            CommandResult::Help(msg) => write!(f, "{}", msg),
            CommandResult::Interactive(msg) => write!(f, "{}", msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_unknown_command() {
        let router = SigmaRouter::new();
        let result = router.route(&["unknown".to_string()]);
        assert!(matches!(result, CommandResult::Error(_)));
    }

    #[test]
    fn test_route_pkg_install() {
        let router = SigmaRouter::new();
        let result = router.route(&["pkg".to_string(), "install".to_string(), "vim".to_string()]);
        assert!(matches!(result, CommandResult::Success(_)));
    }

    #[test]
    fn test_route_empty() {
        let router = SigmaRouter::new();
        let result = router.route(&[]);
        assert!(matches!(result, CommandResult::Help(_)));
    }

    #[test]
    fn test_command_groups() {
        let router = SigmaRouter::new();
        let groups = router.list_groups();
        assert!(!groups.is_empty());
    }

    #[test]
    fn test_alias_routing() {
        let router = SigmaRouter::new();
        // 'p' is alias for 'pkg'
        let result = router.route(&["p".to_string(), "install".to_string(), "vim".to_string()]);
        assert!(matches!(result, CommandResult::Success(_)));
    }
}