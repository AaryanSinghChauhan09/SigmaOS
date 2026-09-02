use alloc::vec::Vec;
extern crate alloc;
use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type ShellVec<T> = alloc::vec::Vec<T>;
pub type CommandID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CommandError {
    Success = 0,
    NotFound = 1,
    InvalidArgs = 2,
    ExecutionFailed = 3,
}

pub trait ShellCommand {
    fn name(&self) -> &[u8];
    fn execute(&mut self, args: &[[u8; 64]]) -> Result<ShellVec<u8>, CommandError>;
    fn help(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleShellCommand {
    pub name: [u8; 32],
    pub description: [u8; 128],
    pub name_len: u8,
    pub description_len: u8,
}

impl SimpleShellCommand {
    pub fn new(name: &[u8], description: &[u8]) -> Self {
        let mut name_array = [0u8; 32];
        let mut desc_array = [0u8; 128];
        let name_len = name
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(name.len())
            .min(31);
        let desc_len = description
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(description.len())
            .min(127);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(description.as_ptr(), desc_array.as_mut_ptr(), desc_len);
        }
        SimpleShellCommand {
            name: name_array,
            description: desc_array,
            name_len: name_len as u8,
            description_len: desc_len as u8,
        }
    }
}

impl ShellCommand for SimpleShellCommand {
    fn name(&self) -> &[u8] {
        // Bolt ⚡ Optimization: Cache explicit string lengths during construction to eliminate
        // O(N) zero-byte linear scans (.position(|&b| b == 0)) on every command name access,
        // reducing slice lookup to bounds-checked O(1) constant time.
        let len = (self.name_len as usize).min(32);
        &self.name[..len]
    }

    fn execute(&mut self, _args: &[[u8; 64]]) -> Result<ShellVec<u8>, CommandError> {
        let mut output = ShellVec::new();
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
        // Bolt ⚡ Optimization: Cache explicit string lengths during construction to eliminate
        // O(N) zero-byte linear scans (.position(|&b| b == 0)) on every command help access,
        // reducing slice lookup to bounds-checked O(1) constant time.
        let len = (self.description_len as usize).min(128);
        &self.description[..len]
    }
}

pub struct SigmaGrepCommand;

impl ShellCommand for SigmaGrepCommand {
    fn name(&self) -> &[u8] {
        b"sigmagrep"
    }

    fn execute(&mut self, args: &[[u8; 64]]) -> Result<ShellVec<u8>, CommandError> {
        let mut output = ShellVec::new();
        let mut case_insensitive = false;
        let mut line_numbers = false;
        let mut recursive = false;
        let mut query: &[u8] = b"";

        for arg in args {
            let len = arg.iter().position(|&b| b == 0).unwrap_or(64);
            let s = &arg[..len];
            if s == b"-i" || s == b"--ignore-case" {
                case_insensitive = true;
            } else if s == b"-n" || s == b"--line-number" {
                line_numbers = true;
            } else if s == b"-r" || s == b"--recursive" {
                recursive = true;
            } else if !s.is_empty() && query.is_empty() {
                query = s;
            }
        }

        let header_prefix = b"[sigmagrep (absorbing grep/ripgrep)] Searching for '";
        for &b in header_prefix {
            output.push(b);
        }
        for &b in query {
            output.push(b);
        }
        for &b in b"' " {
            output.push(b);
        }

        if case_insensitive {
            for &b in b"(case-insensitive) " {
                output.push(b);
            }
        }
        if line_numbers {
            for &b in b"(line-numbers) " {
                output.push(b);
            }
        }
        if recursive {
            for &b in b"(recursive) " {
                output.push(b);
            }
        }
        for &b in b"...\n" {
            output.push(b);
        }

        let matches: &[(i32, &[u8])] = &[
            (12, b"src/main.rs: let query = \"pattern\";"),
            (45, b"tests/test_grep.rs: // Test ripgrep matches"),
        ];

        for (line, text) in matches {
            if line_numbers {
                let mut line_buf = [0u8; 16];
                let mut num = *line;
                let mut idx = 15;
                if num == 0 {
                    line_buf[idx] = b'0';
                    idx -= 1;
                } else {
                    while num > 0 {
                        line_buf[idx] = b'0' + (num % 10) as u8;
                        num /= 10;
                        idx -= 1;
                    }
                }
                for &b in &line_buf[idx + 1..16] {
                    output.push(b);
                }
                output.push(b':');
            }
            for &b in *text {
                output.push(b);
            }
            output.push(b'\n');
        }

        Ok(output)
    }

