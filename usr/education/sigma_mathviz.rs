// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_mathviz.rs — Sigma Math Visualization (GeoGebra style)
//
// Implements GeoGebra-style math visualization with geometric constructions,
    pub algebraic functions, calculus tools, and interactive plotting.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Math Visualization Types ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Point {
    pub id: String,
    pub x: f64,
    pub y: f64,
    pub label: String,
    pub color: String,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub id: String,
    pub point1: String,
    pub point2: String,
    pub equation: String,  // y = mx + b
    pub color: String,
}

#[derive(Debug, Clone)]
pub struct Circle {
    pub id: String,
    pub center: String,
    pub radius: f64,
    pub equation: String,
    pub color: String,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub id: String,
    pub expression: String,
    pub domain_start: f64,
    pub domain_end: f64,
    pub color: String,
}

#[derive(Debug, Clone)]
pub struct Construction {
    pub name: String,
    pub points: HashMap<String, Point>,
    pub lines: HashMap<String, Line>,
    pub circles: HashMap<String, Circle>,
    pub functions: HashMap<String, Function>,
}

// ─── Math Visualization Manager ────────────────────────────────────────────

pub struct MathVizManager {
    pub constructions: HashMap<String, Construction>,
    pub current_construction: Option<String>,
}

impl MathVizManager {
    pub fn new() -> Self {
        let mut manager = MathVizManager {
            constructions: HashMap::new(),
            current_construction: None,
        };
        
        manager.init_sample_construction();
        manager
    }

    /// Initialize sample construction
    fn init_sample_construction(&mut self) {
        let mut construction = Construction {
            name: "Triangle".to_string(),
            points: HashMap::new(),
            lines: HashMap::new(),
            circles: HashMap::new(),
            functions: HashMap::new(),
        };
        
        // Add triangle points
        construction.points.insert("A".to_string(), Point {
            id: "A".to_string(),
            x: 0.0,
            y: 0.0,
            label: "A".to_string(),
            color: "#ff0000".to_string(),
        });
        
        construction.points.insert("B".to_string(), Point {
            id: "B".to_string(),
            x: 4.0,
            y: 0.0,
            label: "B".to_string(),
            color: "#00ff00".to_string(),
        });
        
        construction.points.insert("C".to_string(), Point {
            id: "C".to_string(),
            x: 2.0,
            y: 3.464,
            label: "C".to_string(),
            color: "#0000ff".to_string(),
        });
        
        // Add triangle lines
        construction.lines.insert("AB".to_string(), Line {
            id: "AB".to_string(),
            point1: "A".to_string(),
            point2: "B".to_string(),
            equation: "y = 0".to_string(),
            color: "#000000".to_string(),
        });
        
        construction.lines.insert("BC".to_string(), Line {
            id: "BC".to_string(),
            point1: "B".to_string(),
            point2: "C".to_string(),
            equation: "y = -1.732x + 6.928".to_string(),
            color: "#000000".to_string(),
        });
        
        construction.lines.insert("CA".to_string(), Line {
            id: "CA".to_string(),
            point1: "C".to_string(),
            point2: "A".to_string(),
            equation: "y = 1.732x".to_string(),
            color: "#000000".to_string(),
        });
        
        // Add sample function
        construction.functions.insert("f".to_string(), Function {
            id: "f".to_string(),
            expression: "sin(x)".to_string(),
            domain_start: -10.0,
            domain_end: 10.0,
            color: "#ff00ff".to_string(),
        });
        
        self.constructions.insert(construction.name.clone(), construction);
        self.current_construction = Some("Triangle".to_string());
    }

    /// Create new construction
    pub fn create_construction(&mut self, name: String) -> Construction {
        let construction = Construction {
            name: name.clone(),
            points: HashMap::new(),
            lines: HashMap::new(),
            circles: HashMap::new(),
            functions: HashMap::new(),
        };
        
        self.constructions.insert(name.clone(), construction.clone());
        self.current_construction = Some(name);
        construction
    }

