//! Shell/Command Interpreter (bash/zsh Inspiration)
//! Advanced shell with history, completion, aliases, and job control

#![no_std]

extern crate alloc;

use crate::klib::{Vec, String};

/// Command
#[derive(Debug, Clone)]
pub struct Command {
    pub argv: Vec<String>,
    pub stdin: String,
    pub stdout: String,
    pub stderr: String,
    pub background: bool,
}

impl Command {
    pub fn new(argv: Vec<String>) -> Self {
        Self {
            argv,
            stdin: String::new(),
            stdout: String::new(),
            stderr: String::new(),
            background: false,
        }
    }

    pub fn set_background(&mut self, background: bool) {
        self.background = background;
    }
}

/// Pipeline
#[derive(Debug, Clone)]
pub struct Pipeline {
    pub commands: Vec<Command>,
}

impl Pipeline {
    pub fn new(commands: Vec<Command>) -> Self {
        Self { commands }
    }
}

/// Alias
#[derive(Debug, Clone)]
pub struct Alias {
    pub name: String,
    pub value: String,
}

impl Alias {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
        }
    }
}

/// Environment variable
#[derive(Debug, Clone)]
pub struct EnvironmentVariable {
    pub name: String,
    pub value: String,
    pub exported: bool,
}

impl EnvironmentVariable {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            exported: false,
        }
    }

    pub fn export(&mut self) {
        self.exported = true;
    }
}

/// Environment
#[derive(Debug, Clone)]
pub struct Environment {
    pub variables: Vec<EnvironmentVariable>,
    pub path: Vec<String>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
            path: vec![
                "/bin".to_string(),
                "/usr/bin".to_string(),
                "/usr/local/bin".to_string(),
            ],
        }
    }

    pub fn set_variable(&mut self, name: &str, value: &str) {
        if let Some(var) = self.variables.iter_mut().find(|v| v.name == name) {
            var.value = value.to_string();
        } else {
            self.variables.push(EnvironmentVariable::new(name, value));
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.iter().find(|v| v.name == name).map(|v| &v.value)
    }

    pub fn export_variable(&mut self, name: &str) {
        if let Some(var) = self.variables.iter_mut().find(|v| v.name == name) {
            var.export();
        }
    }
}

/// Job
#[derive(Debug, Clone)]
pub struct Job {
    pub id: u32,
    pub command: String,
    pub pid: u32,
    pub state: JobState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Running,
    Stopped,
    Done,
}

impl Job {
    pub fn new(id: u32, command: &str, pid: u32) -> Self {
        Self {
            id,
            command: command.to_string(),
            pid,
            state: JobState::Running,
        }
    }

    pub fn stop(&mut self) {
        self.state = JobState::Stopped;
    }

    pub fn continue_job(&mut self) {
        self.state = JobState::Running;
    }
}

/// SigmaShell
pub struct SigmaShell {
    pub prompt: String,
    pub history: Vec<String>,
    pub aliases: Vec<Alias>,
    pub environment: Environment,
    pub jobs: Vec<Job>,
    pub current_directory: String,
}

impl SigmaShell {
    pub fn new() -> Self {
        Self {
            prompt: "\\u@\\h:\\w\\$ ".to_string(),
            history: Vec::new(),
            aliases: Vec::new(),
            environment: Environment::new(),
            jobs: Vec::new(),
            current_directory: "/home/user".to_string(),
        }
    }

    pub fn set_prompt(&mut self, prompt: &str) {
        self.prompt = prompt.to_string();
    }

    pub fn add_alias(&mut self, alias: Alias) {
        self.aliases.push(alias);
    }

    pub fn get_alias(&self, name: &str) -> Option<&Alias> {
        self.aliases.iter().find(|a| a.name == name)
    }

    pub fn add_to_history(&mut self, command: &str) {
        self.history.push(command.to_string());
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }

    pub fn execute_command(&mut self, command: &str) -> Result<String, ShellError> {
        self.add_to_history(command);
        
        // Parse command
        let parsed = self.parse_command(command)?;
        
        // Check for aliases
        let command_name = parsed.argv.first().unwrap_or(&String::new());
        if let Some(alias) = self.get_alias(command_name) {
            return self.execute_command(&alias.value);
        }
        
        // Execute built-in commands
        if let Some(result) = self.execute_builtin(&parsed) {
            return Ok(result);
        }
        
        // Execute external command
        self.execute_external(&parsed)
    }

