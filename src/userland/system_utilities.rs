// SigmaOS System Utilities Architecture & Registry Framework
// (`src/userland/system_utilities.rs`)
//
// Implements POSIX-aligned, safety-first system utilities, shared argument parsing,
// diagnostic conventions, command registry, and foundational file inspection tools.

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
        let mut iter = args.into_iter().peekable();

        while let Some(arg_item) = iter.next() {
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
            ("cat", "Concatenate and print files", PrivilegeLevel::User, MutationBehavior::ReadOnly, BackendDependency::Vfs, "SentinelTeam", UtilityStatus::Integrated, "man/cat.1"),
            ("pwd", "Print working directory name", PrivilegeLevel::User, MutationBehavior::ReadOnly, BackendDependency::Vfs, "SentinelTeam", UtilityStatus::Integrated, "man/pwd.1"),
            ("ls", "List directory contents", PrivilegeLevel::User, MutationBehavior::ReadOnly, BackendDependency::Vfs, "SentinelTeam", UtilityStatus::Integrated, "man/ls.1"),
            ("stat", "Display file or filesystem status", PrivilegeLevel::User, MutationBehavior::ReadOnly, BackendDependency::Vfs, "SentinelTeam", UtilityStatus::Integrated, "man/stat.1"),
            ("mkdir", "Make directories", PrivilegeLevel::User, MutationBehavior::MutatingSafe, BackendDependency::Vfs, "SentinelTeam", UtilityStatus::Integrated, "man/mkdir.1"),
            ("touch", "Change file access and modification times", PrivilegeLevel::User, MutationBehavior::MutatingSafe, BackendDependency::Vfs, "SentinelTeam", UtilityStatus::Integrated, "man/touch.1"),
            ("cp", "Copy files and directories", PrivilegeLevel::User, MutationBehavior::MutatingSafe, BackendDependency::Vfs, "SentinelTeam", UtilityStatus::Proposed, "man/cp.1"),
            ("mv", "Move files", PrivilegeLevel::User, MutationBehavior::MutatingSafe, BackendDependency::Vfs, "SentinelTeam", UtilityStatus::Proposed, "man/mv.1"),
            ("rm", "Remove files or directories", PrivilegeLevel::User, MutationBehavior::MutatingDestructive, BackendDependency::Vfs, "SentinelTeam", UtilityStatus::Proposed, "man/rm.1"),
            ("ps", "Report process status", PrivilegeLevel::User, MutationBehavior::ReadOnly, BackendDependency::ProcessManager, "SentinelTeam", UtilityStatus::Proposed, "man/ps.1"),
            ("df", "Report file system disk space usage", PrivilegeLevel::User, MutationBehavior::ReadOnly, BackendDependency::Vfs, "SentinelTeam", UtilityStatus::Proposed, "man/df.1"),
            ("sigma-diag", "SigmaOS system status and diagnostics", PrivilegeLevel::Admin, MutationBehavior::ReadOnly, BackendDependency::SystemInfo, "SentinelTeam", UtilityStatus::Integrated, "man/sigma-diag.1"),
            ("sigma-periodic", "Scheduled maintenance task runner", PrivilegeLevel::Admin, MutationBehavior::ReadOnly, BackendDependency::SystemInfo, "SentinelTeam", UtilityStatus::Proposed, "man/sigma-periodic.1"),
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
            .filter(|c| c.status == UtilityStatus::Integrated || c.status == UtilityStatus::Supported)
            .count()
    }
}

impl Default for SystemUtilityRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. FOUNDATIONAL FILE INSPECTION UTILITY ENGINES
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
    pub fn run(files: &[(&str, Option<&str>)], parser: &SystemUtilityArgParser) -> SystemUtilityOutput {
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
            let json = format!("{{\"status\": {}, \"bytes_read\": {}}}", status, output.stdout.len());
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
            return SystemUtilityOutput::failure(EXIT_FAILURE, format!("stat: cannot stat '{}': No such file or directory\n", path));
        }

        let stdout = format!("  File: {}\n  Size: {}\tBlocks: {}\tIO Block: 4096\n", path, file_size, (file_size + 511) / 512);
        let json = format!("{{\"file\": \"{}\", \"size\": {}}}", path, file_size);
        SystemUtilityOutput::success(stdout).with_json(json)
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
        assert!(registry.total_registered_count() >= 12);
        assert!(registry.integrated_count() >= 6);

        let cat_cmd = registry.get_command("cat").unwrap();
        assert_eq!(cat_cmd.privilege, PrivilegeLevel::User);
        assert_eq!(cat_cmd.mutation, MutationBehavior::ReadOnly);
        assert_eq!(cat_cmd.backend, BackendDependency::Vfs);
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
    fn test_stat_engine() {
        let output_exist = StatEngine::run("/etc/hostname", true, 12);
        assert_eq!(output_exist.exit_status, EXIT_SUCCESS);
        assert!(output_exist.stdout.contains("Size: 12"));

        let output_missing = StatEngine::run("/missing.txt", false, 0);
        assert_eq!(output_missing.exit_status, EXIT_FAILURE);
        assert!(output_missing.stderr.contains("No such file or directory"));
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
}
