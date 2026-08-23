/// OOP-based Desktop Terminal for SigmaOS
/// Implements terminal emulator, ANSI escape interpretation, and shell integration.
/// Inspired by Alacritty, GNOME-Terminal, xterm, and tmux from Linux & BSD distributions.

extern crate alloc;
use alloc::boxed::Box;

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TerminalID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalError { Success = 0, NotFound = 1, CommandFailed = 2 }

pub trait Terminal {
    fn id(&self) -> TerminalID;
    fn title(&self) -> &[u8];
    fn working_directory(&self) -> &[u8];
    fn set_working_directory(&mut self, path: &[u8]);
}

#[repr(C)]
pub struct SimpleTerminal {
    pub id: TerminalID,
    pub title: [u8; 128],
    pub working_directory: [u8; 256],
}

impl SimpleTerminal {
    pub fn new(id: TerminalID, title: &[u8]) -> Self {
        let mut title_array = [0u8; 128];
        let mut dir_array = [0u8; 256];
        let title_len = title.len().min(127);
        let dir_len = b"/home/user".len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(title.as_ptr(), title_array.as_mut_ptr(), title_len);
            core::ptr::copy_nonoverlapping(b"/home/user".as_ptr(), dir_array.as_mut_ptr(), dir_len);
        }
        SimpleTerminal {
            id,
            title: title_array,
            working_directory: dir_array,
        }
    }
}

impl Terminal for SimpleTerminal {
    fn id(&self) -> TerminalID { self.id }
    fn title(&self) -> &[u8] {
        let len = self.title.iter().position(|&b| b == 0).unwrap_or(128);
        &self.title[..len]
    }
    fn working_directory(&self) -> &[u8] {
        let len = self.working_directory.iter().position(|&b| b == 0).unwrap_or(256);
        &self.working_directory[..len]
    }
    
    fn set_working_directory(&mut self, path: &[u8]) {
        let path_len = path.len().min(255);
        unsafe {
            core::ptr::copy_nonoverlapping(path.as_ptr(), self.working_directory.as_mut_ptr(), path_len);
        }
    }
}

pub trait TerminalManager {
    fn create_terminal(&mut self, title: &[u8]) -> Result<TerminalID, TerminalError>;
    fn close_terminal(&mut self, id: TerminalID) -> Result<(), TerminalError>;
    fn get_terminal(&self, id: TerminalID) -> Option<&dyn Terminal>;
    fn execute_command(&mut self, terminal_id: TerminalID, command: &[u8]) -> Result<Vec<u8>, TerminalError>;
}

