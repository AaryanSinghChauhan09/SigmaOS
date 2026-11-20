//! GPU cell renderer for SigmaOS-terminal.

use crate::{Color, Result};

/// A single terminal cell.
#[derive(Debug, Clone)]
pub struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::WHITE,
            bg: Color::BLACK,
            bold: false,
            italic: false,
            underline: false,
        }
    }
}

/// The terminal cell grid.
#[derive(Debug)]
pub struct CellGrid {
    pub cols: usize,
    pub rows: usize,
    cells: Vec<Cell>,
}

impl CellGrid {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            cols,
            rows,
            cells: vec![Cell::default(); cols * rows],
        }
    }

    pub fn get(&self, col: usize, row: usize) -> Option<&Cell> {
        if col < self.cols && row < self.rows {
            Some(&self.cells[row * self.cols + col])
        } else {
            None
        }
    }

    pub fn set(&mut self, col: usize, row: usize, cell: Cell) {
        if col < self.cols && row < self.rows {
            self.cells[row * self.cols + col] = cell;
        }
    }

    pub fn clear(&mut self) {
        self.cells.iter_mut().for_each(|c| *c = Cell::default());
    }
}

/// Renderer state (stub — real impl would hold wgpu device/queue).
#[derive(Debug, Default)]
pub struct Renderer {
    pub frame_count: u64,
}

impl Renderer {
    pub fn new() -> Self { Self::default() }

    /// Render one frame given the current cell grid.
    pub fn render(&mut self, _grid: &CellGrid) -> Result<()> {
        self.frame_count += 1;
        Ok(())
    }
}
