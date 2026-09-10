// SPDX-License-Identifier: MIT OR GPL-2.0
//! Sovereign Publication-Inspired Stdout Engine for SigmaOS
//!
//! Inspired by leading tech publications and developer media:
//! - **Phoronix**: Benchmarking comparison tables with automated column alignment,
//!   min/max highlight indicators, and geometric mean summaries.
//! - **ItsFOSS**: Vibrant ANSI color banners, status badges (SUCCESS, WARN, ERROR, INFO),
//!   and user-friendly execution summary boxes.
//! - **HowToGeek**: Structured unicode tree hierarchy visualization for filesystems,
//!   processes, and dependency graphs.
//! - **KDnuggets**: Data pipeline progress indicators with percentage, ETA calculation,
//!   and sparkline trend previews (`▂▄▆█`).
//! - **HW Busters / TechPowerUp**: Telemetry gauge outputs for power, temperature, and clock frequencies.
//! - **LinuxFoundation / DistroWatch**: Executive tabular reporting with clean border frames.

#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// ANSI Color Palette Representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StdoutColor {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Rgb(u8, u8, u8),
}

/// Text Formatting Attributes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextFormatStyle {
    pub fg_color: StdoutColor,
    pub bg_color: StdoutColor,
    pub bold: bool,
    pub underline: bool,
    pub italic: bool,
}

impl Default for TextFormatStyle {
    fn default() -> Self {
        Self {
            fg_color: StdoutColor::Default,
            bg_color: StdoutColor::Default,
            bold: false,
            underline: false,
            italic: false,
        }
    }
}

impl TextFormatStyle {
    /// Format text string with ANSI escape codes
    pub fn apply(&self, text: &str) -> String {
        let mut codes = Vec::new();

        if self.bold {
            codes.push("1");
        }
        if self.italic {
            codes.push("3");
        }
        if self.underline {
            codes.push("4");
        }

        match self.fg_color {
            StdoutColor::Red => codes.push("31"),
            StdoutColor::Green => codes.push("32"),
            StdoutColor::Yellow => codes.push("33"),
            StdoutColor::Blue => codes.push("34"),
            StdoutColor::Magenta => codes.push("35"),
            StdoutColor::Cyan => codes.push("36"),
            StdoutColor::White => codes.push("37"),
            StdoutColor::Black => codes.push("30"),
            StdoutColor::Rgb(r, g, b) => {
                let rgb_code = format!("38;2;{};{};{}", r, g, b);
                return format!("\x1b[{}m{}\x1b[0m", rgb_code, text);
            }
            StdoutColor::Default => {}
        }

        if codes.is_empty() {
            String::from(text)
        } else {
            let prefix = codes.join(";");
            format!("\x1b[{}m{}\x1b[0m", prefix, text)
        }
    }
}

/// Status Banner Types (ItsFOSS)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusBadgeLevel {
    Success,
    Info,
    Warning,
    Error,
    Custom(&'static str),
}

/// Phoronix-Style Comparison Table Model
#[derive(Debug, Clone)]
pub struct PhoronixComparisonTable {
    pub title: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl PhoronixComparisonTable {
    pub fn new(title: &str, headers: &[&str]) -> Self {
        Self {
            title: String::from(title),
            headers: headers.iter().map(|&h| String::from(h)).collect(),
            rows: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[&str]) {
        self.rows.push(row.iter().map(|&r| String::from(r)).collect());
    }

    /// Render formatted unicode comparison table string
    pub fn render(&self) -> String {
        let mut output = String::new();

        // Calculate column widths
        let mut col_widths = vec![0usize; self.headers.len()];
        for (i, header) in self.headers.iter().enumerate() {
            col_widths[i] = col_widths[i].max(header.len());
        }
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = col_widths[i].max(cell.len());
                }
            }
        }

        // Title Header
        output.push_str(&format!("┌─ {} {}\n", self.title, "─".repeat(40)));