#[repr(C)]
pub struct SimpleTerminalManager {
    pub terminals: Vec<Option<Box<dyn Terminal>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTerminalManager {
    pub fn new() -> Self {
        SimpleTerminalManager {
            terminals: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TerminalManager for SimpleTerminalManager {
    fn create_terminal(&mut self, title: &[u8]) -> Result<TerminalID, TerminalError> {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let terminal = SimpleTerminal::new(id, title);
        self.terminals.push(Some(Box::new(terminal)));
        Ok(id)
    }
    
    fn close_terminal(&mut self, id: TerminalID) -> Result<(), TerminalError> {
        for terminal_option in &mut self.terminals {
            if let Some(ref terminal) = *terminal_option {
                let term_ref: &dyn Terminal = terminal.as_ref();
                if term_ref.id() == id {
                    *terminal_option = None;
                    return Ok(());
                }
            }
        }
        Err(TerminalError::NotFound)
    }
    
    fn get_terminal(&self, id: TerminalID) -> Option<&dyn Terminal> {
        for terminal_option in &self.terminals {
            if let Some(ref terminal) = *terminal_option {
                let term_ref: &dyn Terminal = terminal.as_ref();
                if term_ref.id() == id { return Some(term_ref); }
            }
        }
        None
    }
    
    fn execute_command(&mut self, terminal_id: TerminalID, command: &[u8]) -> Result<Vec<u8>, TerminalError> {
        if self.get_terminal(terminal_id).is_some() {
            let mut output = Vec::new();
            for &byte in command {
                output.push(byte);
            }
            output.push(b'\n');
            Ok(output)
        } else {
            Err(TerminalError::NotFound)
        }
    }
}

pub trait ShellIntegration {
    fn get_shell(&self) -> &[u8];
    fn set_shell(&mut self, shell: &[u8]);
    fn get_env_var(&self, key: &[u8]) -> Option<&[u8]>;
    fn set_env_var(&mut self, key: &[u8], value: &[u8]);
}

#[repr(C)]
pub struct SimpleShellIntegration {
    pub shell: [u8; 64],
    pub env_vars: Vec<([u8; 64], [u8; 256])>,
}

impl SimpleShellIntegration {
    pub fn new() -> Self {
        let mut shell_array = [0u8; 64];
        let shell_len = b"/bin/bash".len().min(63);
        for i in 0..shell_len {
            shell_array[i] = b"/bin/bash"[i];
        }
        SimpleShellIntegration {
            shell: shell_array,
            env_vars: Vec::new(),
        }
    }
}

impl ShellIntegration for SimpleShellIntegration {
    fn get_shell(&self) -> &[u8] {
        let len = self.shell.iter().position(|&b| b == 0).unwrap_or(64);
        &self.shell[..len]
    }
    
    fn set_shell(&mut self, shell: &[u8]) {
        let shell_len = shell.len().min(63);
        for i in 0..shell_len {
            self.shell[i] = shell[i];
        }
    }
    
    fn get_env_var(&self, key: &[u8]) -> Option<&[u8]> {
        for &(ref k, ref v) in &self.env_vars {
            let k_len = k.iter().position(|&b| b == 0).unwrap_or(64);
            if &k[..k_len] == key {
                let v_len = v.iter().position(|&b| b == 0).unwrap_or(256);
                return Some(&v[..v_len]);
            }
        }
        None
    }
    
    fn set_env_var(&mut self, key: &[u8], value: &[u8]) {
        let mut key_array = [0u8; 64];
        let mut value_array = [0u8; 256];
        let key_len = key.len().min(63);
        let value_len = value.len().min(255);
        for i in 0..key_len { key_array[i] = key[i]; }
        for i in 0..value_len { value_array[i] = value[i]; }
        self.env_vars.push((key_array, value_array));
    }
}

// ==============================================================================
// 1. ANSI Escape Code Interpreter (SGR color and attribute parser)
// ==============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextAttribute {
    pub fg_color: u8, // ANSI 8-color model (e.g. 31=Red, 32=Green)
    pub bg_color: u8, // ANSI 8-color model (e.g. 40=Black, 41=Red)
    pub is_bold: bool,
    pub is_blinking: bool,
}

pub struct AnsiEscapeInterpreter {
    pub active_attr: TextAttribute,
}

impl AnsiEscapeInterpreter {
    pub fn new() -> Self {
        Self {
            active_attr: TextAttribute {
                fg_color: 37, // White
                bg_color: 40, // Black
                is_bold: false,
                is_blinking: false,
            },
        }
    }

    pub fn parse_escape_sequence(&mut self, code: &[u8]) -> bool {
        // Parses SGR codes (Select Graphic Rendition) e.g., "\x1b[31;1m" (Bold Red)
        if code.len() >= 3 && code[0] == b'\x1b' && code[1] == b'[' {
            let last_byte = code[code.len() - 1];
            if last_byte == b'm' {
                // Simplistic parser for common ANSI colors
                if code.contains(&b'1') {
                    self.active_attr.is_bold = true;
                }
                if code.contains(&b'0') {
                    self.active_attr.is_bold = false;
                    self.active_attr.is_blinking = false;
                }
                if code.contains(&b'5') {
                    self.active_attr.is_blinking = true;
                }
                // Foregrounds
                if code.contains(&b'3') && code.contains(&b'1') { self.active_attr.fg_color = 31; } // Red
                if code.contains(&b'3') && code.contains(&b'2') { self.active_attr.fg_color = 32; } // Green
                return true;
            }
        }
        false
    }
}

impl Default for AnsiEscapeInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 2. Scrollback Buffer and History Grid
// ==============================================================================
#[derive(Clone, Copy)]
pub struct TerminalCell {
    pub glyph: char,
    pub attribute: TextAttribute,
}

pub struct ScrollbackGrid {
    pub lines: Vec<Vec<TerminalCell>>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub max_scrollback_lines: usize,
}

impl ScrollbackGrid {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            cursor_row: 0,
            cursor_col: 0,
            max_scrollback_lines: 1000,
        }
    }

    pub fn write_character(&mut self, ch: char, attr: TextAttribute) {
        if self.lines.is_empty() {
            self.lines.push(Vec::new());
        }
        let row = self.lines.len() - 1;
        self.lines[row].push(TerminalCell { glyph: ch, attribute: attr });
        self.cursor_col += 1;

        if ch == '\n' {
            self.lines.push(Vec::new());
            self.cursor_row += 1;
            self.cursor_col = 0;
        }

        // Limit scrollback history
        if self.lines.len() > self.max_scrollback_lines {
            self.lines.remove(0);
            if self.cursor_row > 0 {
                self.cursor_row -= 1;
            }
        }
    }
}

impl Default for ScrollbackGrid {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// 3. PTY (Pseudo-Terminal) Session Pair
// ==============================================================================
pub struct PtySessionPair {
    pub master_fd: i32,
    pub slave_fd: i32,
    pub shell_path: [u8; 64],
}

impl PtySessionPair {
    pub fn new(master: i32, slave: i32) -> Self {
        let mut shell_arr = [0u8; 64];
        let path = b"/bin/sigma-shell";
        shell_arr[..path.len()].copy_from_slice(path);
        Self {
            master_fd: master,
            slave_fd: slave,
            shell_path: shell_arr,
        }
    }
}

// ==============================================================================
// 4. UTF-8 Multi-byte Character Decoder
// ==============================================================================
pub struct Utf8Decoder {
    pub expected_bytes: usize,
    pub bytes_collected: Vec<u8>,
}

impl Utf8Decoder {
    pub fn new() -> Self {
        Self {
            expected_bytes: 0,
            bytes_collected: Vec::new(),
        }
    }

