// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_dataviz.rs — Sigma Data Visualization
//
// Implements data visualization tools for graphing and plotting
// lab results, aligned with CBSE science and mathematics curriculum.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Chart Types ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChartType {
    Line,
    Bar,
    Scatter,
    Pie,
    Histogram,
}

#[derive(Debug, Clone)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    pub label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Dataset {
    pub name: String,
    pub points: Vec<DataPoint>,
    pub color: String,
}

#[derive(Debug, Clone)]
pub struct Chart {
    pub id: String,
    pub title: String,
    pub x_label: String,
    pub y_label: String,
    pub chart_type: ChartType,
    pub datasets: Vec<Dataset>,
    pub show_legend: bool,
    pub show_grid: bool,
}

// ─── Data Visualization Application ───────────────────────────────────────────

pub struct DataViz {
    pub charts: HashMap<String, Chart>,
    pub current_chart: Option<String>,
}

impl DataViz {
    pub fn new() -> Self {
        DataViz {
            charts: HashMap::new(),
            current_chart: None,
        }
    }

    /// Create new chart
    pub fn create_chart(&mut self, id: String, title: String, chart_type: ChartType) {
        let chart = Chart {
            id: id.clone(),
            title,
            x_label: "X Axis".to_string(),
            y_label: "Y Axis".to_string(),
            chart_type,
            datasets: Vec::new(),
            show_legend: true,
            show_grid: true,
        };
        
        self.charts.insert(id.clone(), chart);
        self.current_chart = Some(id);
    }

    /// Add dataset to current chart
    pub fn add_dataset(&mut self, name: String, color: String) -> Result<(), String> {
        if let Some(chart_id) = &self.current_chart {
            if let Some(chart) = self.charts.get_mut(chart_id) {
                chart.datasets.push(Dataset {
                    name,
                    points: Vec::new(),
                    color,
                });
                Ok(())
            } else {
                Err("Chart not found".to_string())
            }
        } else {
            Err("No current chart".to_string())
        }
    }

    /// Add data point to current dataset
    pub fn add_point(&mut self, dataset_index: usize, x: f64, y: f64, label: Option<String>) -> Result<(), String> {
        if let Some(chart_id) = &self.current_chart {
            if let Some(chart) = self.charts.get_mut(chart_id) {
                if dataset_index < chart.datasets.len() {
                    chart.datasets[dataset_index].points.push(DataPoint { x, y, label });
                    Ok(())
                } else {
                    Err("Dataset index out of range".to_string())
                }
            } else {
                Err("Chart not found".to_string())
            }
        } else {
            Err("No current chart".to_string())
        }
    }

    /// Set axis labels
    pub fn set_axis_labels(&mut self, x_label: String, y_label: String) -> Result<(), String> {
        if let Some(chart_id) = &self.current_chart {
            if let Some(chart) = self.charts.get_mut(chart_id) {
                chart.x_label = x_label;
                chart.y_label = y_label;
                Ok(())
            } else {
                Err("Chart not found".to_string())
            }
        } else {
            Err("No current chart".to_string())
        }
    }

    /// Toggle legend
    pub fn toggle_legend(&mut self) -> Result<(), String> {
        if let Some(chart_id) = &self.current_chart {
            if let Some(chart) = self.charts.get_mut(chart_id) {
                chart.show_legend = !chart.show_legend;
                Ok(())
            } else {
                Err("Chart not found".to_string())
            }
        } else {
            Err("No current chart".to_string())
        }
    }

    /// Toggle grid
    pub fn toggle_grid(&mut self) -> Result<(), String> {
        if let Some(chart_id) = &self.current_chart {
            if let Some(chart) = self.charts.get_mut(chart_id) {
                chart.show_grid = !chart.show_grid;
                Ok(())
            } else {
                Err("Chart not found".to_string())
            }
        } else {
            Err("No current chart".to_string())
        }
    }

    /// Generate ASCII art representation of chart
    pub fn render_ascii(&self) -> String {
        if let Some(chart_id) = &self.current_chart {
            if let Some(chart) = self.charts.get(chart_id) {
                match chart.chart_type {
                    ChartType::Line | ChartType::Scatter => self.render_scatter_ascii(chart),
                    ChartType::Bar => self.render_bar_ascii(chart),
                    ChartType::Pie => self.render_pie_ascii(chart),
                    ChartType::Histogram => self.render_histogram_ascii(chart),
                }
            } else {
                "Chart not found".to_string()
            }
        } else {
            "No current chart".to_string()
        }
    }

    /// Render scatter/line chart as ASCII
    fn render_scatter_ascii(&self, chart: &Chart) -> String {
        let mut output = format!("{}\n", chart.title);
        output.push_str(&format!("{}: {}\n", chart.y_label, chart.x_label));
        output.push_str(&format!("{}\n", "-".repeat(40)));
        
        // Simple ASCII scatter plot
        let width = 40;
        let height = 20;
        
        if let Some(dataset) = chart.datasets.first() {
            if dataset.points.is_empty() {
                return "No data points".to_string();
            }
            
            let x_min = dataset.points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
            let x_max = dataset.points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
            let y_min = dataset.points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
            let y_max = dataset.points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
            
            let x_range = x_max - x_min;
            let y_range = y_max - y_min;
            
            let mut grid = vec![vec![' '; width]; height];
            
            for point in &dataset.points {
                let x = ((point.x - x_min) / x_range * (width - 1) as f64) as usize;
                let y = height - 1 - ((point.y - y_min) / y_range * (height - 1) as f64) as usize;
                
                if x < width && y < height {
                    grid[y][x] = '*';
                }
            }
            
            for row in grid {
                output.push_str(&format!("|{}\n", row.iter().collect::<String>()));
            }
        }
        
        output
    }

