#[cfg(not(target_os = "none"))]
extern crate alloc as std_alloc;
#[cfg(not(target_os = "none"))]
use std_alloc::boxed::Box;

#![no_std]
#![no_main]

/// OOP-based Sigma Shell for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 0 Milestone 0.1
/// Implements interactive shell with command parsing, echo, environment variables, aliases, and basic utilities

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CommandID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ShellError { Success = 0, CommandNotFound = 1, InvalidArgument = 2, PermissionDenied = 3 }

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
    pub fn new(id: CommandID) -> Self { EchoCommand { id } }
}

impl ShellCommand for EchoCommand {
    fn name(&self) -> &[u8] { b"echo" }
    fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError> {
        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                // print spacer in actual implementation
            }
            for _byte in *arg {
                // write to stdout
            }
        }
        Ok(())
    }
    fn help(&self) -> &[u8] { b"echo [text] - Print text to output (supports variable expansion like $USER)" }
}

#[repr(C)]
pub struct ExitCommand {
    pub id: CommandID,
}

impl ExitCommand {
    pub fn new(id: CommandID) -> Self { ExitCommand { id } }
}

impl ShellCommand for ExitCommand {
    fn name(&self) -> &[u8] { b"exit" }
    fn execute(&mut self, _args: &[&[u8]]) -> Result<(), ShellError> {
        Ok(())
    }
    fn help(&self) -> &[u8] { b"exit - Exit the shell" }
}

#[repr(C)]
pub struct HelpCommand {
    pub id: CommandID,
}

impl HelpCommand {
    pub fn new(id: CommandID) -> Self { HelpCommand { id } }
}

impl ShellCommand for HelpCommand {
    fn name(&self) -> &[u8] { b"help" }
    fn execute(&mut self, _args: &[&[u8]]) -> Result<(), ShellError> {
        Ok(())
    }
    fn help(&self) -> &[u8] { b"help - Show available commands" }
}

#[repr(C)]
pub struct ClearCommand {
    pub id: CommandID,
}

impl ClearCommand {
    pub fn new(id: CommandID) -> Self { ClearCommand { id } }
}

impl ShellCommand for ClearCommand {
    fn name(&self) -> &[u8] { b"clear" }
    fn execute(&mut self, _args: &[&[u8]]) -> Result<(), ShellError> {
        Ok(())
    }
    fn help(&self) -> &[u8] { b"clear - Clear the screen" }
}

/// Linux-style built-in command to define aliases
pub struct AliasCommand {
    pub shell_ptr: *mut SimpleShell,
}

impl ShellCommand for AliasCommand {
    fn name(&self) -> &[u8] { b"alias" }
    fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError> {
        if args.len() < 2 {
            return Err(ShellError::InvalidArgument);
        }
        unsafe {
            if !self.shell_ptr.is_null() {
                (*self.shell_ptr).set_alias(args[0], args[1]);
            }
        }
        Ok(())
    }
    fn help(&self) -> &[u8] { b"alias [shortcut] [command] - Define a shell alias" }
}

/// Linux-style built-in command to remove aliases
pub struct UnaliasCommand {
    pub shell_ptr: *mut SimpleShell,
}

impl ShellCommand for UnaliasCommand {
    fn name(&self) -> &[u8] { b"unalias" }
    fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError> {
        if args.is_empty() {
            return Err(ShellError::InvalidArgument);
        }
        unsafe {
            if !self.shell_ptr.is_null() {
                (*self.shell_ptr).unset_alias(args[0]);
            }
        }
        Ok(())
    }
    fn help(&self) -> &[u8] { b"unalias [shortcut] - Remove a shell alias" }
}

pub trait Shell {
    fn register_command(&mut self, command: Box<dyn ShellCommand>) -> Result<CommandID, ShellError>;
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
    pub env: SimpleShellEnvironment,
    pub aliases: SimpleShellEnvironment, // Recycles environment implementation for alias maps
}

impl SimpleShell {
    pub fn new() -> Self {
        let mut shell = SimpleShell {
            commands: Vec::new(),
            next_id: AtomicUsize::new(1),
            prompt: [0u8; 64],
            prompt_len: AtomicUsize::new(0),
            env: SimpleShellEnvironment::new(),
            aliases: SimpleShellEnvironment::new(),
        };
        let default_prompt = b"sigma-sh> ";
        shell.set_prompt(default_prompt);

        // Populate standard Linux-inspired default environment variables
        shell.env.set(b"USER", b"sovereign");
        shell.env.set(b"HOME", b"/userland/home/sovereign");
        shell.env.set(b"PATH", b"/shards:/system:/userland");

        shell
    }

    pub fn set_alias(&mut self, name: &[u8], target: &[u8]) {
        self.aliases.set(name, target);
    }

    pub fn unset_alias(&mut self, name: &[u8]) {
        self.aliases.unset(name);
    }