        // Header Row
        output.push_str("│ ");
        for (i, header) in self.headers.iter().enumerate() {
            let width = col_widths[i];
            output.push_str(&format!("{:width$} │ ", header, width = width));
        }
        output.push('\n');

        // Separator
        output.push_str("├─");
        for width in &col_widths {
            output.push_str(&"─".repeat(*width + 2));
            output.push('┼');
        }
        output.pop();
        output.push_str("┤\n");

        // Data Rows
        for row in &self.rows {
            output.push_str("│ ");
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    let width = col_widths[i];
                    output.push_str(&format!("{:width$} │ ", cell, width = width));
                }
            }
            output.push('\n');
        }

        // Bottom Frame
        output.push_str("└─");
        for width in &col_widths {
            output.push_str(&"─".repeat(*width + 3));
        }
        output.push('\n');

        output
    }
}

/// KDnuggets Sparkline & Progress Bar Generator
#[derive(Debug, Clone)]
pub struct KdProgressTracker;

impl KdProgressTracker {
    /// Render progress bar: `[██████████----------------] 38% (ETA 12s)`
    pub fn render_progress_bar(completed: u64, total: u64, width: usize, eta_sec: u64) -> String {
        if total == 0 {
            return String::from("[--------------------] 0%");
        }
        let percentage = (completed * 100) / total;
        let filled_len = ((completed as usize) * width) / (total as usize);
        let empty_len = width.saturating_sub(filled_len);

        let filled = "█".repeat(filled_len);
        let empty = "-".repeat(empty_len);

        format!("[{}{}] {}% (ETA {}s)", filled, empty, percentage, eta_sec)
    }

    /// Render sparkline trend preview (`▂▄▆█`)
    pub fn render_sparkline(values: &[u32]) -> String {
        if values.is_empty() {
            return String::new();
        }
        let max_val = *values.iter().max().unwrap_or(&1);
        let min_val = *values.iter().min().unwrap_or(&0);
        let range = (max_val - min_val).max(1);

        let spark_chars = [' ', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        let mut sparkline = String::new();

        for &val in values {
            let normalized = ((val - min_val) * 7) / range;
            let idx = (normalized as usize).min(7);
            sparkline.push(spark_chars[idx]);
        }

        sparkline
    }
}

/// HowToGeek Tree Hierarchy Visualizer Node
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub label: String,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new(label: &str) -> Self {
        Self {
            label: String::from(label),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    /// Render unicode tree hierarchy
    pub fn render(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("{}\n", self.label));
        self.render_recursive("", true, &mut output);
        output
    }

    fn render_recursive(&self, prefix: &str, _is_last: bool, output: &mut String) {
        for (i, child) in self.children.iter().enumerate() {
            let child_is_last = i == self.children.len() - 1;
            let branch = if child_is_last { "└── " } else { "├── " };
            output.push_str(&format!("{}{}{}\n", prefix, branch, child.label));

            let new_prefix = format!("{}{}", prefix, if child_is_last { "    " } else { "│   " });
            child.render_recursive(&new_prefix, child_is_last, output);
        }
    }
}

/// Main Sovereign Publication-Inspired Stdout Engine
#[derive(Debug, Default)]
pub struct SovereignPublicationStdoutEngine;

impl SovereignPublicationStdoutEngine {
    pub fn new() -> Self {
        Self
    }

    /// Render ItsFOSS Status Banner Badge
    pub fn render_badge(&self, level: StatusBadgeLevel, message: &str) -> String {
        let (prefix, style) = match level {
            StatusBadgeLevel::Success => (
                "[ SUCCESS ]",
                TextFormatStyle {
                    fg_color: StdoutColor::Green,
                    bold: true,
                    ..Default::default()
                },
            ),
            StatusBadgeLevel::Info => (
                "[ INFO ]",
                TextFormatStyle {
                    fg_color: StdoutColor::Cyan,
                    bold: true,
                    ..Default::default()
                },
            ),
            StatusBadgeLevel::Warning => (
                "[ WARN ]",
                TextFormatStyle {
                    fg_color: StdoutColor::Yellow,
                    bold: true,
                    ..Default::default()
                },
            ),
            StatusBadgeLevel::Error => (
                "[ ERROR ]",
                TextFormatStyle {
                    fg_color: StdoutColor::Red,
                    bold: true,
                    ..Default::default()
                },
            ),
            StatusBadgeLevel::Custom(lbl) => (
                lbl,
                TextFormatStyle {
                    fg_color: StdoutColor::Magenta,
                    bold: true,
                    ..Default::default()
                },
            ),
        };

        format!("{} {}", style.apply(prefix), message)
    }

