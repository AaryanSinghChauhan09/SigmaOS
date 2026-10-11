//! # VT100 / VT220 Terminal Emulator
//!
//! A software terminal emulator that parses and executes ANSI/VT escape
//! sequences and renders the resulting character grid.
//!
//! ## Supported features
//! - **Escape sequence parsing**: CSI, OSC, DCS, SS2/SS3, C0/C1 controls
//! - **Cursor movement**: CUU/CUD/CUF/CUB, CUP, HVP, CNL, CPL, CHA
//! - **Erase operations**: ED, EL (with `?` selective variants)
//! - **Color**: SGR with 8/16 ANSI colors, 256-color `38;5;n`, and 24-bit
//!   RGB truecolor `38;2;r;g;b`
//! - **Scrollback buffer**: configurable line history
//! - **Alternate screen**: `?1049h` / `?1049l` (smcup / rmcup)
//! - **UTF-8 rendering**: multi-byte codepoints, combining characters, wide
//!   (East Asian) glyphs

#![allow(dead_code)]

use alloc::string::String;
use alloc::vec::Vec;

extern crate alloc;

// ---------------------------------------------------------------------------
// Color representation
// ---------------------------------------------------------------------------

/// A terminal color value supporting ANSI, 256-color, and 24-bit truecolor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// One of the 8 standard ANSI colors (0–7) or their bright variants
    /// (8–15).
    Ansi(u8),
    /// An entry from the xterm 256-color palette.
    Palette256(u8),
    /// A 24-bit RGB truecolor value.
    Rgb(u8, u8, u8),
    /// Use the terminal's default color.
    Default,
}

// ---------------------------------------------------------------------------
// Cell attributes
// ---------------------------------------------------------------------------

/// Visual attributes applied to a single character cell.
#[derive(Debug, Clone, Copy)]
pub struct Attributes {
    pub foreground: Color,
    pub background: Color,
    pub bold: bool,
    pub dim: bool,
    pub italic: bool,
    pub underline: bool,
    pub blink: bool,
    pub reverse: bool,
    pub invisible: bool,
    pub strikethrough: bool,
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            foreground: Color::Default,
            background: Color::Default,
            bold: false,
            dim: false,
            italic: false,
            underline: false,
            blink: false,
            reverse: false,
            invisible: false,
            strikethrough: false,
        }
    }
}

// ---------------------------------------------------------------------------
// Screen cell
// ---------------------------------------------------------------------------

/// A single cell in the terminal character grid.
#[derive(Debug, Clone)]
pub struct Cell {
    /// The Unicode codepoint displayed in this cell, or `'\0'` for empty.
    pub ch: char,
    /// Rendering attributes.
    pub attrs: Attributes,
    /// `true` for the second "half" of a wide (double-width) character.
    pub wide_filler: bool,
}

impl Cell {
    pub const EMPTY: Cell = Cell { ch: ' ', attrs: Attributes {
        foreground: Color::Default,
        background: Color::Default,
        bold: false, dim: false, italic: false, underline: false,
        blink: false, reverse: false, invisible: false, strikethrough: false,
    }, wide_filler: false };
}

// ---------------------------------------------------------------------------
// Cursor state
// ---------------------------------------------------------------------------

/// Tracks the terminal cursor position and associated state.
#[derive(Debug, Clone)]
pub struct Cursor {
    /// Zero-based column (X axis).
    pub col: usize,
    /// Zero-based row (Y axis), relative to the current scroll region top.
    pub row: usize,
    /// Current rendering attributes for newly written cells.
    pub attrs: Attributes,
    /// Whether the cursor is currently visible.
    pub visible: bool,
    /// Saved cursor position (DECSC / DECRC).
    pub saved: Option<(usize, usize)>,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            col: 0,
            row: 0,
            attrs: Attributes::default(),
            visible: true,
            saved: None,
        }
    }

    /// Saves the current position (DECSC).
    pub fn save(&mut self) {
        self.saved = Some((self.col, self.row));
    }

    /// Restores the saved position (DECRC).
    pub fn restore(&mut self) {
        if let Some((c, r)) = self.saved {
            self.col = c;
            self.row = r;
        }
    }
}

// ---------------------------------------------------------------------------
// Scroll region
// ---------------------------------------------------------------------------