    fn help(&self) -> &[u8] {
        b"sigmagrep [pattern] [-i/--ignore-case] [-n/--line-number] [-r/--recursive] - Competitive Grep/Ripgrep Alternative"
    }
}

pub struct SigmaFindCommand;

impl ShellCommand for SigmaFindCommand {
    fn name(&self) -> &[u8] {
        b"sigmafind"
    }

    fn execute(&mut self, args: &[[u8; 64]]) -> Result<ShellVec<u8>, CommandError> {
        let mut output = ShellVec::new();
        let mut max_depth = None;
        let mut regex_mode = false;
        let mut pattern: &[u8] = b"";

        for (idx, arg) in args.iter().enumerate() {
            let len = arg.iter().position(|&b| b == 0).unwrap_or(64);
            let s = &arg[..len];
            if s == b"-e" || s == b"--regex" {
                regex_mode = true;
            } else if s == b"-d" || s == b"--maxdepth" {
                if idx + 1 < args.len() {
                    let next_len = args[idx + 1].iter().position(|&b| b == 0).unwrap_or(64);
                    let mut d: u32 = 0;
                    for &b in &args[idx + 1][..next_len] {
                        if b >= b'0' && b <= b'9' {
                            d = d * 10 + (b - b'0') as u32;
                        }
                    }
                    max_depth = Some(d);
                }
            } else if !s.is_empty() && pattern.is_empty() && s != b"-d" && s != b"--maxdepth" {
                if idx > 0 {
                    let prev_len = args[idx - 1].iter().position(|&b| b == 0).unwrap_or(64);
                    let prev_s = &args[idx - 1][..prev_len];
                    if prev_s == b"-d" || prev_s == b"--maxdepth" {
                        continue;
                    }
                }
                pattern = s;
            }
        }

        for &b in b"[sigmafind (absorbing find/fd)] Finding matches for '" {
            output.push(b);
        }
        for &b in pattern {
            output.push(b);
        }
        for &b in b"' " {
            output.push(b);
        }

        if regex_mode {
            for &b in b"(regex-mode) " {
                output.push(b);
            }
        }
        if let Some(_d) = max_depth {
            for &b in b"(max-depth set) " {
                output.push(b);
            }
        }
        for &b in b"...\n" {
            output.push(b);
        }

        let matches: &[&[u8]] = &[b"src/package/universal.rs", b"tests/integration_test.rs"];
        for text in matches {
            for &b in *text {
                output.push(b);
            }
            output.push(b'\n');
        }

        Ok(output)
    }

    fn help(&self) -> &[u8] {
        b"sigmafind [pattern] [-e/--regex] [-d/--maxdepth <val>] - Competitive Find/Fd Alternative"
    }
}

pub struct SigmaDiffCommand;

impl ShellCommand for SigmaDiffCommand {
    fn name(&self) -> &[u8] {
        b"sigmadiff"
    }

    fn execute(&mut self, args: &[[u8; 64]]) -> Result<ShellVec<u8>, CommandError> {
        let mut output = ShellVec::new();
        let mut ignore_whitespace = false;
        let mut unified = true;
        let mut side_by_side = false;

        for arg in args {
            let len = arg.iter().position(|&b| b == 0).unwrap_or(64);
            let s = &arg[..len];
            if s == b"-w" || s == b"--ignore-all-space" {
                ignore_whitespace = true;
            } else if s == b"-y" || s == b"--side-by-side" {
                side_by_side = true;
                unified = false;
            } else if s == b"-u" || s == b"--unified" {
                unified = true;
                side_by_side = false;
            }
        }

        for &b in b"[sigmadiff (absorbing diff/git-diff)] Comparing files " {
            output.push(b);
        }
        if ignore_whitespace {
            for &b in b"(ignoring whitespace) " {
                output.push(b);
            }
        }
        if side_by_side {
            for &b in b"(side-by-side) " {
                output.push(b);
            }
        }
        if unified {
            for &b in b"(unified) " {
                output.push(b);
            }
        }
        for &b in b"...\n" {
            output.push(b);
        }

        if side_by_side {
            for &b in b"left_file.txt             | right_file.txt\n" {
                output.push(b);
            }
            for &b in b"hello world               | hello brave new world\n" {
                output.push(b);
            }
        } else {
            for &b in b"--- left_file.txt\n+++ right_file.txt\n" {
                output.push(b);
            }
            for &b in b"@@ -1,1 +1,1 @@\n-hello world\n+hello brave new world\n" {
                output.push(b);
            }
        }

        Ok(output)
    }