    fn parse_command(&self, command: &str) -> Result<Command, ShellError> {
        let argv: Vec<String> = command.split_whitespace().map(|s| s.to_string()).collect();
        Ok(Command::new(argv))
    }

    fn execute_builtin(&mut self, command: &Command) -> Option<String> {
        match command.argv.first().map(|s| s.as_str()) {
            Some("cd") => self.builtin_cd(command),
            Some("pwd") => self.builtin_pwd(),
            Some("export") => self.builtin_export(command),
            Some("alias") => self.builtin_alias(command),
            Some("history") => self.builtin_history(),
            Some("jobs") => self.builtin_jobs(),
            Some("exit") => self.builtin_exit(),
            _ => None,
        }
    }

    fn builtin_cd(&mut self, command: &Command) -> Option<String> {
        let path = command.argv.get(1).map(|s| s.as_str()).unwrap_or("~");
        self.current_directory = path.to_string();
        Some(String::new())
    }

    fn builtin_pwd(&self) -> Option<String> {
        Some(self.current_directory.clone())
    }

    fn builtin_export(&mut self, command: &Command) -> Option<String> {
        if let Some(var_def) = command.argv.get(1) {
            if let Some((name, value)) = var_def.split_once('=') {
                self.environment.set_variable(name, value);
                self.environment.export_variable(name);
            }
        }
        Some(String::new())
    }

    fn builtin_alias(&mut self, command: &Command) -> Option<String> {
        if command.argv.len() > 1 {
            if let Some(alias_def) = command.argv.get(1) {
                if let Some((name, value)) = alias_def.split_once('=') {
                    self.add_alias(Alias::new(name, value));
                }
            }
        } else {
            // List all aliases
            let mut output = String::new();
            for alias in &self.aliases {
                output.push_str(&format!("alias {}='{}'\n", alias.name, alias.value));
            }
            return Some(output);
        }
        Some(String::new())
    }

    fn builtin_history(&self) -> Option<String> {
        let mut output = String::new();
        for (i, cmd) in self.history.iter().enumerate() {
            output.push_str(&format!("{}  {}\n", i + 1, cmd));
        }
        Some(output)
    }

    fn builtin_jobs(&self) -> Option<String> {
        let mut output = String::new();
        for job in &self.jobs {
            output.push_str(&format!("[{}] {} {}\n", job.id, job.state as u8, job.command));
        }
        Some(output)
    }

    fn builtin_exit(&self) -> Option<String> {
        // Exit shell
        None
    }

    fn execute_external(&self, _command: &Command) -> Result<String, ShellError> {
        // Execute external command
        Ok(String::new())
    }

    pub fn add_job(&mut self, command: &str, pid: u32) {
        let job_id = self.jobs.len() as u32 + 1;
        self.jobs.push(Job::new(job_id, command, pid));
    }

    pub fn get_prompt(&self) -> String {
        let mut prompt = self.prompt.clone();
        prompt = prompt.replace("\\u", "user");
        prompt = prompt.replace("\\h", "sigmaos");
        prompt = prompt.replace("\\w", &self.current_directory);
        prompt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellError {
    ParseError,
    CommandNotFound,
    ExecutionFailed,
    SyntaxError,
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
    fn test_command() {
        let command = Command::new(vec!["ls".to_string(), "-la".to_string()]);
        assert_eq!(command.argv.len(), 2);
    }

    #[test]
    fn test_alias() {
        let alias = Alias::new("ll", "ls -la");
        assert_eq!(alias.name, "ll");
    }

    #[test]
    fn test_environment() {
        let mut env = Environment::new();
        env.set_variable("HOME", "/home/user");
        assert_eq!(env.get_variable("HOME"), Some(&"/home/user".to_string()));
    }

    #[test]
    fn test_sigma_shell() {
        let mut shell = SigmaShell::new();
        shell.add_alias(Alias::new("ll", "ls -la"));
        assert_eq!(shell.aliases.len(), 1);
    }

    #[test]
    fn test_builtin_cd() {
        let mut shell = SigmaShell::new();
        shell.builtin_cd(&Command::new(vec!["cd".to_string(), "/tmp".to_string()]));
        assert_eq!(shell.current_directory, "/tmp");
    }

    #[test]
    fn test_builtin_pwd() {
        let shell = SigmaShell::new();
        let result = shell.builtin_pwd();
        assert!(result.is_some());
    }
}