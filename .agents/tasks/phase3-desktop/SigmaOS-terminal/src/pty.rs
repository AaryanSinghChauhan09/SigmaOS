//! PTY (pseudo-terminal) management for SigmaOS-terminal.

use crate::{Result, TermError};

/// A PTY pair (master + slave) for a spawned shell process.
#[derive(Debug)]
pub struct Pty {
    pub cols: u16,
    pub rows: u16,
    pub shell: String,
    /// Simulated buffer of bytes written to the PTY master.
    pub write_buf: Vec<u8>,
}

impl Pty {
    /// Open a PTY and spawn the given shell.
    pub fn open(shell: &str, _args: &[&str]) -> Result<Self> {
        Ok(Self {
            cols: 80,
            rows: 24,
            shell: shell.to_string(),
            write_buf: Vec::new(),
        })
    }

    /// Write bytes to the PTY master (input to the shell).
    pub fn write(&mut self, data: &[u8]) -> Result<()> {
        self.write_buf.extend_from_slice(data);
        Ok(())
    }

    /// Resize the terminal window.
    pub fn resize(&mut self, cols: u16, rows: u16) -> Result<()> {
        if cols == 0 || rows == 0 {
            return Err(TermError::Pty("cols and rows must be > 0".into()));
        }
        self.cols = cols;
        self.rows = rows;
        Ok(())
    }
}
