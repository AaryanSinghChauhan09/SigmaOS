//! # Flint Chart Engine for SigmaOS
//!
//! A Microsoft Flint-style declarative chart specification language parser and compiler.
//! It allows users to write declarative charting specifications (e.g. data, marks, encodings)
//! and compile/render them into structured chart primitives for the Zenith display compositor.
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlintMark {
    Bar,
    Line,
    Point,
    Area,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlintValue {
    String(String),
    Number(f64),
    Array(Vec<f64>),
}

#[derive(Debug, Clone)]
pub struct FlintEncoding {
    pub x_field: String,
    pub y_field: String,
    pub color_field: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FlintChartSpec {
    pub title: String,
    pub mark: FlintMark,
    pub data_x: Vec<String>,
    pub data_y: Vec<f64>,
    pub encoding: FlintEncoding,
    pub width: u32,
    pub height: u32,
}

pub struct FlintChartEngine;

impl FlintChartEngine {
    /// Parses a simple declarative Flint DSL line-by-line.
    /// Example syntax:
    /// ```text
    /// title: Sales 2026
    /// mark: Bar
    /// data_x: Q1, Q2, Q3, Q4
    /// data_y: 120.5, 150.0, 180.2, 210.0
    /// width: 800
    /// height: 600
    /// x: Quarter
    /// y: Revenue
    /// ```
    pub fn parse_spec(spec_str: &str) -> Result<FlintChartSpec, &'static str> {
        let mut title = String::new();
        let mut mark = FlintMark::Bar;
        let mut data_x = Vec::new();
        let mut data_y = Vec::new();
        let mut x_field = String::from("x");
        let mut y_field = String::from("y");
        let mut color_field = None;
        let mut width = 600;
        let mut height = 400;

        for line in spec_str.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let mut parts = line.splitn(2, ':');
            let key = parts.next().ok_or("Invalid spec syntax")?.trim();
            let val = parts.next().ok_or("Invalid spec syntax")?.trim();

            match key {
                "title" => title = val.to_string(),
                "mark" => {
                    mark = match val {
                        "Bar" => FlintMark::Bar,
                        "Line" => FlintMark::Line,
                        "Point" => FlintMark::Point,
                        "Area" => FlintMark::Area,
                        _ => return Err("Unsupported mark type"),
                    }
                }
                "data_x" => {
                    data_x = val.split(',').map(|s| s.trim().to_string()).collect();
                }
                "data_y" => {
                    data_y = val
                        .split(',')
                        .map(|s| {
                            s.trim()
                                .parse::<f64>()
                                .map_err(|_| "Invalid number in data_y")
                        })
                        .collect::<Result<Vec<f64>, _>>()?;
                }
                "width" => width = val.parse::<u32>().map_err(|_| "Invalid width")?,
                "height" => height = val.parse::<u32>().map_err(|_| "Invalid height")?,
                "x" => x_field = val.to_string(),
                "y" => y_field = val.to_string(),
                "color" => color_field = Some(val.to_string()),
                _ => {} // Ignore unknown keys for future-proofing
            }
        }

        if data_x.len() != data_y.len() {
            return Err("Mismatched data_x and data_y dimensions");
        }

        Ok(FlintChartSpec {
            title,
            mark,
            data_x,
            data_y,
            encoding: FlintEncoding {
                x_field,
                y_field,
                color_field,
            },
            width,
            height,
        })
    }

    /// Renders the Flint Chart spec into raw visual rectangles (representing chart marks)
    /// representing coordinates and sizes, ready to be plotted by the Zenith compositor.
    pub fn compile_to_render_elements(spec: &FlintChartSpec) -> Vec<(String, f32, f32, f32, f32)> {
        let mut elements = Vec::new();
        if spec.data_y.is_empty() {
            return elements;
        }

        let max_val = spec.data_y.iter().copied().fold(0.0f64, f64::max);
        let max_val = if max_val == 0.0 { 1.0 } else { max_val };

        let num_bars = spec.data_y.len();
        let margin_x = 50.0f32;
        let margin_y = 50.0f32;
        let plot_width = spec.width as f32 - (2.0 * margin_x);
        let plot_height = spec.height as f32 - (2.0 * margin_y);

        let bar_width = (plot_width / num_bars as f32) * 0.8;
        let spacing = (plot_width / num_bars as f32) * 0.2;

        for (idx, &val) in spec.data_y.iter().enumerate() {
            let label = &spec.data_x[idx];
            let normalized = val as f32 / max_val as f32;
            let height = plot_height * normalized;

            let x = margin_x + (idx as f32 * (bar_width + spacing));
            let y = spec.height as f32 - margin_y - height;

            elements.push((label.clone(), x, y, bar_width, height));
        }

        elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flint_parser_spec() {
        let spec_str = r#"
            title: Q4 Growth
            mark: Bar
            data_x: Oct, Nov, Dec
            data_y: 100.0, 120.0, 150.0
            width: 800
            height: 600
            x: Month
            y: Performance
        "#;

        let spec = FlintChartEngine::parse_spec(spec_str).unwrap();
        assert_eq!(spec.title, "Q4 Growth");
        assert_eq!(spec.mark, FlintMark::Bar);
        assert_eq!(spec.data_x, vec!["Oct", "Nov", "Dec"]);
        assert_eq!(spec.data_y, vec![100.0, 120.0, 150.0]);
        assert_eq!(spec.width, 800);
        assert_eq!(spec.height, 600);
        assert_eq!(spec.encoding.x_field, "Month");
        assert_eq!(spec.encoding.y_field, "Performance");
    }

    #[test]
    fn test_flint_parser_invalid_dimensions() {
        let spec_str = r#"
            title: Bad Chart
            data_x: Oct, Nov
            data_y: 100.0
        "#;
        let result = FlintChartEngine::parse_spec(spec_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_flint_compile_elements() {
        let spec = FlintChartSpec {
            title: "Quick Test".to_string(),
            mark: FlintMark::Bar,
            data_x: vec!["A".to_string(), "B".to_string()],
            data_y: vec![10.0, 20.0],
            encoding: FlintEncoding {
                x_field: "X".to_string(),
                y_field: "Y".to_string(),
                color_field: None,
            },
            width: 500,
            height: 500,
        };

        let elems = FlintChartEngine::compile_to_render_elements(&spec);
        assert_eq!(elems.len(), 2);
        assert_eq!(elems[0].0, "A");
        assert!(elems[1].4 > elems[0].4, "Second element should be taller");
    }
}
