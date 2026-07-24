// SigmaOS Shell REPL (Read-Eval-Print Loop)
// Interactive shell for SigmaOS

use std::io::{self, BufRead, Write};

/// Shell command type
#[derive(Debug, Clone)]
pub enum ShellCommand {
    Help,
    ListProcesses,
    ListFiles,
    Exit,
    Echo {
        message: String,
    },
    Set {
        variable: String,
        value: String,
    },
    Get {
        variable: String,
    },
    Pwd,
    WhoAmI,
    Su {
        username: String,
        password: Option<String>,
    },
    Cat {
        filename: String,
    },
    Systemctl {
        action: String,
        service: String,
    },
    Apt {
        subcommand: String,
        package: Option<String>,
    },
    Ai {
        query: String,
    },
    Display {
        subcommand: String,
        args: Vec<String>,
    },
    Theme {
        subcommand: String,
        args: Vec<String>,
    },
    Profile {
        subcommand: String,
        args: Vec<String>,
    },
    Window {
        subcommand: String,
        args: Vec<String>,
    },
    Accessibility {
        subcommand: String,
        args: Vec<String>,
    },
    Screenshot {
        subcommand: String,
        args: Vec<String>,
    },
    Record {
        subcommand: String,
        args: Vec<String>,
    },
    Clipboard {
        subcommand: String,
        args: Vec<String>,
    },
    Unknown(String),
}

/// Shell REPL
pub struct ShellRepl {
    running: bool,
    variables: std::collections::HashMap<String, String>,
    prompt: String,
    current_user: String,
    current_dir: String,
    services: std::collections::HashMap<String, String>,
    installed_packages: std::collections::HashSet<String>,

    // Persistent state matching GUI Capabilities (Zenith Desktop)
    active_theme: String,
    active_profile: String,
    active_layout: String,
    clipboard_content: String,
    screen_reader_enabled: bool,
    high_contrast_enabled: bool,
    magnifier_zoom: f32,
    color_blind_mode: String,
    recording_active: bool,
    displays: Vec<String>,
    windows: Vec<String>,
}

impl ShellRepl {
    pub fn new() -> Self {
        let mut services = std::collections::HashMap::new();
        services.insert("systemd-networkd".to_string(), "Running".to_string());
        services.insert("systemd-logind".to_string(), "Running".to_string());
        services.insert("cron".to_string(), "Stopped".to_string());
        services.insert("udev".to_string(), "Running".to_string());

        let mut installed_packages = std::collections::HashSet::new();
        installed_packages.insert("sigma-sh".to_string());
        installed_packages.insert("sigma-core".to_string());

        let mut displays = Vec::new();
        displays.push("DP-1: primary 2560x1440@144Hz scale=1.0 hdr=false".to_string());
        displays.push("HDMI-1: secondary 1920x1080@60Hz scale=1.0 hdr=false".to_string());

        let mut windows = Vec::new();
        windows.push("ID=1 Title='SigmaTerminal' App='sigma.terminal' Geom=0,0,800,600 State=Normal Focused=true".to_string());
        windows.push("ID=2 Title='SigmaOffice' App='sigma.office' Geom=100,100,1024,768 State=Normal Focused=false".to_string());

        Self {
            running: true,
            variables: std::collections::HashMap::new(),
            prompt: "ubuntu@sigmaos:~$ ".to_string(),
            current_user: "ubuntu".to_string(),
            current_dir: "/home/ubuntu".to_string(),
            services,
            installed_packages,
            active_theme: "Sovereign Dark".to_string(),
            active_profile: "standard".to_string(),
            active_layout: "Adaptive".to_string(),
            clipboard_content: "Initial clipboard context (0x5001)".to_string(),
            screen_reader_enabled: false,
            high_contrast_enabled: false,
            magnifier_zoom: 1.0,
            color_blind_mode: "none".to_string(),
            recording_active: false,
            displays,
            windows,
        }
    }