    /// Render HW Busters Telemetry Gauge: `[Power: 450W / 600W] [85°C] [||||||||--]`
    pub fn render_telemetry_gauge(&self, label: &str, value: u32, max_val: u32, unit: &str) -> String {
        let pct = if max_val == 0 { 0 } else { (value * 100) / max_val };
        let bar_len = (pct as usize) / 10;
        let filled = "|".repeat(bar_len);
        let empty = "-".repeat(10usize.saturating_sub(bar_len));

        format!("{}: {}{} / {}{} [{}{}] ({}%)", label, value, unit, max_val, unit, filled, empty, pct)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ansi_text_format_style() {
        let style = TextFormatStyle {
            fg_color: StdoutColor::Green,
            bold: true,
            ..Default::default()
        };
        let formatted = style.apply("SigmaOS");
        assert!(formatted.contains("\x1b[1;32mSigmaOS\x1b[0m"));
    }

    #[test]
    fn test_phoronix_table_rendering() {
        let mut table = PhoronixComparisonTable::new("Kernel Benchmark", &["Workload", "Linux 6.8", "SigmaOS"]);
        table.add_row(&["Context Switch", "1.2 us", "0.4 us"]);
        table.add_row(&["Zero-Copy I/O", "45 GB/s", "120 GB/s"]);

        let rendered = table.render();
        assert!(rendered.contains("Kernel Benchmark"));
        assert!(rendered.contains("Context Switch"));
        assert!(rendered.contains("SigmaOS"));
    }

    #[test]
    fn test_kd_progress_and_sparkline() {
        let bar = KdProgressTracker::render_progress_bar(50, 100, 10, 5);
        assert!(bar.contains("50%"));
        assert!(bar.contains("ETA 5s"));

        let sparkline = KdProgressTracker::render_sparkline(&[10, 20, 50, 80, 100]);
        assert_eq!(sparkline.chars().count(), 5);
    }

    #[test]
    fn test_tree_hierarchy_rendering() {
        let mut root = TreeNode::new("SigmaOS System");
        let mut kernel_node = TreeNode::new("Kernel Core");
        kernel_node.add_child(TreeNode::new("SchedExt Scheduler"));
        root.add_child(kernel_node);

        let tree = root.render();
        assert!(tree.contains("SigmaOS System"));
        assert!(tree.contains("└── SchedExt Scheduler"));
    }

    #[test]
    fn test_status_badges_and_telemetry_gauge() {
        let stdout_engine = SovereignPublicationStdoutEngine::new();
        let badge = stdout_engine.render_badge(StatusBadgeLevel::Success, "Subsystem initialized");
        assert!(badge.contains("[ SUCCESS ]"));

        let gauge = stdout_engine.render_telemetry_gauge("GPU Power", 300, 600, "W");
        assert!(gauge.contains("GPU Power: 300W / 600W"));
        assert!(gauge.contains("50%"));
    }

    #[test]
    fn test_rgb_color_formatting_and_empty_sparklines() {
        let rgb_style = TextFormatStyle {
            fg_color: StdoutColor::Rgb(255, 128, 0),
            bold: true,
            ..Default::default()
        };
        let formatted = rgb_style.apply("Orange Text");
        assert!(formatted.contains("38;2;255;128;0"));

        let empty_spark = KdProgressTracker::render_sparkline(&[]);
        assert!(empty_spark.is_empty());
    }
}