    pub fn decode_byte(&mut self, b: u8) -> Option<char> {
        if self.expected_bytes == 0 {
            if b & 0x80 == 0 {
                return Some(b as char); // Standard 1-byte ASCII
            } else if b & 0xE0 == 0xC0 {
                self.expected_bytes = 2;
                self.bytes_collected.push(b);
            } else if b & 0xF0 == 0xE0 {
                self.expected_bytes = 3;
                self.bytes_collected.push(b);
            } else if b & 0xF8 == 0xF0 {
                self.expected_bytes = 4;
                self.bytes_collected.push(b);
            }
        } else {
            self.bytes_collected.push(b);
            if self.bytes_collected.len() == self.expected_bytes {
                // Decode multi-byte into char
                let ch = match self.expected_bytes {
                    2 => {
                        let c = (((self.bytes_collected[0] & 0x1F) as u32) << 6) | ((self.bytes_collected[1] & 0x3F) as u32);
                        core::char::from_u32(c)
                    }
                    3 => {
                        let c = (((self.bytes_collected[0] & 0x0F) as u32) << 12) | (((self.bytes_collected[1] & 0x3F) as u32) << 6) | ((self.bytes_collected[2] & 0x3F) as u32);
                        core::char::from_u32(c)
                    }
                    _ => Some('?'),
                };
                self.expected_bytes = 0;
                self.bytes_collected.clear();
                return ch;
            }
        }
        None
    }
}

impl Default for Utf8Decoder {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// Vec Implementation
// ==============================================================================
pub struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    pub fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    pub fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    pub fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
        }
    }
    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn clear(&mut self) {
        while self.len > 0 {
            self.remove(0);
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