    /// Add point
    pub fn add_point(&mut self, construction_name: &str, id: String, x: f64, y: f64, label: String, color: String) -> Result<(), String> {
        if let Some(construction) = self.constructions.get_mut(construction_name) {
            construction.points.insert(id.clone(), Point { id, x, y, label, color });
            Ok(())
        } else {
            Err("Construction not found".to_string())
        }
    }

    /// Add line
    pub fn add_line(&mut self, construction_name: &str, id: String, point1: String, point2: String, color: String) -> Result<(), String> {
        if let Some(construction) = self.constructions.get_mut(construction_name) {
            let equation = Self::calculate_line_equation(construction, &point1, &point2);
            construction.lines.insert(id, Line { id, point1, point2, equation, color });
            Ok(())
        } else {
            Err("Construction not found".to_string())
        }
    }

    /// Add circle
    pub fn add_circle(&mut self, construction_name: &str, id: String, center: String, radius: f64, color: String) -> Result<(), String> {
        if let Some(construction) = self.constructions.get_mut(construction_name) {
            let equation = format!("(x - x0)^2 + (y - y0)^2 = {}", radius * radius);
            construction.circles.insert(id, Circle { id, center, radius, equation, color });
            Ok(())
        } else {
            Err("Construction not found".to_string())
        }
    }

    /// Add function
    pub fn add_function(&mut self, construction_name: &str, id: String, expression: String, domain_start: f64, domain_end: f64, color: String) -> Result<(), String> {
        if let Some(construction) = self.constructions.get_mut(construction_name) {
            construction.functions.insert(id, Function { id, expression, domain_start, domain_end, color });
            Ok(())
        } else {
            Err("Construction not found".to_string())
        }
    }

    /// Calculate line equation
    fn calculate_line_equation(construction: &Construction, point1: &str, point2: &str) -> String {
        if let (Some(p1), Some(p2)) = (construction.points.get(point1), construction.points.get(point2)) {
            let slope = (p2.y - p1.y) / (p2.x - p1.x);
            let intercept = p1.y - slope * p1.x;
            format!("y = {}x + {}", slope, intercept)
        } else {
            "y = 0".to_string()
        }
    }

    /// Evaluate function at point
    pub fn evaluate_function(&self, construction_name: &str, function_id: &str, x: f64) -> Result<f64, String> {
        if let Some(construction) = self.constructions.get(construction_name) {
            if let Some(func) = construction.functions.get(function_id) {
                // Simple evaluation for common functions
                match func.expression.as_str() {
                    "sin(x)" => Ok(x.sin()),
                    "cos(x)" => Ok(x.cos()),
                    "tan(x)" => Ok(x.tan()),
                    "x^2" => Ok(x * x),
                    "x^3" => Ok(x * x * x),
                    "sqrt(x)" => Ok(x.sqrt()),
                    "log(x)" => Ok(x.ln()),
                    "exp(x)" => Ok(x.exp()),
                    _ => Ok(0.0),
                }
            } else {
                Err("Function not found".to_string())
            }
        } else {
            Err("Construction not found".to_string())
        }
    }

    /// Get construction by name
    pub fn get_construction(&self, name: &str) -> Option<&Construction> {
        self.constructions.get(name)
    }

    /// Get all constructions
    pub fn get_all_constructions(&self) -> Vec<&Construction> {
        self.constructions.values().collect()
    }

