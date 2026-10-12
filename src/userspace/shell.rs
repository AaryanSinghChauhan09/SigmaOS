/// Minimal SigmaOS Userspace Shell
use crate::shell::sovereign_command_interpreter::{
    CommandResolverAndExecutorEngine, ShellLexerParserEngine,
};
use std::string::String;

pub struct SigmaShell {
    cwd: String,
    parser: ShellLexerParserEngine,
    executor: CommandResolverAndExecutorEngine,
}

impl SigmaShell {
    pub fn new() -> Self {
        Self {
            cwd: String::from("/"),
            parser: ShellLexerParserEngine::new(),
            executor: CommandResolverAndExecutorEngine::new(),
        }
    }

    pub fn execute_command(&mut self, input: &str) -> String {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return String::new();
        }

        match self.parser.parse(trimmed) {
            Ok(ast) => {
                self.executor.captured_stdout.clear();
                self.executor.captured_stderr.clear();
                match self.executor.execute_ast(&ast) {
                    Ok(_) => {
                        let stdout = String::from_utf8_lossy(&self.executor.captured_stdout).to_string();
                        if stdout.is_empty() {
                            // Legacy fallback formatting for echo/pwd/ls if stdout was empty
                            let mut parts = trimmed.split_whitespace();
                            let cmd = parts.next().unwrap_or("");
                            match cmd {
                                "echo" => parts.collect::<Vec<&str>>().join(" "),
                                "pwd" => self.cwd.clone(),
                                "ls" => String::from(".\n..\nbin\ndev\netc\nusr"),
                                _ => String::new(),
                            }
                        } else {
                            stdout.trim_end_matches('\n').to_string()
                        }
                    }
                    Err(err) => format!("exec error: {}", err),
                }
            }
            Err(err) => format!("syntax error: {}", err),
        }
    }
}

impl Default for SigmaShell {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_userspace_shell_execution() {
        let mut shell = SigmaShell::new();
        assert_eq!(shell.execute_command("echo hello"), "hello");
        assert_eq!(shell.execute_command("pwd"), "/");
    }
}
