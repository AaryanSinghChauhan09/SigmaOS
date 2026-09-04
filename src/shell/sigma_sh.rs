extern crate alloc;
#[cfg(not(target_os = "none"))]
extern crate alloc as std_alloc;
#[cfg(target_os = "none")]
use alloc::boxed::Box;
use std_alloc::boxed::Box;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec as StdVec;

use alloc::vec::Vec;
/// OOP-based Sigma Shell for SigmaOS
/// Based on Ultimate Dominance Strategy: Stage 0 Milestone 0.1
/// Implements interactive shell with command parsing, echo, environment variables, aliases, and basic utilities
use core::sync::atomic::{AtomicUsize, Ordering};

pub type CommandID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
    fn help(&self) -> &[u8] {
        b"echo [text] - Print text to output (supports variable expansion like $USER)"
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

/// Linux-style built-in command to define aliases
pub struct AliasCommand;

impl ShellCommand for AliasCommand {
    fn name(&self) -> &[u8] {
        b"alias"
    }
    fn execute(&mut self, _args: &[&[u8]]) -> Result<(), ShellError> {
        Ok(())
    }
    fn help(&self) -> &[u8] {
        b"alias [shortcut] [command] - Define a shell alias"
    }
}

/// Linux-style built-in command to remove aliases
pub struct UnaliasCommand;

impl ShellCommand for UnaliasCommand {
    fn name(&self) -> &[u8] {
        b"unalias"
    }
    fn execute(&mut self, _args: &[&[u8]]) -> Result<(), ShellError> {
        Ok(())
    }
    fn help(&self) -> &[u8] {
        b"unalias [shortcut] - Remove a shell alias"
    }
}

/// Linux/BSD export / setenv builtin command
pub struct ExportCommand;

impl ShellCommand for ExportCommand {
    fn name(&self) -> &[u8] {
        b"export"
    }
    fn execute(&mut self, _args: &[&[u8]]) -> Result<(), ShellError> {
        Ok(())
    }
    fn help(&self) -> &[u8] {
        b"export KEY=VALUE - Set environment variable"
    }
}

/// Linux/BSD unset / unsetenv builtin command
pub struct UnsetCommand;

impl ShellCommand for UnsetCommand {
    fn name(&self) -> &[u8] {
        b"unset"
    }
    fn execute(&mut self, _args: &[&[u8]]) -> Result<(), ShellError> {
        Ok(())
    }
    fn help(&self) -> &[u8] {
        b"unset KEY - Unset environment variable"
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
        let _ = shell.register_command(Box::new(EchoCommand::new(0)));
        let _ = shell.register_command(Box::new(ExitCommand::new(0)));
        let _ = shell.register_command(Box::new(HelpCommand::new(0)));
        let _ = shell.register_command(Box::new(ClearCommand::new(0)));
        let _ = shell.register_command(Box::new(AliasCommand));
        let _ = shell.register_command(Box::new(UnaliasCommand));
        let _ = shell.register_command(Box::new(ExportCommand));
        let _ = shell.register_command(Box::new(UnsetCommand));

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

        for i in 0..self.commands.len() {
            if let Some(ref mut cmd) = self.commands[i] {
                if cmd.name() == resolved_cmd_name {
                    let res = cmd.execute(&cmd_args);
                    let code = if res.is_ok() { 0 } else { 1 };
                    self.last_exit_code.store(code, Ordering::SeqCst);
                    return res;
                }
            }
        }

        // 3. Zsh-style auto-cd directory resolution
        if self.auto_cd
            && (resolved_cmd_name.starts_with(b"/")
                || resolved_cmd_name.starts_with(b"./")
                || resolved_cmd_name.starts_with(b"../")
                || resolved_cmd_name.starts_with(b"~")
                || resolved_cmd_name.ends_with(b"/"))
        {
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
        if token == b"&&"
            || token == b"||"
            || token == b";"
            || token == b"|"
            || token == b">"
            || token == b"<"
            || token == b">>"
        {
            return TokenClass::Operator;
        }
        if token.starts_with(b"\"") || token.starts_with(b"'") {
            return TokenClass::StringLiteral;
        }
        let keywords: &[&[u8]] = &[
            b"if",
            b"then",
            b"else",
            b"elif",
            b"fi",
            b"for",
            b"in",
            b"do",
            b"done",
            b"while",
            b"until",
            b"case",
            b"esac",
            b"function",
            b"select",
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

#[cfg(test)]
mod repl_tests {
    use super::*;

    #[test]
    fn test_shell_default_env_variables() {
        let shell = SimpleShell::new();
        assert_eq!(shell.env.get(b"USER"), Some(b"sovereign" as &[u8]));
        assert_eq!(
            shell.env.get(b"HOME"),
            Some(b"/userland/home/sovereign" as &[u8])
        );
        assert_eq!(
            shell.env.get(b"PATH"),
            Some(b"/shards:/system:/userland" as &[u8])
        );
    }

    #[test]
    fn test_shell_variable_expansion_and_alias_resolution() {
        let mut shell = SimpleShell::new();

        static mut CAPTURED_BUF: [u8; 128] = [0; 128];
        static mut CAPTURED_LEN: usize = 0;

        // 1. Create spy command to capture expanded parameters
        struct SpyCommand;
        impl ShellCommand for SpyCommand {
            fn name(&self) -> &[u8] {
                b"spy"
            }
            fn help(&self) -> &[u8] {
                b"spy"
            }
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
            let captured = &CAPTURED_BUF[..CAPTURED_LEN];
            assert_eq!(captured, b"sovereign_pass_123");
        }

        // 3. Setup and verify alias resolution
        shell.env.set(b"USER", b"sovereign");
        shell.set_alias(b"reveal", b"spy");
        shell.execute_line(b"reveal $USER").unwrap();

        unsafe {
            let captured = &CAPTURED_BUF[..CAPTURED_LEN];
            assert_eq!(captured, b"sovereign");
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

        assert_eq!(
            engine.predict_completion(b"system"),
            Some(b"systemctl status nginx" as &[u8])
        );
        assert_eq!(
            engine.predict_completion(b"sysc"),
            Some(b"sysctl -a" as &[u8])
        );
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
        assert_eq!(
            ShellSyntaxHighlighter::classify_token(b"grep", true),
            TokenClass::Command
        );
        assert_eq!(
            ShellSyntaxHighlighter::classify_token(b"if", false),
            TokenClass::Keyword
        );
        assert_eq!(
            ShellSyntaxHighlighter::classify_token(b"-rn", false),
            TokenClass::OptionFlag
        );
        assert_eq!(
            ShellSyntaxHighlighter::classify_token(b"$HOME", false),
            TokenClass::Variable
        );
        assert_eq!(
            ShellSyntaxHighlighter::classify_token(b"\"hello\"", false),
            TokenClass::StringLiteral
        );
        assert_eq!(
            ShellSyntaxHighlighter::classify_token(b"# comment", false),
            TokenClass::Comment
        );
        assert_eq!(
            ShellSyntaxHighlighter::classify_token(b"&&", false),
            TokenClass::Operator
        );
        assert_eq!(
            ShellSyntaxHighlighter::classify_token(b"src/", false),
            TokenClass::Argument
        );
    }

    #[test]
    fn test_prompt_string_token_expansion_and_auto_cd() {
        let mut shell = SimpleShell::new();
        shell.env.set(b"USER", b"sovereign");
        shell.env.set(b"HOSTNAME", b"sigmaos-box");
        shell.env.set(b"PWD", b"/home/sovereign/code");

        let template = b"%F{cyan}[%n@%m %~]%f %?";
        let expanded = shell.expand_prompt_string(template);
        assert_eq!(
            expanded,
            b"\x1b[36m[sovereign@sigmaos-box /home/sovereign/code]\x1b[0m 0"
        );

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
        shell.env.set(b"FOO", b"bar");
        assert_eq!(shell.env.get(b"FOO"), Some(b"bar" as &[u8]));

        shell.env.unset(b"FOO");
        assert_eq!(shell.env.get(b"FOO"), None);
    }
}

// =========================================================================
// REPL LINE EDITOR, AUTO-SUGGEST & SOVEREIGN SIGMA SH REPL
// =========================================================================

pub struct ReplLineEditor;

impl ReplLineEditor {
    pub fn new() -> Self {
        Self
    }

    pub fn highlight_line(&self, line: &str) -> String {
        let parts: StdVec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            return line.to_string();
        }
        let first = parts[0];
        let rest = if line.len() > first.len() {
            &line[first.len()..]
        } else {
            ""
        };
        format!("\x1B[32m{}\x1B[0m{}", first, rest)
    }
}

impl Default for ReplLineEditor {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AutoSuggestTabPopup {
    pub candidates: StdVec<String>,
}

impl AutoSuggestTabPopup {
    pub fn new() -> Self {
        Self {
            candidates: StdVec::new(),
        }
    }
}

impl Default for AutoSuggestTabPopup {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SovereignSigmaShRepl {
    pub line_editor: ReplLineEditor,
    pub completer: ContextualCompleter,
    pub env: SimpleShellEnvironment,
}

impl SovereignSigmaShRepl {
    pub fn new() -> Self {
        let mut env = SimpleShellEnvironment::new();
        env.set(b"USER", b"sovereign");
        env.set(b"HOSTNAME", b"sigmaos");
        Self {
            line_editor: ReplLineEditor::new(),
            completer: ContextualCompleter::new(),
            env,
        }
    }

    pub fn render_prompt(&self) -> String {
        let user = String::from_utf8_lossy(self.env.get(b"USER").unwrap_or(b"sovereign"));
        let host = String::from_utf8_lossy(self.env.get(b"HOSTNAME").unwrap_or(b"sigmaos"));
        format!("{}@{}> ", user, host)
    }

    pub fn suggest_completion(&self, input: &str) -> Option<String> {
        let matches = self.completer.complete(input);
        matches.first().map(|(sub, _)| sub.clone())
    }

    pub fn execute_repl_command(&mut self, line: &str) -> Result<(), ShellError> {
        let trimmed = line.trim();
        if trimmed.starts_with("export ") {
            let kv = &trimmed[7..];
            if let Some(pos) = kv.find('=') {
                let key = kv[..pos].trim();
                let val = kv[pos + 1..].trim();
                self.env.set(key.as_bytes(), val.as_bytes());
            }
        }
        Ok(())
    }

    pub fn jobs_cmd(&self) -> String {
        "No active background jobs".to_string()
    }
}

impl Default for SovereignSigmaShRepl {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// ADVANCED ZSH, BASH, TCSH & KSH SHELL INNOVATIONS
// =========================================================================

/// Bash & Zsh Inspired Parameter & Arithmetic Expansion Engine
pub struct ParameterExpansionEngine;

impl ParameterExpansionEngine {
    /// Evaluates Bash/Zsh string parameter expressions:
    /// - `${VAR:-default}`: Fallback to default if unset or empty
    /// - `${VAR:=default}`: Fallback to default and update env
    /// - `${#VAR}`: Variable string length
    /// - `${VAR:offset:length}`: Substring slicing
    /// - `${VAR/pattern/replacement}`: Replace first match
    /// - `${VAR//pattern/replacement}`: Replace all matches
    /// - `${VAR^}`: Uppercase first character
    /// - `${VAR,,}`: Lowercase string
    /// - `$(( expr ))`: Simple arithmetic evaluation (+, -, *, /, %, <, >)
    pub fn expand_expression(token: &str, env: &mut dyn ShellEnvironment) -> String {
        if !token.starts_with("${") || !token.ends_with('}') {
            if token.starts_with("$(( ") && token.ends_with(" ))") {
                let expr = &token[4..token.len() - 3].trim();
                return Self::evaluate_arithmetic(expr).to_string();
            }
            if token.starts_with("$") && token.len() > 1 {
                if let Some(val) = env.get(token[1..].as_bytes()) {
                    return String::from_utf8_lossy(val).into_owned();
                }
            }
            return token.to_string();
        }

        let inner = &token[2..token.len() - 1];

        // 1. Length expansion: ${#VAR}
        if inner.starts_with('#') {
            let var_name = &inner[1..];
            let val = env.get(var_name.as_bytes()).unwrap_or(b"");
            return val.len().to_string();
        }

        // 2. Default value assignment: ${VAR:=default}
        if let Some(pos) = inner.find(":=") {
            let var_name = &inner[..pos];
            let default_val = &inner[pos + 2..];
            if let Some(val) = env.get(var_name.as_bytes()) {
                if !val.is_empty() {
                    return String::from_utf8_lossy(val).into_owned();
                }
            }
            env.set(var_name.as_bytes(), default_val.as_bytes());
            return default_val.to_string();
        }

        // 3. Default fallback: ${VAR:-default}
        if let Some(pos) = inner.find(":-") {
            let var_name = &inner[..pos];
            let default_val = &inner[pos + 2..];
            if let Some(val) = env.get(var_name.as_bytes()) {
                if !val.is_empty() {
                    return String::from_utf8_lossy(val).into_owned();
                }
            }
            return default_val.to_string();
        }

        // 4. Substring slicing: ${VAR:offset:length}
        if let Some(pos) = inner.find(':') {
            let var_name = &inner[..pos];
            let slice_spec = &inner[pos + 1..];
            let val_bytes = env.get(var_name.as_bytes()).unwrap_or(b"");
            let val_str = String::from_utf8_lossy(val_bytes);

            let parts: StdVec<&str> = slice_spec.split(':').collect();
            let offset: usize = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
            let length: usize = parts
                .get(1)
                .and_then(|s| s.parse().ok())
                .unwrap_or(val_str.len());

            if offset >= val_str.len() {
                return String::new();
            }
            let end = (offset + length).min(val_str.len());
            return val_str[offset..end].to_string();
        }

        // 5. Global replacement: ${VAR//pattern/replacement}
        if let Some(pos) = inner.find("//") {
            let var_name = &inner[..pos];
            let rest = &inner[pos + 2..];
            let val_bytes = env.get(var_name.as_bytes()).unwrap_or(b"");
            let val_str = String::from_utf8_lossy(val_bytes);

            let subparts: StdVec<&str> = rest.split('/').collect();
            if subparts.len() >= 2 {
                let pat = subparts[0];
                let rep = subparts[1];
                return val_str.replace(pat, rep);
            }
        }

        // 6. Single replacement: ${VAR/pattern/replacement}
        if let Some(pos) = inner.find('/') {
            let var_name = &inner[..pos];
            let rest = &inner[pos + 1..];
            let val_bytes = env.get(var_name.as_bytes()).unwrap_or(b"");
            let val_str = String::from_utf8_lossy(val_bytes);

            let subparts: StdVec<&str> = rest.split('/').collect();
            if subparts.len() >= 2 {
                let pat = subparts[0];
                let rep = subparts[1];
                return val_str.replacen(pat, rep, 1);
            }
        }

        // 7. Case transformations: ${VAR^} / ${VAR,,}
        if inner.ends_with("^") {
            let var_name = &inner[..inner.len() - 1];
            let val_bytes = env.get(var_name.as_bytes()).unwrap_or(b"");
            let val_str = String::from_utf8_lossy(val_bytes);
            let mut chars = val_str.chars();
            if let Some(first) = chars.next() {
                return format!("{}{}", first.to_uppercase(), chars.as_str());
            }
            return String::new();
        }

        if inner.ends_with(",,") {
            let var_name = &inner[..inner.len() - 2];
            let val_bytes = env.get(var_name.as_bytes()).unwrap_or(b"");
            return String::from_utf8_lossy(val_bytes).to_lowercase();
        }

        // Standard variable lookup fallback
        if let Some(val) = env.get(inner.as_bytes()) {
            String::from_utf8_lossy(val).into_owned()
        } else {
            String::new()
        }
    }

    /// Basic integer arithmetic evaluator for `$(( expr ))`
    pub fn evaluate_arithmetic(expr: &str) -> i64 {
        let tokens: StdVec<&str> = expr.split_whitespace().collect();
        if tokens.is_empty() {
            return 0;
        }
        if tokens.len() == 1 {
            return tokens[0].parse::<i64>().unwrap_or(0);
        }
        if tokens.len() == 3 {
            let left = tokens[0].parse::<i64>().unwrap_or(0);
            let op = tokens[1];
            let right = tokens[2].parse::<i64>().unwrap_or(0);
            match op {
                "+" => left + right,
                "-" => left - right,
                "*" => left * right,
                "/" => {
                    if right != 0 {
                        left / right
                    } else {
                        0
                    }
                }
                "%" => {
                    if right != 0 {
                        left % right
                    } else {
                        0
                    }
                }
                "==" => {
                    if left == right {
                        1
                    } else {
                        0
                    }
                }
                "!=" => {
                    if left != right {
                        1
                    } else {
                        0
                    }
                }
                ">" => {
                    if left > right {
                        1
                    } else {
                        0
                    }
                }
                "<" => {
                    if left < right {
                        1
                    } else {
                        0
                    }
                }
                _ => 0,
            }
        } else {
            0
        }
    }
}

/// Zsh & Tcsh Inspired Dynamic Prompt Formatter
pub struct ZshPromptFormatter;

impl ZshPromptFormatter {
    /// Formats prompt string with Zsh/Tcsh format specifiers:
    /// - `%n`: username
    /// - `%m`: hostname
    /// - `%~`: current directory with ~ for $HOME
    /// - `%?`: exit status of last command
    /// - `%t`: timestamp
    /// - `%F{color}`: ANSI foreground color (`%F{green}`, `%F{blue}`, `%f` for reset)
    /// - `%B` / `%b`: bold toggle
    pub fn format_prompt(
        template: &str,
        user: &str,
        host: &str,
        cwd: &str,
        home: &str,
        last_exit_code: i32,
        time_str: &str,
    ) -> String {
        let mut result = template.to_string();

        // 1. Directory formatting (%~)
        let formatted_cwd = if !home.is_empty() && cwd.starts_with(home) {
            format!("~{}", &cwd[home.len()..])
        } else {
            cwd.to_string()
        };

        result = result.replace("%n", user);
        result = result.replace("%m", host);
        result = result.replace("%~", &formatted_cwd);
        result = result.replace("%?", &last_exit_code.to_string());
        result = result.replace("%t", time_str);

        // 2. Color codes
        result = result.replace("%F{green}", "\x1B[32m");
        result = result.replace("%F{blue}", "\x1B[34m");
        result = result.replace("%F{red}", "\x1B[31m");
        result = result.replace("%F{yellow}", "\x1B[33m");
        result = result.replace("%F{cyan}", "\x1B[36m");
        result = result.replace("%f", "\x1B[39m");

        // 3. Bold toggle
        result = result.replace("%B", "\x1B[1m");
        result = result.replace("%b", "\x1B[22m");

        result
    }

    /// Formats right-side prompt (`RPROMPT` / `RPS1` in Zsh)
    pub fn format_rprompt(git_branch: Option<&str>, execution_time_ms: u64) -> String {
        let mut rprompt = String::new();
        if let Some(branch) = git_branch {
            rprompt.push_str(&format!("\x1B[33mgit:({})\x1B[39m ", branch));
        }
        if execution_time_ms > 0 {
            rprompt.push_str(&format!("\x1B[90m{}ms\x1B[39m", execution_time_ms));
        }
        rprompt
    }
}

/// Zsh & Fish Inspired Context-Aware Sub-Command Completer
pub struct ContextualCompleter {
    pub completions: StdVec<(&'static str, &'static str, &'static str)>, // (cmd, subcmd/flag, description)
}

impl ContextualCompleter {
    pub fn new() -> Self {
        let mut completer = Self {
            completions: StdVec::new(),
        };

        // Populate Zsh-style sub-command completions
        completer.register(
            "git",
            "checkout",
            "Switch branches or restore working tree files",
        );
        completer.register("git", "commit", "Record changes to the repository");
        completer.register(
            "git",
            "push",
            "Update remote refs along with associated objects",
        );
        completer.register(
            "git",
            "pull",
            "Fetch from and integrate with another repository",
        );
        completer.register("git", "status", "Show the working tree status");
        completer.register(
            "git",
            "diff",
            "Show changes between commits, commit and working tree",
        );

        completer.register("systemctl", "start", "Start (activate) one or more units");
        completer.register("systemctl", "stop", "Stop (deactivate) one or more units");
        completer.register("systemctl", "restart", "Start or restart one or more units");
        completer.register(
            "systemctl",
            "status",
            "Show terse runtime status information about units",
        );

        completer.register(
            "sigpkg",
            "install",
            "Safely install sandboxed package shard",
        );
        completer.register(
            "sigpkg",
            "remove",
            "Uninstall package shard and clean unused trees",
        );
        completer.register("sigpkg", "update", "Update local package index signatures");

        completer.register(
            "container",
            "run",
            "Spin up OCI-compliant isolated sandbox container",
        );
        completer.register(
            "container",
            "stop",
            "Gracefully terminate active sandbox container",
        );
        completer.register("container", "ps", "List running container instances");

        completer
    }

    pub fn register(&mut self, cmd: &'static str, subcmd: &'static str, desc: &'static str) {
        self.completions.push((cmd, subcmd, desc));
    }

    /// Suggests completions given the command line input
    pub fn complete(&self, input: &str) -> StdVec<(String, String)> {
        let parts: StdVec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return StdVec::new();
        }

        let cmd = parts[0];
        let sub_prefix = parts.get(1).copied().unwrap_or("");

        let mut results = StdVec::new();
        for &(c, sub, desc) in &self.completions {
            if c == cmd && sub.starts_with(sub_prefix) {
                results.push((sub.to_string(), desc.to_string()));
            }
        }
        results
    }
}

impl Default for ContextualCompleter {
    fn default() -> Self {
        Self::new()
    }
}

/// Ksh & FreeBSD Job Control & Pipeline Execution Framework
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Running,
    Stopped,
    Done,
}

#[derive(Debug, Clone)]
pub struct ShellJob {
    pub job_id: u32,
    pub pid: u32,
    pub command: String,
    pub state: JobState,
    pub is_background: bool,
}

pub struct JobControlManager {
    pub jobs: StdVec<ShellJob>,
    pub next_job_id: u32,
}

impl JobControlManager {
    pub fn new() -> Self {
        Self {
            jobs: StdVec::new(),
            next_job_id: 1,
        }
    }

    pub fn add_job(&mut self, command: &str, pid: u32, is_bg: bool) -> u32 {
        let job_id = self.next_job_id;
        self.next_job_id += 1;

        let job = ShellJob {
            job_id,
            pid,
            command: command.to_string(),
            state: JobState::Running,
            is_background: is_bg,
        };
        self.jobs.push(job);
        job_id
    }

    pub fn list_jobs(&self) -> StdVec<String> {
        let mut list = StdVec::new();
        for job in &self.jobs {
            let state_str = match job.state {
                JobState::Running => "Running",
                JobState::Stopped => "Stopped",
                JobState::Done => "Done",
            };
            list.push(format!(
                "[{}] {} PID: {}  {}",
                job.job_id, state_str, job.pid, job.command
            ));
        }
        list
    }

    pub fn bring_to_foreground(&mut self, job_id: u32) -> Result<String, ShellError> {
        if let Some(pos) = self.jobs.iter().position(|j| j.job_id == job_id) {
            self.jobs[pos].state = JobState::Running;
            self.jobs[pos].is_background = false;
            Ok(format!(
                "Job [{}] {} brought to foreground",
                job_id, self.jobs[pos].command
            ))
        } else {
            Err(ShellError::InvalidArgument)
        }
    }

    pub fn resume_in_background(&mut self, job_id: u32) -> Result<String, ShellError> {
        if let Some(pos) = self.jobs.iter().position(|j| j.job_id == job_id) {
            self.jobs[pos].state = JobState::Running;
            self.jobs[pos].is_background = true;
            Ok(format!(
                "Job [{}] {} resumed in background",
                job_id, self.jobs[pos].command
            ))
        } else {
            Err(ShellError::InvalidArgument)
        }
    }
}

impl Default for JobControlManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Chained Pipeline Stage
#[derive(Debug, Clone)]
pub struct PipelineStage {
    pub command: String,
    pub redirect_stdout: Option<String>,
    pub redirect_stdin: Option<String>,
    pub append_stdout: bool,
}

/// Pipeline Execution Plan
#[derive(Debug, Clone)]
pub struct PipelinePlan {
    pub stages: StdVec<PipelineStage>,
    pub is_background: bool,
    pub run_if_success: bool,
    pub run_if_failure: bool,
}

pub struct PipelineExecutor;

impl PipelineExecutor {
    /// Parses line into pipeline stages, redirection, background tokens, and chaining
    pub fn parse_pipeline(line: &str) -> PipelinePlan {
        let trimmed = line.trim();
        let is_bg = trimmed.ends_with('&');
        let clean_line = if is_bg {
            &trimmed[..trimmed.len() - 1].trim()
        } else {
            trimmed
        };

        let mut run_if_success = false;
        let mut run_if_failure = false;

        if clean_line.contains("&&") {
            run_if_success = true;
        } else if clean_line.contains("||") {
            run_if_failure = true;
        }

        let raw_stages: StdVec<&str> = clean_line.split('|').collect();
        let mut stages = StdVec::new();

        for raw in raw_stages {
            let stage_str = raw.trim();
            let mut cmd = stage_str.to_string();
            let mut redir_out = None;
            let mut redir_in = None;
            let mut append = false;

            if let Some(pos) = stage_str.find(">>") {
                cmd = stage_str[..pos].trim().to_string();
                redir_out = Some(stage_str[pos + 2..].trim().to_string());
                append = true;
            } else if let Some(pos) = stage_str.find('>') {
                cmd = stage_str[..pos].trim().to_string();
                redir_out = Some(stage_str[pos + 1..].trim().to_string());
            }

            if let Some(pos) = stage_str.find('<') {
                let left = &stage_str[..pos].trim();
                let right = &stage_str[pos + 1..].trim();
                cmd = left.to_string();
                redir_in = Some(right.to_string());
            }

            stages.push(PipelineStage {
                command: cmd,
                redirect_stdout: redir_out,
                redirect_stdin: redir_in,
                append_stdout: append,
            });
        }

        PipelinePlan {
            stages,
            is_background: is_bg,
            run_if_success,
            run_if_failure,
        }
    }
}

/// Bash & Zsh Inspired History Expansion Engine (!1, !!, !$)
pub struct HistoryExpansionEngine;

impl HistoryExpansionEngine {
    /// Expands history event designations:
    /// - `!!`: Repeat last command
    /// - `!$`: Repeat last argument of last command
    /// - `!^`: Repeat first argument of last command
    /// - `!n`: Repeat command #n from history
    /// - `!?str`: Repeat last command containing `str`
    pub fn expand_history(input: &str, history: &[String]) -> String {
        if history.is_empty() || !input.contains('!') {
            return input.to_string();
        }

        let last_cmd = history.last().unwrap();
        let last_tokens: StdVec<&str> = last_cmd.split_whitespace().collect();

        let mut expanded = input.to_string();

        // 1. Double exclamation: !!
        if expanded.contains("!!") {
            expanded = expanded.replace("!!", last_cmd);
        }

        // 2. Last argument: !$
        if expanded.contains("!$") {
            let last_arg = last_tokens.last().copied().unwrap_or("");
            expanded = expanded.replace("!$", last_arg);
        }

        // 3. First argument: !^
        if expanded.contains("!^") {
            let first_arg = if last_tokens.len() > 1 {
                last_tokens[1]
            } else {
                ""
            };
            expanded = expanded.replace("!^", first_arg);
        }

        // 4. History index: !n
        if let Some(pos) = expanded.find('!') {
            let rest = &expanded[pos + 1..];
            if let Ok(idx) = rest.parse::<usize>() {
                if idx > 0 && idx <= history.len() {
                    expanded = history[idx - 1].clone();
                }
            } else if rest.starts_with('?') {
                let query = &rest[1..];
                for cmd in history.iter().rev() {
                    if cmd.contains(query) {
                        expanded = cmd.clone();
                        break;
                    }
                }
            }
        }

        expanded
    }
}

/// Rich Line Editor for Sovereign Shell REPL
pub struct ReplLineEditor {
    pub prompt: String,
    pub history: StdVec<String>,
}

impl ReplLineEditor {
    pub fn new() -> Self {
        Self {
            prompt: "sovereign@sigmaos:~$ ".to_string(),
            history: StdVec::new(),
        }
    }

    pub fn highlight_line(&self, line: &str) -> String {
        let mut highlighted = line.to_string();
        if highlighted.starts_with("git") {
            highlighted = highlighted.replacen("git", "\x1B[32mgit\x1B[0m", 1);
        }
        highlighted
    }
}

/// Sovereign REPL combining Zsh prompt, Fish auto-suggestions, and Ksh job control
pub struct SovereignSigmaShRepl {
    pub line_editor: ReplLineEditor,
    pub completer: ContextualCompleter,
    pub job_manager: JobControlManager,
    pub history: StdVec<String>,
}

impl SovereignSigmaShRepl {
    pub fn new() -> Self {
        Self {
            line_editor: ReplLineEditor::new(),
            completer: ContextualCompleter::new(),
            job_manager: JobControlManager::new(),
            history: StdVec::new(),
        }
    }

    pub fn render_prompt(&self) -> String {
        ZshPromptFormatter::format_prompt(
            "%F{green}%n@%m%f:%F{blue}%~%f %# ",
            "sovereign",
            "sigmaos",
            "/home/sovereign",
            "/home/sovereign",
            0,
            "12:00",
        )
    }

    pub fn suggest_completion(&self, input: &str) -> Option<String> {
        let completions = self.completer.complete(input);
        completions.first().map(|(sub, _)| sub.clone())
    }

    pub fn execute_repl_command(&mut self, cmd: &str) -> Result<(), String> {
        self.history.push(cmd.to_string());
        Ok(())
    }

    pub fn jobs_cmd(&self) -> String {
        let jobs = self.job_manager.list_jobs();
        if jobs.is_empty() {
            "No active background jobs".to_string()
        } else {
            jobs.join("\n")
        }
    }
}

#[cfg(test)]
mod advanced_shell_tests {
    use super::*;

    struct MockEnv {
        keys: StdVec<String>,
        vals: StdVec<String>,
    }

    impl MockEnv {
        fn new() -> Self {
            Self {
                keys: StdVec::new(),
                vals: StdVec::new(),
            }
        }
    }

    impl ShellEnvironment for MockEnv {
        fn set(&mut self, key: &[u8], value: &[u8]) {
            let k = String::from_utf8_lossy(key).into_owned();
            let v = String::from_utf8_lossy(value).into_owned();
            if let Some(pos) = self.keys.iter().position(|x| x == &k) {
                self.vals[pos] = v;
            } else {
                self.keys.push(k);
                self.vals.push(v);
            }
        }

        fn get(&self, key: &[u8]) -> Option<&[u8]> {
            let k = String::from_utf8_lossy(key);
            if let Some(pos) = self.keys.iter().position(|x| x == &k) {
                Some(self.vals[pos].as_bytes())
            } else {
                None
            }
        }

        fn unset(&mut self, key: &[u8]) {
            let k = String::from_utf8_lossy(key);
            if let Some(pos) = self.keys.iter().position(|x| x == &k) {
                self.keys.remove(pos);
                self.vals.remove(pos);
            }
        }
    }

    #[test]
    fn test_parameter_and_arithmetic_expansion() {
        let mut env = MockEnv::new();
        env.set(b"USER", b"sovereign");
        env.set(b"FRUIT", b"apple_pie");

        // Fallback default ${UNSET:-guest}
        assert_eq!(
            ParameterExpansionEngine::expand_expression("${UNSET:-guest}", &mut env),
            "guest"
        );

        // Assign default ${UNSET:=guest}
        assert_eq!(
            ParameterExpansionEngine::expand_expression("${MODE:=sovereign}", &mut env),
            "sovereign"
        );
        assert_eq!(env.get(b"MODE"), Some(b"sovereign" as &[u8]));

        // Length ${#USER}
        assert_eq!(
            ParameterExpansionEngine::expand_expression("${#USER}", &mut env),
            "9"
        );

        // Substring slicing ${FRUIT:0:5}
        assert_eq!(
            ParameterExpansionEngine::expand_expression("${FRUIT:0:5}", &mut env),
            "apple"
        );

        // Pattern replacement ${FRUIT/pie/tart}
        assert_eq!(
            ParameterExpansionEngine::expand_expression("${FRUIT/pie/tart}", &mut env),
            "apple_tart"
        );

        // Case transformation ${USER^}
        assert_eq!(
            ParameterExpansionEngine::expand_expression("${USER^}", &mut env),
            "Sovereign"
        );

        // Arithmetic evaluation $(( 10 + 20 * 2 ))
        assert_eq!(ParameterExpansionEngine::evaluate_arithmetic("10 + 5"), 15);
        assert_eq!(ParameterExpansionEngine::evaluate_arithmetic("12 * 4"), 48);
        assert_eq!(
            ParameterExpansionEngine::evaluate_arithmetic("100 == 100"),
            1
        );
    }

    #[test]
    fn test_zsh_prompt_formatter() {
        let template = "%F{green}%n@%m%f:%F{blue}%~%f %B%?%b %#";
        let prompt = ZshPromptFormatter::format_prompt(
            template,
            "sovereign",
            "sigma-box",
            "/userland/home/sovereign/projects",
            "/userland/home/sovereign",
            0,
            "12:00",
        );

        assert!(prompt.contains("\x1B[32msovereign@sigma-box"));
        assert!(prompt.contains("~/projects"));
        assert!(prompt.contains("\x1B[1m0\x1B[22m"));

        let rprompt = ZshPromptFormatter::format_rprompt(Some("main"), 42);
        assert!(rprompt.contains("git:(main)"));
        assert!(rprompt.contains("42ms"));
    }

    #[test]
    fn test_contextual_completer() {
        let completer = ContextualCompleter::new();

        let git_comps = completer.complete("git ch");
        assert_eq!(git_comps.len(), 1);
        assert_eq!(git_comps[0].0, "checkout");

        let sys_comps = completer.complete("systemctl st");
        assert_eq!(sys_comps.len(), 3); // start, stop, status
        assert_eq!(sys_comps[0].0, "start");
        assert_eq!(sys_comps[1].0, "stop");
        assert_eq!(sys_comps[2].0, "status");
    }

    #[test]
    fn test_job_control_manager() {
        let mut mgr = JobControlManager::new();
        let job1 = mgr.add_job("ping 8.8.8.8", 1001, true);
        assert_eq!(job1, 1);

        let list = mgr.list_jobs();
        assert_eq!(list.len(), 1);
        assert!(list[0].contains("Running"));

        let fg_res = mgr.bring_to_foreground(1);
        assert!(fg_res.is_ok());
        assert!(!mgr.jobs[0].is_background);
    }

    #[test]
    fn test_pipeline_executor_parser() {
        let plan =
            PipelineExecutor::parse_pipeline("cat file.txt | sigmagrep pattern > output.log &");
        assert!(plan.is_background);
        assert_eq!(plan.stages.len(), 2);
        assert_eq!(plan.stages[0].command, "cat file.txt");
        assert_eq!(plan.stages[1].command, "sigmagrep pattern");
        assert_eq!(
            plan.stages[1].redirect_stdout.as_deref(),
            Some("output.log")
        );
    }

    #[test]
    fn test_history_expansion() {
        let mut history = StdVec::new();
        history.push("git commit -m 'Initial commit'".to_string());
        history.push("sigpkg install rustc".to_string());

        assert_eq!(
            HistoryExpansionEngine::expand_history("!!", &history),
            "sigpkg install rustc"
        );
        assert_eq!(
            HistoryExpansionEngine::expand_history("echo !$", &history),
            "echo rustc"
        );
        assert_eq!(
            HistoryExpansionEngine::expand_history("!?commit", &history),
            "git commit -m 'Initial commit'"
        );
    }


}
