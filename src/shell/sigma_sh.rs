extern crate alloc;
use alloc::boxed::Box;

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

/// Linux/BSD export / setenv builtin command
pub struct ExportCommand {
    pub shell_ptr: *mut SimpleShell,
}

impl ShellCommand for ExportCommand {
    fn name(&self) -> &[u8] { b"export" }
    fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError> {
        if args.is_empty() { return Ok(()); }
        for arg in args {
            if let Some(pos) = arg.iter().position(|&b| b == b'=') {
                let key = &arg[..pos];
                let val = &arg[pos + 1..];
                unsafe {
                    if !self.shell_ptr.is_null() {
                        (*self.shell_ptr).env.set(key, val);
                    }
                }
            }
        }
        Ok(())
    }
    fn help(&self) -> &[u8] { b"export KEY=VALUE - Set environment variable" }
}

/// Linux/BSD unset / unsetenv builtin command
pub struct UnsetCommand {
    pub shell_ptr: *mut SimpleShell,
}

impl ShellCommand for UnsetCommand {
    fn name(&self) -> &[u8] { b"unset" }
    fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError> {
        for arg in args {
            unsafe {
                if !self.shell_ptr.is_null() {
                    (*self.shell_ptr).env.unset(arg);
                }
            }
        }
        Ok(())
    }
    fn help(&self) -> &[u8] { b"unset KEY - Unset environment variable" }
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
    pub last_exit_code: AtomicUsize,
    pub auto_cd: bool,
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
            last_exit_code: AtomicUsize::new(0),
            auto_cd: true,
        };
        let default_prompt = b"sigma-sh> ";
        shell.set_prompt(default_prompt);

        // Populate standard Linux-inspired default environment variables
        shell.env.set(b"USER", b"sovereign");
        shell.env.set(b"HOSTNAME", b"sigmaos");
        shell.env.set(b"HOME", b"/userland/home/sovereign");
        shell.env.set(b"PWD", b"/userland/home/sovereign");
        shell.env.set(b"PATH", b"/shards:/system:/userland");

        // Register built-in commands (echo, exit, help, clear, alias, unalias, export, unset)
        let shell_ptr = &mut shell as *mut SimpleShell;
        let _ = shell.register_command(Box::new(EchoCommand::new(0)));
        let _ = shell.register_command(Box::new(ExitCommand::new(0)));
        let _ = shell.register_command(Box::new(HelpCommand::new(0)));
        let _ = shell.register_command(Box::new(ClearCommand::new(0)));
        let _ = shell.register_command(Box::new(AliasCommand { shell_ptr }));
        let _ = shell.register_command(Box::new(UnaliasCommand { shell_ptr }));
        let _ = shell.register_command(Box::new(ExportCommand { shell_ptr }));
        let _ = shell.register_command(Box::new(UnsetCommand { shell_ptr }));

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

    /// Zsh/Bash/Fish-inspired prompt string token expansion (%n, %m, %~, %?, %F{color}, %f)
    pub fn expand_prompt_string(&self, template: &[u8]) -> alloc::vec::Vec<u8> {
        let mut result = alloc::vec::Vec::new();
        let mut i = 0;
        while i < template.len() {
            if template[i] == b'%' && i + 1 < template.len() {
                match template[i + 1] {
                    b'n' => {
                        let user = self.env.get(b"USER").unwrap_or(b"sovereign");
                        result.extend_from_slice(user);
                        i += 2;
                    }
                    b'm' => {
                        let host = self.env.get(b"HOSTNAME").unwrap_or(b"sigmaos");
                        result.extend_from_slice(host);
                        i += 2;
                    }
                    b'~' | b'w' => {
                        let pwd = self.env.get(b"PWD").unwrap_or(b"~");
                        result.extend_from_slice(pwd);
                        i += 2;
                    }
                    b'?' => {
                        let code = self.last_exit_code.load(Ordering::SeqCst);
                        let mut buf = [0u8; 16];
                        let mut n = code;
                        let mut idx = 15;
                        if n == 0 {
                            buf[idx] = b'0';
                            idx -= 1;
                        } else {
                            while n > 0 {
                                buf[idx] = b'0' + (n % 10) as u8;
                                n /= 10;
                                idx -= 1;
                            }
                        }
                        result.extend_from_slice(&buf[idx + 1..16]);
                        i += 2;
                    }
                    b'f' => {
                        result.extend_from_slice(b"\x1b[0m");
                        i += 2;
                    }
                    b'F' if i + 2 < template.len() && template[i + 2] == b'{' => {
                        let mut end_idx = i + 3;
                        while end_idx < template.len() && template[end_idx] != b'}' {
                            end_idx += 1;
                        }
                        if end_idx < template.len() {
                            let color_name = &template[i + 3..end_idx];
                            match color_name {
                                b"red" => result.extend_from_slice(b"\x1b[31m"),
                                b"green" => result.extend_from_slice(b"\x1b[32m"),
                                b"yellow" => result.extend_from_slice(b"\x1b[33m"),
                                b"blue" => result.extend_from_slice(b"\x1b[34m"),
                                b"magenta" => result.extend_from_slice(b"\x1b[35m"),
                                b"cyan" => result.extend_from_slice(b"\x1b[36m"),
                                b"white" => result.extend_from_slice(b"\x1b[37m"),
                                b"reset" => result.extend_from_slice(b"\x1b[0m"),
                                _ => {}
                            }
                            i = end_idx + 1;
                        } else {
                            result.push(template[i]);
                            i += 1;
                        }
                    }
                    _ => {
                        result.push(template[i]);
                        i += 1;
                    }
                }
            } else {
                result.push(template[i]);
                i += 1;
            }
        }
        result
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
        let mut target_buf = [0u8; 128];
        let mut target_len = 0;
        let is_aliased = if let Some(alias_target) = self.get_alias(args[0]) {
            let len = alias_target.len().min(127);
            target_buf[..len].copy_from_slice(&alias_target[..len]);
            target_len = len;
            true
        } else {
            false
        };

        let resolved_cmd_name: &[u8] = if is_aliased {
            &target_buf[..target_len]
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

        let cmd_args: Vec<&[u8]> = expanded_args.clone();
        
        for cmd_option in &mut self.commands {
            if let Some(ref mut cmd) = *cmd_option {
                if cmd.name() == resolved_cmd_name {
                    let res = cmd.execute(&cmd_args);
                    let code = if res.is_ok() { 0 } else { 1 };
                    self.last_exit_code.store(code, Ordering::SeqCst);
                    return res;
                }
            }
        }

        // 3. Zsh-style auto-cd directory resolution
        if self.auto_cd && (resolved_cmd_name.starts_with(b"/") || resolved_cmd_name.starts_with(b"./") || resolved_cmd_name.starts_with(b"../") || resolved_cmd_name.starts_with(b"~") || resolved_cmd_name.ends_with(b"/")) {
            self.env.set(b"PWD", resolved_cmd_name);
            self.last_exit_code.store(0, Ordering::SeqCst);
            return Ok(());
        }
        
        self.last_exit_code.store(1, Ordering::SeqCst);
        Err(ShellError::CommandNotFound)
    }
    
    fn get_prompt(&self) -> &[u8] {
        let len = self.prompt_len.load(Ordering::SeqCst);
        let template = &self.prompt[..len];
        let expanded = self.expand_prompt_string(template);
        if !expanded.is_empty() {
            // Static storage for formatted prompt reference
            static mut PROMPT_BUF: [u8; 128] = [0u8; 128];
            static mut PROMPT_BUF_LEN: usize = 0;
            let copy_len = expanded.len().min(127);
            unsafe {
                PROMPT_BUF[..copy_len].copy_from_slice(&expanded[..copy_len]);
                PROMPT_BUF_LEN = copy_len;
                &PROMPT_BUF[..PROMPT_BUF_LEN]
            }
        } else {
            template
        }
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
    pub timestamps: Vec<u64>,
    pub next_index: AtomicUsize,
}

impl SimpleShellHistory {
    pub fn new() -> Self {
        SimpleShellHistory {
            history: Vec::new(),
            lengths: Vec::new(),
            timestamps: Vec::new(),
            next_index: AtomicUsize::new(0),
        }
    }

    /// Add command to history with explicit timestamp (Zsh history format inspired)
    pub fn add_with_timestamp(&mut self, command: &[u8], timestamp: u64) {
        let len = command.len().min(255);
        let mut entry = [0u8; 256];
        for i in 0..len {
            entry[i] = command[i];
        }
        self.history.push(entry);
        self.lengths.push(len);
        self.timestamps.push(timestamp);
        self.next_index.fetch_add(1, Ordering::SeqCst);
    }

    /// Search history for entries starting with prefix (Zsh history-beginning-search-backward inspired)
    pub fn history_search_prefix(&self, prefix: &[u8]) -> Vec<&[u8]> {
        let mut matches = Vec::new();
        if prefix.is_empty() {
            return matches;
        }
        for i in (0..self.history.len()).rev() {
            let len = self.lengths[i];
            let entry = &self.history[i][..len];
            if entry.starts_with(prefix) {
                matches.push(entry);
            }
        }
        matches
    }

    /// Search history for entries containing substring (Fish/Bash Ctrl+R substring search inspired)
    pub fn history_search_substring(&self, pattern: &[u8]) -> Vec<&[u8]> {
        let mut matches = Vec::new();
        if pattern.is_empty() {
            return matches;
        }
        for i in (0..self.history.len()).rev() {
            let len = self.lengths[i];
            let entry = &self.history[i][..len];
            if entry.windows(pattern.len()).any(|w| w == pattern) {
                matches.push(entry);
            }
        }
        matches
    }
}

impl ShellHistory for SimpleShellHistory {
    fn add(&mut self, command: &[u8]) {
        self.add_with_timestamp(command, 0);
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

/// Fish Shell Inspired Auto-Suggestion Engine
pub struct ShellAutoSuggestEngine {
    pub suggestions: Vec<[u8; 128]>,
    pub suggestion_lens: Vec<usize>,
}

impl ShellAutoSuggestEngine {
    pub fn new() -> Self {
        ShellAutoSuggestEngine {
            suggestions: Vec::new(),
            suggestion_lens: Vec::new(),
        }
    }

    pub fn add_candidate(&mut self, suggestion: &[u8]) {
        let len = suggestion.len().min(127);
        let mut entry = [0u8; 128];
        for i in 0..len {
            entry[i] = suggestion[i];
        }
        self.suggestions.push(entry);
        self.suggestion_lens.push(len);
    }

    /// Fish-style prefix match suggestion lookup
    pub fn predict_completion<'a>(&'a self, input_prefix: &[u8]) -> Option<&'a [u8]> {
        if input_prefix.is_empty() {
            return None;
        }
        for i in 0..self.suggestions.len() {
            let len = self.suggestion_lens[i];
            let cand = &self.suggestions[i][..len];
            if cand.starts_with(input_prefix) {
                return Some(cand);
            }
        }
        None
    }
}

/// OpenBSD Ksh Inspired Pledge & Unveil Sandbox Guard for Subshells
pub struct ShellPledgeUnveilGuard {
    pub pledge_mask: u32, // Bitmask: 0x1 (stdio), 0x2 (rpath), 0x4 (wpath), 0x8 (exec), 0x10 (proc)
}

impl ShellPledgeUnveilGuard {
    pub fn new(pledge_mask: u32) -> Self {
        ShellPledgeUnveilGuard { pledge_mask }
    }

    pub fn is_pledge_permitted(&self, req_flag: u32) -> bool {
        (self.pledge_mask & req_flag) != 0
    }

    pub fn restrict_pledge(&mut self, new_mask: u32) -> Result<(), ShellError> {
        if (new_mask & !self.pledge_mask) != 0 {
            // Cannot gain privileges
            return Err(ShellError::PermissionDenied);
        }
        self.pledge_mask &= new_mask;
        Ok(())
    }
}

/// Zsh Inspired Colorized Syntax Highlighter Tokenizer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenClass {
    Command,
    Keyword,
    OptionFlag,
    Argument,
    Variable,
    StringLiteral,
    Comment,
    Operator,
    Unknown,
}

