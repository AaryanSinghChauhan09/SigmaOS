#![no_std]
#![no_main]

/// OOP-based Shell Command System for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 696
/// Implements command parsing, execution, and built-in commands

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CommandID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CommandError { Success = 0, NotFound = 1, InvalidArgs = 2, ExecutionFailed = 3 }

pub trait ShellCommand {
    fn name(&self) -> &[u8];
    fn execute(&mut self, args: &[[u8; 64]]) -> Result<Vec<u8>, CommandError>;
    fn help(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleShellCommand {
    pub name: [u8; 32],
    pub description: [u8; 128],
}

impl SimpleShellCommand {
    pub fn new(name: &[u8], description: &[u8]) -> Self {
        let mut name_array = [0u8; 32];
        let mut desc_array = [0u8; 128];
        let name_len = name.len().min(31);
        let desc_len = description.len().min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(description.as_ptr(), desc_array.as_mut_ptr(), desc_len);
        }
        SimpleShellCommand {
            name: name_array,
            description: desc_array,
        }
    }
}

impl ShellCommand for SimpleShellCommand {
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(32);
        &self.name[..len]
    }

    fn execute(&mut self, _args: &[[u8; 64]]) -> Result<Vec<u8>, CommandError> {
        let mut output = Vec::new();
        let name = self.name();
        for &byte in name { output.push(byte); }
        output.push(b':');
        output.push(b' ');
        output.push(b'o');
        output.push(b'k');
        output.push(b'\n');
        Ok(output)
    }

    fn help(&self) -> &[u8] {
        let len = self.description.iter().position(|&b| b == 0).unwrap_or(128);
        &self.description[..len]
    }
}

pub trait CommandParser {
    fn parse(&self, input: &[u8]) -> Result<([u8; 32], Vec<[u8; 64]>), CommandError>;
    fn validate(&self, command: &[u8], args: &[[u8; 64]]) -> Result<(), CommandError>;
}

#[repr(C)]
pub struct SimpleCommandParser;

impl SimpleCommandParser {
    pub fn new() -> Self { SimpleCommandParser }
}

impl CommandParser for SimpleCommandParser {
    fn parse(&self, input: &[u8]) -> Result<([u8; 32], Vec<[u8; 64]>), CommandError> {
        let mut command = [0u8; 32];
        let mut args = Vec::new();
        let mut current_arg = [0u8; 64];
        let mut arg_index = 0;
        let mut in_command = true;
        let mut command_index = 0;

        for &byte in input {
            if byte == b' ' || byte == b'\n' || byte == b'\t' {
                if in_command && command_index > 0 {
                    in_command = false;
                } else if arg_index > 0 {
                    args.push(current_arg);
                    current_arg = [0u8; 64];
                    arg_index = 0;
                }
            } else {
                if in_command {
                    if command_index < 31 {
                        command[command_index] = byte;
                        command_index += 1;
                    }
                } else {
                    if arg_index < 63 {
                        current_arg[arg_index] = byte;
                        arg_index += 1;
                    }
                }
            }
        }

        if arg_index > 0 {
            args.push(current_arg);
        }

        Ok((command, args))
    }

    fn validate(&self, _command: &[u8], _args: &[[u8; 64]]) -> Result<(), CommandError> {
        Ok(())
    }
}

pub trait CommandRegistry {
    fn register(&mut self, command: Box<dyn ShellCommand>) -> Result<(), CommandError>;
    fn unregister(&mut self, name: &[u8]) -> Result<(), CommandError>;
    fn get(&self, name: &[u8]) -> Option<&dyn ShellCommand>;
    fn list(&self) -> Vec<&[u8]>;
}

#[repr(C)]
pub struct SimpleCommandRegistry {
    pub commands: Vec<Option<Box<dyn ShellCommand>>>,
}

impl SimpleCommandRegistry {
    pub fn new() -> Self {
        SimpleCommandRegistry {
            commands: Vec::new(),
        }
    }

    pub fn register_builtins(&mut self) {
        let echo = SimpleShellCommand::new(b"echo", b"Print arguments to stdout");
        self.commands.push(Some(Box::new(echo)));

        let ls = SimpleShellCommand::new(b"ls", b"List directory contents");
        self.commands.push(Some(Box::new(ls)));

        let cd = SimpleShellCommand::new(b"cd", b"Change directory");
        self.commands.push(Some(Box::new(cd)));

        let pwd = SimpleShellCommand::new(b"pwd", b"Print working directory");
        self.commands.push(Some(Box::new(pwd)));
    }
}

impl CommandRegistry for SimpleCommandRegistry {
    fn register(&mut self, command: Box<dyn ShellCommand>) -> Result<(), CommandError> {
        self.commands.push(Some(command));
        Ok(())
    }