    /// Render bar chart as ASCII
    fn render_bar_ascii(&self, chart: &Chart) -> String {
        let mut output = format!("{}\n", chart.title);
        output.push_str(&format!("{}\n", "-".repeat(40)));
        
        if let Some(dataset) = chart.datasets.first() {
            let max_y = dataset.points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
            
            for point in &dataset.points {
                let bar_length = (point.y / max_y * 30.0) as usize;
                let label = point.label.as_ref().unwrap_or(&format!("{:.1}", point.x));
                output.push_str(&format!("{} | {}\n", label, "*".repeat(bar_length)));
            }
        }
        
        output
    }

    /// Render pie chart as ASCII
    fn render_pie_ascii(&self, chart: &Chart) -> String {
        let mut output = format!("{}\n", chart.title);
        output.push_str(&format!("{}\n", "-".repeat(40)));
        
        if let Some(dataset) = chart.datasets.first() {
            let total: f64 = dataset.points.iter().map(|p| p.y).sum();
            
            for point in &dataset.points {
                let percentage = (point.y / total * 100.0) as usize;
                let label = point.label.as_ref().unwrap_or(&format!("{:.1}", point.x));
                output.push_str(&format!("{}: {}% ({:.1})\n", label, percentage, point.y));
            }
        }
        
        output
    }

    /// Render histogram as ASCII
    fn render_histogram_ascii(&self, chart: &Chart) -> String {
        let mut output = format!("{}\n", chart.title);
        output.push_str(&format!("{}\n", "-".repeat(40)));
        
        if let Some(dataset) = chart.datasets.first() {
            let max_y = dataset.points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
            
            for point in &dataset.points {
                let bar_length = (point.y / max_y * 30.0) as usize;
                let label = point.label.as_ref().unwrap_or(&format!("{:.1}", point.x));
                output.push_str(&format!("{} | {}\n", label, "#".repeat(bar_length)));
            }
        }
        
        output
    }

    /// Calculate linear regression (y = mx + b)
    pub fn linear_regression(&self, dataset_index: usize) -> Result<(f64, f64), String> {
        if let Some(chart_id) = &self.current_chart {
            if let Some(chart) = self.charts.get(chart_id) {
                if dataset_index < chart.datasets.len() {
                    let points = &chart.datasets[dataset_index].points;
                    
                    if points.len() < 2 {
                        return Err("Need at least 2 points".to_string());
                    }
                    
                    let n = points.len() as f64;
                    let sum_x: f64 = points.iter().map(|p| p.x).sum();
                    let sum_y: f64 = points.iter().map(|p| p.y).sum();
                    let sum_xy: f64 = points.iter().map(|p| p.x * p.y).sum();
                    let sum_x2: f64 = points.iter().map(|p| p.x * p.x).sum();
                    
                    let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
                    let intercept = (sum_y - slope * sum_x) / n;
                    
                    Ok((slope, intercept))
                } else {
                    Err("Dataset index out of range".to_string())
                }
            } else {
                Err("Chart not found".to_string())
            }
        } else {
            Err("No current chart".to_string())
        }
    }

    /// Get current chart
    pub fn get_current_chart(&self) -> Option<&Chart> {
        self.current_chart.as_ref()
            .and_then(|id| self.charts.get(id))
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut viz = DataViz::new();
    
    println!("Sigma Data Visualization v0.1 - Graphing & Plotting");
    
    loop {
        println!("\nCommands: create <id> <title> <type>, dataset <name> <color>, point <idx> <x> <y> [label], labels <x> <y>, legend, grid, render, regression <idx>, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "create" => {
                if parts.len() >= 4 {
                    let id = parts[1].to_string();
                    let title = parts[2].to_string();
                    let chart_type = match parts[3] {
                        "line" => ChartType::Line,
                        "bar" => ChartType::Bar,
                        "scatter" => ChartType::Scatter,
                        "pie" => ChartType::Pie,
                        "histogram" => ChartType::Histogram,
                        _ => {
                            println!("Unknown chart type");
                            continue;
                        }
                    };
                    viz.create_chart(id, title, chart_type);
                    println!("Chart created");
                }
            }
            "dataset" => {
                if parts.len() >= 3 {
                    let name = parts[1].to_string();
                    let color = parts[2].to_string();
                    match viz.add_dataset(name, color) {
                        Ok(_) => println!("Dataset added"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "point" => {
                if parts.len() >= 4 {
                    if let (Ok(idx), Ok(x), Ok(y)) = (parts[1].parse::<usize>(), parts[2].parse::<f64>(), parts[3].parse::<f64>()) {
                        let label = if parts.len() >= 5 { Some(parts[4].to_string()) } else { None };
                        match viz.add_point(idx, x, y, label) {
                            Ok(_) => println!("Point added"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "labels" => {
                if parts.len() >= 3 {
                    let x_label = parts[1].to_string();
                    let y_label = parts[2].to_string();
                    match viz.set_axis_labels(x_label, y_label) {
                        Ok(_) => println!("Labels set"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "legend" => {
                match viz.toggle_legend() {
                    Ok(_) => println!("Legend toggled"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "grid" => {
                match viz.toggle_grid() {
                    Ok(_) => println!("Grid toggled"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "render" => {
                println!("\n--- Chart ---");
                println!("{}", viz.render_ascii());
            }
            "regression" => {
                if let Some(arg) = parts.get(1) {
                    if let Ok(idx) = arg.parse::<usize>() {
                        match viz.linear_regression(idx) {
                            Ok((m, b)) => println!("y = {:.2}x + {:.2}", m, b),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
