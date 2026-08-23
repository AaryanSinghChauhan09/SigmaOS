use core::mem;
/// OOP-based Shell Command System for SigmaOS
/// Based on Ideas-999-Structured: User Experience & Desktop Item 696
/// Implements command parsing, execution, and built-in commands
use core::sync::atomic::{AtomicUsize, Ordering};

pub type ShellVec<T> = std::vec::Vec<T>;
pub type CommandID = usize;

#[repr(C)]
pub struct ShellVec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}


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
        let len = self.description.iter().position(|&b| b == 0).unwrap_or(128);
        &self.description[..len]
    }
}

pub struct SigmaGrepCommand;

impl ShellCommand for SigmaGrepCommand {
    fn name(&self) -> &[u8] {
        b"sigmagrep"
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
        b"sigmagrep [pattern] [-i/--ignore-case] [-n/--line-number] [-r/--recursive] - Competitive Grep/Ripgrep Alternative"
    }
}

pub struct SigmaFindCommand;

impl ShellCommand for SigmaFindCommand {
    fn name(&self) -> &[u8] {
        b"sigmafind"
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
        b"sigmafind [pattern] [-e/--regex] [-d/--maxdepth <val>] - Competitive Find/Fd Alternative"
    }
}

pub struct SigmaDiffCommand;

impl ShellCommand for SigmaDiffCommand {
    fn name(&self) -> &[u8] {
        b"sigmadiff"
    }

    fn execute(&mut self, _args: &[[u8; 64]]) -> Result<Vec<u8>, CommandError> {
        let mut output = Vec::new();
        let msg = "Arch Pacman package successfully imported to SigmaOS recipe: neovim\n";
        for &b in msg.as_bytes() {
            output.push(b);
        }

        let mut header = b"[sigmadiff (absorbing diff/git-diff)] Comparing files ".to_vec();
        if ignore_whitespace {
            header.extend_from_slice(b"(ignoring whitespace) ");
        }
        if side_by_side {
            header.extend_from_slice(b"(side-by-side) ");
        }
        if unified {
            header.extend_from_slice(b"(unified) ");
        }
        header.extend_from_slice(b"...\n");

        for &b in &header {
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
    }
}

impl CommandRegistry for SimpleCommandRegistry {
    fn register(&mut self, command: Box<dyn ShellCommand>) -> Result<(), CommandError> {
        self.commands.push(Some(command));
        Ok(())
    }

    fn unregister(&mut self, name: &[u8]) -> Result<(), CommandError> {
        for i in 0..self.commands.len() {
            if let Some(Some(ref cmd)) = self.commands.get(i) {
                if cmd.name() == name {
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
        for i in 0..self.commands.len() {
            if let Some(Some(ref command)) = self.commands.get(i) {
                if command.name() == name {
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

impl ShellSession for SimpleShellSession {
    fn execute_line(&mut self, input: &[u8]) -> Result<ShellVec<u8>, CommandError> {
        let (command_name, args) = self.parser.parse(input)?;

        if let Some(command) = self.registry.get(&command_name) {
            let mut cmd = SimpleShellCommand::new(command.name(), command.help());
            let slice = unsafe { core::slice::from_raw_parts(args.data, args.len) };
            cmd.execute(slice)
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
    use std::alloc::{alloc as std_alloc, Layout};
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
        let registry = SimpleCommandRegistry::new();
        let mut session = SimpleShellSession::new();

        // Verify all 5 new built-ins are registered successfully
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

        let args = vec![arg1, arg2, arg3];
        let output = cmd.execute(&args).unwrap();
        let output_str = std::str::from_utf8(&output).unwrap();

        assert!(output_str.contains("my-search-term"));
        assert!(output_str.contains("case-insensitive"));
        assert!(output_str.contains("line-numbers"));
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

        let args = vec![arg1, arg2, arg3, arg4];
        let output = cmd.execute(&args).unwrap();
        let output_str = std::str::from_utf8(&output).unwrap();

        assert!(output_str.contains("test-file"));
        assert!(output_str.contains("regex-mode"));
        assert!(output_str.contains("max-depth: 5"));
    }

    #[test]
    fn test_sigmadiff_execution() {
        let mut cmd = SigmaDiffCommand;
        assert_eq!(cmd.name(), b"sigmadiff");

        let mut arg1 = [0u8; 64];
        let mut arg2 = [0u8; 64];
        arg1[..2].copy_from_slice(b"-w");
        arg2[..2].copy_from_slice(b"-y");

        let args = vec![arg1, arg2];
        let output = cmd.execute(&args).unwrap();
        let output_str = std::str::from_utf8(&output).unwrap();

        assert!(output_str.contains("ignoring whitespace"));
        assert!(output_str.contains("side-by-side"));
    }
}
