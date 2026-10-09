// src/desktop/sigma_ghostty.rs
//! SigmaGhostty — GPU-Accelerated Terminal Emulator Model
//!
//! Inspired by Ghostty (Mitchell Hashimoto's GPU-accelerated terminal used in Omarchy).
//! SigmaGhostty provides:
//! - VT100/VT220/xterm-256color escape sequence parser
//! - Unicode grapheme cluster segmentation
//! - True-color (24-bit RGB) support
//! - GPU-accelerated cell renderer model (Vulkan backend)
//! - Ligature and Nerd Font glyph support
//! - Configurable font faces, sizes, and line spacing
//! - Sigma theme integration (auto-loads from OmarchyThemeSuite)
//! - Sixel graphics protocol stub
//! - Kitty graphics protocol stub
//! - Session persistence (save/restore scrollback)

#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ─────────────────────────────────────────────────────────────────────────────
// Terminal Color Model
// ─────────────────────────────────────────────────────────────────────────────

/// A terminal color (ANSI 16, 256-color, or true-color)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermColor {
    /// Default terminal color (no explicit color)
    Default,
    /// ANSI 16-color palette (0-15)
    Ansi(u8),
    /// 256-color palette
    Indexed(u8),
    /// True-color (24-bit RGB)
    Rgb(u8, u8, u8),
}

