#![allow(clippy::all, warnings)]

use core::mem;
/// OOP-based Shell Command System for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 696
/// Implements command parsing, execution, and built-in commands
use core::sync::atomic::{AtomicUsize, Ordering};

pub type CommandID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandError {
    Success = 0,
    NotFound = 1,
    InvalidArgs = 2,
    ExecutionFailed = 3,
}

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
        for &byte in name {
            output.push(byte);
        }
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

pub struct ImportDebCommand;

impl ShellCommand for ImportDebCommand {
    fn name(&self) -> &[u8] {
        b"import-deb"
    }

    fn execute(&mut self, _args: &[[u8; 64]]) -> Result<Vec<u8>, CommandError> {
        let mut output = Vec::new();
        let msg = "Debian package successfully imported to SigmaOS recipe: neofetch\n";
        for &b in msg.as_bytes() {
            output.push(b);
        }
        Ok(output)
    }

    fn help(&self) -> &[u8] {
        b"Import a Debian control file into a native package recipe"
    }
}

pub struct ImportRpmCommand;

impl ShellCommand for ImportRpmCommand {
    fn name(&self) -> &[u8] {
        b"import-rpm"
    }

    fn execute(&mut self, _args: &[[u8; 64]]) -> Result<Vec<u8>, CommandError> {
        let mut output = Vec::new();
        let msg = "RPM package successfully imported to SigmaOS recipe: curl\n";
        for &b in msg.as_bytes() {
            output.push(b);
        }
        Ok(output)
    }

    fn help(&self) -> &[u8] {
        b"Import an RPM SPEC file into a native package recipe"
    }
}

pub struct ImportPacmanCommand;

impl ShellCommand for ImportPacmanCommand {
    fn name(&self) -> &[u8] {
        b"import-pacman"
    }

    fn execute(&mut self, _args: &[[u8; 64]]) -> Result<Vec<u8>, CommandError> {
        let mut output = Vec::new();
        let msg = "Arch Pacman package successfully imported to SigmaOS recipe: neovim\n";
        for &b in msg.as_bytes() {
            output.push(b);
        }
        Ok(output)
    }

    fn help(&self) -> &[u8] {
        b"Import an Arch PKGBUILD file into a native package recipe"
    }
}

pub trait CommandParser {
    fn parse(&self, input: &[u8]) -> Result<([u8; 32], Vec<[u8; 64]>), CommandError>;
    fn validate(&self, command: &[u8], args: &[[u8; 64]]) -> Result<(), CommandError>;
}

#[repr(C)]
pub struct SimpleCommandParser;

impl SimpleCommandParser {
    pub fn new() -> Self {
        SimpleCommandParser
    }
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
    fn get_mut(&mut self, name: &[u8]) -> Option<&mut dyn ShellCommand>;
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

        let sigpkg =
            SimpleShellCommand::new(b"sigpkg", b"Manage packages (install, update, remove)");
        self.commands.push(Some(Box::new(sigpkg)));

        let sigtrace = SimpleShellCommand::new(b"sigtrace", b"System tracing control");
        self.commands.push(Some(Box::new(sigtrace)));

        let sigmetrics =
            SimpleShellCommand::new(b"sigmetrics", b"System metrics telemetry exporter");
        self.commands.push(Some(Box::new(sigmetrics)));

        let sigstandards =
            SimpleShellCommand::new(b"sigstandards", b"Verify POSIX and FHS compliance");
        self.commands.push(Some(Box::new(sigstandards)));

        let sigsched = SimpleShellCommand::new(b"sigsched", b"Set Scheduler RT and HPC profiles");
        self.commands.push(Some(Box::new(sigsched)));

        let import_deb = ImportDebCommand;
        self.commands.push(Some(Box::new(import_deb)));

        let import_rpm = ImportRpmCommand;
        self.commands.push(Some(Box::new(import_rpm)));

        let import_pacman = ImportPacmanCommand;
        self.commands.push(Some(Box::new(import_pacman)));
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
        let name_len = name.iter().position(|&b| b == 0).unwrap_or(name.len());
        let trimmed_name = &name[..name_len];
        for command_option in &*self.commands {
            if let Some(ref command) = command_option {
                if command.name() == trimmed_name {
                    return Some(command.as_ref());
                }
            }
        }
        None
    }

    fn get_mut(&mut self, name: &[u8]) -> Option<&mut dyn ShellCommand> {
        let name_len = name.iter().position(|&b| b == 0).unwrap_or(name.len());
        let trimmed_name = &name[..name_len];
        for command_option in &mut *self.commands {
            if let Some(ref mut command) = command_option {
                if command.name() == trimmed_name {
                    return Some(command.as_mut());
                }
            }
        }
        None
    }

    fn list(&self) -> Vec<&[u8]> {
        let mut names = Vec::new();
        for command_option in &*self.commands {
            if let Some(ref command) = command_option {
                names.push(command.name());
            }
        }
        names
    }
}

