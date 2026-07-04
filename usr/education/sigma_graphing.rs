// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_graphing.rs — Sigma Interactive Graphing Tools
//
// Implements dynamic plotting for equations and lab data aligned with
// CBSE science and mathematics curriculum.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Graphing Types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GraphType {
    Line,
    Scatter,
    Bar,
    Histogram,
    Pie,
}

#[derive(Debug, Clone)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
    pub label: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub id: String,
    pub title: String,
    pub graph_type: GraphType,
    pub x_label: String,
    pub y_label: String,
    pub data: Vec<DataPoint>,
    pub equation: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Equation {
    pub id: String,
    pub expression: String,
    pub variable: String,
    pub domain_start: f64,
    pub domain_end: f64,
    pub step: f64,
}

// ─── Graphing Engine ───────────────────────────────────────────────────────

pub struct GraphingEngine {
    pub graphs: HashMap<String, Graph>,
    pub equations: HashMap<String, Equation>,
}

impl GraphingEngine {
    pub fn new() -> Self {
        let mut engine = GraphingEngine {
            graphs: HashMap::new(),
            equations: HashMap::new(),
        };
        
        engine.init_sample_equations();
        engine
    }

    /// Initialize sample equations from CBSE syllabus
    fn init_sample_equations(&mut self) {
        // Quadratic: y = x²
        self.equations.insert("eq_001".to_string(), Equation {
            id: "eq_001".to_string(),
            expression: "x^2".to_string(),
            variable: "x".to_string(),
            domain_start: -10.0,
            domain_end: 10.0,
            step: 0.5,
        });

        // Linear: y = mx + c
        self.equations.insert("eq_002".to_string(), Equation {
            id: "eq_002".to_string(),
            expression: "2*x + 3".to_string(),
            variable: "x".to_string(),
            domain_start: -5.0,
            domain_end: 5.0,
            step: 0.5,
        });

        // Sine wave: y = sin(x)
        self.equations.insert("eq_003".to_string(), Equation {
            id: "eq_003".to_string(),
            expression: "sin(x)".to_string(),
            variable: "x".to_string(),
            domain_start: 0.0,
            domain_end: 6.28,  // 2π
            step: 0.1,
        });

        // Exponential: y = e^x
        self.equations.insert("eq_004".to_string(), Equation {
            id: "eq_004".to_string(),
            expression: "exp(x)".to_string(),
            variable: "x".to_string(),
            domain_start: -2.0,
            domain_end: 2.0,
            step: 0.2,
        });
    }

    /// Evaluate equation at given x value
    pub fn evaluate_equation(&self, equation: &Equation, x: f64) -> f64 {
        match equation.expression.as_str() {
            "x^2" => x * x,
            "2*x + 3" => 2.0 * x + 3.0,
            "sin(x)" => x.sin(),
            "cos(x)" => x.cos(),
            "tan(x)" => x.tan(),
            "exp(x)" => x.exp(),
            "log(x)" if x > 0.0 => x.ln(),
            "sqrt(x)" if x >= 0.0 => x.sqrt(),
            "abs(x)" => x.abs(),
            _ => {
                // Simple polynomial parsing for ax^2 + bx + c
                if equation.expression.contains("x^2") {
                    let parts: Vec<&str> = equation.expression.split('+').collect();
                    let a = parts.get(0).and_then(|s| s.trim().trim_end_matches("*x^2").parse().ok()).unwrap_or(1.0);
                    let b = parts.get(1).and_then(|s| s.trim().trim_end_matches("*x").parse().ok()).unwrap_or(0.0);
                    let c = parts.get(2).and_then(|s| s.trim().parse().ok()).unwrap_or(0.0);
                    a * x * x + b * x + c
                } else {
                    x
                }
            }
        }
    }

    /// Generate graph from equation
    pub fn plot_equation(&mut self, equation_id: &str, title: String) -> Result<Graph, String> {
        if let Some(equation) = self.equations.get(equation_id) {
            let mut data = Vec::new();
            let mut x = equation.domain_start;
            
            while x <= equation.domain_end {
                let y = self.evaluate_equation(equation, x);
                data.push(DataPoint {
                    x,
                    y,
                    label: None,
                });
                x += equation.step;
            }
            
            let graph = Graph {
                id: format!("graph_{}", self.graphs.len()),
                title,
                graph_type: GraphType::Line,
                x_label: equation.variable.clone(),
                y_label: "y".to_string(),
                data,
                equation: Some(equation.expression.clone()),
            };
            
            self.graphs.insert(graph.id.clone(), graph.clone());
            Ok(graph)
        } else {
            Err("Equation not found".to_string())
        }
    }

    /// Create graph from data points
    pub fn create_graph(&mut self, title: String, graph_type: GraphType, x_label: String, y_label: String, data: Vec<DataPoint>) -> Graph {
        let graph = Graph {
            id: format!("graph_{}", self.graphs.len()),
            title,
            graph_type,
            x_label,
            y_label,
            data,
            equation: None,
        };
        
        self.graphs.insert(graph.id.clone(), graph.clone());
        graph
    }

    /// Get graph by ID
    pub fn get_graph(&self, id: &str) -> Option<&Graph> {
        self.graphs.get(id)
    }

