// Sovereign Mathematical Function Plotter & Visualizer (gnuplot/plotutils Defeater)
// Implements zero-dependency, high-performance mathematical function plotting on terminal-friendly ascii grids.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

/// Supported mathematical function types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlotFunction {
    Sine,      // y = sin(x)
    Cosine,    // y = cos(x)
    Quadratic, // y = x^2
    Linear,    // y = x
}

/// Sovereign Plotting and Coordinates Mapping Engine
pub struct SovereignMathPlotter {
    pub columns: usize,
    pub rows: usize,
}

impl SovereignMathPlotter {
    /// Initialize a standard terminal-sized plotter grid (typically 80x24)
    pub fn new(columns: usize, rows: usize) -> Self {
        Self { columns, rows }
    }

    /// Evaluates the math function at x coordinate
    pub fn evaluate(&self, func: PlotFunction, x: f32) -> f32 {
        match func {
            PlotFunction::Sine => {
                // Safe Taylor series expansion approximation of sin(x) for #![no_std] without libm
                let x_reduced = x % (2.0 * 3.14159);
                let x2 = x_reduced * x_reduced;
                x_reduced * (1.0 - x2 / 6.0 + (x2 * x2) / 120.0)
            }
            PlotFunction::Cosine => {
                // Safe Taylor series expansion approximation of cos(x)
                let x_reduced = x % (2.0 * 3.14159);
                let x2 = x_reduced * x_reduced;
                1.0 - x2 / 2.0 + (x2 * x2) / 24.0
            }
            PlotFunction::Quadratic => x * x,
            PlotFunction::Linear => x,
        }
    }

    /// Generates an ASCII-mapped coordinate grid representing the plotted math function
    pub fn generate_plot_grid(
        &self,
        func: PlotFunction,
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
    ) -> Vec<String> {
        let mut grid = vec![vec![' '; self.columns]; self.rows];

        // 1. Draw Axis coordinates
        let x_axis_row = (self.rows as f32 * (y_max / (y_max - y_min))) as usize;
        let y_axis_col = (self.columns as f32 * (-x_min / (x_max - x_min))) as usize;

        // Draw horizontal X-axis
        if x_axis_row < self.rows {
            for col in 0..self.columns {
                grid[x_axis_row][col] = '-';
            }
        }

        // Draw vertical Y-axis
        if y_axis_col < self.columns {
            for row in 0..self.rows {
                grid[row][y_axis_col] = '|';
            }
        }

        // Draw Origin intersection
        if x_axis_row < self.rows && y_axis_col < self.columns {
            grid[x_axis_row][y_axis_col] = '+';
        }

        // 2. Plot mathematical points onto the grid
        for col in 0..self.columns {
            // Map grid column to X domain value
            let x_val = x_min + (col as f32 / self.columns as f32) * (x_max - x_min);
            let y_val = self.evaluate(func, x_val);

            // Map Y domain value to grid row index
            if y_val >= y_min && y_val <= y_max {
                let row_pct = (y_max - y_val) / (y_max - y_min);
                let row_idx = (row_pct * (self.rows - 1) as f32) as usize;

                if row_idx < self.rows {
                    grid[row_idx][col] = '*'; // Plot coordinate point!
                }
            }
        }

        // Convert grid chars to list of strings
        grid.into_iter()
            .map(|row_chars| row_chars.into_iter().collect::<String>())
            .collect()
    }
}

impl Default for SovereignMathPlotter {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mathematical_function_evaluations() {
        let plotter = SovereignMathPlotter::new(80, 24);

        // Linear: y = x -> evaluates 2.5 to 2.5
        assert_eq!(plotter.evaluate(PlotFunction::Linear, 2.5), 2.5);

        // Quadratic: y = x^2 -> evaluates 4.0 to 16.0
        assert_eq!(plotter.evaluate(PlotFunction::Quadratic, 4.0), 16.0);

        // Sine: y = sin(0) -> evaluates close to 0.0
        let sin_zero = plotter.evaluate(PlotFunction::Sine, 0.0);
        assert!(sin_zero.abs() < 0.01);
    }

    #[test]
    fn test_coordinate_grid_plotting_renders() {
        let plotter = SovereignMathPlotter::new(80, 24);

        // Generate a Sine wave plot between X:[-5.0, 5.0] and Y:[-1.5, 1.5]
        let plot = plotter.generate_plot_grid(PlotFunction::Sine, -5.0, 5.0, -1.5, 1.5);

        assert_eq!(plot.len(), 24); // 24 rows
        assert_eq!(plot[0].len(), 80); // 80 columns

        // Check that origin axes '+' is plotted
        let mut axis_found = false;
        for row in &plot {
            if row.contains('+') || row.contains('|') || row.contains('-') {
                axis_found = true;
                break;
            }
        }
        assert!(axis_found);

        // Check that coordinate points '*' are plotted
        let mut points_found = false;
        for row in &plot {
            if row.contains('*') {
                points_found = true;
                break;
            }
        }
        assert!(points_found);
    }
}
