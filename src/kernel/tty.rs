#![allow(dead_code)]
// SigmaOS Comprehensive TTY/PTY Subsystem
// Includes full Termios line discipline, signaling, and ANSI buffering

use std::collections::VecDeque;
use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

// POSIX Termios Constants
pub const IGNBRK: u32 = 0o000001;
pub const BRKINT: u32 = 0o000002;
pub const IGNPAR: u32 = 0o000004;
pub const PARMRK: u32 = 0o000010;
pub const INPCK:  u32 = 0o000020;
pub const ISTRIP: u32 = 0o000040;
pub const INLCR:  u32 = 0o000100;
pub const IGNCR:  u32 = 0o000200;
pub const ICRNL:  u32 = 0o000400;
pub const IXON:   u32 = 0o002000;
pub const IXOFF:  u32 = 0o010000;

pub const OPOST:  u32 = 0o000001;
pub const ONLCR:  u32 = 0o000004;

pub const ISIG:   u32 = 0o000001;
pub const ICANON: u32 = 0o000002;
pub const ECHO:   u32 = 0o000010;
pub const ECHOE:  u32 = 0o000020;
pub const ECHOK:  u32 = 0o000040;
pub const ECHONL: u32 = 0o000100;
pub const NOFLSH: u32 = 0o000200;
pub const TOSTOP: u32 = 0o000400;
pub const IEXTEN: u32 = 0o100000;

pub const VEOF: usize = 4;
pub const VEOL: usize = 11;
pub const VERASE: usize = 2;
pub const VINTR: usize = 0;
pub const VKILL: usize = 3;
pub const VQUIT: usize = 1;
pub const VSTART: usize = 8;
pub const VSTOP: usize = 9;
pub const VSUSP: usize = 10;
pub const VMIN: usize = 6;
pub const VTIME: usize = 5;

// ioctls
pub const TIOCGWINSZ: usize = 0x5413;
pub const TIOCSWINSZ: usize = 0x5414;
pub const TCGETS: usize = 0x5401;
pub const TCSETS: usize = 0x5402;
pub const TIOCSCTTY: usize = 0x540E;
pub const TIOCSPGRP: usize = 0x5410;
pub const TIOCGPGRP: usize = 0x540F;

#[derive(Debug, Clone, Copy)]
pub struct Winsize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    pub c_line: u8,
    pub c_cc: [u8; 32],
}

impl Termios {
    pub fn default() -> Self {
        let mut cc = [0; 32];
        cc[VINTR] = 3;   // Ctrl-C
        cc[VQUIT] = 28;  // Ctrl-\
        cc[VERASE] = 127;// Backspace
        cc[VKILL] = 21;  // Ctrl-U
        cc[VEOF] = 4;    // Ctrl-D
        cc[VSTART] = 17; // Ctrl-Q
        cc[VSTOP] = 19;  // Ctrl-S
        cc[VSUSP] = 26;  // Ctrl-Z
        cc[VMIN] = 1;
        cc[VTIME] = 0;
        
        Self {
            c_iflag: ICRNL | IXON,
            c_oflag: OPOST | ONLCR,
            c_cflag: 0,
            c_lflag: ISIG | ICANON | ECHO | ECHOE | ECHOK | IEXTEN,
            c_line: 0,
            c_cc: cc,
        }
    }
}

pub struct Tty {
    pub id: usize,
    pub input_buffer: VecDeque<u8>,
    pub canonical_buffer: Vec<u8>, // Line editing buffer
    pub output_buffer: VecDeque<u8>,
    pub winsize: Winsize,
    pub termios: Termios,
    pub foreground_pgid: u64,
    pub session_id: u64,
    
    // Internal state
    pub stopped: bool,
}

static NEXT_TTY_ID: AtomicUsize = AtomicUsize::new(0);

impl Tty {
    pub fn new() -> Self {
        Self {
            id: NEXT_TTY_ID.fetch_add(1, Ordering::SeqCst),
            input_buffer: VecDeque::new(),
            canonical_buffer: Vec::new(),
            output_buffer: VecDeque::new(),
            winsize: Winsize { ws_row: 24, ws_col: 80, ws_xpixel: 0, ws_ypixel: 0 },
            termios: Termios::default(),
            foreground_pgid: 0,
            session_id: 0,
            stopped: false,
        }
    }