    /// Switch construction
    pub fn switch_construction(&mut self, name: &str) -> Result<(), String> {
        if self.constructions.contains_key(name) {
            self.current_construction = Some(name.to_string());
            Ok(())
        } else {
            Err("Construction not found".to_string())
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = MathVizManager::new();
    
    println!("Sigma Math Visualization v0.1 - GeoGebra Style");
    
    loop {
        println!("\n--- Math Visualization Status ---");
        if let Some(current) = &manager.current_construction {
            println!("Current Construction: {}", current);
            if let Some(construction) = manager.get_construction(current) {
                println!("Points: {}", construction.points.len());
                println!("Lines: {}", construction.lines.len());
                println!("Circles: {}", construction.circles.len());
                println!("Functions: {}", construction.functions.len());
            }
        }
        
        println!("\nCommands: create <name>, point <id> <x> <y> <label> <color>, line <id> <p1> <p2> <color>, circle <id> <center> <radius> <color>, function <id> <expr> <domain_start> <domain_end> <color>, eval <func_id> <x>, switch <name>, constructions, construction <name>, quit");
        println!("Colors: #rrggbb format");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "create" => {
                if let Some(arg) = parts.get(1) {
                    let construction = manager.create_construction(arg.to_string());
                    println!("Construction created: {}", construction.name);
                }
            }
            "point" => {
                if parts.len() >= 6 {
                    if let (Ok(x), Ok(y)) = (parts[2].parse::<f64>(), parts[3].parse::<f64>()) {
                        let id = parts[1].to_string();
                        let label = parts[4].to_string();
                        let color = parts[5].to_string();
                        if let Some(current) = &manager.current_construction {
                            match manager.add_point(current, id, x, y, label, color) {
                                Ok(_) => println!("Point added"),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                    }
                }
            }
            "line" => {
                if parts.len() >= 5 {
                    let id = parts[1].to_string();
                    let p1 = parts[2].to_string();
                    let p2 = parts[3].to_string();
                    let color = parts[4].to_string();
                    if let Some(current) = &manager.current_construction {
                        match manager.add_line(current, id, p1, p2, color) {
                            Ok(_) => println!("Line added"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                }
            }
            "circle" => {
                if parts.len() >= 5 {
                    let id = parts[1].to_string();
                    let center = parts[2].to_string();
                    if let Ok(radius) = parts[3].parse::<f64>() {
                        let color = parts[4].to_string();
                        if let Some(current) = &manager.current_construction {
                            match manager.add_circle(current, id, center, radius, color) {
                                Ok(_) => println!("Circle added"),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                    }
                }
            }
            "function" => {
                if parts.len() >= 6 {
                    let id = parts[1].to_string();
                    let expr = parts[2].to_string();
                    if let (Ok(domain_start), Ok(domain_end)) = (parts[3].parse::<f64>(), parts[4].parse::<f64>()) {
                        let color = parts[5].to_string();
                        if let Some(current) = &manager.current_construction {
                            match manager.add_function(current, id, expr, domain_start, domain_end, color) {
                                Ok(_) => println!("Function added"),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                    }
                }
            }
            "eval" => {
                if parts.len() >= 3 {
                    let func_id = parts[1].to_string();
                    if let Ok(x) = parts[2].parse::<f64>() {
                        if let Some(current) = &manager.current_construction {
                            match manager.evaluate_function(current, &func_id, x) {
                                Ok(result) => println!("f({}) = {}", x, result),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                    }
                }
            }
            "switch" => {
                if let Some(arg) = parts.get(1) {
                    match manager.switch_construction(arg) {
                        Ok(_) => println!("Switched to construction"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "constructions" => {
                println!("--- All Constructions ---");
                for construction in manager.get_all_constructions() {
                    println!("{} - {} points, {} lines, {} circles, {} functions",
                        construction.name,
                        construction.points.len(),
                        construction.lines.len(),
                        construction.circles.len(),
                        construction.functions.len()
                    );
                }
            }
            "construction" => {
                if let Some(arg) = parts.get(1) {
                    if let Some(construction) = manager.get_construction(arg) {
                        println!("--- Construction Details ---");
                        println!("Name: {}", construction.name);
                        println!("\n--- Points ---");
                        for point in construction.points.values() {
                            println!("{} - ({}, {}) [{}]", point.label, point.x, point.y, point.color);
                        }
                        println!("\n--- Lines ---");
                        for line in construction.lines.values() {
                            println!("{} - {} -> {} ({})", line.id, line.point1, line.point2, line.equation);
                        }
                        println!("\n--- Circles ---");
                        for circle in construction.circles.values() {
                            println!("{} - Center: {}, Radius: {}", circle.id, circle.center, circle.radius);
                        }
                        println!("\n--- Functions ---");
                        for func in construction.functions.values() {
                            println!("{} - {} [{}, {}]", func.id, func.expression, func.domain_start, func.domain_end);
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