    fn unregister(&mut self, name: &[u8]) -> Result<(), CommandError> {
        for i in 0..self.commands.len() {
            if let Some(ref cmd) = self.commands[i] {
                if cmd.name() == name {
                    self.commands[i] = None;
                    return Ok(());
                }
            }
        }
        Err(CommandError::NotFound)
    }

    fn get(&self, name: &[u8]) -> Option<&dyn ShellCommand> {
        for command_option in &self.commands {
            if let Some(ref command) = *command_option {
                if command.name() == name {
                    return Some(command.as_ref());
                }
            }
        }
        None
    }

    fn list(&self) -> Vec<&[u8]> {
        let mut names = Vec::new();
        for command_option in &self.commands {
            if let Some(ref command) = *command_option {
                names.push(command.name());
            }
        }
        names
    }
}

pub trait ShellSession {
    fn execute_line(&mut self, input: &[u8]) -> Result<Vec<u8>, CommandError>;
    fn set_environment(&mut self, key: &[u8], value: &[u8]);
    fn get_environment(&self, key: &[u8]) -> Option<&[u8]>;
}

#[repr(C)]
pub struct SimpleShellSession {
    pub registry: SimpleCommandRegistry,
    pub parser: SimpleCommandParser,
    pub environment: Vec<([u8; 64], [u8; 128])>,
}

impl SimpleShellSession {
    pub fn new() -> Self {
        let mut registry = SimpleCommandRegistry::new();
        registry.register_builtins();
        SimpleShellSession {
            registry,
            parser: SimpleCommandParser::new(),
            environment: Vec::new(),
        }
    }
}

impl ShellSession for SimpleShellSession {
    fn execute_line(&mut self, input: &[u8]) -> Result<Vec<u8>, CommandError> {
        let (command_name, args) = self.parser.parse(input)?;

        if let Some(command) = self.registry.get(&command_name) {
            let mut cmd = SimpleShellCommand::new(command.name(), command.help());
            cmd.execute(&args)
        } else {
            Err(CommandError::NotFound)
        }
    }

    fn set_environment(&mut self, key: &[u8], value: &[u8]) {
        let mut key_array = [0u8; 64];
        let mut value_array = [0u8; 128];
        let key_len = key.len().min(63);
        let value_len = value.len().min(127);
        for i in 0..key_len { key_array[i] = key[i]; }
        for i in 0..value_len { value_array[i] = value[i]; }
        self.environment.push((key_array, value_array));
    }

    fn get_environment(&self, key: &[u8]) -> Option<&[u8]> {
        for &(ref k, ref v) in &self.environment {
            let len = k.iter().position(|&b| b == 0).unwrap_or(64);
            if &k[..len] == key {
                let vlen = v.iter().position(|&b| b == 0).unwrap_or(128);
                return Some(&v[..vlen]);
            }
        }
        None
    }
}

pub trait CommandHistory {
    fn add(&mut self, command: &[u8]);
    fn get_previous(&self) -> Option<&[u8]>;
    fn get_next(&self) -> Option<&[u8]>;
    fn list(&self) -> Vec<&[u8]>;
}

#[repr(C)]
pub struct SimpleCommandHistory {
    pub history: Vec<[u8; 256]>,
    pub current_index: AtomicUsize,
}

impl SimpleCommandHistory {
    pub fn new() -> Self {
        SimpleCommandHistory {
            history: Vec::new(),
            current_index: AtomicUsize::new(0),
        }
    }
}

impl CommandHistory for SimpleCommandHistory {
    fn add(&mut self, command: &[u8]) {
        let mut cmd_array = [0u8; 256];
        let cmd_len = command.len().min(255);
        for i in 0..cmd_len { cmd_array[i] = command[i]; }
        self.history.push(cmd_array);
        self.current_index.store(self.history.len(), Ordering::SeqCst);
    }

    fn get_previous(&self) -> Option<&[u8]> {
        let idx = self.current_index.load(Ordering::SeqCst);
        if idx > 0 && idx <= self.history.len() {
            let len = self.history[idx - 1].iter().position(|&b| b == 0).unwrap_or(256);
            Some(&self.history[idx - 1][..len])
        } else {
            None
        }
    }

    fn get_next(&self) -> Option<&[u8]> {
        let idx = self.current_index.load(Ordering::SeqCst);
        if idx < self.history.len() {
            let len = self.history[idx].iter().position(|&b| b == 0).unwrap_or(256);
            Some(&self.history[idx][..len])
        } else {
            None
        }
    }

    fn list(&self) -> Vec<&[u8]> {
        let mut commands = Vec::new();
        for cmd in &self.history {
            let len = cmd.iter().position(|&b| b == 0).unwrap_or(256);
            commands.push(&cmd[..len]);
        }
        commands
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