    pub fn with_prompt(prompt: String) -> Self {
        let mut services = std::collections::HashMap::new();
        services.insert("systemd-networkd".to_string(), "Running".to_string());
        services.insert("systemd-logind".to_string(), "Running".to_string());

        Self {
            running: true,
            variables: std::collections::HashMap::new(),
            prompt,
            current_user: "ubuntu".to_string(),
            current_dir: "/home/ubuntu".to_string(),
            services,
            installed_packages: std::collections::HashSet::new(),
            active_theme: "Sovereign Dark".to_string(),
            active_profile: "standard".to_string(),
            active_layout: "Adaptive".to_string(),
            clipboard_content: String::new(),
            screen_reader_enabled: false,
            high_contrast_enabled: false,
            magnifier_zoom: 1.0,
            color_blind_mode: "none".to_string(),
            recording_active: false,
            displays: Vec::new(),
            windows: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        println!("SigmaOS Shell v0.1.0");
        println!("Type 'help' for available commands\n");

        let stdin = io::stdin();
        let mut stdout = io::stdout();

        while self.running {
            print!("{}", self.prompt);
            stdout.flush().unwrap();

            let mut input = String::new();
            stdin.lock().read_line(&mut input).unwrap();

            let input = input.trim();
            if !input.is_empty() {
                self.execute_line(input);
            }
        }

        println!("Goodbye!");
    }

    fn execute_line(&mut self, line: &str) {
        let command = self.parse_command(line);
        let result = self.execute_command(command);

        match result {
            Ok(output) => {
                if !output.is_empty() {
                    println!("{}", output);
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
    }

    fn parse_command(&self, input: &str) -> ShellCommand {
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            return ShellCommand::Unknown(input.to_string());
        }

        match parts[0] {
            "help" => ShellCommand::Help,
            "ps" => ShellCommand::ListProcesses,
            "ls" => ShellCommand::ListFiles,
            "exit" | "quit" => ShellCommand::Exit,
            "pwd" => ShellCommand::Pwd,
            "whoami" => ShellCommand::WhoAmI,
            "echo" => {
                let message = parts[1..].join(" ");
                ShellCommand::Echo { message }
            }
            "su" => {
                if parts.len() >= 2 {
                    let password = if parts.len() >= 3 {
                        Some(parts[2].to_string())
                    } else {
                        None
                    };
                    ShellCommand::Su {
                        username: parts[1].to_string(),
                        password,
                    }
                } else {
                    ShellCommand::Su {
                        username: "root".to_string(),
                        password: None,
                    }
                }
            }
            "cat" => {
                if parts.len() >= 2 {
                    ShellCommand::Cat {
                        filename: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "systemctl" => {
                if parts.len() >= 2 {
                    let action = parts[1].to_string();
                    let service = if parts.len() >= 3 {
                        parts[2].to_string()
                    } else {
                        String::new()
                    };
                    ShellCommand::Systemctl { action, service }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "apt" => {
                if parts.len() >= 2 {
                    let subcommand = parts[1].to_string();
                    let package = if parts.len() >= 3 {
                        Some(parts[2].to_string())
                    } else {
                        None
                    };
                    ShellCommand::Apt {
                        subcommand,
                        package,
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "set" => {
                if parts.len() >= 3 {
                    ShellCommand::Set {
                        variable: parts[1].to_string(),
                        value: parts[2..].join(" "),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "get" => {
                if parts.len() >= 2 {
                    ShellCommand::Get {
                        variable: parts[1].to_string(),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "ai" => {
                if parts.len() >= 2 {
                    ShellCommand::Ai {
                        query: parts[1..].join(" "),
                    }
                } else {
                    ShellCommand::Unknown(input.to_string())
                }
            }
            "display" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "list".to_string() };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Display { subcommand, args }
            }
            "theme" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "list".to_string() };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Theme { subcommand, args }
            }
            "profile" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "list".to_string() };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Profile { subcommand, args }
            }
            "window" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "list".to_string() };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Window { subcommand, args }
            }
            "accessibility" | "acc" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "status".to_string() };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Accessibility { subcommand, args }
            }
            "screenshot" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "capture".to_string() };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Screenshot { subcommand, args }
            }
            "record" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "start".to_string() };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Record { subcommand, args }
            }
            "clipboard" => {
                let subcommand = if parts.len() >= 2 { parts[1].to_string() } else { "get".to_string() };
                let args = parts[2..].iter().map(|s| s.to_string()).collect();
                ShellCommand::Clipboard { subcommand, args }
            }
            _ => ShellCommand::Unknown(input.to_string()),
        }
    }

    fn execute_command(&mut self, command: ShellCommand) -> Result<String, String> {
        match command {
            ShellCommand::Help => Ok("Available commands:\n\
                   help         - Show this help message\n\
                   ps           - List running processes\n\
                   ls           - List files\n\
                   pwd          - Print working directory\n\
                   whoami       - Print current logged-in user\n\
                   su <user>    - Switch user account (try 'su root' or 'su guest')\n\
                   cat <file>   - Display file contents\n\
                   systemctl    - Manage systemd services (try 'systemctl list' or 'systemctl status <service>')\n\
                   apt <cmd>    - Advanced Package Tool (try 'apt update', 'apt search <pkg>', or 'apt install <pkg>')\n\
                   echo         - Print a message\n\
                   set          - Set a variable\n\
                   get          - Get a variable\n\
                   ai <query>   - Natural language command AI\n\
                   display      - Manage displays (try 'display list', 'display scale', or 'display rate')\n\
                   theme        - Manage themes (try 'theme list', 'theme set <theme>', or 'theme auto')\n\
                   profile      - Manage UX profiles (try 'profile list', 'profile switch <profile>', or 'profile layout')\n\
                   window       - Manage windows (try 'window list', 'window create', 'window state', or 'window close')\n\
                   accessibility / acc - Manage accessibility (try 'acc status', 'acc screen-reader', or 'acc magnifier')\n\
                   screenshot   - Capture desktop screenshot\n\
                   record       - Manage video recording (try 'record start' or 'record stop')\n\
                   clipboard    - Manage clipboard (try 'clipboard get' or 'clipboard set <text>')\n\
                   exit         - Exit the shell"
                .to_string()),
            ShellCommand::ListProcesses => Ok("PID  NAME        STATE\n\
                   1    sigma-sh    Running\n\
                   2    systemd     Running\n\
                   3    udevd       Running"
                .to_string()),
            ShellCommand::ListFiles => Ok("README.md\n\
                   Cargo.toml\n\
                   src/\n\
                   tests/"
                .to_string()),
            ShellCommand::Exit => {
                self.running = false;
                Ok(String::new())
            }
            ShellCommand::Pwd => Ok(self.current_dir.clone()),
            ShellCommand::WhoAmI => Ok(self.current_user.clone()),
            ShellCommand::Su { username, password } => {
                if username == "root" {
                    let pwd = password.unwrap_or_default();
                    if pwd == "admin" || pwd == "root" {
                        self.current_user = "root".to_string();
                        self.current_dir = "/root".to_string();
                        self.prompt = "root@sigmaos:# ".to_string();
                        Ok("Successfully logged in as root.".to_string())
                    } else {
                        Err("su: Authentication failure (hint: use 'su root admin')".to_string())
                    }
                } else {
                    self.current_user = username.clone();
                    self.current_dir = format!("/home/{}", username);
                    self.prompt = format!("{}@sigmaos:~$ ", username);
                    Ok(format!("Logged in as {}.", username))
                }
            }
            ShellCommand::Cat { filename } => {
                if filename == "README.md" {
                    Ok("# 🛡️ SigmaOS — Sovereign, AI-Native Operating System".to_string())
                } else if filename == "Cargo.toml" {
                    Ok("[package]\nname = \"sigmaos\"\nversion = \"0.1.0\"".to_string())
                } else {
                    Err(format!("cat: {}: No such file or directory", filename))
                }
            }
            ShellCommand::Systemctl { action, service } => {
                if action == "list" || action == "status" && service.is_empty() {
                    let mut list_str = "UNIT                ACTIVE   SUB\n".to_string();
                    for (s, st) in &self.services {
                        list_str.push_str(&format!("{:<20} {}  {}\n", s, if st == "Running" { "active" } else { "inactive" }, st));
                    }
                    Ok(list_str)
                } else if action == "start" {
                    if self.services.contains_key(&service) {
                        self.services.insert(service.clone(), "Running".to_string());
                        Ok(format!("Started {} service.", service))
                    } else {
                        Err(format!("Failed to start {}.service: Unit not found.", service))
                    }
                } else if action == "stop" {
                    if self.services.contains_key(&service) {
                        self.services.insert(service.clone(), "Stopped".to_string());
                        Ok(format!("Stopped {} service.", service))
                    } else {
                        Err(format!("Failed to stop {}.service: Unit not found.", service))
                    }
                } else if action == "status" {
                    if let Some(status) = self.services.get(&service) {
                        Ok(format!("● {}.service\n   Active: {} ({})\n   Main PID: 1234", service, if status == "Running" { "active" } else { "inactive" }, status))
                    } else {
                        Err(format!("Unit {}.service could not be found.", service))
                    }
                } else {
                    Err(format!("systemctl: Unknown action '{}'", action))
                }
            }
            ShellCommand::Apt { subcommand, package } => {
                if subcommand == "update" {
                    Ok("Hit:1 http://archive.ubuntu.com/ubuntu noble InRelease\n\
                        Get:2 http://security.ubuntu.com/ubuntu noble-security InRelease\n\
                        Reading package lists... Done\n\
                        Building dependency tree... Done\n\
                        All packages are up to date."
                        .to_string())
                } else if subcommand == "list" {
                    let mut list_str = "Listing installed packages...\n".to_string();
                    for pkg in &self.installed_packages {
                        list_str.push_str(&format!("{}/noble,now 1.0.0 amd64 [installed]\n", pkg));
                    }
                    Ok(list_str)
                } else if subcommand == "search" {
                    let query = package.unwrap_or_default();
                    if query.is_empty() {
                        Ok("sigma-sh - Sovereign Shell\n\
                            sigma-vim - High-fidelity Editor\n\
                            sigma-curl - Lightweight HTTP Client"
                            .to_string())
                    } else {
                        let mut results = Vec::new();
                        let all_packages = ["sigma-sh", "sigma-vim", "sigma-curl", "sigma-gcc", "sigma-git", "sigma-python"];
                        for pkg in &all_packages {
                            if pkg.contains(&query) {
                                results.push(format!("{} - Package matching query", pkg));
                            }
                        }
                        if results.is_empty() {
                            Ok("No matching packages found.".to_string())
                        } else {
                            Ok(results.join("\n"))
                        }
                    }
                } else if subcommand == "install" {
                    let pkg = package.ok_or_else(|| "apt: Please specify a package to install".to_string())?;
                    self.installed_packages.insert(pkg.clone());
                    Ok(format!("Reading package lists...\n\
                                Building dependency tree...\n\
                                The following NEW packages will be installed:\n\
                                  {}\n\
                                Preparing to unpack ...\n\
                                Unpacking {} ...\n\
                                Setting up {} ...\n\
                                Successfully installed.", pkg, pkg, pkg))
                } else {
                    Err(format!("apt: Unknown command '{}'", subcommand))
                }
            }
            ShellCommand::Echo { message } => Ok(message),
            ShellCommand::Set { variable, value } => {
                self.variables.insert(variable.clone(), value.clone());
                Ok(format!("{} = {}", variable, value))
            }
            ShellCommand::Get { variable } => match self.variables.get(&variable) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Variable '{}' not found", variable)),
            },
            ShellCommand::Ai { query } => {
                let mut aid = crate::ml::SigmaAid::new(0);
                let _ = aid.load_gguf_model("/models/sigma.gguf");
                let cmd = aid.execute_prompt(&query);
                Ok(format!("AI suggested command: {}", cmd))
            },
            ShellCommand::Display { subcommand, args } => {
                if subcommand == "list" {
                    Ok(self.displays.join("\n"))
                } else if subcommand == "set" {
                    Ok("Set layout: primary DP-1, secondary HDMI-1, arrange right".to_string())
                } else if subcommand == "scale" {
                    let output = args.get(0).cloned().unwrap_or_else(|| "DP-1".to_string());
                    let factor = args.get(1).cloned().unwrap_or_else(|| "2.0".to_string());
                    Ok(format!("Adjusted scaling factor for {} to {}.", output, factor))
                } else if subcommand == "rate" {
                    let output = args.get(0).cloned().unwrap_or_else(|| "DP-1".to_string());
                    let rate = args.get(1).cloned().unwrap_or_else(|| "144".to_string());
                    Ok(format!("Adjusted refresh rate for {} to {}Hz.", output, rate))
                } else if subcommand == "hdr" {
                    let output = args.get(0).cloned().unwrap_or_else(|| "DP-1".to_string());
                    let enable = args.get(1).cloned().unwrap_or_else(|| "true".to_string());
                    Ok(format!("HDR support on {} set to {}.", output, enable))
                } else {
                    Err("Unknown display command".to_string())
                }
            }
            ShellCommand::Theme { subcommand, args } => {
                if subcommand == "list" {
                    Ok("Sovereign Dark, Banaras Gold, Kashmir Blue, Midnight Teal, Paper White".to_string())
                } else if subcommand == "set" {
                    let theme = args.get(0).cloned().unwrap_or_else(|| "Banaras Gold".to_string());
                    self.active_theme = theme.clone();
                    Ok(format!("Theme changed to {}.", theme))
                } else if subcommand == "configure" {
                    Ok("Configured theme: accent=#6C63FF, blur=true, radius=20px.".to_string())
                } else if subcommand == "auto" {
                    Ok("Configured dynamic theming: enabled=true, mode=time.".to_string())
                } else {
                    Err("Unknown theme command".to_string())
                }
            }
            ShellCommand::Profile { subcommand, args } => {
                if subcommand == "list" {
                    Ok("standard, developer, gamer, guest".to_string())
                } else if subcommand == "switch" {
                    let profile = args.get(0).cloned().unwrap_or_else(|| "developer".to_string());
                    self.active_profile = profile.clone();
                    Ok(format!("UX profile changed to {}.", profile))
                } else if subcommand == "layout" {
                    let layout = args.get(0).cloned().unwrap_or_else(|| "tiling".to_string());
                    self.active_layout = layout.clone();
                    Ok(format!("Compositor layout set to {}.", layout))
                } else {
                    Err("Unknown profile command".to_string())
                }
            }
            ShellCommand::Window { subcommand, args } => {
                if subcommand == "list" {
                    Ok(self.windows.join("\n"))
                } else if subcommand == "create" {
                    let id = self.windows.len() + 1;
                    let title = args.get(0).cloned().unwrap_or_else(|| "SigmaApp".to_string());
                    let app = args.get(1).cloned().unwrap_or_else(|| "sigma.app".to_string());
                    let geom = args.get(2).cloned().unwrap_or_else(|| "100,100,500,400".to_string());
                    let w_str = format!("ID={} Title='{}' App='{}' Geom={} State=Normal Focused=true", id, title, app, geom);
                    self.windows.push(w_str);
                    Ok(format!("Created window ID {}: '{}' ({}) at geom {}", id, title, app, geom))
                } else if subcommand == "state" {
                    let id = args.get(0).cloned().unwrap_or_else(|| "1".to_string());
                    let state = args.get(1).cloned().unwrap_or_else(|| "Maximized".to_string());
                    Ok(format!("Window {} state changed to {}", id, state))
                } else if subcommand == "move" {
                    let id = args.get(0).cloned().unwrap_or_else(|| "1".to_string());
                    Ok(format!("Window {} moved/resized successfully.", id))
                } else if subcommand == "focus" {
                    let id = args.get(0).cloned().unwrap_or_else(|| "1".to_string());
                    Ok(format!("Window {} is now focused.", id))
                } else if subcommand == "close" {
                    let id = args.get(0).cloned().unwrap_or_else(|| "2".to_string());
                    Ok(format!("Window {} closed/destroyed.", id))
                } else {
                    Err("Unknown window command".to_string())
                }
            }
            ShellCommand::Accessibility { subcommand, args } => {
                if subcommand == "status" {
                    Ok(format!("Screen Reader: {}\nHigh Contrast: {}\nMagnifier: {}x\nColorblind: {}",
                        if self.screen_reader_enabled { "Enabled" } else { "Disabled" },
                        if self.high_contrast_enabled { "Enabled" } else { "Disabled" },
                        self.magnifier_zoom,
                        self.color_blind_mode
                    ))
                } else if subcommand == "screen-reader" {
                    let enable = args.get(0).cloned().unwrap_or_else(|| "true".to_string()) == "true";
                    self.screen_reader_enabled = enable;
                    Ok(format!("Screen reader enabled={}.", enable))
                } else if subcommand == "contrast" {
                    let enable = args.get(0).cloned().unwrap_or_else(|| "true".to_string()) == "true";
                    self.high_contrast_enabled = enable;
                    Ok(format!("High-contrast mode set to {}.", enable))
                } else if subcommand == "magnifier" {
                    let zoom = args.get(0).cloned().unwrap_or_else(|| "2.0".to_string()).parse::<f32>().unwrap_or(1.0);
                    self.magnifier_zoom = zoom;
                    Ok(format!("Magnification scale set to {}x.", zoom))
                } else if subcommand == "colorblind" {
                    let mode = args.get(0).cloned().unwrap_or_else(|| "protanopia".to_string());
                    self.color_blind_mode = mode.clone();
                    Ok(format!("Color blind correction mode set to {}.", mode))
                } else {
                    Err("Unknown accessibility command".to_string())
                }
            }
            ShellCommand::Screenshot { subcommand, args } => {
                let output = args.get(0).cloned().unwrap_or_else(|| "/home/ubuntu/screenshot.png".to_string());
                let region = args.get(1).cloned().unwrap_or_else(|| "full".to_string());
                Ok(format!("Screenshot captured successfully! Saved to {} ({}).", output, region))
            }
            ShellCommand::Record { subcommand, args } => {
                if subcommand == "start" {
                    self.recording_active = true;
                    let output = args.get(0).cloned().unwrap_or_else(|| "/home/ubuntu/recording.mp4".to_string());
                    let codec = args.get(1).cloned().unwrap_or_else(|| "av1".to_string());
                    let quality = args.get(2).cloned().unwrap_or_else(|| "high".to_string());
                    Ok(format!("Screen recording initiated. Output saved to {} using {} codec ({} quality).", output, codec, quality))
                } else if subcommand == "stop" {
                    self.recording_active = false;
                    Ok("Screen recording stopped and finalized.".to_string())
                } else {
                    Err("Unknown record command".to_string())
                }
            }
            ShellCommand::Clipboard { subcommand, args } => {
                if subcommand == "get" {
                    Ok(format!("Clipboard content: {} (capability token 0x5001 verified)", self.clipboard_content))
                } else if subcommand == "set" {
                    let text = args.join(" ");
                    self.clipboard_content = text.clone();
                    Ok(format!("Copied to clipboard: {}", text))
                } else {
                    Err("Unknown clipboard command".to_string())
                }
            }
            ShellCommand::Unknown(cmd) => Err(format!("Unknown command: {}", cmd)),
        }
    }
}

impl Default for ShellRepl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_creation() {
        let repl = ShellRepl::new();
        assert!(repl.running);
        assert_eq!(repl.prompt, "ubuntu@sigmaos:~$ ");
    }

    #[test]
    fn test_parse_help() {
        let repl = ShellRepl::new();
        let command = repl.parse_command("help");
        assert!(matches!(command, ShellCommand::Help));
    }

    #[test]
    fn test_parse_echo() {
        let repl = ShellRepl::new();
        let command = repl.parse_command("echo hello world");
        assert!(matches!(command, ShellCommand::Echo { .. }));
    }

    #[test]
    fn test_execute_echo() {
        let mut repl = ShellRepl::new();
        let command = ShellCommand::Echo {
            message: "test".to_string(),
        };
        let result = repl.execute_command(command);
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn test_set_get_variable() {
        let mut repl = ShellRepl::new();
        let set_cmd = ShellCommand::Set {
            variable: "test".to_string(),
            value: "value".to_string(),
        };
        repl.execute_command(set_cmd).unwrap();

        let get_cmd = ShellCommand::Get {
            variable: "test".to_string(),
        };
        let result = repl.execute_command(get_cmd);
        assert_eq!(result.unwrap(), "value");
    }

    #[test]
    fn test_exit() {
        let mut repl = ShellRepl::new();
        let command = ShellCommand::Exit;
        repl.execute_command(command).unwrap();
        assert!(!repl.running);
    }

    #[test]
    fn test_pwd_whoami() {
        let mut repl = ShellRepl::new();
        assert_eq!(
            repl.execute_command(ShellCommand::Pwd).unwrap(),
            "/home/ubuntu"
        );
        assert_eq!(
            repl.execute_command(ShellCommand::WhoAmI).unwrap(),
            "ubuntu"
        );
    }

    #[test]
    fn test_su_root() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Su {
                username: "root".to_string(),
                password: Some("admin".to_string())
            })
            .is_ok());
        assert_eq!(repl.execute_command(ShellCommand::WhoAmI).unwrap(), "root");
        assert_eq!(repl.execute_command(ShellCommand::Pwd).unwrap(), "/root");
    }

    #[test]
    fn test_cat_command() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Cat {
                filename: "README.md".to_string()
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Cat {
                filename: "nonexistent.txt".to_string()
            })
            .is_err());
    }

    #[test]
    fn test_systemctl_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Systemctl {
                action: "list".to_string(),
                service: String::new()
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Systemctl {
                action: "stop".to_string(),
                service: "cron".to_string()
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Systemctl {
                action: "start".to_string(),
                service: "cron".to_string()
            })
            .is_ok());
    }

    #[test]
    fn test_apt_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl
            .execute_command(ShellCommand::Apt {
                subcommand: "update".to_string(),
                package: None
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Apt {
                subcommand: "search".to_string(),
                package: Some("vim".to_string())
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Apt {
                subcommand: "install".to_string(),
                package: Some("sigma-vim".to_string())
            })
            .is_ok());
        assert!(repl
            .execute_command(ShellCommand::Apt {
                subcommand: "list".to_string(),
                package: None
            })
            .is_ok());
    }

    #[test]
    fn test_display_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl.execute_command(ShellCommand::Display { subcommand: "list".to_string(), args: vec![] }).is_ok());
        assert!(repl.execute_command(ShellCommand::Display { subcommand: "set".to_string(), args: vec![] }).is_ok());
        assert!(repl.execute_command(ShellCommand::Display { subcommand: "scale".to_string(), args: vec!["DP-1".to_string(), "2.0".to_string()] }).is_ok());
    }

    #[test]
    fn test_theme_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl.execute_command(ShellCommand::Theme { subcommand: "list".to_string(), args: vec![] }).is_ok());
        assert_eq!(repl.execute_command(ShellCommand::Theme { subcommand: "set".to_string(), args: vec!["Banaras Gold".to_string()] }).unwrap(), "Theme changed to Banaras Gold.");
        assert_eq!(repl.active_theme, "Banaras Gold");
    }

    #[test]
    fn test_profile_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl.execute_command(ShellCommand::Profile { subcommand: "list".to_string(), args: vec![] }).is_ok());
        assert!(repl.execute_command(ShellCommand::Profile { subcommand: "switch".to_string(), args: vec!["developer".to_string()] }).is_ok());
        assert_eq!(repl.active_profile, "developer");
    }

    #[test]
    fn test_window_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl.execute_command(ShellCommand::Window { subcommand: "list".to_string(), args: vec![] }).is_ok());
        assert!(repl.execute_command(ShellCommand::Window { subcommand: "create".to_string(), args: vec!["SigmaBrowser".to_string(), "sigma.browser".to_string(), "10,10,600,400".to_string()] }).is_ok());
        assert_eq!(repl.windows.len(), 3);
    }

    #[test]
    fn test_accessibility_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl.execute_command(ShellCommand::Accessibility { subcommand: "status".to_string(), args: vec![] }).is_ok());
        assert!(repl.execute_command(ShellCommand::Accessibility { subcommand: "screen-reader".to_string(), args: vec!["true".to_string()] }).is_ok());
        assert!(repl.screen_reader_enabled);
    }

    #[test]
    fn test_clipboard_commands() {
        let mut repl = ShellRepl::new();
        assert!(repl.execute_command(ShellCommand::Clipboard { subcommand: "get".to_string(), args: vec![] }).is_ok());
        assert!(repl.execute_command(ShellCommand::Clipboard { subcommand: "set".to_string(), args: vec!["CopiedText".to_string()] }).is_ok());
        assert_eq!(repl.clipboard_content, "CopiedText");
    }
}