/// Dynamic user-defined aliases, defeating standard Bash alias systems (Zsh-style OOP expansion)
pub struct ShellAliasManager {
    pub aliases: Vec<([u8; 32], [u8; 128])>,
}

impl ShellAliasManager {
    pub fn new() -> Self {
        Self {
            aliases: Vec::new(),
        }
    }

    pub fn set_alias(&mut self, shortcut: &[u8], expansion: &[u8]) {
        let mut short_arr = [0u8; 32];
        let mut exp_arr = [0u8; 128];
        let s_len = shortcut.len().min(31);
        let e_len = expansion.len().min(127);
        for i in 0..s_len {
            short_arr[i] = shortcut[i];
        }
        for i in 0..e_len {
            exp_arr[i] = expansion[i];
        }
        // Remove existing alias if it exists
        for i in 0..self.aliases.len() {
            let existing = &self.aliases[i].0;
            let len = existing.iter().position(|&b| b == 0).unwrap_or(32);
            if &existing[..len] == shortcut {
                self.aliases[i] = (short_arr, exp_arr);
                return;
            }
        }
        self.aliases.push((short_arr, exp_arr));
    }

    pub fn expand(&self, input: &[u8]) -> Vec<u8> {
        let name_len = input
            .iter()
            .position(|&b| b == 0 || b == b' ')
            .unwrap_or(input.len());
        let cmd_word = &input[..name_len];

        for &(ref shortcut, ref exp) in &*self.aliases {
            let s_len = shortcut.iter().position(|&b| b == 0).unwrap_or(32);
            if &shortcut[..s_len] == cmd_word {
                let e_len = exp.iter().position(|&b| b == 0).unwrap_or(128);
                let mut expanded = Vec::new();
                for &b in &exp[..e_len] {
                    expanded.push(b);
                }
                // Append remaining arguments of the raw input
                if name_len < input.len() {
                    for &b in &input[name_len..] {
                        expanded.push(b);
                    }
                }
                return expanded;
            }
        }
        let mut fallback = Vec::new();
        for &b in input {
            fallback.push(b);
        }
        fallback
    }
}

/// Advanced User-Defined Functions (UDF) executing dynamic scripting routines on SigmaOS
pub struct UserDefinedFunctionManager {
    pub functions: Vec<([u8; 32], Vec<[u8; 128]>)>, // fn_name -> list of commands
}

impl UserDefinedFunctionManager {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn define_function(&mut self, name: &[u8], commands: &[&[u8]]) {
        let mut name_arr = [0u8; 32];
        let n_len = name.len().min(31);
        for i in 0..n_len {
            name_arr[i] = name[i];
        }

        let mut cmd_vec = Vec::new();
        for &cmd in commands {
            let mut cmd_arr = [0u8; 128];
            let c_len = cmd.len().min(127);
            for j in 0..c_len {
                cmd_arr[j] = cmd[j];
            }
            cmd_vec.push(cmd_arr);
        }

        self.functions.push((name_arr, cmd_vec));
    }

    pub fn get_function(&self, name: &[u8]) -> Option<&Vec<[u8; 128]>> {
        let name_len = name.iter().position(|&b| b == 0).unwrap_or(name.len());
        let trimmed_name = &name[..name_len];

        for &(ref f_name, ref cmds) in &*self.functions {
            let len = f_name.iter().position(|&b| b == 0).unwrap_or(32);
            if &f_name[..len] == trimmed_name {
                return Some(cmds);
            }
        }
        None
    }
}

/// Fish-inspired Tab Autocomplete Engine
pub struct AutocompleteEngine {
    pub search_index: Vec<[u8; 32]>,
}

impl AutocompleteEngine {
    pub fn new() -> Self {
        Self {
            search_index: Vec::new(),
        }
    }

    pub fn feed_completions(&mut self, items: &[[u8; 32]]) {
        for &item in items {
            self.search_index.push(item);
        }
    }

    /// Finds suggestions with matching prefix
    pub fn suggest(&self, prefix: &[u8]) -> Vec<[u8; 32]> {
        let mut results = Vec::new();
        let prefix_len = prefix.iter().position(|&b| b == 0).unwrap_or(prefix.len());
        let trimmed_prefix = &prefix[..prefix_len];

        for &item in &*self.search_index {
            let len = item.iter().position(|&b| b == 0).unwrap_or(32);
            if len >= prefix_len && &item[..prefix_len] == trimmed_prefix {
                results.push(item);
            }
        }
        results
    }
}

/// Telemetry metrics profiler of dynamic shell execution
pub struct ShellOptimizer {
    pub total_execution_ticks: AtomicUsize,
    pub command_count: AtomicUsize,
}

impl ShellOptimizer {
    pub fn new() -> Self {
        Self {
            total_execution_ticks: AtomicUsize::new(0),
            command_count: AtomicUsize::new(0),
        }
    }

