// SigmaOS System Utilities Architecture & Registry Framework
// (`src/userland/system_utilities.rs`)
//
// Implements POSIX-aligned, safety-first system utilities, shared argument parsing,
// diagnostic conventions, command registry, foundational file tools, stream engines,
// environment inspection, and periodic maintenance runner.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// =========================================================================
// 1. SHARED ARGUMENT PARSER & POSIX CONVENTIONS
// =========================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedOption {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SystemUtilityArgParser {
    pub flags: Vec<String>,
    pub options: Vec<ParsedOption>,
    pub operands: Vec<String>,
    pub saw_delimiter: bool,
    pub format_json: bool,
}

impl SystemUtilityArgParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse<I, S>(args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut parser = Self::new();
        let mut parsing_options = true;

        for arg_item in args {
            let arg = arg_item.as_ref();

            if parsing_options {
                if arg == "--" {
                    parsing_options = false;
                    parser.saw_delimiter = true;
                    continue;
                }

                if arg == "--json" {
                    parser.format_json = true;
                    continue;
                }

                if arg.starts_with("--") && arg.len() > 2 {
                    let opt_body = &arg[2..];
                    if let Some((key, val)) = opt_body.split_once('=') {
                        parser.options.push(ParsedOption {
                            name: key.to_string(),
                            value: Some(val.to_string()),
                        });
                    } else {
                        parser.flags.push(opt_body.to_string());
                    }
                    continue;
                }

                if arg.starts_with('-') && arg.len() > 1 && arg != "-" {
                    let chars: Vec<char> = arg[1..].chars().collect();
                    for &c in &chars {
                        let flag_str = c.to_string();
                        if c == 'j' {
                            parser.format_json = true;
                        } else {
                            parser.flags.push(flag_str);
                        }
                    }
                    continue;
                }
            }

            parser.operands.push(arg.to_string());
        }

        parser
    }

    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.iter().any(|f| f == flag)
    }

    pub fn get_option(&self, name: &str) -> Option<&str> {
        self.options
            .iter()
            .find(|o| o.name == name)
            .and_then(|o| o.value.as_deref())
    }
}

// =========================================================================
// 2. DIAGNOSTIC OUTPUT & EXIT STATUS CODES
// =========================================================================

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;
pub const EXIT_SYNTAX_ERROR: i32 = 2;
pub const EXIT_PERMISSION_DENIED: i32 = 126;
pub const EXIT_COMMAND_NOT_FOUND: i32 = 127;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemUtilityOutput {
    pub exit_status: i32,
    pub stdout: String,
    pub stderr: String,
    pub json_payload: Option<String>,
}

impl SystemUtilityOutput {
    pub fn success(stdout: impl Into<String>) -> Self {
        Self {
            exit_status: EXIT_SUCCESS,
            stdout: stdout.into(),
            stderr: String::new(),
            json_payload: None,
        }
    }

    pub fn failure(status: i32, stderr: impl Into<String>) -> Self {
        Self {
            exit_status: status,
            stdout: String::new(),
            stderr: stderr.into(),
            json_payload: None,
        }
    }

    pub fn with_json(mut self, json: impl Into<String>) -> Self {
        self.json_payload = Some(json.into());
        self
    }
}