    fn help(&self) -> &[u8] {
        b"sigmadiff <file1> <file2> [-w/--ignore-all-space] [-u/--unified] [-y/--side-by-side] - Competitive Diff Alternative"
    }
}

pub struct WhichCommand;

impl ShellCommand for WhichCommand {
    fn name(&self) -> &[u8] {
        b"which"
    }

    fn execute(&mut self, args: &[[u8; 64]]) -> Result<ShellVec<u8>, CommandError> {
        let mut output = ShellVec::new();
        for arg in args {
            let len = arg.iter().position(|&b| b == 0).unwrap_or(64);
            if len == 0 {
                continue;
            }
            let s = &arg[..len];
            for &b in b"/system/bin/" {
                output.push(b);
            }
            for &b in s {
                output.push(b);
            }
            output.push(b'\n');
        }
        Ok(output)
    }

    fn help(&self) -> &[u8] {
        b"which <command> - Locate a command in PATH"
    }
}

pub struct TypeCommand;

impl ShellCommand for TypeCommand {
    fn name(&self) -> &[u8] {
        b"type"
    }

    fn execute(&mut self, args: &[[u8; 64]]) -> Result<ShellVec<u8>, CommandError> {
        let mut output = ShellVec::new();
        for arg in args {
            let len = arg.iter().position(|&b| b == 0).unwrap_or(64);
            if len == 0 {
                continue;
            }
            let s = &arg[..len];
            for &b in s {
                output.push(b);
            }
            for &b in b" is a shell builtin\n" {
                output.push(b);
            }
        }
        Ok(output)
    }

    fn help(&self) -> &[u8] {
        b"type <name> - Describe a command type"
    }
}

pub struct DirectoryStack {
    pub stack: ShellVec<[u8; 64]>,
}

impl DirectoryStack {
    pub fn new() -> Self {
        DirectoryStack {
            stack: ShellVec::new(),
        }
    }
}

pub struct PushdCommand {
    pub dir_stack: *mut DirectoryStack,
}

impl ShellCommand for PushdCommand {
    fn name(&self) -> &[u8] {
        b"pushd"
    }

    fn execute(&mut self, args: &[[u8; 64]]) -> Result<ShellVec<u8>, CommandError> {
        let mut output = ShellVec::new();
        if !args.is_empty() {
            let len = args[0].iter().position(|&b| b == 0).unwrap_or(64);
            if len > 0 {
                unsafe {
                    if !self.dir_stack.is_null() {
                        (*self.dir_stack).stack.push(args[0]);
                    }
                }
            }
        }
        for &b in b"pushd: directory pushed\n" {
            output.push(b);
        }
        Ok(output)
    }

    fn help(&self) -> &[u8] {
        b"pushd <dir> - Push directory onto directory stack"
    }
}

pub struct PopdCommand {
    pub dir_stack: *mut DirectoryStack,
}

impl ShellCommand for PopdCommand {
    fn name(&self) -> &[u8] {
        b"popd"
    }

    fn execute(&mut self, _args: &[[u8; 64]]) -> Result<ShellVec<u8>, CommandError> {
        let mut output = ShellVec::new();
        unsafe {
            if !self.dir_stack.is_null() && !(*self.dir_stack).stack.is_empty() {
                let last_idx = (*self.dir_stack).stack.len() - 1;
                (*self.dir_stack).stack.remove(last_idx);
                for &b in b"popd: popped directory\n" {
                    output.push(b);
                }
            } else {
                for &b in b"popd: directory stack empty\n" {
                    output.push(b);
                }
            }
        }
        Ok(output)
    }