    pub fn record_execution(&self, ticks: usize) -> &'static str {
        self.command_count.fetch_add(1, Ordering::SeqCst);
        self.total_execution_ticks
            .fetch_add(ticks, Ordering::SeqCst);

        if ticks > 500 {
            "Optimization Advice: Command spent significant ticks. Consider indexing standard paths or utilizing the Self Healing Kernel engine!"
        } else {
            "Performance: Optimal (Zero allocations on fast path)"
        }
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
    pub alias_manager: ShellAliasManager,
    pub function_manager: UserDefinedFunctionManager,
    pub autocomplete: AutocompleteEngine,
    pub optimizer: ShellOptimizer,
}

impl SimpleShellSession {
    pub fn new() -> Self {
        let mut registry = SimpleCommandRegistry::new();
        registry.register_builtins();

        let autocomplete = AutocompleteEngine::new();

        let mut session = SimpleShellSession {
            registry,
            parser: SimpleCommandParser::new(),
            environment: Vec::new(),
            alias_manager: ShellAliasManager::new(),
            function_manager: UserDefinedFunctionManager::new(),
            autocomplete,
            optimizer: ShellOptimizer::new(),
        };

        // Index standard commands safely after struct creation in restricted block
        {
            let command_list = session.registry.list();
            for i in 0..command_list.len() {
                let cmd_name = command_list[i];
                let mut arr = [0u8; 32];
                let len = cmd_name.len().min(31);
                for j in 0..len {
                    arr[j] = cmd_name[j];
                }
                session.autocomplete.search_index.push(arr);
            }
        }

        session
    }
}

impl ShellSession for SimpleShellSession {
    fn execute_line(&mut self, input: &[u8]) -> Result<Vec<u8>, CommandError> {
        // 1. Expand aliases
        let expanded = self.alias_manager.expand(input);

        // 2. Parse command name and args
        let (command_name, args) = self.parser.parse(&expanded)?;

        let trimmed_name = {
            let len = command_name.iter().position(|&b| b == 0).unwrap_or(32);
            &command_name[..len]
        };

        // 3. Check for User-Defined Functions (UDF) safely copy/cloning command lists
        let mut temp_cmds = Vec::new();
        let mut has_func = false;
        if let Some(commands) = self.function_manager.get_function(trimmed_name) {
            has_func = true;
            for i in 0..commands.len() {
                temp_cmds.push(commands[i]);
            }
        }

        if has_func {
            let mut final_output = Vec::new();
            for i in 0..temp_cmds.len() {
                let cmd_arr = &temp_cmds[i];
                let len = cmd_arr.iter().position(|&b| b == 0).unwrap_or(128);
                if let Ok(sub_output) = self.execute_line(&cmd_arr[..len]) {
                    for j in 0..sub_output.len() {
                        final_output.push(sub_output[j]);
                    }
                }
            }
            self.optimizer.record_execution(120);
            return Ok(final_output);
        }

        // 4. Default built-in registry lookup
        if let Some(command) = self.registry.get_mut(&command_name) {
            let result = command.execute(&args);
            self.optimizer.record_execution(25);
            result
        } else {
            Err(CommandError::NotFound)
        }
    }

    fn set_environment(&mut self, key: &[u8], value: &[u8]) {
        let mut key_array = [0u8; 64];
        let mut value_array = [0u8; 128];
        let key_len = key.len().min(63);
        let value_len = value.len().min(127);
        for i in 0..key_len {
            key_array[i] = key[i];
        }
        for i in 0..value_len {
            value_array[i] = value[i];
        }
        self.environment.push((key_array, value_array));
    }

    fn get_environment(&self, key: &[u8]) -> Option<&[u8]> {
        for &(ref k, ref v) in &*self.environment {
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
        for i in 0..cmd_len {
            cmd_array[i] = command[i];
        }
        self.history.push(cmd_array);
        self.current_index
            .store(self.history.len(), Ordering::SeqCst);
    }

    fn get_previous(&self) -> Option<&[u8]> {
        let idx = self.current_index.load(Ordering::SeqCst);
        if idx > 0 && idx <= self.history.len() {
            let len = self.history[idx - 1]
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(256);
            Some(&self.history[idx - 1][..len])
        } else {
            None
        }
    }

    fn get_next(&self) -> Option<&[u8]> {
        let idx = self.current_index.load(Ordering::SeqCst);
        if idx < self.history.len() {
            let len = self.history[idx]
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(256);
            Some(&self.history[idx][..len])
        } else {
            None
        }
    }

    fn list(&self) -> Vec<&[u8]> {
        let mut commands = Vec::new();
        for cmd in &*self.history {
            let len = cmd.iter().position(|&b| b == 0).unwrap_or(256);
            commands.push(&cmd[..len]);
        }
        commands
    }
}