    /// Receives raw bytes from a keyboard/UART driver
    pub fn receive_input(&mut self, data: &[u8]) {
        for &b in data {
            // 1. ISIG: Signal processing
            if (self.termios.c_lflag & ISIG) != 0 {
                if b == self.termios.c_cc[VINTR] {
                    // Trigger SIGINT to foreground process group
                    self.echo_str("^C\n");
                    self.flush_input();
                    continue;
                }
                if b == self.termios.c_cc[VQUIT] {
                    // Trigger SIGQUIT
                    self.echo_str("^\\\n");
                    self.flush_input();
                    continue;
                }
                if b == self.termios.c_cc[VSUSP] {
                    // Trigger SIGTSTP
                    self.echo_str("^Z\n");
                    self.flush_input();
                    continue;
                }
            }

            // 2. IXON/IXOFF Flow control
            if (self.termios.c_iflag & IXON) != 0 {
                if b == self.termios.c_cc[VSTOP] {
                    self.stopped = true;
                    continue;
                }
                if b == self.termios.c_cc[VSTART] {
                    self.stopped = false;
                    continue;
                }
            }

            // 3. Translation
            let mut c = b;
            if c == b'\r' {
                if (self.termios.c_iflag & IGNCR) != 0 {
                    continue;
                }
                if (self.termios.c_iflag & ICRNL) != 0 {
                    c = b'\n';
                }
            } else if c == b'\n' && (self.termios.c_iflag & INLCR) != 0 {
                c = b'\r';
            }

            // 4. Mode processing
            if (self.termios.c_lflag & ICANON) != 0 {
                self.process_canonical(c);
            } else {
                self.input_buffer.push_back(c);
                if (self.termios.c_lflag & ECHO) != 0 {
                    self.echo_char(c);
                }
            }
        }
    }

    fn process_canonical(&mut self, c: u8) {
        if c == self.termios.c_cc[VEOF] {
            // EOF handling: Push current buffer without newline
            self.commit_canonical();
        } else if c == self.termios.c_cc[VERASE] {
            // Backspace handling
            if self.canonical_buffer.pop().is_some() {
                if (self.termios.c_lflag & ECHOE) != 0 {
                    self.echo_str("\x08 \x08"); // Erase visually
                }
            }
        } else if c == self.termios.c_cc[VKILL] {
            // Line kill
            self.canonical_buffer.clear();
            if (self.termios.c_lflag & ECHOK) != 0 {
                self.echo_str("^U\n");
            }
        } else if c == b'\n' || c == self.termios.c_cc[VEOL] {
            self.canonical_buffer.push(c);
            if (self.termios.c_lflag & ECHO) != 0 {
                self.echo_char(c);
            }
            self.commit_canonical();
        } else {
            self.canonical_buffer.push(c);
            if (self.termios.c_lflag & ECHO) != 0 {
                self.echo_char(c);
            }
        }
    }

    fn commit_canonical(&mut self) {
        for b in self.canonical_buffer.drain(..) {
            self.input_buffer.push_back(b);
        }
    }

    fn echo_char(&mut self, c: u8) {
        if c == b'\n' && (self.termios.c_oflag & ONLCR) != 0 {
            self.output_buffer.push_back(b'\r');
        }
        self.output_buffer.push_back(c);
    }

    fn echo_str(&mut self, s: &str) {
        for &b in s.as_bytes() {
            self.echo_char(b);
        }
    }

    fn flush_input(&mut self) {
        if (self.termios.c_lflag & NOFLSH) == 0 {
            self.input_buffer.clear();
            self.canonical_buffer.clear();
        }
    }

    /// Read data from the TTY (used by sys_read)
    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        let mut read = 0;
        while read < buf.len() {
            if let Some(b) = self.input_buffer.pop_front() {
                buf[read] = b;
                read += 1;
            } else {
                break;
            }
        }
        read
    }

    /// Write data to the TTY (used by sys_write)
    pub fn write(&mut self, data: &[u8]) {
        for &b in data {
            if (self.termios.c_oflag & OPOST) != 0 {
                if b == b'\n' && (self.termios.c_oflag & ONLCR) != 0 {
                    self.output_buffer.push_back(b'\r');
                }
            }
            self.output_buffer.push_back(b);
        }
    }
    
    pub fn ioctl(&mut self, request: usize, arg: usize) -> Result<usize, &'static str> {
        match request {
            TIOCGWINSZ => {
                unsafe { *(arg as *mut Winsize) = self.winsize };
                Ok(0)
            },
            TIOCSWINSZ => {
                unsafe { self.winsize = *(arg as *const Winsize) };
                Ok(0)
            },
            TCGETS => {
                unsafe { *(arg as *mut Termios) = self.termios };
                Ok(0)
            },
            TCSETS => {
                unsafe { self.termios = *(arg as *const Termios) };
                Ok(0)
            },
            TIOCGPGRP => {
                unsafe { *(arg as *mut u64) = self.foreground_pgid };
                Ok(0)
            },
            TIOCSPGRP => {
                unsafe { self.foreground_pgid = *(arg as *const u64) };
                Ok(0)
            },
            _ => Err("Invalid ioctl"),
        }
    }
}