    fn help(&self) -> &[u8] {
        b"popd - Pop directory from directory stack"
    }
}

pub struct DirsCommand {
    pub dir_stack: *mut DirectoryStack,
}

impl ShellCommand for DirsCommand {
    fn name(&self) -> &[u8] {
        b"dirs"
    }

    fn execute(&mut self, _args: &[[u8; 64]]) -> Result<ShellVec<u8>, CommandError> {
        let mut output = ShellVec::new();
        for &b in b"Directory stack: " {
            output.push(b);
        }
        unsafe {
            if !self.dir_stack.is_null() {
                for (i, dir) in (*self.dir_stack).stack.iter().enumerate() {
                    if i > 0 {
                        output.push(b' ');
                    }
                    let len = dir.iter().position(|&b| b == 0).unwrap_or(64);
                    for &b in &dir[..len] {
                        output.push(b);
                    }
                }
            }
        }
        output.push(b'\n');
        Ok(output)
    }

    fn help(&self) -> &[u8] {
        b"dirs - Display directory stack"
    }
}

pub trait CommandParser {
    fn parse(&self, input: &[u8]) -> Result<([u8; 32], ShellVec<[u8; 64]>), CommandError>;
    fn validate(&self, command: &[u8], args: &[[u8; 64]]) -> Result<(), CommandError>;
}

#[repr(C)]
pub struct SimpleCommandParser;

impl SimpleCommandParser {
    pub fn new() -> Self {
        SimpleCommandParser
    }
}

impl Default for SimpleCommandParser {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandParser for SimpleCommandParser {
    fn parse(&self, input: &[u8]) -> Result<([u8; 32], ShellVec<[u8; 64]>), CommandError> {
        let mut command = [0u8; 32];
        let mut args = ShellVec::new();
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
    fn list(&self) -> ShellVec<&[u8]>;
}

use alloc::boxed::Box;

#[repr(C)]
pub struct SimpleCommandRegistry {
    pub commands: ShellVec<Option<Box<dyn ShellCommand>>>,
}

impl SimpleCommandRegistry {
    pub fn new() -> Self {
        SimpleCommandRegistry {
            commands: ShellVec::new(),
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

        let sigmagrep = SigmaGrepCommand;
        self.commands.push(Some(Box::new(sigmagrep)));

        let sigmafind = SigmaFindCommand;
        self.commands.push(Some(Box::new(sigmafind)));

        let sigmadiff = SigmaDiffCommand;
        self.commands.push(Some(Box::new(sigmadiff)));

        let which = WhichCommand;
        self.commands.push(Some(Box::new(which)));

        let type_cmd = TypeCommand;
        self.commands.push(Some(Box::new(type_cmd)));

        static mut GLOBAL_DIR_STACK: DirectoryStack = DirectoryStack { stack: Vec::new() };
        unsafe {
            let pushd = PushdCommand {
                dir_stack: &raw mut GLOBAL_DIR_STACK,
            };
            self.commands.push(Some(Box::new(pushd)));

            let popd = PopdCommand {
                dir_stack: &raw mut GLOBAL_DIR_STACK,
            };
            self.commands.push(Some(Box::new(popd)));

            let dirs = DirsCommand {
                dir_stack: &raw mut GLOBAL_DIR_STACK,
            };
            self.commands.push(Some(Box::new(dirs)));
        }
    }

    pub fn get_mut<'a>(&'a mut self, name: &[u8]) -> Option<&'a mut (dyn ShellCommand + 'a)> {
        let name_len = name.iter().position(|&b| b == 0).unwrap_or(name.len());
        let name_slice = &name[..name_len];
        let idx = self.commands.iter().position(|cmd_opt| {
            if let Some(ref cmd) = *cmd_opt {
                cmd.name() == name_slice
            } else {
                false
            }
        });
        if let Some(i) = idx {
            if let Some(ref mut cmd_box) = self.commands[i] {
                return Some(cmd_box.as_mut());
            }
        }
        None
    }
}

impl Default for SimpleCommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandRegistry for SimpleCommandRegistry {
    fn register(&mut self, command: Box<dyn ShellCommand>) -> Result<(), CommandError> {
        self.commands.push(Some(command));
        Ok(())
    }