pub struct ShellSyntaxHighlighter;

impl ShellSyntaxHighlighter {
    pub fn classify_token(token: &[u8], is_first: bool) -> TokenClass {
        if token.is_empty() {
            return TokenClass::Unknown;
        }
        if token.starts_with(b"#") {
            return TokenClass::Comment;
        }
        if token == b"&&" || token == b"||" || token == b";" || token == b"|" || token == b">" || token == b"<" || token == b">>" {
            return TokenClass::Operator;
        }
        if token.starts_with(b"\"") || token.starts_with(b"'") {
            return TokenClass::StringLiteral;
        }
        let keywords: &[&[u8]] = &[
            b"if", b"then", b"else", b"elif", b"fi", b"for", b"in", b"do", b"done",
            b"while", b"until", b"case", b"esac", b"function", b"select",
        ];
        for kw in keywords {
            if token == *kw {
                return TokenClass::Keyword;
            }
        }
        if is_first {
            TokenClass::Command
        } else if token.starts_with(b"-") {
            TokenClass::OptionFlag
        } else if token.starts_with(b"$") {
            TokenClass::Variable
        } else {
            TokenClass::Argument
        }
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
        let layout = core::alloc::Layout::array::<T>(new_capacity).unwrap();
        let new_data = alloc::alloc::alloc(layout) as *mut T;
        if new_data.is_null() {
            panic!("out of memory");
        }
        if !self.data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 {
                let old_layout = core::alloc::Layout::array::<T>(self.capacity).unwrap();
                alloc::alloc::dealloc(self.data as *mut u8, old_layout);
            }
        }
        self.data = new_data;
        self.capacity = new_capacity;
    }
}


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

        static mut CAPTURED_BUF: [u8; 128] = [0; 128];
        static mut CAPTURED_LEN: usize = 0;

        // 1. Create spy command to capture expanded parameters
        struct SpyCommand;
        impl ShellCommand for SpyCommand {
            fn name(&self) -> &[u8] { b"spy" }
            fn help(&self) -> &[u8] { b"spy" }
            fn execute(&mut self, args: &[&[u8]]) -> Result<(), ShellError> {
                if !args.is_empty() {
                    let len = args[0].len().min(127);
                    unsafe {
                        CAPTURED_BUF[..len].copy_from_slice(&args[0][..len]);
                        CAPTURED_LEN = len;
                    }
                }
                Ok(())
            }
        }

        let spy = Box::new(SpyCommand);

        // Register spy
        let _ = shell.register_command(spy);

        // 2. Setup environment variable and execute line
        shell.env.set(b"SECRET_KEY", b"sovereign_pass_123");

        // Execute 'spy $SECRET_KEY'
        shell.execute_line(b"spy $SECRET_KEY").unwrap();

        // Inspect captured variable inside spy command
        unsafe {
            assert_eq!(&CAPTURED_BUF[..CAPTURED_LEN], b"sovereign_pass_123");
        }

        // 3. Setup and verify alias resolution
        shell.set_alias(b"reveal", b"spy");
        shell.execute_line(b"reveal $USER").unwrap();

        unsafe {
            assert_eq!(&CAPTURED_BUF[..CAPTURED_LEN], b"sovereign");
        }

        // 4. Remove alias
        shell.unset_alias(b"reveal");
        assert!(shell.execute_line(b"reveal $USER").is_err()); // Command reveal not found
    }

    #[test]
    fn test_fish_auto_suggest_engine() {
        let mut engine = ShellAutoSuggestEngine::new();
        engine.add_candidate(b"systemctl status nginx");
        engine.add_candidate(b"sysctl -a");

        assert_eq!(engine.predict_completion(b"system"), Some(b"systemctl status nginx" as &[u8]));
        assert_eq!(engine.predict_completion(b"sysc"), Some(b"sysctl -a" as &[u8]));
        assert_eq!(engine.predict_completion(b"unknown"), None);
    }

    #[test]
    fn test_openbsd_pledge_unveil_shell_guard() {
        // Pledge: stdio (0x1) | rpath (0x2) | exec (0x8)
        let mut guard = ShellPledgeUnveilGuard::new(0x1 | 0x2 | 0x8);

        assert!(guard.is_pledge_permitted(0x1)); // stdio allowed
        assert!(guard.is_pledge_permitted(0x2)); // rpath allowed
        assert!(!guard.is_pledge_permitted(0x4)); // wpath not permitted

        // Restrict pledge to stdio | rpath
        assert!(guard.restrict_pledge(0x1 | 0x2).is_ok());
        assert!(!guard.is_pledge_permitted(0x8)); // exec dropped

        // Attempting to regain exec (0x8) fails
        assert!(guard.restrict_pledge(0x1 | 0x8).is_err());
    }

    #[test]
    fn test_zsh_syntax_highlighter_tokens() {
        assert_eq!(ShellSyntaxHighlighter::classify_token(b"grep", true), TokenClass::Command);
        assert_eq!(ShellSyntaxHighlighter::classify_token(b"if", false), TokenClass::Keyword);
        assert_eq!(ShellSyntaxHighlighter::classify_token(b"-rn", false), TokenClass::OptionFlag);
        assert_eq!(ShellSyntaxHighlighter::classify_token(b"$HOME", false), TokenClass::Variable);
        assert_eq!(ShellSyntaxHighlighter::classify_token(b"\"hello\"", false), TokenClass::StringLiteral);
        assert_eq!(ShellSyntaxHighlighter::classify_token(b"# comment", false), TokenClass::Comment);
        assert_eq!(ShellSyntaxHighlighter::classify_token(b"&&", false), TokenClass::Operator);
        assert_eq!(ShellSyntaxHighlighter::classify_token(b"src/", false), TokenClass::Argument);
    }

    #[test]
    fn test_prompt_string_token_expansion_and_auto_cd() {
        let mut shell = SimpleShell::new();
        shell.env.set(b"USER", b"sovereign");
        shell.env.set(b"HOSTNAME", b"sigmaos-box");
        shell.env.set(b"PWD", b"/home/sovereign/code");

        let template = b"%F{cyan}[%n@%m %~]%f %?";
        let expanded = shell.expand_prompt_string(template);
        assert_eq!(expanded, b"\x1b[36m[sovereign@sigmaos-box /home/sovereign/code]\x1b[0m 0");

        // Auto-cd test
        assert!(shell.execute_line(b"/var/log").is_ok());
        assert_eq!(shell.env.get(b"PWD"), Some(b"/var/log" as &[u8]));
    }

    #[test]
    fn test_history_search_and_export_builtins() {
        let mut history = SimpleShellHistory::new();
        history.add_with_timestamp(b"git status", 100);
        history.add_with_timestamp(b"git log -n 5", 101);
        history.add_with_timestamp(b"cargo build", 102);

        let prefix_matches = history.history_search_prefix(b"git");
        assert_eq!(prefix_matches.len(), 2);
        assert_eq!(prefix_matches[0], b"git log -n 5");

        let sub_matches = history.history_search_substring(b"build");
        assert_eq!(sub_matches.len(), 1);
        assert_eq!(sub_matches[0], b"cargo build");

        let mut shell = SimpleShell::new();
        let export_cmd = ExportCommand { shell_ptr: &raw mut shell };
        let mut export_box: Box<dyn ShellCommand> = Box::new(export_cmd);
        export_box.execute(&[b"FOO=bar"]).unwrap();
        assert_eq!(shell.env.get(b"FOO"), Some(b"bar" as &[u8]));

        let unset_cmd = UnsetCommand { shell_ptr: &raw mut shell };
        let mut unset_box: Box<dyn ShellCommand> = Box::new(unset_cmd);
        unset_box.execute(&[b"FOO"]).unwrap();
        assert_eq!(shell.env.get(b"FOO"), None);
    }
}