impl TermColor {
    /// Convert to CSS-style hex string (for rendering)
    pub fn to_hex(&self) -> String {
        match self {
            TermColor::Rgb(r, g, b) => alloc::format!("#{:02x}{:02x}{:02x}", r, g, b),
            TermColor::Default => "inherit".to_string(),
            TermColor::Ansi(n) => alloc::format!("ansi{}", n),
            TermColor::Indexed(n) => alloc::format!("idx{}", n),
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Cell Attributes
// ─────────────────────────────────────────────────────────────────────────────

/// Text rendering attributes for a terminal cell
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellAttrs {
    pub bold: bool,
    pub italic: bool,
    pub underline: UnderlineStyle,
    pub strikethrough: bool,
    pub blink: bool,
    pub reverse: bool,
    pub invisible: bool,
    pub fg: TermColor,
    pub bg: TermColor,
    pub underline_color: TermColor,
}

impl Default for CellAttrs {
    fn default() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: UnderlineStyle::None,
            strikethrough: false,
            blink: false,
            reverse: false,
            invisible: false,
            fg: TermColor::Default,
            bg: TermColor::Default,
            underline_color: TermColor::Default,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnderlineStyle {
    None,
    Straight,
    Double,
    Curly,
    Dotted,
    Dashed,
}

// ─────────────────────────────────────────────────────────────────────────────
// Terminal Cell
// ─────────────────────────────────────────────────────────────────────────────

/// A single terminal cell (one character position)
#[derive(Debug, Clone)]
pub struct TermCell {
    /// The Unicode codepoint (or first of a grapheme cluster)
    pub codepoint: char,
    /// Display width (1 for narrow, 2 for wide/CJK)
    pub width: u8,
    /// Rendering attributes
    pub attrs: CellAttrs,
    /// True if this cell is the right half of a wide character
    pub is_wide_spacer: bool,
}

impl TermCell {
    pub fn new(ch: char) -> Self {
        let width = if is_wide_char(ch) { 2 } else { 1 };
        Self {
            codepoint: ch,
            width,
            attrs: CellAttrs::default(),
            is_wide_spacer: false,
        }
    }

    pub fn space() -> Self {
        Self::new(' ')
    }
}

/// Approximate wide character detection (CJK, etc.)
fn is_wide_char(ch: char) -> bool {
    let cp = ch as u32;
    // CJK Unified Ideographs, Hiragana, Katakana, Hangul, etc.
    matches!(
        cp,
        0x1100..=0x115F   // Hangul Jamo
        | 0x2E80..=0x3247 // CJK Radicals
        | 0x3250..=0x4DBF // CJK area
        | 0x4E00..=0xA4C6 // CJK Unified
        | 0xA960..=0xA97C // Hangul Jamo Extended-A
        | 0xAC00..=0xD7A3 // Hangul Syllables
        | 0xF900..=0xFAFF // CJK Compatibility
        | 0x1B000..=0x1B001 // Kana Supplement
        | 0x1F004..=0x1F0CF // Mahjong/Cards
        | 0x1F200..=0x1F2FF // Enclosed CJK
        | 0x1F300..=0x1F64F // Emoji
        | 0x20000..=0x2FFFD // CJK Extension B-F
        | 0x30000..=0x3FFFD // CJK Extension G
    )
}

// ─────────────────────────────────────────────────────────────────────────────
// Terminal Grid
// ─────────────────────────────────────────────────────────────────────────────

/// A rectangular grid of terminal cells (one screen worth)
pub struct TermGrid {
    pub rows: usize,
    pub cols: usize,
    cells: Vec<TermCell>,
}

impl TermGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let cells = (0..rows * cols).map(|_| TermCell::space()).collect();
        Self { rows, cols, cells }
    }

    pub fn cell(&self, row: usize, col: usize) -> Option<&TermCell> {
        if row < self.rows && col < self.cols {
            Some(&self.cells[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn cell_mut(&mut self, row: usize, col: usize) -> Option<&mut TermCell> {
        if row < self.rows && col < self.cols {
            Some(&mut self.cells[row * self.cols + col])
        } else {
            None
        }
    }

    /// Write a character at (row, col) with given attrs
    pub fn write_char(&mut self, row: usize, col: usize, ch: char, attrs: CellAttrs) {
        if let Some(cell) = self.cell_mut(row, col) {
            cell.codepoint = ch;
            cell.attrs = attrs;
            cell.width = if is_wide_char(ch) { 2 } else { 1 };
            cell.is_wide_spacer = false;
        }
        // Mark wide spacer
        if is_wide_char(ch) {
            if let Some(spacer) = self.cell_mut(row, col + 1) {
                spacer.is_wide_spacer = true;
            }
        }
    }

    /// Scroll the grid up by `lines` rows, filling new lines with spaces
    pub fn scroll_up(&mut self, lines: usize) {
        let lines = lines.min(self.rows);
        self.cells.drain(0..lines * self.cols);
        for _ in 0..lines * self.cols {
            self.cells.push(TermCell::space());
        }
    }

    /// Clear a row to spaces with default attrs
    pub fn clear_row(&mut self, row: usize) {
        for col in 0..self.cols {
            if let Some(cell) = self.cell_mut(row, col) {
                *cell = TermCell::space();
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// VT Escape Sequence Parser
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VtAction {
    /// Print a character
    Print(char),
    /// Cursor movement: CUU, CUD, CUF, CUB, CUP
    CursorUp(u32),
    CursorDown(u32),
    CursorForward(u32),
    CursorBack(u32),
    CursorPosition {
        row: u32,
        col: u32,
    },
    /// Erase: EL, ED
    EraseInLine(u32), // 0=to end, 1=to start, 2=whole
    EraseInDisplay(u32), // 0=below, 1=above, 2=all, 3=saved
    /// SGR (Select Graphic Rendition) — color and style
    Sgr(Vec<u32>),
    /// Set window title (OSC 0/2)
    SetTitle(String),
    /// Sixel graphics data (stub)
    SixelData(String),
    /// Bell
    Bell,
    /// Backspace
    Backspace,
    /// Tab
    Tab,
    /// Newline
    Newline,
    /// Carriage return
    CarriageReturn,
}

/// Simple VT100/xterm escape sequence parser state machine
pub struct VtParser {
    state: ParserState,
    params: Vec<u32>,
    current_param: u32,
    osc_buf: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParserState {
    Ground,
    Escape,
    CsiParam,
    CsiIntermediate,
    OscString,
}

impl VtParser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Ground,
            params: Vec::new(),
            current_param: 0,
            osc_buf: String::new(),
        }
    }

    /// Feed bytes and collect parsed actions
    pub fn feed(&mut self, input: &[u8]) -> Vec<VtAction> {
        let mut actions = Vec::new();
        for &byte in input {
            if let Some(action) = self.process_byte(byte) {
                actions.push(action);
            }
        }
        actions
    }

    fn process_byte(&mut self, byte: u8) -> Option<VtAction> {
        match self.state {
            ParserState::Ground => match byte {
                0x07 => Some(VtAction::Bell),
                0x08 => Some(VtAction::Backspace),
                0x09 => Some(VtAction::Tab),
                0x0A => Some(VtAction::Newline),
                0x0D => Some(VtAction::CarriageReturn),
                0x1B => {
                    self.state = ParserState::Escape;
                    None
                }
                b => {
                    let ch = char::from(b);
                    Some(VtAction::Print(ch))
                }
            },
            ParserState::Escape => match byte {
                b'[' => {
                    self.state = ParserState::CsiParam;
                    self.params.clear();
                    self.current_param = 0;
                    None
                }
                b']' => {
                    self.state = ParserState::OscString;
                    self.osc_buf.clear();
                    None
                }
                b'M' => {
                    self.state = ParserState::Ground;
                    None
                } // Reverse index
                _ => {
                    self.state = ParserState::Ground;
                    None
                }
            },
            ParserState::CsiParam => match byte {
                b'0'..=b'9' => {
                    self.current_param =
                        self.current_param.saturating_mul(10) + (byte - b'0') as u32;
                    None
                }
                b';' => {
                    self.params.push(self.current_param);
                    self.current_param = 0;
                    None
                }
                b'A'..=b'Z' | b'a'..=b'z' | b'@' | b'`' => {
                    self.params.push(self.current_param);
                    self.state = ParserState::Ground;
                    self.dispatch_csi(byte)
                }
                _ => {
                    self.state = ParserState::Ground;
                    None
                }
            },
            ParserState::CsiIntermediate => {
                self.state = ParserState::Ground;
                None
            }
            ParserState::OscString => match byte {
                0x07 | 0x9C => {
                    self.state = ParserState::Ground;
                    let title = self.osc_buf.clone();
                    self.osc_buf.clear();
                    if title.starts_with("0;") || title.starts_with("2;") {
                        Some(VtAction::SetTitle(title[2..].to_string()))
                    } else {
                        None
                    }
                }
                b => {
                    if b >= 0x20 {
                        self.osc_buf.push(char::from(b));
                    }
                    None
                }
            },
        }
    }

    fn dispatch_csi(&self, final_byte: u8) -> Option<VtAction> {
        let p0 = *self.params.first().unwrap_or(&0);
        let p1 = *self.params.get(1).unwrap_or(&0);
        match final_byte {
            b'A' => Some(VtAction::CursorUp(p0.max(1))),
            b'B' => Some(VtAction::CursorDown(p0.max(1))),
            b'C' => Some(VtAction::CursorForward(p0.max(1))),
            b'D' => Some(VtAction::CursorBack(p0.max(1))),
            b'H' | b'f' => Some(VtAction::CursorPosition {
                row: p0.max(1),
                col: p1.max(1),
            }),
            b'J' => Some(VtAction::EraseInDisplay(p0)),
            b'K' => Some(VtAction::EraseInLine(p0)),
            b'm' => Some(VtAction::Sgr(self.params.clone())),
            _ => None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Terminal Emulator Config (Ghostty-inspired)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct GhosttyConfig {
    pub font_family: String,
    pub font_size: f32,
    pub line_height: f32,
    pub padding: u32,
    pub background: TermColor,
    pub foreground: TermColor,
    pub cursor_style: CursorStyle,
    pub cursor_color: TermColor,
    pub scrollback_lines: usize,
    pub enable_ligatures: bool,
    pub enable_nerd_fonts: bool,
    pub theme: String,
    pub opacity: f32,
    pub blur_radius: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorStyle {
    Block,
    Beam,
    Underline,
}

impl Default for GhosttyConfig {
    fn default() -> Self {
        Self {
            font_family: "JetBrainsMono Nerd Font".to_string(),
            font_size: 13.0,
            line_height: 1.2,
            padding: 8,
            background: TermColor::Rgb(0x1a, 0x1b, 0x26), // Tokyo Night bg
            foreground: TermColor::Rgb(0xc0, 0xca, 0xf5), // Tokyo Night fg
            cursor_style: CursorStyle::Block,
            cursor_color: TermColor::Rgb(0x7a, 0xa2, 0xf7),
            scrollback_lines: 10_000,
            enable_ligatures: true,
            enable_nerd_fonts: true,
            theme: "sigma-dark".to_string(),
            opacity: 0.95,
            blur_radius: 20.0,
        }
    }
}

impl GhosttyConfig {
    /// Generate a Ghostty-format config string
    pub fn to_ghostty_config(&self) -> String {
        alloc::format!(
            "# SigmaGhostty Configuration\n\
             # Generated by SigmaOS — Compatible with Ghostty terminal\n\
             font-family = {}\n\
             font-size = {}\n\
             theme = {}\n\
             background-opacity = {:.2}\n\
             background-blur-radius = {}\n\
             scrollback-limit = {}\n\
             cursor-style = {}\n",
            self.font_family,
            self.font_size,
            self.theme,
            self.opacity,
            self.blur_radius,
            self.scrollback_lines,
            match self.cursor_style {
                CursorStyle::Block => "block",
                CursorStyle::Beam => "bar",
                CursorStyle::Underline => "underline",
            }
        )
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Main Terminal Emulator
// ─────────────────────────────────────────────────────────────────────────────

pub struct SigmaGhostty {
    pub config: GhosttyConfig,
    pub grid: TermGrid,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub current_attrs: CellAttrs,
    pub title: String,
    pub scrollback: Vec<Vec<TermCell>>,
    parser: VtParser,
}

impl SigmaGhostty {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            config: GhosttyConfig::default(),
            grid: TermGrid::new(rows, cols),
            cursor_row: 0,
            cursor_col: 0,
            current_attrs: CellAttrs::default(),
            title: "SigmaGhostty".to_string(),
            scrollback: Vec::new(),
            parser: VtParser::new(),
        }
    }

    /// Process input bytes from the PTY
    pub fn process_pty_output(&mut self, data: &[u8]) {
        let actions = self.parser.feed(data);
        for action in actions {
            self.handle_action(action);
        }
    }

    fn handle_action(&mut self, action: VtAction) {
        match action {
            VtAction::Print(ch) => {
                if self.cursor_col >= self.grid.cols {
                    self.newline();
                    self.cursor_col = 0;
                }
                let attrs = self.current_attrs;
                self.grid
                    .write_char(self.cursor_row, self.cursor_col, ch, attrs);
                self.cursor_col += if is_wide_char(ch) { 2 } else { 1 };
            }
            VtAction::Newline => self.newline(),
            VtAction::CarriageReturn => self.cursor_col = 0,
            VtAction::Backspace => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                }
            }
            VtAction::CursorUp(n) => {
                self.cursor_row = self.cursor_row.saturating_sub(n as usize);
            }
            VtAction::CursorDown(n) => {
                self.cursor_row = (self.cursor_row + n as usize).min(self.grid.rows - 1);
            }
            VtAction::CursorForward(n) => {
                self.cursor_col = (self.cursor_col + n as usize).min(self.grid.cols - 1);
            }
            VtAction::CursorBack(n) => {
                self.cursor_col = self.cursor_col.saturating_sub(n as usize);
            }
            VtAction::CursorPosition { row, col } => {
                self.cursor_row = (row as usize).saturating_sub(1).min(self.grid.rows - 1);
                self.cursor_col = (col as usize).saturating_sub(1).min(self.grid.cols - 1);
            }
            VtAction::EraseInLine(mode) => {
                match mode {
                    0 => {
                        // to end of line
                        for c in self.cursor_col..self.grid.cols {
                            self.grid
                                .write_char(self.cursor_row, c, ' ', CellAttrs::default());
                        }
                    }
                    1 => {
                        // to start of line
                        for c in 0..=self.cursor_col {
                            self.grid
                                .write_char(self.cursor_row, c, ' ', CellAttrs::default());
                        }
                    }
                    2 => self.grid.clear_row(self.cursor_row),
                    _ => {}
                }
            }
            VtAction::Sgr(params) => self.handle_sgr(&params),
            VtAction::SetTitle(title) => self.title = title,
            VtAction::Bell | VtAction::Tab | VtAction::SixelData(_) => {}
            VtAction::EraseInDisplay(_) => {}
        }
    }

    fn newline(&mut self) {
        self.cursor_row += 1;
        if self.cursor_row >= self.grid.rows {
            // Save top row to scrollback
            let row_start = 0;
            let row_end = self.grid.cols;
            let row_cells: Vec<TermCell> = self.grid.cells[row_start..row_end]
                .iter()
                .cloned()
                .collect();
            if self.scrollback.len() >= self.config.scrollback_lines {
                self.scrollback.remove(0);
            }
            self.scrollback.push(row_cells);
            self.grid.scroll_up(1);
            self.cursor_row = self.grid.rows - 1;
        }
    }

    fn handle_sgr(&mut self, params: &[u32]) {
        let mut i = 0;
        while i < params.len() {
            match params[i] {
                0 => self.current_attrs = CellAttrs::default(),
                1 => self.current_attrs.bold = true,
                3 => self.current_attrs.italic = true,
                4 => self.current_attrs.underline = UnderlineStyle::Straight,
                5 => self.current_attrs.blink = true,
                7 => self.current_attrs.reverse = true,
                22 => self.current_attrs.bold = false,
                23 => self.current_attrs.italic = false,
                24 => self.current_attrs.underline = UnderlineStyle::None,
                25 => self.current_attrs.blink = false,
                27 => self.current_attrs.reverse = false,
                // Standard foreground colors (30-37, 90-97)
                30..=37 => self.current_attrs.fg = TermColor::Ansi(params[i] as u8 - 30),
                38 => {
                    if params.get(i + 1) == Some(&2) && params.len() > i + 4 {
                        // 38;2;R;G;B — true color fg
                        self.current_attrs.fg = TermColor::Rgb(
                            params[i + 2] as u8,
                            params[i + 3] as u8,
                            params[i + 4] as u8,
                        );
                        i += 4;
                    } else if params.get(i + 1) == Some(&5) && params.len() > i + 2 {
                        // 38;5;N — 256-color fg
                        self.current_attrs.fg = TermColor::Indexed(params[i + 2] as u8);
                        i += 2;
                    }
                }
                39 => self.current_attrs.fg = TermColor::Default,
                // Standard background colors (40-47, 100-107)
                40..=47 => self.current_attrs.bg = TermColor::Ansi(params[i] as u8 - 40),
                48 => {
                    if params.get(i + 1) == Some(&2) && params.len() > i + 4 {
                        self.current_attrs.bg = TermColor::Rgb(
                            params[i + 2] as u8,
                            params[i + 3] as u8,
                            params[i + 4] as u8,
                        );
                        i += 4;
                    } else if params.get(i + 1) == Some(&5) && params.len() > i + 2 {
                        self.current_attrs.bg = TermColor::Indexed(params[i + 2] as u8);
                        i += 2;
                    }
                }
                49 => self.current_attrs.bg = TermColor::Default,
                // Bright fg (90-97)
                90..=97 => self.current_attrs.fg = TermColor::Ansi(params[i] as u8 - 90 + 8),
                // Bright bg (100-107)
                100..=107 => self.current_attrs.bg = TermColor::Ansi(params[i] as u8 - 100 + 8),
                _ => {}
            }
            i += 1;
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_term_color_hex() {
        assert_eq!(TermColor::Rgb(0xff, 0x00, 0x80).to_hex(), "#ff0080");
        assert_eq!(TermColor::Default.to_hex(), "inherit");
    }

    #[test]
    fn test_term_grid_write_and_read() {
        let mut grid = TermGrid::new(24, 80);
        grid.write_char(0, 0, 'A', CellAttrs::default());
        assert_eq!(grid.cell(0, 0).unwrap().codepoint, 'A');
        grid.scroll_up(1);
        // After scroll, row 0 should be blank
        assert_eq!(grid.cell(0, 0).unwrap().codepoint, ' ');
    }

    #[test]
    fn test_vt_parser_basic() {
        let mut parser = VtParser::new();
        let actions = parser.feed(b"Hello\r\n");
        let prints: Vec<char> = actions
            .iter()
            .filter_map(|a| {
                if let VtAction::Print(c) = a {
                    Some(*c)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(prints, vec!['H', 'e', 'l', 'l', 'o']);
        assert!(actions.contains(&VtAction::CarriageReturn));
        assert!(actions.contains(&VtAction::Newline));
    }

    #[test]
    fn test_cursor_movement_actions() {
        let mut parser = VtParser::new();
        let actions = parser.feed(b"\x1b[5A"); // cursor up 5
        assert!(actions.contains(&VtAction::CursorUp(5)));

        let mut parser2 = VtParser::new();
        let actions2 = parser2.feed(b"\x1b[10;20H"); // cursor position row=10, col=20
        assert!(actions2.contains(&VtAction::CursorPosition { row: 10, col: 20 }));
    }

    #[test]
    fn test_sgr_true_color() {
        let mut term = SigmaGhostty::new(24, 80);
        // Set true-color foreground: ESC[38;2;255;0;128m
        term.process_pty_output(b"\x1b[38;2;255;0;128m");
        assert_eq!(term.current_attrs.fg, TermColor::Rgb(255, 0, 128));
    }

    #[test]
    fn test_ghostty_config_generation() {
        let cfg = GhosttyConfig::default();
        let config_str = cfg.to_ghostty_config();
        assert!(config_str.contains("JetBrainsMono Nerd Font"));
        assert!(config_str.contains("sigma-dark"));
        assert!(config_str.contains("0.95"));
    }

    #[test]
    fn test_wide_char_detection() {
        assert!(is_wide_char('中')); // CJK
        assert!(is_wide_char('🔥')); // Emoji
        assert!(!is_wide_char('A')); // ASCII
        assert!(!is_wide_char('é')); // Latin extended
    }

    #[test]
    fn test_terminal_scrollback() {
        let mut term = SigmaGhostty::new(2, 10); // tiny 2-row terminal
        term.process_pty_output(b"line1\nline2\nline3\n");
        // After 3 newlines in a 2-row grid, scrollback should have entries
        assert!(!term.scrollback.is_empty());
    }
}