    /// Get all graphs
    pub fn get_all_graphs(&self) -> Vec<&Graph> {
        self.graphs.values().collect()
    }

    /// Get equation by ID
    pub fn get_equation(&self, id: &str) -> Option<&Equation> {
        self.equations.get(id)
    }

    /// Get all equations
    pub fn get_all_equations(&self) -> Vec<&Equation> {
        self.equations.values().collect()
    }

    /// Calculate statistics from graph data
    pub fn calculate_statistics(&self, graph_id: &str) -> Option<HashMap<String, f64>> {
        if let Some(graph) = self.get_graph(graph_id) {
            let mut stats = HashMap::new();
            
            let y_values: Vec<f64> = graph.data.iter().map(|p| p.y).collect();
            
            if !y_values.is_empty() {
                let sum: f64 = y_values.iter().sum();
                let mean = sum / y_values.len() as f64;
                let min = y_values.iter().cloned().fold(f64::INFINITY, f64::min);
                let max = y_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                
                let variance = y_values.iter()
                    .map(|&val| (val - mean).powi(2))
                    .sum::<f64>() / y_values.len() as f64;
                let std_dev = variance.sqrt();
                
                stats.insert("mean".to_string(), mean);
                stats.insert("min".to_string(), min);
                stats.insert("max".to_string(), max);
                stats.insert("std_dev".to_string(), std_dev);
                stats.insert("count".to_string(), y_values.len() as f64);
            }
            
            Some(stats)
        } else {
            None
        }
    }

    /// Add custom equation
    pub fn add_equation(&mut self, equation: Equation) {
        self.equations.insert(equation.id.clone(), equation);
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut engine = GraphingEngine::new();
    
    println!("Sigma Interactive Graphing Tools v0.1 - CBSE Science & Math");
    
    loop {
        println!("\n--- Available Equations ---");
        for eq in engine.get_all_equations() {
            println!("{} - y = {} ({} from {} to {})", eq.id, eq.expression, eq.variable, eq.domain_start, eq.domain_end);
        }
        
        println!("\nCommands: plot <eq_id> <title>, graph <type> <title> <x_label> <y_label>, add_data <x> <y>, graphs, graph <id>, stats <id>, equations, quit");
        println!("Types: line, scatter, bar, histogram, pie");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "plot" => {
                if parts.len() >= 3 {
                    let eq_id = parts[1];
                    let title = parts[2..].join(" ");
                    match engine.plot_equation(eq_id, title) {
                        Ok(graph) => {
                            println!("Graph created: {}", graph.id);
                            println!("Title: {}", graph.title);
                            println!("Data points: {}", graph.data.len());
                            println!("Sample points:");
                            for (i, point) in graph.data.iter().enumerate().take(5) {
                                println!("  ({:.2}, {:.2})", point.x, point.y);
                            }
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "graph" => {
                if parts.len() >= 5 {
                    let graph_type = match parts[1] {
                        "line" => GraphType::Line,
                        "scatter" => GraphType::Scatter,
                        "bar" => GraphType::Bar,
                        "histogram" => GraphType::Histogram,
                        "pie" => GraphType::Pie,
                        _ => {
                            println!("Unknown graph type");
                            continue;
                        }
                    };
                    let title = parts[2].to_string();
                    let x_label = parts[3].to_string();
                    let y_label = parts[4].to_string();
                    let graph = engine.create_graph(title, graph_type, x_label, y_label, Vec::new());
                    println!("Graph created: {}", graph.id);
                }
            }
            "add_data" => {
                if parts.len() >= 3 {
                    if let (Ok(x), Ok(y)) = (parts[1].parse::<f64>(), parts[2].parse::<f64>()) {
                        println!("Enter graph ID to add data to:");
                        let mut graph_id = String::new();
                        std::io::stdin().read_line(&mut graph_id).unwrap();
                        
                        if let Some(graph) = engine.graphs.get_mut(&graph_id.trim()) {
                            graph.data.push(DataPoint { x, y, label: None });
                            println!("Data point added");
                        }
                    }
                }
            }
            "graphs" => {
                println!("--- All Graphs ---");
                for graph in engine.get_all_graphs() {
                    println!("{} - {} ({:?})", graph.id, graph.title, graph.graph_type);
                }
            }
            "graph" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(graph) = engine.get_graph(arg) {
                        println!("--- Graph Details ---");
                        println!("Title: {}", graph.title);
                        println!("Type: {:?}", graph.graph_type);
                        println!("X Label: {}", graph.x_label);
                        println!("Y Label: {}", graph.y_label);
                        if let Some(eq) = &graph.equation {
                            println!("Equation: y = {}", eq);
                        }
                        println!("Data Points: {}", graph.data.len());
                        println!("Sample Data:");
                        for (i, point) in graph.data.iter().enumerate().take(10) {
                            println!("  {}. ({:.2}, {:.2})", i + 1, point.x, point.y);
                        }
                    }
                }
            }
            "stats" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(stats) = engine.calculate_statistics(arg) {
                        println!("--- Statistics ---");
                        for (key, value) in stats {
                            println!("{}: {:.4}", key, value);
                        }
                    }
                }
            }
            "equations" => {
                println!("--- All Equations ---");
                for eq in engine.get_all_equations() {
                    println!("{} - y = {}", eq.id, eq.expression);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
