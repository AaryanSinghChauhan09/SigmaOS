//! SigmaOS Terminal — GPU-accelerated terminal emulator.

pub mod config;
pub mod pty;
pub mod renderer;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum TermError {
    #[error("PTY error: {0}")]
    Pty(String),
    #[error("Render error: {0}")]
    Render(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, TermError>;

/// RGB color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };
}

/// Top-level terminal instance.
pub struct Terminal {
    pub config: config::TermConfig,
    pty: Option<pty::Pty>,
}

impl Terminal {
    pub fn new(config: config::TermConfig) -> Result<Self> {
        Ok(Self { config, pty: None })
    }

    pub fn spawn_shell(&mut self, shell: &str) -> Result<()> {
        let pty = pty::Pty::open(shell, &[])?;
        self.pty = Some(pty);
        Ok(())
    }

    pub fn write_input(&mut self, data: &[u8]) -> Result<()> {
        if let Some(pty) = &mut self.pty {
            pty.write(data)
        } else {
            Err(TermError::Pty("no PTY open".into()))
        }
    }

    pub fn resize(&mut self, cols: u16, rows: u16) -> Result<()> {
        if let Some(pty) = &mut self.pty {
            pty.resize(cols, rows)
        } else {
            Ok(())
        }
    }
}
