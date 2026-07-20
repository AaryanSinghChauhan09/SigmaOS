// BusyBox-Style: Multi-Call `sigma-sh` Command Parser
// Combining utilities into a single executable reduces binary overhead by up to 90%

#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysCommandType {
    Echo,
    WhoAmI,
    Pwd,
    Cat,
    Ls,
    Mkdir,
    Rm,
    Cp,
    Mv,
    Date,
    Uname,
    Unsupported,
}

pub struct MultiCallShell;

impl MultiCallShell {
    /// Maps a command invocation string directly to internal command execution blocks
    pub fn parse_multicall_invocation(executable_name: &str) -> SysCommandType {
        match executable_name {
            "echo" | "sigma-echo" => SysCommandType::Echo,
            "whoami" | "sigma-whoami" => SysCommandType::WhoAmI,
            "pwd" | "sigma-pwd" => SysCommandType::Pwd,
            "cat" | "sigma-cat" => SysCommandType::Cat,
            "ls" | "sigma-ls" => SysCommandType::Ls,
            "mkdir" | "sigma-mkdir" => SysCommandType::Mkdir,
            "rm" | "sigma-rm" => SysCommandType::Rm,
            "cp" | "sigma-cp" => SysCommandType::Cp,
            "mv" | "sigma-mv" => SysCommandType::Mv,
            "date" | "sigma-date" => SysCommandType::Date,
            "uname" | "sigma-uname" => SysCommandType::Uname,
            _ => SysCommandType::Unsupported,
        }
    }

    /// Execute a command with arguments
    pub fn execute_command(command: SysCommandType, args: &[&str]) -> Result<String, &'static str> {
        match command {
            SysCommandType::Echo => {
                let output = args.join(" ");
                Ok(output)
            }
            SysCommandType::WhoAmI => Ok("sigma".to_string()),
            SysCommandType::Pwd => Ok("/home/sigma".to_string()),
            SysCommandType::Date => Ok("2026-07-20".to_string()),
            SysCommandType::Uname => Ok("SigmaOS 1.0.0".to_string()),
            _ => Err("Command not implemented"),
        }
    }

    /// Get help for a specific command
    pub fn get_command_help(command: SysCommandType) -> &'static str {
        match command {
            SysCommandType::Echo => "Usage: echo [TEXT]\nPrint text to stdout",
            SysCommandType::WhoAmI => "Usage: whoami\nPrint current username",
            SysCommandType::Pwd => "Usage: pwd\nPrint current working directory",
            SysCommandType::Cat => "Usage: cat [FILE]\nConcatenate and print files",
            SysCommandType::Ls => "Usage: ls [DIRECTORY]\nList directory contents",
            SysCommandType::Mkdir => "Usage: mkdir [DIRECTORY]\nCreate directories",
            SysCommandType::Rm => "Usage: rm [FILE]\nRemove files",
            SysCommandType::Cp => "Usage: cp [SOURCE] [DEST]\nCopy files",
            SysCommandType::Mv => "Usage: mv [SOURCE] [DEST]\nMove files",
            SysCommandType::Date => "Usage: date\nPrint system date",
            SysCommandType::Uname => "Usage: uname\nPrint system information",
            SysCommandType::Unsupported => "Unknown command",
        }
    }

    /// List all supported commands
    pub fn list_supported_commands() -> &'static [&'static str] {
        &["echo", "whoami", "pwd", "cat", "ls", "mkdir", "rm", "cp", "mv", "date", "uname"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_busybox_style_multicall() {
        // Simulates invoking utilities via system symlinks
        assert_eq!(MultiCallShell::parse_multicall_invocation("echo"), SysCommandType::Echo);
        assert_eq!(MultiCallShell::parse_multicall_invocation("sigma-whoami"), SysCommandType::WhoAmI);
        assert_eq!(MultiCallShell::parse_multicall_invocation("pwd"), SysCommandType::Pwd);
        assert_eq!(MultiCallShell::parse_multicall_invocation("ls"), SysCommandType::Unsupported);
    }

    #[test]
    fn test_command_execution() {
        let result = MultiCallShell::execute_command(SysCommandType::Echo, &["Hello", "World"]);
        assert_eq!(result.unwrap(), "Hello World");

        let result = MultiCallShell::execute_command(SysCommandType::WhoAmI, &[]);
        assert_eq!(result.unwrap(), "sigma");
    }

    #[test]
    fn test_command_help() {
        let help = MultiCallShell::get_command_help(SysCommandType::Echo);
        assert!(help.contains("echo"));
        assert!(help.contains("Print text"));
    }

    #[test]
    fn test_supported_commands_list() {
        let commands = MultiCallShell::list_supported_commands();
        assert!(commands.contains(&"echo"));
        assert!(commands.contains(&"ls"));
        assert!(commands.contains(&"pwd"));
    }
}