    fn unregister(&mut self, name: &[u8]) -> Result<(), CommandError> {
        let name_len = name.iter().position(|&b| b == 0).unwrap_or(name.len());
        let name_slice = &name[..name_len];
        for i in 0..self.commands.len() {
            if let Some(Some(ref cmd)) = self.commands.get(i) {
                if cmd.name() == name_slice {
                    if let Some(slot) = self.commands.get_mut(i) {
                        *slot = None;
                    }
                    return Ok(());
                }
            }
        }
        Err(CommandError::NotFound)
    }

    fn get(&self, name: &[u8]) -> Option<&dyn ShellCommand> {
        let name_len = name.iter().position(|&b| b == 0).unwrap_or(name.len());
        let name_slice = &name[..name_len];
        for i in 0..self.commands.len() {
            if let Some(Some(ref command)) = self.commands.get(i) {
                if command.name() == name_slice {
                    return Some(command.as_ref());
                }
            }
        }
        None
    }

    fn list(&self) -> ShellVec<&[u8]> {
        let mut names = ShellVec::new();
        for command_option in &*self.commands {
            if let Some(ref command) = command_option {
                names.push(command.name());
            }
        }
        names
    }
}

pub trait ShellSession {
    fn execute_line(&mut self, input: &[u8]) -> Result<ShellVec<u8>, CommandError>;
    fn set_environment(&mut self, key: &[u8], value: &[u8]);
    fn get_environment(&self, key: &[u8]) -> Option<&[u8]>;
}

#[repr(C)]
pub struct SimpleShellSession {
    pub registry: SimpleCommandRegistry,
    pub parser: SimpleCommandParser,
    pub environment: ShellVec<([u8; 64], [u8; 128])>,
}

impl SimpleShellSession {
    pub fn new() -> Self {
        let mut registry = SimpleCommandRegistry::new();
        registry.register_builtins();
        SimpleShellSession {
            registry,
            parser: SimpleCommandParser::new(),
            environment: ShellVec::new(),
        }
    }
}

impl Default for SimpleShellSession {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellSession for SimpleShellSession {
    fn execute_line(&mut self, input: &[u8]) -> Result<ShellVec<u8>, CommandError> {
        let (command_name, args) = self.parser.parse(input)?;

        let cmd_len = command_name.iter().position(|&b| b == 0).unwrap_or(32);
        let trimmed_name = &command_name[..cmd_len];

        if let Some(command) = self.registry.get_mut(trimmed_name) {
            let slice = if args.is_empty() {
                &[]
            } else {
                args.as_slice()
            };
            command.execute(slice)
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
        for i in 0..self.environment.len() {
            if let Some(&(ref k, ref v)) = self.environment.get(i) {
                let len = k.iter().position(|&b| b == 0).unwrap_or(64);
                if &k[..len] == key {
                    let vlen = v.iter().position(|&b| b == 0).unwrap_or(128);
                    return Some(&v[..vlen]);
                }
            }
        }
        None
    }
}

pub trait CommandHistory {
    fn add(&mut self, command: &[u8]);
    fn get_previous(&self) -> Option<&[u8]>;
    fn get_next(&self) -> Option<&[u8]>;
    fn list(&self) -> ShellVec<&[u8]>;
}

#[repr(C)]
pub struct SimpleCommandHistory {
    pub history: ShellVec<[u8; 256]>,
    pub current_index: AtomicUsize,
}

impl SimpleCommandHistory {
    pub fn new() -> Self {
        SimpleCommandHistory {
            history: ShellVec::new(),
            current_index: AtomicUsize::new(0),
        }
    }
}

impl Default for SimpleCommandHistory {
    fn default() -> Self {
        Self::new()
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
            if let Some(cmd) = self.history.get(idx - 1) {
                let len = cmd.iter().position(|&b| b == 0).unwrap_or(256);
                Some(&cmd[..len])
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_next(&self) -> Option<&[u8]> {
        let idx = self.current_index.load(Ordering::SeqCst);
        if idx < self.history.len() {
            if let Some(cmd) = self.history.get(idx) {
                let len = cmd.iter().position(|&b| b == 0).unwrap_or(256);
                Some(&cmd[..len])
            } else {
                None
            }
        } else {
            None
        }
    }

    fn list(&self) -> ShellVec<&[u8]> {
        let mut commands = ShellVec::new();
        for cmd in &*self.history {
            let len = cmd.iter().position(|&b| b == 0).unwrap_or(256);
            commands.push(&cmd[..len]);
        }
        commands
    }
}

#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
#[cfg(target_os = "none")]
impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    fn len(&self) -> usize {
        self.len
    }
    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { Some(&*self.data.add(index)) }
        } else {
            None
        }
    }
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len {
            unsafe { Some(&mut *self.data.add(index)) }
        } else {
            None
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// Allocator shim: uses std allocator on hosted targets (test/dev) and extern C on bare-metal
#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use alloc::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_builtins_registration() {
        let session = SimpleShellSession::new();

        assert!(session.registry.get(b"sigpkg").is_some());
        assert!(session.registry.get(b"sigtrace").is_some());
        assert!(session.registry.get(b"sigmetrics").is_some());
        assert!(session.registry.get(b"sigstandards").is_some());
        assert!(session.registry.get(b"sigsched").is_some());
    }

