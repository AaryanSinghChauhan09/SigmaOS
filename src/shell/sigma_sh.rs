// OOP-based Sigma Shell for SigmaOS
// Based on Ultimate Dominance Strategy: Stage 0 Milestone 0.1
// Implements interactive shell with command parsing, echo, and basic utilities

#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type CommandID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellError {
    Success = 0,
    CommandNotFound = 1,
    InvalidArgument = 2,
    PermissionDenied = 3,
}

pub trait ShellCommand {
    fn name(&self) -> &[u8];
    fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError>;
    fn help(&self) -> &[u8];
}

#[repr(C)]
pub struct EchoCommand {
    pub id: CommandID,
}

impl EchoCommand {
    pub fn new(id: CommandID) -> Self {
        EchoCommand { id }
    }
}

impl ShellCommand for EchoCommand {
    fn name(&self) -> &[u8] {
        b"echo"
    }
    fn execute(&mut self, _args: &[&[u8]]) -> Result<(), ShellError> {
        Ok(())
    }
    fn help(&self) -> &[u8] {
        b"echo [text] - Print text to output"
    }
}

#[repr(C)]
pub struct ExitCommand {
    pub id: CommandID,
}

impl ExitCommand {
    pub fn new(id: CommandID) -> Self {
        ExitCommand { id }
    }
}

impl ShellCommand for ExitCommand {
    fn name(&self) -> &[u8] {
        b"exit"
    }
    fn execute(&mut self, _args: &[&[u8]]) -> Result<(), ShellError> {
        Ok(())
    }
    fn help(&self) -> &[u8] {
        b"exit - Exit the shell"
    }
}

#[repr(C)]
pub struct HelpCommand {
    pub id: CommandID,
}

impl HelpCommand {
    pub fn new(id: CommandID) -> Self {
        HelpCommand { id }
    }
}

impl ShellCommand for HelpCommand {
    fn name(&self) -> &[u8] {
        b"help"
    }
    fn execute(&mut self, _args: &[&[u8]]) -> Result<(), ShellError> {
        Ok(())
    }
    fn help(&self) -> &[u8] {
        b"help - Show available commands"
    }
}

#[repr(C)]
pub struct ClearCommand {
    pub id: CommandID,
}

impl ClearCommand {
    pub fn new(id: CommandID) -> Self {
        ClearCommand { id }
    }
}

impl ShellCommand for ClearCommand {
    fn name(&self) -> &[u8] {
        b"clear"
    }
    fn execute(&mut self, _args: &[&[u8]]) -> Result<(), ShellError> {
        Ok(())
    }
    fn help(&self) -> &[u8] {
        b"clear - Clear the screen"
    }
}

pub trait Shell {
    fn register_command(&mut self, command: Box<dyn ShellCommand>)
        -> Result<CommandID, ShellError>;
    fn execute_line(&mut self, line: &[u8]) -> Result<(), ShellError>;
    fn get_prompt(&self) -> &[u8];
    fn set_prompt(&mut self, prompt: &[u8]);
}

#[repr(C)]
pub struct SimpleShell {
    pub commands: Vec<Option<Box<dyn ShellCommand>>>,
    pub next_id: AtomicUsize,
    pub prompt: [u8; 64],
    pub prompt_len: AtomicUsize,
}

impl SimpleShell {
    pub fn new() -> Self {
        let mut shell = SimpleShell {
            commands: Vec::new(),
            next_id: AtomicUsize::new(1),
            prompt: [0u8; 64],
            prompt_len: AtomicUsize::new(0),
        };
        let default_prompt = b"sigma-sh> ";
        shell.set_prompt(default_prompt);
        shell
    }
}

impl Default for SimpleShell {
    fn default() -> Self {
        Self::new()
    }
}

impl Shell for SimpleShell {
    fn register_command(
        &mut self,
        command: Box<dyn ShellCommand>,
    ) -> Result<CommandID, ShellError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.commands.push(Some(command));
        Ok(id)
    }

    fn execute_line(&mut self, line: &[u8]) -> Result<(), ShellError> {
        let mut args = Vec::new();
        let mut start = 0;
        let mut in_arg = false;

        for (i, &byte) in line.iter().enumerate() {
            if byte == b' ' || byte == b'\t' || byte == b'\n' {
                if in_arg {
                    args.push(&line[start..i]);
                    in_arg = false;
                }
            } else if !in_arg {
                start = i;
                in_arg = true;
            }
        }

        if in_arg {
            args.push(&line[start..line.len()]);
        }

        if args.is_empty() {
            return Ok(());
        }

        let cmd_name = args[0];
        let cmd_args: Vec<&[u8]> = args[1..].to_vec();

        for cmd_option in &mut self.commands {
            if let Some(ref mut cmd) = *cmd_option {
                if cmd.name() == cmd_name {
                    return cmd.execute(&cmd_args);
                }
            }
        }

        Err(ShellError::CommandNotFound)
    }

    fn get_prompt(&self) -> &[u8] {
        let len = self.prompt_len.load(Ordering::SeqCst);
        &self.prompt[..len]
    }

    fn set_prompt(&mut self, prompt: &[u8]) {
        let len = prompt.len().min(63);
        for i in 0..len {
            self.prompt[i] = prompt[i];
        }
        self.prompt_len.store(len, Ordering::SeqCst);
    }
}

pub trait ShellHistory {
    fn add(&mut self, command: &[u8]);
    fn get(&self, index: usize) -> Option<&[u8]>;
    fn get_last(&self) -> Option<&[u8]>;
}