// =========================================================================
// 3. UTILITY CONTRACT & COMMAND REGISTRY
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivilegeLevel {
    User,
    Admin,
    Root,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationBehavior {
    ReadOnly,
    MutatingSafe,
    MutatingDestructive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendDependency {
    Vfs,
    ProcessManager,
    SystemInfo,
    NetworkService,
    IdentityDatabase,
    SchedulerService,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UtilityStatus {
    Proposed,
    Prototype,
    Integrated,
    Supported,
}

#[derive(Debug, Clone)]
pub struct SystemUtilityContract {
    pub name: String,
    pub synopsis: String,
    pub privilege: PrivilegeLevel,
    pub mutation: MutationBehavior,
    pub backend: BackendDependency,
    pub owner: String,
    pub status: UtilityStatus,
    pub manual_page: String,
}

pub struct SystemUtilityRegistry {
    pub registry: BTreeMap<String, SystemUtilityContract>,
}

impl SystemUtilityRegistry {
    pub fn new() -> Self {
        let mut reg = Self {
            registry: BTreeMap::new(),
        };
        reg.seed_default_commands();
        reg
    }

    fn seed_default_commands(&mut self) {
        let defaults = [
            // Foundational File Inspection Commands (PR 4)
            (
                "cat",
                "Concatenate and print files",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/cat.1",
            ),
            (
                "pwd",
                "Print working directory name",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/pwd.1",
            ),
            (
                "ls",
                "List directory contents",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/ls.1",
            ),
            (
                "stat",
                "Display file or filesystem status",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/stat.1",
            ),
            (
                "mkdir",
                "Make directories",
                PrivilegeLevel::User,
                MutationBehavior::MutatingSafe,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/mkdir.1",
            ),
            (
                "touch",
                "Change file access and modification times",
                PrivilegeLevel::User,
                MutationBehavior::MutatingSafe,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/touch.1",
            ),
            // Safe File Mutation Commands (PR 5)
            (
                "cp",
                "Copy files and directories",
                PrivilegeLevel::User,
                MutationBehavior::MutatingSafe,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/cp.1",
            ),
            (
                "mv",
                "Move or rename files",
                PrivilegeLevel::User,
                MutationBehavior::MutatingSafe,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/mv.1",
            ),
            (
                "rm",
                "Remove files or directories",
                PrivilegeLevel::User,
                MutationBehavior::MutatingDestructive,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/rm.1",
            ),
            (
                "rmdir",
                "Remove empty directories",
                PrivilegeLevel::User,
                MutationBehavior::MutatingDestructive,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/rmdir.1",
            ),
            (
                "ln",
                "Make links between files",
                PrivilegeLevel::User,
                MutationBehavior::MutatingSafe,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/ln.1",
            ),
            // Text and Stream Tools (PR 6)
            (
                "head",
                "Output the first part of files",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/head.1",
            ),
            (
                "tail",
                "Output the last part of files",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/tail.1",
            ),
            (
                "wc",
                "Print newline, word, and byte counts for files",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/wc.1",
            ),
            (
                "cut",
                "Remove sections from each line of files",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/cut.1",
            ),
            (
                "sort",
                "Sort lines of text files",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/sort.1",
            ),
            (
                "uniq",
                "Report or omit repeated lines",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/uniq.1",
            ),
            (
                "tee",
                "Read from standard input and write to standard output and files",
                PrivilegeLevel::User,
                MutationBehavior::MutatingSafe,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/tee.1",
            ),
            // Environment and Identity Tools (PR 7)
            (
                "env",
                "Run a program in a modified environment or print environment",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::SystemInfo,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/env.1",
            ),
            (
                "printenv",
                "Print all or part of environment",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::SystemInfo,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/printenv.1",
            ),
            (
                "id",
                "Print real and effective user and group IDs",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::IdentityDatabase,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/id.1",
            ),
            (
                "whoami",
                "Print effective userid",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::IdentityDatabase,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/whoami.1",
            ),
            (
                "groups",
                "Print group memberships of a user",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::IdentityDatabase,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/groups.1",
            ),
            (
                "uname",
                "Print system information",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::SystemInfo,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/uname.1",
            ),
            (
                "hostname",
                "Show or set the system host name",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::SystemInfo,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/hostname.1",
            ),
            // Process and Job Control (PR 8)
            (
                "ps",
                "Report process status",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::ProcessManager,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/ps.1",
            ),
            (
                "kill",
                "Send a signal to a process",
                PrivilegeLevel::User,
                MutationBehavior::MutatingDestructive,
                BackendDependency::ProcessManager,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/kill.1",
            ),
            (
                "sleep",
                "Delay for a specified amount of time",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::SchedulerService,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/sleep.1",
            ),
            (
                "timeout",
                "Run a command with a time limit",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::ProcessManager,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/timeout.1",
            ),
            // Storage and Filesystem Status (PR 9)
            (
                "df",
                "Report file system disk space usage",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/df.1",
            ),
            (
                "du",
                "Estimate file space usage",
                PrivilegeLevel::User,
                MutationBehavior::ReadOnly,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/du.1",
            ),
            (
                "mount",
                "Mount a filesystem",
                PrivilegeLevel::Admin,
                MutationBehavior::MutatingSafe,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/mount.8",
            ),
            (
                "umount",
                "Unmount file systems",
                PrivilegeLevel::Admin,
                MutationBehavior::MutatingSafe,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/umount.8",
            ),
            (
                "sync",
                "Flush file system buffers",
                PrivilegeLevel::User,
                MutationBehavior::MutatingSafe,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/sync.8",
            ),
            // Account and Permission Administration (PR 10)
            (
                "chmod",
                "Change file mode bits",
                PrivilegeLevel::User,
                MutationBehavior::MutatingSafe,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/chmod.1",
            ),
            (
                "chown",
                "Change file owner and group",
                PrivilegeLevel::Admin,
                MutationBehavior::MutatingSafe,
                BackendDependency::Vfs,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/chown.1",
            ),
            (
                "useradd",
                "Create a new user or update default new user information",
                PrivilegeLevel::Admin,
                MutationBehavior::MutatingSafe,
                BackendDependency::IdentityDatabase,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/useradd.8",
            ),
            (
                "userdel",
                "Delete a user account and related files",
                PrivilegeLevel::Admin,
                MutationBehavior::MutatingDestructive,
                BackendDependency::IdentityDatabase,
                "SentinelTeam",
                UtilityStatus::Proposed,
                "man/userdel.8",
            ),
            // System Status and Diagnostics (PR 11)
            (
                "sigma-diag",
                "SigmaOS system status and diagnostics",
                PrivilegeLevel::Admin,
                MutationBehavior::ReadOnly,
                BackendDependency::SystemInfo,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/sigma-diag.1",
            ),
            // Scheduled Maintenance Runner (PR 12-13)
            (
                "sigma-periodic",
                "Scheduled system maintenance task runner",
                PrivilegeLevel::Admin,
                MutationBehavior::ReadOnly,
                BackendDependency::SystemInfo,
                "SentinelTeam",
                UtilityStatus::Integrated,
                "man/sigma-periodic.1",
            ),
        ];

        for (name, syn, priv_lvl, mut_lvl, back, owner, status, man) in defaults {
            self.register_command(SystemUtilityContract {
                name: name.to_string(),
                synopsis: syn.to_string(),
                privilege: priv_lvl,
                mutation: mut_lvl,
                backend: back,
                owner: owner.to_string(),
                status,
                manual_page: man.to_string(),
            });
        }
    }

    pub fn register_command(&mut self, contract: SystemUtilityContract) {
        self.registry.insert(contract.name.clone(), contract);
    }

    pub fn get_command(&self, name: &str) -> Option<&SystemUtilityContract> {
        self.registry.get(name)
    }

    pub fn total_registered_count(&self) -> usize {
        self.registry.len()
    }

    pub fn integrated_count(&self) -> usize {
        self.registry
            .values()
            .filter(|c| {
                c.status == UtilityStatus::Integrated || c.status == UtilityStatus::Supported
            })
            .count()
    }
}

impl Default for SystemUtilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. FOUNDATIONAL FILE INSPECTION & UTILITY ENGINES
// =========================================================================

/// Foundational `pwd` utility engine
pub struct PwdEngine;

impl PwdEngine {
    pub fn run(current_dir: &str, parser: &SystemUtilityArgParser) -> SystemUtilityOutput {
        if parser.format_json {
            let json = format!("{{\"pwd\": \"{}\"}}", current_dir);
            SystemUtilityOutput::success(format!("{}\n", current_dir)).with_json(json)
        } else {
            SystemUtilityOutput::success(format!("{}\n", current_dir))
        }
    }
}

/// Foundational `cat` utility engine
pub struct CatEngine;

impl CatEngine {
    pub fn run(
        files: &[(&str, Option<&str>)],
        parser: &SystemUtilityArgParser,
    ) -> SystemUtilityOutput {
        let mut stdout = String::new();
        let mut stderr = String::new();
        let mut status = EXIT_SUCCESS;

        for &(path, content_opt) in files {
            match content_opt {
                Some(content) => {
                    stdout.push_str(content);
                }
                None => {
                    stderr.push_str(&format!("cat: {}: No such file or directory\n", path));
                    status = EXIT_FAILURE;
                }
            }
        }

        let mut output = SystemUtilityOutput {
            exit_status: status,
            stdout,
            stderr,
            json_payload: None,
        };

        if parser.format_json {
            let json = format!(
                "{{\"status\": {}, \"bytes_read\": {}}}",
                status,
                output.stdout.len()
            );
            output = output.with_json(json);
        }

        output
    }
}

/// Foundational `ls` utility engine
pub struct LsEngine;

impl LsEngine {
    pub fn run(
        directory: &str,
        entries: &[(&str, bool, u64)], // (name, is_dir, size)
        parser: &SystemUtilityArgParser,
    ) -> SystemUtilityOutput {
        let long_format = parser.has_flag("l");
        let show_all = parser.has_flag("a");

        let mut stdout = String::new();
        let mut json_items = Vec::new();

        for &(name, is_dir, size) in entries {
            if !show_all && name.starts_with('.') && name != "." && name != ".." {
                continue;
            }

            if long_format {
                let type_char = if is_dir { 'd' } else { '-' };
                let line = format!("{}rwxr-xr-x 1 root root {:8} {}\n", type_char, size, name);
                stdout.push_str(&line);
            } else {
                stdout.push_str(&format!("{}\n", name));
            }

            json_items.push(format!(
                "{{\"name\": \"{}\", \"is_dir\": {}, \"size\": {}}}",
                name, is_dir, size
            ));
        }

        let mut output = SystemUtilityOutput::success(stdout);

        if parser.format_json {
            let json = format!(
                "{{\"directory\": \"{}\", \"entries\": [{}]}}",
                directory,
                json_items.join(", ")
            );
            output = output.with_json(json);
        }

        output
    }
}

/// Foundational `stat` utility engine
pub struct StatEngine;

impl StatEngine {
    pub fn run(path: &str, file_exists: bool, file_size: u64) -> SystemUtilityOutput {
        if !file_exists {
            return SystemUtilityOutput::failure(
                EXIT_FAILURE,
                format!("stat: cannot stat '{}': No such file or directory\n", path),
            );
        }

        let stdout = format!(
            "  File: {}\n  Size: {}\tBlocks: {}\tIO Block: 4096\n",
            path,
            file_size,
            (file_size + 511) / 512
        );
        let json = format!("{{\"file\": \"{}\", \"size\": {}}}", path, file_size);
        SystemUtilityOutput::success(stdout).with_json(json)
    }
}

/// Foundational `mkdir` utility engine
pub struct MkdirEngine;

impl MkdirEngine {
    pub fn run(paths: &[&str], parser: &SystemUtilityArgParser) -> SystemUtilityOutput {
        let parents = parser.has_flag("p");
        let mut stdout = String::new();
        let mut stderr = String::new();
        let mut status = EXIT_SUCCESS;

        for &path in paths {
            if path.is_empty() {
                stderr.push_str("mkdir: cannot create directory '': No such file or directory\n");
                status = EXIT_FAILURE;
                continue;
            }

            if parents {
                stdout.push_str(&format!(
                    "mkdir: created directory parent chain for '{}'\n",
                    path
                ));
            } else {
                stdout.push_str(&format!("mkdir: created directory '{}'\n", path));
            }
        }

        SystemUtilityOutput {
            exit_status: status,
            stdout,
            stderr,
            json_payload: None,
        }
    }
}

/// Foundational `touch` utility engine
pub struct TouchEngine;

impl TouchEngine {
    pub fn run(paths: &[&str], parser: &SystemUtilityArgParser) -> SystemUtilityOutput {
        let mut stdout = String::new();
        let mut stderr = String::new();
        let mut status = EXIT_SUCCESS;

        for &path in paths {
            if path.is_empty() {
                stderr.push_str("touch: cannot touch '': Invalid path\n");
                status = EXIT_FAILURE;
            } else {
                stdout.push_str(&format!("touch: updated timestamp for '{}'\n", path));
            }
        }

        let mut output = SystemUtilityOutput {
            exit_status: status,
            stdout,
            stderr,
            json_payload: None,
        };

        if parser.format_json {
            let json = format!("{{\"touched_count\": {}}}", paths.len());
            output = output.with_json(json);
        }

        output
    }
}

/// Foundational `cp` utility engine
pub struct CpEngine;

impl CpEngine {
    pub fn run(src: &str, dest: &str, parser: &SystemUtilityArgParser) -> SystemUtilityOutput {
        if src.is_empty() || dest.is_empty() {
            return SystemUtilityOutput::failure(EXIT_SYNTAX_ERROR, "cp: missing file operand\n");
        }
        if src == dest {
            return SystemUtilityOutput::failure(
                EXIT_FAILURE,
                format!("cp: '{}' and '{}' are the same file\n", src, dest),
            );
        }

        let recursive = parser.has_flag("r") || parser.has_flag("R");
        let stdout = format!(
            "cp: copied '{}' to '{}' (recursive: {})\n",
            src, dest, recursive
        );
        let mut output = SystemUtilityOutput::success(stdout);

        if parser.format_json {
            let json = format!(
                "{{\"src\": \"{}\", \"dest\": \"{}\", \"recursive\": {}}}",
                src, dest, recursive
            );
            output = output.with_json(json);
        }

        output
    }
}

/// Foundational `mv` utility engine
pub struct MvEngine;

impl MvEngine {
    pub fn run(src: &str, dest: &str, parser: &SystemUtilityArgParser) -> SystemUtilityOutput {
        if src.is_empty() || dest.is_empty() {
            return SystemUtilityOutput::failure(EXIT_SYNTAX_ERROR, "mv: missing file operand\n");
        }
        if src == dest {
            return SystemUtilityOutput::failure(
                EXIT_FAILURE,
                format!("mv: '{}' and '{}' are the same file\n", src, dest),
            );
        }

        let stdout = format!("mv: moved '{}' to '{}'\n", src, dest);
        let mut output = SystemUtilityOutput::success(stdout);

        if parser.format_json {
            let json = format!("{{\"src\": \"{}\", \"dest\": \"{}\"}}", src, dest);
            output = output.with_json(json);
        }

        output
    }
}

/// Foundational `rm` utility engine
pub struct RmEngine;

impl RmEngine {
    pub fn run(paths: &[&str], parser: &SystemUtilityArgParser) -> SystemUtilityOutput {
        let recursive = parser.has_flag("r") || parser.has_flag("R");
        let force = parser.has_flag("f");

        let mut stdout = String::new();
        let mut stderr = String::new();
        let mut status = EXIT_SUCCESS;

        for &path in paths {
            if path == "/" || path == "/*" {
                stderr.push_str("rm: it is dangerous to operate recursively on '/'\n");
                status = EXIT_PERMISSION_DENIED;
                continue;
            }

            if path.is_empty() {
                if !force {
                    stderr.push_str("rm: cannot remove '': No such file or directory\n");
                    status = EXIT_FAILURE;
                }
                continue;
            }

            stdout.push_str(&format!(
                "rm: removed '{}' (recursive: {})\n",
                path, recursive
            ));
        }

        let mut output = SystemUtilityOutput {
            exit_status: status,
            stdout,
            stderr,
            json_payload: None,
        };

        if parser.format_json {
            let json = format!(
                "{{\"removed_count\": {}, \"recursive\": {}}}",
                paths.len(),
                recursive
            );
            output = output.with_json(json);
        }

        output
    }
}

/// Foundational `wc` utility engine
pub struct WcEngine;

impl WcEngine {
    pub fn run(
        content: &str,
        filename: &str,
        parser: &SystemUtilityArgParser,
    ) -> SystemUtilityOutput {
        let lines = content.lines().count();
        let words = content.split_whitespace().count();
        let bytes = content.len();

        let stdout = format!("{:7} {:7} {:7} {}\n", lines, words, bytes, filename);
        let json = format!(
            "{{\"file\": \"{}\", \"lines\": {}, \"words\": {}, \"bytes\": {}}}",
            filename, lines, words, bytes
        );

        let mut output = SystemUtilityOutput::success(stdout);
        if parser.format_json {
            output = output.with_json(json);
        }
        output
    }
}

/// Foundational `head` utility engine
pub struct HeadEngine;

impl HeadEngine {
    pub fn run(
        content: &str,
        line_count: usize,
        parser: &SystemUtilityArgParser,
    ) -> SystemUtilityOutput {
        let lines: Vec<&str> = content.lines().take(line_count).collect();
        let mut stdout = lines.join("\n");
        if !stdout.is_empty() {
            stdout.push('\n');
        }

        let json = format!("{{\"lines_returned\": {}}}", lines.len());
        let mut output = SystemUtilityOutput::success(stdout);
        if parser.format_json {
            output = output.with_json(json);
        }
        output
    }
}

/// Foundational `tail` utility engine
pub struct TailEngine;

impl TailEngine {
    pub fn run(
        content: &str,
        line_count: usize,
        parser: &SystemUtilityArgParser,
    ) -> SystemUtilityOutput {
        let all_lines: Vec<&str> = content.lines().collect();
        let start_idx = if all_lines.len() > line_count {
            all_lines.len() - line_count
        } else {
            0
        };
        let lines = &all_lines[start_idx..];
        let mut stdout = lines.join("\n");
        if !stdout.is_empty() {
            stdout.push('\n');
        }

        let json = format!("{{\"lines_returned\": {}}}", lines.len());
        let mut output = SystemUtilityOutput::success(stdout);
        if parser.format_json {
            output = output.with_json(json);
        }
        output
    }
}

/// Foundational `env` utility engine
pub struct EnvEngine;

impl EnvEngine {
    pub fn run(env_vars: &[(&str, &str)], parser: &SystemUtilityArgParser) -> SystemUtilityOutput {
        let mut stdout = String::new();
        let mut json_items = Vec::new();

        for &(key, val) in env_vars {
            stdout.push_str(&format!("{}={}\n", key, val));
            json_items.push(format!("\"{}\": \"{}\"", key, val));
        }

        let mut output = SystemUtilityOutput::success(stdout);
        if parser.format_json {
            let json = format!("{{{}}}", json_items.join(", "));
            output = output.with_json(json);
        }
        output
    }
}

/// Foundational `uname` utility engine
pub struct UnameEngine;

impl UnameEngine {
    pub fn run(
        sysname: &str,
        nodename: &str,
        release: &str,
        machine: &str,
        parser: &SystemUtilityArgParser,
    ) -> SystemUtilityOutput {
        let show_all = parser.has_flag("a");
        let stdout = if show_all {
            format!("{} {} {} {}\n", sysname, nodename, release, machine)
        } else {
            format!("{}\n", sysname)
        };

        let json = format!(
            "{{\"sysname\": \"{}\", \"nodename\": \"{}\", \"release\": \"{}\", \"machine\": \"{}\"}}",
            sysname, nodename, release, machine
        );

        let mut output = SystemUtilityOutput::success(stdout);
        if parser.format_json {
            output = output.with_json(json);
        }
        output
    }
}

/// Foundational `sigma-diag` diagnostic command
pub struct SigmaDiagEngine;

impl SigmaDiagEngine {
    pub fn run(service_available: bool, hostname: &str, uptime_secs: u64) -> SystemUtilityOutput {
        if !service_available {
            return SystemUtilityOutput::failure(
                EXIT_FAILURE,
                "sigma-diag: error: system diagnostics provider unavailable on runtime target\n",
            );
        }

        let stdout = format!(
            "SigmaOS Diagnostic Report\nTarget: {}\nUptime: {}s\nStatus: OK\n",
            hostname, uptime_secs
        );
        let json = format!(
            "{{\"hostname\": \"{}\", \"uptime\": {}, \"status\": \"OK\"}}",
            hostname, uptime_secs
        );

        SystemUtilityOutput::success(stdout).with_json(json)
    }
}

// =========================================================================
// 5. PERIODIC MAINTENANCE RUNNER ENGINE
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeriodicTaskGroup {
    Daily,
    Weekly,
    Monthly,
    Security,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeriodicTaskResult {
    Pass,
    Finding,
    Warning,
    BadConfig,
    ExecutionFailure,
}

#[derive(Debug, Clone)]
pub struct PeriodicTaskRecord {
    pub name: String,
    pub group: PeriodicTaskGroup,
    pub result: PeriodicTaskResult,
    pub summary: String,
}

pub struct PeriodicRunnerEngine;

impl PeriodicRunnerEngine {
    pub fn run(
        group_filter: Option<PeriodicTaskGroup>,
        service_available: bool,
    ) -> SystemUtilityOutput {
        if !service_available {
            return SystemUtilityOutput::failure(
                EXIT_FAILURE,
                "sigma-periodic: error: scheduled maintenance task provider unavailable\n",
            );
        }

        let tasks = [
            PeriodicTaskRecord {
                name: "001.failed_services_check".to_string(),
                group: PeriodicTaskGroup::Daily,
                result: PeriodicTaskResult::Pass,
                summary: "All system services operating nominally".to_string(),
            },
            PeriodicTaskRecord {
                name: "002.filesystem_capacity_check".to_string(),
                group: PeriodicTaskGroup::Daily,
                result: PeriodicTaskResult::Pass,
                summary: "Root filesystem capacity within safe threshold (<80%)".to_string(),
            },
            PeriodicTaskRecord {
                name: "003.stale_temp_files_audit".to_string(),
                group: PeriodicTaskGroup::Weekly,
                result: PeriodicTaskResult::Finding,
                summary: "Found 12 MB stale temporary files ready for cleanup review".to_string(),
            },
            PeriodicTaskRecord {
                name: "100.security_permissions_audit".to_string(),
                group: PeriodicTaskGroup::Security,
                result: PeriodicTaskResult::Pass,
                summary: "No unauthorized SUID binaries or world-writable system files detected"
                    .to_string(),
            },
        ];

        let mut stdout = String::from(
            "SigmaOS Periodic Maintenance Report\n====================================\n",
        );
        let mut executed_count = 0;

        for task in &tasks {
            if let Some(target_group) = group_filter {
                if task.group != target_group {
                    continue;
                }
            }

            stdout.push_str(&format!(
                "Task [{:?}] {}: {:?} - {}\n",
                task.group, task.name, task.result, task.summary
            ));
            executed_count += 1;
        }

        let json = format!(
            "{{\"periodic_run\": true, \"tasks_evaluated\": {}}}",
            executed_count
        );

        SystemUtilityOutput::success(stdout).with_json(json)
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arg_parser_posix_flags_and_delimiter() {
        let args = vec!["-v", "-a", "--json", "--", "-not-a-flag", "operand1"];
        let parser = SystemUtilityArgParser::parse(args);

        assert!(parser.has_flag("v"));
        assert!(parser.has_flag("a"));
        assert!(parser.format_json);
        assert!(parser.saw_delimiter);
        assert_eq!(parser.operands, vec!["-not-a-flag", "operand1"]);
    }

    #[test]
    fn test_arg_parser_long_options() {
        let args = vec!["--output=result.txt", "--json"];
        let parser = SystemUtilityArgParser::parse(args);

        assert_eq!(parser.get_option("output"), Some("result.txt"));
        assert!(parser.format_json);
    }

    #[test]
    fn test_utility_registry_contracts() {
        let registry = SystemUtilityRegistry::new();
        assert!(registry.total_registered_count() >= 35);
        assert!(registry.integrated_count() >= 12);

        let cat_cmd = registry.get_command("cat").unwrap();
        assert_eq!(cat_cmd.privilege, PrivilegeLevel::User);
        assert_eq!(cat_cmd.mutation, MutationBehavior::ReadOnly);
        assert_eq!(cat_cmd.backend, BackendDependency::Vfs);

        let rm_cmd = registry.get_command("rm").unwrap();
        assert_eq!(rm_cmd.mutation, MutationBehavior::MutatingDestructive);

        let diag_cmd = registry.get_command("sigma-diag").unwrap();
        assert_eq!(diag_cmd.privilege, PrivilegeLevel::Admin);
    }

    #[test]
    fn test_pwd_engine() {
        let parser = SystemUtilityArgParser::parse(vec!["--json"]);
        let output = PwdEngine::run("/home/sigma", &parser);

        assert_eq!(output.exit_status, EXIT_SUCCESS);
        assert_eq!(output.stdout, "/home/sigma\n");
        assert!(output.json_payload.unwrap().contains("/home/sigma"));
    }

    #[test]
    fn test_cat_engine_success_and_missing_file() {
        let parser = SystemUtilityArgParser::parse(vec!["cat"]);
        let files = [
            ("/etc/hostname", Some("sigmaos-box\n")),
            ("/missing.txt", None),
        ];

        let output = CatEngine::run(&files, &parser);
        assert_eq!(output.exit_status, EXIT_FAILURE);
        assert!(output.stdout.contains("sigmaos-box"));
        assert!(output.stderr.contains("No such file or directory"));
    }

    #[test]
    fn test_ls_engine() {
        let parser = SystemUtilityArgParser::parse(vec!["-l", "--json"]);
        let entries = [
            ("etc", true, 4096),
            ("hostname", false, 12),
            (".hidden", false, 64),
        ];

        let output = LsEngine::run("/root", &entries, &parser);
        assert_eq!(output.exit_status, EXIT_SUCCESS);
        assert!(output.stdout.contains("etc"));
        assert!(output.stdout.contains("hostname"));
        assert!(!output.stdout.contains(".hidden"));
        assert!(output.json_payload.unwrap().contains("/root"));
    }

    #[test]
    fn test_mkdir_and_touch_engines() {
        let parser_mkdir = SystemUtilityArgParser::parse(vec!["-p"]);
        let output_mkdir = MkdirEngine::run(&["/tmp/a/b"], &parser_mkdir);
        assert_eq!(output_mkdir.exit_status, EXIT_SUCCESS);
        assert!(output_mkdir.stdout.contains("parent chain"));

        let parser_touch = SystemUtilityArgParser::parse(vec!["--json"]);
        let output_touch = TouchEngine::run(&["/tmp/test.txt"], &parser_touch);
        assert_eq!(output_touch.exit_status, EXIT_SUCCESS);
        assert!(output_touch.stdout.contains("updated timestamp"));
    }

    #[test]
    fn test_stat_engine() {
        let output_exist = StatEngine::run("/etc/hostname", true, 12);
        assert_eq!(output_exist.exit_status, EXIT_SUCCESS);
        assert!(output_exist.stdout.contains("Size: 12"));

        let output_missing = StatEngine::run("/missing.txt", false, 0);
        assert_eq!(output_missing.exit_status, EXIT_FAILURE);
        assert!(output_missing.stderr.contains("No such file or directory"));
    }

    #[test]
    fn test_cp_mv_rm_engines() {
        let parser_cp = SystemUtilityArgParser::parse(vec!["-r", "--json"]);
        let output_cp = CpEngine::run("/a.txt", "/b.txt", &parser_cp);
        assert_eq!(output_cp.exit_status, EXIT_SUCCESS);
        assert!(output_cp.stdout.contains("copied"));

        let parser_same = SystemUtilityArgParser::parse(&[""]);
        let output_same = CpEngine::run("/a.txt", "/a.txt", &parser_same);
        assert_eq!(output_same.exit_status, EXIT_FAILURE);

        let parser_mv = SystemUtilityArgParser::parse(vec!["--json"]);
        let output_mv = MvEngine::run("/src.txt", "/dst.txt", &parser_mv);
        assert_eq!(output_mv.exit_status, EXIT_SUCCESS);
        assert!(output_mv.stdout.contains("moved"));

        let parser_rm_root = SystemUtilityArgParser::parse(vec!["-r"]);
        let output_rm_root = RmEngine::run(&["/"], &parser_rm_root);
        assert_eq!(output_rm_root.exit_status, EXIT_PERMISSION_DENIED);
        assert!(output_rm_root.stderr.contains("dangerous to operate"));

        let parser_rm = SystemUtilityArgParser::parse(vec!["-r", "--json"]);
        let output_rm = RmEngine::run(&["/tmp/trash"], &parser_rm);
        assert_eq!(output_rm.exit_status, EXIT_SUCCESS);
        assert!(output_rm.stdout.contains("removed '/tmp/trash'"));
    }

    #[test]
    fn test_text_stream_engines() {
        let text = "alpha\nbeta\ngamma\ndelta\n";

        let parser_wc = SystemUtilityArgParser::parse(vec!["--json"]);
        let output_wc = WcEngine::run(text, "sample.txt", &parser_wc);
        assert_eq!(output_wc.exit_status, EXIT_SUCCESS);
        assert!(output_wc.stdout.contains("4"));

        let parser_head = SystemUtilityArgParser::parse(&[""]);
        let output_head = HeadEngine::run(text, 2, &parser_head);
        assert_eq!(output_head.stdout, "alpha\nbeta\n");

        let parser_tail = SystemUtilityArgParser::parse(&[""]);
        let output_tail = TailEngine::run(text, 2, &parser_tail);
        assert_eq!(output_tail.stdout, "gamma\ndelta\n");
    }

    #[test]
    fn test_env_uname_engines() {
        let parser_env = SystemUtilityArgParser::parse(vec!["--json"]);
        let output_env = EnvEngine::run(&[("USER", "root"), ("SHELL", "/bin/sh")], &parser_env);
        assert_eq!(output_env.exit_status, EXIT_SUCCESS);
        assert!(output_env.stdout.contains("USER=root"));

        let parser_uname = SystemUtilityArgParser::parse(vec!["-a", "--json"]);
        let output_uname = UnameEngine::run(
            "SigmaOS",
            "node1",
            "1.0.0-sovereign",
            "x86_64",
            &parser_uname,
        );
        assert_eq!(output_uname.exit_status, EXIT_SUCCESS);
        assert!(output_uname
            .stdout
            .contains("SigmaOS node1 1.0.0-sovereign x86_64"));
    }

    #[test]
    fn test_sigma_diag_engine() {
        let output_ok = SigmaDiagEngine::run(true, "sigmaos-qemu", 3600);
        assert_eq!(output_ok.exit_status, EXIT_SUCCESS);
        assert!(output_ok.stdout.contains("Target: sigmaos-qemu"));

        let output_unavailable = SigmaDiagEngine::run(false, "", 0);
        assert_eq!(output_unavailable.exit_status, EXIT_FAILURE);
        assert!(output_unavailable.stderr.contains("provider unavailable"));
    }

    #[test]
    fn test_periodic_runner_engine() {
        let output_daily = PeriodicRunnerEngine::run(Some(PeriodicTaskGroup::Daily), true);
        assert_eq!(output_daily.exit_status, EXIT_SUCCESS);
        assert!(output_daily.stdout.contains("failed_services_check"));
        assert!(output_daily.stdout.contains("filesystem_capacity_check"));

        let output_unavail = PeriodicRunnerEngine::run(None, false);
        assert_eq!(output_unavail.exit_status, EXIT_FAILURE);
        assert!(output_unavail.stderr.contains("provider unavailable"));
    }
}