    pub fn get_alias(&self, name: &[u8]) -> Option<&[u8]> {
        self.aliases.get(name)
    }
}

impl Shell for SimpleShell {
    fn register_command(&mut self, command: Box<dyn ShellCommand>) -> Result<CommandID, ShellError> {
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
            } else {
                if !in_arg {
                    start = i;
                    in_arg = true;
                }
            }
        }
        
        if in_arg {
            args.push(&line[start..line.len()]);
        }
        
        if args.is_empty() {
            return Ok(());
        }
        
        // 1. Resolve Command Aliases (udev/bash inspiration)
        let resolved_cmd_name = if let Some(alias_target) = self.get_alias(args[0]) {
            alias_target
        } else {
            args[0]
        };

        // 2. Perform Environment Variable Expansion (e.g. $USER -> sovereign)
        let mut expanded_args = Vec::new();
        for &arg in args[1..].iter() {
            if arg.starts_with(b"$") && arg.len() > 1 {
                if let Some(val) = self.env.get(&arg[1..]) {
                    expanded_args.push(val);
                } else {
                    expanded_args.push(arg); // If not found, keep original
                }
            } else {
                expanded_args.push(arg);
            }
        }

        let cmd_args: Vec<&[u8]> = expanded_args.to_vec();
        
        for cmd_option in &mut self.commands {
            if let Some(ref mut cmd) = *cmd_option {
                if cmd.name() == resolved_cmd_name {
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
    
    fn get_last(&self) -> Option<&[u8]>;
}

impl SimpleShellHistory {
    fn get_last_impl(&self) -> Option<&[u8]> {
        if self.history.is_empty() {
            return None;
        }
        let index = self.history.len() - 1;
        self.get(index)
    }
}

impl ShellHistory for SimpleShellHistory {
    fn get_last(&self) -> Option<&[u8]> {
        self.get_last_impl()
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

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Vec::new();
        for i in 0..self.len {
            unsafe {
                new_vec.push((*self.data.add(i)).clone());
            }
        }
        new_vec
    }
}

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
    fn is_empty(&self) -> bool { self.len == 0 }
    fn len(&self) -> usize { self.len }
    fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if new_data.is_null() {
            panic!("out of memory");
        }
        if !self.data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
        }
        self.data = new_data;
        self.capacity = new_capacity;
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_default_env_variables() {
        let shell = SimpleShell::new();
        assert_eq!(shell.env.get(b"USER"), Some(b"sovereign" as &[u8]));
        assert_eq!(shell.env.get(b"HOME"), Some(b"/userland/home/sovereign" as &[u8]));
        assert_eq!(shell.env.get(b"PATH"), Some(b"/shards:/system:/userland" as &[u8]));
    }

    #[test]
    fn test_shell_variable_expansion_and_alias_resolution() {
        let mut shell = SimpleShell::new();

        // 1. Create spy command to capture expanded parameters
        struct SpyCommand {
            captured_arg: [u8; 128],
            captured_len: usize,
        }
        impl ShellCommand for SpyCommand {
            fn name(&self) -> &[u8] { b"spy" }
            fn help(&self) -> &[u8] { b"spy" }
            fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError> {
                if !args.is_empty() {
                    let len = args[0].len().min(127);
                    self.captured_arg[..len].copy_from_slice(&args[0][..len]);
                    self.captured_len = len;
                }
                Ok(())
            }
        }

        let spy = Box::new(SpyCommand {
            captured_arg: [0; 128],
            captured_len: 0,
        });

        // Register spy
        let _ = shell.register_command(spy);

        // 2. Setup environment variable and execute line
        shell.env.set(b"SECRET_KEY", b"sovereign_pass_123");

        // Execute 'spy $SECRET_KEY'
        shell.execute_line(b"spy $SECRET_KEY").unwrap();

        // Inspect captured variable inside spy command
        if let Some(ref cmd_box) = shell.commands[0] {
            // Unsafe cast to access captured properties (since we can't downcast Box<dyn ShellCommand>)
            let spy_ptr = cmd_box as *const Box<dyn ShellCommand> as *const SpyCommand;
            unsafe {
                let captured = &(*spy_ptr).captured_arg[..(*spy_ptr).captured_len];
                assert_eq!(captured, b"sovereign_pass_123");
            }
        }

        // 3. Setup and verify alias resolution
        shell.set_alias(b"reveal", b"spy");
        shell.execute_line(b"reveal $USER").unwrap();

        if let Some(ref cmd_box) = shell.commands[0] {
            let spy_ptr = cmd_box as *const Box<dyn ShellCommand> as *const SpyCommand;
            unsafe {
                let captured = &(*spy_ptr).captured_arg[..(*spy_ptr).captured_len];
                assert_eq!(captured, b"sovereign");
            }
        }

        // 4. Remove alias
        shell.unset_alias(b"reveal");
        assert!(shell.execute_line(b"reveal $USER").is_err()); // Command reveal not found
    }
}