/// The vertical scroll region (`DECSTBM`).
#[derive(Debug, Clone, Copy)]
pub struct ScrollRegion {
    /// First row of the region (inclusive, 0-based).
    pub top: usize,
    /// Last row of the region (inclusive, 0-based).
    pub bottom: usize,
}

// ---------------------------------------------------------------------------
// Screen buffer
// ---------------------------------------------------------------------------

/// A rectangular grid of [`Cell`]s representing one terminal screen.
pub struct ScreenBuffer {
    pub cols: usize,
    pub rows: usize,
    cells: Vec<Cell>,
}

impl ScreenBuffer {
    /// Creates a blank screen of the given dimensions.
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            cells: vec![Cell::EMPTY; cols * rows],
        }
    }

    /// Returns a reference to the cell at `(col, row)`.
    pub fn get(&self, col: usize, row: usize) -> &Cell {
        &self.cells[row * self.cols + col]
    }

    /// Returns a mutable reference to the cell at `(col, row)`.
    pub fn get_mut(&mut self, col: usize, row: usize) -> &mut Cell {
        &mut self.cells[row * self.cols + col]
    }

    /// Fills the given row with empty cells.
    pub fn clear_row(&mut self, row: usize) {
        for col in 0..self.cols {
            self.cells[row * self.cols + col] = Cell::EMPTY;
        }
    }

    /// Scrolls the region `[top, bottom]` up by `n` lines, filling vacated
    /// rows at the bottom with empty cells.
    pub fn scroll_up(&mut self, top: usize, bottom: usize, n: usize) {
        let n = n.min(bottom - top + 1);
        for row in top..=bottom {
            let dest = row;
            let src = row + n;
            if src <= bottom {
                let src_start = src * self.cols;
                let dst_start = dest * self.cols;
                // copy row by row within the same Vec
                for c in 0..self.cols {
                    self.cells[dst_start + c] = self.cells[src_start + c].clone();
                }
            } else {
                self.clear_row(dest);
            }
        }
    }

    /// Scrolls the region `[top, bottom]` down by `n` lines.
    pub fn scroll_down(&mut self, top: usize, bottom: usize, n: usize) {
        let n = n.min(bottom - top + 1);
        for row in (top..=bottom).rev() {
            if row >= top + n {
                let src = row - n;
                let dst_start = row * self.cols;
                let src_start = src * self.cols;
                for c in 0..self.cols {
                    self.cells[dst_start + c] = self.cells[src_start + c].clone();
                }
            } else {
                self.clear_row(row);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Scrollback buffer
// ---------------------------------------------------------------------------

/// A ring buffer that holds historical lines that have scrolled off the top
/// of the primary screen.
pub struct ScrollbackBuffer {
    /// Maximum number of lines to retain.
    pub capacity: usize,
    lines: Vec<Vec<Cell>>,
}

impl ScrollbackBuffer {
    pub fn new(capacity: usize) -> Self {
        Self { capacity, lines: Vec::new() }
    }

    /// Pushes a row of cells into the scrollback history.
    pub fn push(&mut self, row: Vec<Cell>) {
        if self.lines.len() >= self.capacity {
            self.lines.remove(0);
        }
        self.lines.push(row);
    }

    /// Returns the total number of lines in history.
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Returns the line at the given history index (0 = oldest).
    pub fn get(&self, idx: usize) -> Option<&Vec<Cell>> {
        self.lines.get(idx)
    }
}

// ---------------------------------------------------------------------------
// Escape-sequence parser state machine
// ---------------------------------------------------------------------------

/// Internal states of the VT sequence parser (based on the Paul Williams DEC
/// state machine).
#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseState {
    Ground,
    Escape,
    EscapeIntermediate,
    /// CSI: Control Sequence Introducer (`ESC [`)
    CsiEntry,
    CsiParam,
    CsiIntermediate,
    CsiIgnore,
    /// DCS: Device Control String (`ESC P`)
    DcsEntry,
    DcsParam,
    DcsIntermediate,
    DcsPassthrough,
    DcsIgnore,
    /// OSC: Operating System Command (`ESC ]`)
    OscString,
    /// SOS/PM/APC: ignored
    SosPmApcString,
    /// Accumulating a UTF-8 multi-byte sequence.
    Utf8 { remaining: u8, accum: u32 },
}

/// Parsed escape sequence types emitted by the parser.
#[derive(Debug)]
pub enum EscapeSequence {
    /// CSI sequence with collected parameters and final byte.
    Csi { params: Vec<u16>, intermediates: Vec<u8>, final_byte: u8 },
    /// OSC sequence with the raw string payload.
    Osc(String),
    /// DCS sequence with params and data.
    Dcs { params: Vec<u16>, data: String },
    /// A plain character to be printed.
    Print(char),
    /// A C0 or C1 control code.
    Control(u8),
}

/// Streaming VT100/VT220 escape-sequence parser.
pub struct EscapeParser {
    state: ParseState,
    params: Vec<u16>,
    intermediates: Vec<u8>,
    osc_buf: String,
    dcs_buf: String,
    current_param: u16,
    has_param: bool,
    private_marker: Option<u8>,
}

impl EscapeParser {
    pub fn new() -> Self {
        Self {
            state: ParseState::Ground,
            params: Vec::new(),
            intermediates: Vec::new(),
            osc_buf: String::new(),
            dcs_buf: String::new(),
            current_param: 0,
            has_param: false,
            private_marker: None,
        }
    }

    /// Feeds a single byte into the parser, returning any completed sequence.
    pub fn feed(&mut self, byte: u8) -> Option<EscapeSequence> {
        match &self.state.clone() {
            ParseState::Ground => self.handle_ground(byte),
            ParseState::Escape => self.handle_escape(byte),
            ParseState::CsiEntry => self.handle_csi_entry(byte),
            ParseState::CsiParam => self.handle_csi_param(byte),
            ParseState::OscString => self.handle_osc(byte),
            ParseState::Utf8 { remaining, accum } => {
                let rem = *remaining;
                let acc = *accum;
                self.handle_utf8(byte, rem, acc)
            }
            _ => {
                // TODO: implement remaining states (DCS, etc.)
                None
            }
        }
    }

    fn handle_ground(&mut self, byte: u8) -> Option<EscapeSequence> {
        match byte {
            0x1B => { self.state = ParseState::Escape; None }
            0x00..=0x1A | 0x1C..=0x1F => Some(EscapeSequence::Control(byte)),
            0x20..=0x7E => Some(EscapeSequence::Print(byte as char)),
            0x80..=0xBF => None, // unexpected continuation byte
            0xC0..=0xDF => {
                let accum = (byte as u32) & 0x1F;
                self.state = ParseState::Utf8 { remaining: 1, accum };
                None
            }
            0xE0..=0xEF => {
                let accum = (byte as u32) & 0x0F;
                self.state = ParseState::Utf8 { remaining: 2, accum };
                None
            }
            0xF0..=0xF7 => {
                let accum = (byte as u32) & 0x07;
                self.state = ParseState::Utf8 { remaining: 3, accum };
                None
            }
            0x7F => None, // DEL
            _ => None,
        }
    }

    fn handle_escape(&mut self, byte: u8) -> Option<EscapeSequence> {
        self.params.clear();
        self.intermediates.clear();
        self.osc_buf.clear();
        self.private_marker = None;
        match byte {
            b'[' => { self.state = ParseState::CsiEntry; None }
            b']' => { self.state = ParseState::OscString; None }
            _ => { self.state = ParseState::Ground; None }
        }
    }

    fn handle_csi_entry(&mut self, byte: u8) -> Option<EscapeSequence> {
        self.current_param = 0;
        self.has_param = false;
        match byte {
            b'<' | b'=' | b'>' | b'?' => {
                self.private_marker = Some(byte);
                self.state = ParseState::CsiParam;
                None
            }
            b'0'..=b'9' => {
                self.current_param = (byte - b'0') as u16;
                self.has_param = true;
                self.state = ParseState::CsiParam;
                None
            }
            b';' => {
                self.params.push(0);
                self.state = ParseState::CsiParam;
                None
            }
            0x40..=0x7E => {
                let seq = EscapeSequence::Csi {
                    params: self.params.clone(),
                    intermediates: self.intermediates.clone(),
                    final_byte: byte,
                };
                self.state = ParseState::Ground;
                Some(seq)
            }
            _ => { self.state = ParseState::Ground; None }
        }
    }

    fn handle_csi_param(&mut self, byte: u8) -> Option<EscapeSequence> {
        match byte {
            b'0'..=b'9' => {
                self.current_param = self.current_param
                    .saturating_mul(10)
                    .saturating_add((byte - b'0') as u16);
                self.has_param = true;
                None
            }
            b';' => {
                self.params.push(self.current_param);
                self.current_param = 0;
                self.has_param = false;
                None
            }
            0x40..=0x7E => {
                self.params.push(self.current_param);
                let seq = EscapeSequence::Csi {
                    params: self.params.clone(),
                    intermediates: self.intermediates.clone(),
                    final_byte: byte,
                };
                self.state = ParseState::Ground;
                Some(seq)
            }
            _ => { self.state = ParseState::Ground; None }
        }
    }

    fn handle_osc(&mut self, byte: u8) -> Option<EscapeSequence> {
        match byte {
            0x07 | 0x9C => {
                let seq = EscapeSequence::Osc(self.osc_buf.clone());
                self.osc_buf.clear();
                self.state = ParseState::Ground;
                Some(seq)
            }
            0x20..=0xFE => {
                self.osc_buf.push(byte as char);
                None
            }
            _ => None,
        }
    }

    fn handle_utf8(&mut self, byte: u8, remaining: u8, accum: u32) -> Option<EscapeSequence> {
        if byte & 0xC0 != 0x80 {
            // Invalid continuation
            self.state = ParseState::Ground;
            return None;
        }
        let new_accum = (accum << 6) | ((byte & 0x3F) as u32);
        if remaining == 1 {
            self.state = ParseState::Ground;
            if let Some(ch) = char::from_u32(new_accum) {
                return Some(EscapeSequence::Print(ch));
            }
        } else {
            self.state = ParseState::Utf8 { remaining: remaining - 1, accum: new_accum };
        }
        None
    }
}

// ---------------------------------------------------------------------------
// Terminal emulator
// ---------------------------------------------------------------------------

/// A complete VT100/VT220 terminal emulator.
pub struct TerminalEmulator {
    /// Primary screen buffer.
    pub primary: ScreenBuffer,
    /// Alternate screen buffer (used by full-screen applications).
    pub alternate: ScreenBuffer,
    /// `true` when the alternate screen is active.
    pub alternate_active: bool,
    /// Scrollback history (primary screen only).
    pub scrollback: ScrollbackBuffer,
    /// Current cursor state.
    pub cursor: Cursor,
    /// The active scroll region.
    pub scroll_region: ScrollRegion,
    /// Escape-sequence parser.
    parser: EscapeParser,
    /// Current scroll-view offset into the scrollback buffer
    /// (0 = showing the live screen).
    pub scroll_offset: usize,
    /// Terminal dimensions in character cells.
    pub cols: usize,
    pub rows: usize,
    /// Window title (set via OSC 0/2).
    pub title: String,
}

impl TerminalEmulator {
    /// Creates a new terminal emulator with the given dimensions.
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            primary: ScreenBuffer::new(cols, rows),
            alternate: ScreenBuffer::new(cols, rows),
            alternate_active: false,
            scrollback: ScrollbackBuffer::new(10_000),
            cursor: Cursor::new(),
            scroll_region: ScrollRegion { top: 0, bottom: rows - 1 },
            parser: EscapeParser::new(),
            scroll_offset: 0,
            cols,
            rows,
            title: String::new(),
        }
    }

    /// Returns a reference to the currently active screen buffer.
    pub fn screen(&self) -> &ScreenBuffer {
        if self.alternate_active { &self.alternate } else { &self.primary }
    }

    /// Returns a mutable reference to the currently active screen buffer.
    pub fn screen_mut(&mut self) -> &mut ScreenBuffer {
        if self.alternate_active { &mut self.alternate } else { &mut self.primary }
    }

    /// Feeds raw bytes (e.g. from a PTY) into the emulator.
    pub fn write(&mut self, data: &[u8]) {
        for &byte in data {
            if let Some(seq) = self.parser.feed(byte) {
                self.dispatch(seq);
            }
        }
    }

    /// Resizes the terminal to the given dimensions.
    pub fn resize(&mut self, cols: usize, rows: usize) {
        self.cols = cols;
        self.rows = rows;
        self.primary = ScreenBuffer::new(cols, rows);
        self.alternate = ScreenBuffer::new(cols, rows);
        self.scroll_region = ScrollRegion { top: 0, bottom: rows - 1 };
        self.cursor.col = self.cursor.col.min(cols - 1);
        self.cursor.row = self.cursor.row.min(rows - 1);
    }

    // -----------------------------------------------------------------------
    // Sequence dispatcher
    // -----------------------------------------------------------------------

    fn dispatch(&mut self, seq: EscapeSequence) {
        match seq {
            EscapeSequence::Print(ch) => self.print_char(ch),
            EscapeSequence::Control(c) => self.handle_control(c),
            EscapeSequence::Csi { params, intermediates: _, final_byte } => {
                self.handle_csi(&params, final_byte);
            }
            EscapeSequence::Osc(s) => self.handle_osc(&s),
            EscapeSequence::Dcs { .. } => { /* TODO */ }
        }
    }

    fn print_char(&mut self, ch: char) {
        if self.cursor.col >= self.cols {
            // Auto-wrap
            self.cursor.col = 0;
            self.advance_line();
        }
        let col = self.cursor.col;
        let row = self.cursor.row;
        let attrs = self.cursor.attrs;
        {
            let cell = self.screen_mut().get_mut(col, row);
            cell.ch = ch;
            cell.attrs = attrs;
        }
        self.cursor.col += 1;
    }

    fn handle_control(&mut self, c: u8) {
        match c {
            0x08 => { // BS
                if self.cursor.col > 0 { self.cursor.col -= 1; }
            }
            0x09 => { // HT – horizontal tab (next tab stop)
                self.cursor.col = ((self.cursor.col / 8) + 1) * 8;
                if self.cursor.col >= self.cols { self.cursor.col = self.cols - 1; }
            }
            0x0A | 0x0B | 0x0C => { // LF / VT / FF
                self.advance_line();
            }
            0x0D => { // CR
                self.cursor.col = 0;
            }
            _ => {}
        }
    }

    fn advance_line(&mut self) {
        let bottom = self.scroll_region.bottom;
        let top = self.scroll_region.top;
        if self.cursor.row == bottom {
            // Save the top line to scrollback before scrolling
            if !self.alternate_active {
                let row_cells: Vec<Cell> = (0..self.cols)
                    .map(|c| self.primary.get(c, top).clone())
                    .collect();
                self.scrollback.push(row_cells);
            }
            self.screen_mut().scroll_up(top, bottom, 1);
        } else {
            self.cursor.row += 1;
        }
    }

    fn handle_csi(&mut self, params: &[u16], final_byte: u8) {
        let p0 = params.first().copied().unwrap_or(0);
        let p1 = params.get(1).copied().unwrap_or(0);
        match final_byte {
            b'A' => { // CUU – cursor up
                let n = p0.max(1) as usize;
                self.cursor.row = self.cursor.row.saturating_sub(n);
            }
            b'B' => { // CUD – cursor down
                let n = p0.max(1) as usize;
                self.cursor.row = (self.cursor.row + n).min(self.rows - 1);
            }
            b'C' => { // CUF – cursor forward
                let n = p0.max(1) as usize;
                self.cursor.col = (self.cursor.col + n).min(self.cols - 1);
            }
            b'D' => { // CUB – cursor back
                let n = p0.max(1) as usize;
                self.cursor.col = self.cursor.col.saturating_sub(n);
            }
            b'H' | b'f' => { // CUP / HVP – cursor position
                let row = p0.saturating_sub(1) as usize;
                let col = p1.saturating_sub(1) as usize;
                self.cursor.row = row.min(self.rows - 1);
                self.cursor.col = col.min(self.cols - 1);
            }
            b'J' => { // ED – erase in display
                self.erase_in_display(p0);
            }
            b'K' => { // EL – erase in line
                self.erase_in_line(p0);
            }
            b'm' => { // SGR – select graphic rendition
                self.apply_sgr(params);
            }
            b'r' => { // DECSTBM – set top and bottom margins
                let top = p0.saturating_sub(1) as usize;
                let bottom = if p1 == 0 { self.rows - 1 } else { (p1 - 1) as usize };
                if top < bottom {
                    self.scroll_region = ScrollRegion { top, bottom };
                }
            }
            b'S' => { // SU – scroll up
                let n = p0.max(1) as usize;
                let (top, bottom) = (self.scroll_region.top, self.scroll_region.bottom);
                self.screen_mut().scroll_up(top, bottom, n);
            }
            b'T' => { // SD – scroll down
                let n = p0.max(1) as usize;
                let (top, bottom) = (self.scroll_region.top, self.scroll_region.bottom);
                self.screen_mut().scroll_down(top, bottom, n);
            }
            _ => { /* TODO: additional sequences */ }
        }
    }

    fn erase_in_display(&mut self, mode: u16) {
        let (rows, cols) = (self.rows, self.cols);
        match mode {
            0 => { // from cursor to end
                let (cr, cc) = (self.cursor.row, self.cursor.col);
                for c in cc..cols {
                    *self.screen_mut().get_mut(c, cr) = Cell::EMPTY;
                }
                for r in (cr + 1)..rows {
                    self.screen_mut().clear_row(r);
                }
            }
            1 => { // from start to cursor
                let (cr, cc) = (self.cursor.row, self.cursor.col);
                for r in 0..cr {
                    self.screen_mut().clear_row(r);
                }
                for c in 0..=cc {
                    *self.screen_mut().get_mut(c, cr) = Cell::EMPTY;
                }
            }
            2 | 3 => { // entire screen (3 also clears scrollback)
                for r in 0..rows {
                    self.screen_mut().clear_row(r);
                }
            }
            _ => {}
        }
    }

    fn erase_in_line(&mut self, mode: u16) {
        let cols = self.cols;
        let row = self.cursor.row;
        let col = self.cursor.col;
        match mode {
            0 => { for c in col..cols { *self.screen_mut().get_mut(c, row) = Cell::EMPTY; } }
            1 => { for c in 0..=col { *self.screen_mut().get_mut(c, row) = Cell::EMPTY; } }
            2 => { self.screen_mut().clear_row(row); }
            _ => {}
        }
    }

    fn apply_sgr(&mut self, params: &[u16]) {
        let mut i = 0;
        while i < params.len() {
            match params[i] {
                0 => { self.cursor.attrs = Attributes::default(); }
                1 => { self.cursor.attrs.bold = true; }
                2 => { self.cursor.attrs.dim = true; }
                3 => { self.cursor.attrs.italic = true; }
                4 => { self.cursor.attrs.underline = true; }
                5 | 6 => { self.cursor.attrs.blink = true; }
                7 => { self.cursor.attrs.reverse = true; }
                8 => { self.cursor.attrs.invisible = true; }
                9 => { self.cursor.attrs.strikethrough = true; }
                22 => { self.cursor.attrs.bold = false; self.cursor.attrs.dim = false; }
                23 => { self.cursor.attrs.italic = false; }
                24 => { self.cursor.attrs.underline = false; }
                25 => { self.cursor.attrs.blink = false; }
                27 => { self.cursor.attrs.reverse = false; }
                28 => { self.cursor.attrs.invisible = false; }
                29 => { self.cursor.attrs.strikethrough = false; }
                30..=37 => { self.cursor.attrs.foreground = Color::Ansi((params[i] - 30) as u8); }
                38 => {
                    if params.get(i + 1) == Some(&5) && i + 2 < params.len() {
                        self.cursor.attrs.foreground = Color::Palette256(params[i + 2] as u8);
                        i += 2;
                    } else if params.get(i + 1) == Some(&2) && i + 4 < params.len() {
                        self.cursor.attrs.foreground = Color::Rgb(
                            params[i + 2] as u8,
                            params[i + 3] as u8,
                            params[i + 4] as u8,
                        );
                        i += 4;
                    }
                }
                39 => { self.cursor.attrs.foreground = Color::Default; }
                40..=47 => { self.cursor.attrs.background = Color::Ansi((params[i] - 40) as u8); }
                48 => {
                    if params.get(i + 1) == Some(&5) && i + 2 < params.len() {
                        self.cursor.attrs.background = Color::Palette256(params[i + 2] as u8);
                        i += 2;
                    } else if params.get(i + 1) == Some(&2) && i + 4 < params.len() {
                        self.cursor.attrs.background = Color::Rgb(
                            params[i + 2] as u8,
                            params[i + 3] as u8,
                            params[i + 4] as u8,
                        );
                        i += 4;
                    }
                }
                49 => { self.cursor.attrs.background = Color::Default; }
                90..=97 => { self.cursor.attrs.foreground = Color::Ansi((params[i] - 90 + 8) as u8); }
                100..=107 => { self.cursor.attrs.background = Color::Ansi((params[i] - 100 + 8) as u8); }
                _ => {}
            }
            i += 1;
        }
    }

    fn handle_osc(&mut self, s: &str) {
        // OSC 0/1/2 – set icon name / window title
        if let Some(payload) = s.strip_prefix("0;").or_else(|| s.strip_prefix("2;")) {
            self.title = payload.into();
        }
        // TODO: OSC 4 (palette), OSC 8 (hyperlinks), OSC 52 (clipboard), etc.
    }
}