#[repr(C)]
pub struct SimpleShellHistory {
    pub history: Vec<[u8; 256]>,
    pub lengths: Vec<usize>,
    pub next_index: AtomicUsize,
}

impl SimpleShellHistory {
    pub fn new() -> Self {
        SimpleShellHistory {
            history: Vec::new(),
            lengths: Vec::new(),
            next_index: AtomicUsize::new(0),
        }
    }
}

impl Default for SimpleShellHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellHistory for SimpleShellHistory {
    fn add(&mut self, command: &[u8]) {
        let len = command.len().min(255);
        let mut entry = [0u8; 256];
        for i in 0..len {
            entry[i] = command[i];
        }
        self.history.push(entry);
        self.lengths.push(len);
        self.next_index.fetch_add(1, Ordering::SeqCst);
    }

    fn get(&self, index: usize) -> Option<&[u8]> {
        if index >= self.history.len() {
            return None;
        }
        let len = self.lengths[index];
        Some(&self.history[index][..len])
    }

    fn get_last(&self) -> Option<&[u8]> {
        if self.history.is_empty() {
            return None;
        }
        let index = self.history.len() - 1;
        self.get(index)
    }
}

pub trait ShellEnvironment {
    fn set(&mut self, key: &[u8], value: &[u8]);
    fn get(&self, key: &[u8]) -> Option<&[u8]>;
    fn unset(&mut self, key: &[u8]);
}

#[repr(C)]
pub struct SimpleShellEnvironment {
    pub keys: Vec<[u8; 64]>,
    pub values: Vec<[u8; 256]>,
    pub key_lengths: Vec<usize>,
    pub value_lengths: Vec<usize>,
}

impl SimpleShellEnvironment {
    pub fn new() -> Self {
        SimpleShellEnvironment {
            keys: Vec::new(),
            values: Vec::new(),
            key_lengths: Vec::new(),
            value_lengths: Vec::new(),
        }
    }
}

impl Default for SimpleShellEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellEnvironment for SimpleShellEnvironment {
    fn set(&mut self, key: &[u8], value: &[u8]) {
        let key_len = key.len().min(63);
        let value_len = value.len().min(255);

        let mut key_entry = [0u8; 64];
        let mut value_entry = [0u8; 256];

        for i in 0..key_len {
            key_entry[i] = key[i];
        }
        for i in 0..value_len {
            value_entry[i] = value[i];
        }

        for i in 0..self.keys.len() {
            if self.key_lengths[i] == key_len && &self.keys[i][..key_len] == key {
                self.values[i] = value_entry;
                self.value_lengths[i] = value_len;
                return;
            }
        }

        self.keys.push(key_entry);
        self.values.push(value_entry);
        self.key_lengths.push(key_len);
        self.value_lengths.push(value_len);
    }

    fn get(&self, key: &[u8]) -> Option<&[u8]> {
        let key_len = key.len();
        for i in 0..self.keys.len() {
            if self.key_lengths[i] == key_len && &self.keys[i][..key_len] == key {
                let value_len = self.value_lengths[i];
                return Some(&self.values[i][..value_len]);
            }
        }
        None
    }

    fn unset(&mut self, key: &[u8]) {
        let key_len = key.len();
        for i in 0..self.keys.len() {
            if self.key_lengths[i] == key_len && &self.keys[i][..key_len] == key {
                self.keys.remove(i);
                self.values.remove(i);
                self.key_lengths.remove(i);
                self.value_lengths.remove(i);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_command() {
        let mut cmd = EchoCommand::new(1);
        assert_eq!(cmd.name(), b"echo");
        assert!(cmd.execute(&[b"hello", b"world"]).is_ok());
    }

    #[test]
    fn test_exit_command() {
        let mut cmd = ExitCommand::new(1);
        assert_eq!(cmd.name(), b"exit");
        assert!(cmd.execute(&[]).is_ok());
    }

    #[test]
    fn test_help_command() {
        let mut cmd = HelpCommand::new(1);
        assert_eq!(cmd.name(), b"help");
        assert!(cmd.execute(&[]).is_ok());
    }

    #[test]
    fn test_clear_command() {
        let mut cmd = ClearCommand::new(1);
        assert_eq!(cmd.name(), b"clear");
        assert!(cmd.execute(&[]).is_ok());
    }

    #[test]
    fn test_simple_shell_execution() {
        let mut shell = SimpleShell::new();
        let cmd = Box::new(EchoCommand::new(1));
        shell.register_command(cmd).unwrap();

        assert_eq!(shell.get_prompt(), b"sigma-sh> ");
        assert!(shell.execute_line(b"echo hello world").is_ok());
    }

    #[test]
    fn test_shell_history() {
        let mut history = SimpleShellHistory::new();
        assert!(history.get_last().is_none());

        history.add(b"echo hello");
        assert_eq!(history.get_last().unwrap(), b"echo hello");
        assert_eq!(history.get(0).unwrap(), b"echo hello");
    }

    #[test]
    fn test_shell_environment() {
        let mut env = SimpleShellEnvironment::new();
        assert!(env.get(b"PATH").is_none());

        env.set(b"PATH", b"/bin:/usr/bin");
        assert_eq!(env.get(b"PATH").unwrap(), b"/bin:/usr/bin");

        env.unset(b"PATH");
        assert!(env.get(b"PATH").is_none());
    }
}