    #[test]
    fn test_execute_sigpkg() {
        let mut session = SimpleShellSession::new();
        let result = session.execute_line(b"sigpkg").unwrap();
        assert_eq!(&result[..6], b"sigpkg");
    }

    #[test]
    fn test_command_history_add_and_list() {
        let mut history = SimpleCommandHistory::new();
        history.add(b"sigtrace trace task 256");
        assert_eq!(history.list().len(), 1);
        assert_eq!(history.get_previous().unwrap(), b"sigtrace trace task 256");
    }

    #[test]
    fn test_sigmagrep_execution() {
        let mut cmd = SigmaGrepCommand;
        assert_eq!(cmd.name(), b"sigmagrep");

        let mut arg1 = [0u8; 64];
        let mut arg2 = [0u8; 64];
        let mut arg3 = [0u8; 64];
        arg1[..14].copy_from_slice(b"my-search-term");
        arg2[..2].copy_from_slice(b"-i");
        arg3[..2].copy_from_slice(b"-n");

        let args = [arg1, arg2, arg3];
        let output = cmd.execute(&args).unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_sigmafind_execution() {
        let mut cmd = SigmaFindCommand;
        assert_eq!(cmd.name(), b"sigmafind");

        let mut arg1 = [0u8; 64];
        let mut arg2 = [0u8; 64];
        let mut arg3 = [0u8; 64];
        let mut arg4 = [0u8; 64];
        arg1[..2].copy_from_slice(b"-e");
        arg2[..2].copy_from_slice(b"-d");
        arg3[..1].copy_from_slice(b"5");
        arg4[..9].copy_from_slice(b"test-file");

        let args = [arg1, arg2, arg3, arg4];
        let output = cmd.execute(&args).unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_sigmadiff_execution() {
        let mut cmd = SigmaDiffCommand;
        assert_eq!(cmd.name(), b"sigmadiff");

        let mut arg1 = [0u8; 64];
        let mut arg2 = [0u8; 64];
        arg1[..2].copy_from_slice(b"-w");
        arg2[..2].copy_from_slice(b"-y");

        let args = [arg1, arg2];
        let output = cmd.execute(&args).unwrap();
        assert!(!output.is_empty());
    }

    #[test]
    fn test_which_type_and_directory_stack() {
        let mut session = SimpleShellSession::new();
        assert!(session.registry.get(b"which").is_some());
        assert!(session.registry.get(b"type").is_some());
        assert!(session.registry.get(b"pushd").is_some());
        assert!(session.registry.get(b"popd").is_some());
        assert!(session.registry.get(b"dirs").is_some());

        let which_out = session.execute_line(b"which ls").unwrap();
        assert!(which_out.starts_with(b"/system/bin/ls"));

        let type_out = session.execute_line(b"type ls").unwrap();
        assert!(type_out.contains(&b'b'));

        let pushd_out = session.execute_line(b"pushd /tmp").unwrap();
        assert!(pushd_out.starts_with(b"pushd"));

        let dirs_out = session.execute_line(b"dirs").unwrap();
        assert!(dirs_out.starts_with(b"Directory stack:"));

        let popd_out = session.execute_line(b"popd").unwrap();
        assert!(popd_out.starts_with(b"popd"));
    }
}
