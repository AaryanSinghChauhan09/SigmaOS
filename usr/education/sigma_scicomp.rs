// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_scicomp.rs — Sigma Scientific Computing (Scilab/Octave)
//
// Implements Scilab/Octave-style scientific computing with matrix operations,
// numerical analysis, plotting, and scripting capabilities.
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Scientific Computing Types ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Matrix {
    pub name: String,
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: String,  // Can be scalar, matrix, or string
    pub var_type: String,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct Plot {
    pub id: String,
    pub x_data: Vec<f64>,
    pub y_data: Vec<f64>,
    pub plot_type: String,  // line, scatter, bar, histogram
    pub title: String,
    pub xlabel: String,
    pub ylabel: String,
}

// ─── Scientific Computing Manager ────────────────────────────────────────────

pub struct SciCompManager {
    pub variables: HashMap<String, Variable>,
    pub matrices: HashMap<String, Matrix>,
    pub functions: HashMap<String, Function>,
    pub plots: Vec<Plot>,
    pub scripts: Vec<String>,
}

impl SciCompManager {
    pub fn new() -> Self {
        let mut manager = SciCompManager {
            variables: HashMap::new(),
            matrices: HashMap::new(),
            functions: HashMap::new(),
            plots: Vec::new(),
            scripts: Vec::new(),
        };
        
        manager.init_sample_variables();
        manager.init_sample_matrices();
        manager.init_sample_functions();
        manager
    }

    /// Initialize sample variables
    fn init_sample_variables(&mut self) {
        self.variables.insert("pi".to_string(), Variable {
            name: "pi".to_string(),
            value: "3.14159265359".to_string(),
            var_type: "scalar".to_string(),
        });

        self.variables.insert("e".to_string(), Variable {
            name: "e".to_string(),
            value: "2.71828182846".to_string(),
            var_type: "scalar".to_string(),
        });
    }

    /// Initialize sample matrices
    fn init_sample_matrices(&mut self) {
        self.matrices.insert("A".to_string(), Matrix {
            name: "A".to_string(),
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0, 6.0],
                vec![7.0, 8.0, 9.0],
            ],
        });

        self.matrices.insert("I".to_string(), Matrix {
            name: "I".to_string(),
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0],
                vec![0.0, 0.0, 1.0],
            ],
        });
    }

    /// Initialize sample functions
    fn init_sample_functions(&mut self) {
        self.functions.insert("sin".to_string(), Function {
            name: "sin".to_string(),
            parameters: vec!["x".to_string()],
            body: "return sin(x)".to_string(),
        });

        self.functions.insert("cos".to_string(), Function {
            name: "cos".to_string(),
            parameters: vec!["x".to_string()],
            body: "return cos(x)".to_string(),
        });
    }

    /// Create matrix
    pub fn create_matrix(&mut self, name: String, rows: usize, cols: usize, data: Vec<Vec<f64>>) -> Matrix {
        let matrix = Matrix {
            name: name.clone(),
            rows,
            cols,
            data,
        };
        
        self.matrices.insert(name, matrix.clone());
        matrix
    }

    /// Matrix addition
    pub fn matrix_add(&self, mat1: &str, mat2: &str) -> Result<Matrix, String> {
        if let (Some(m1), Some(m2)) = (self.matrices.get(mat1), self.matrices.get(mat2)) {
            if m1.rows != m2.rows || m1.cols != m2.cols {
                return Err("Matrix dimensions must match".to_string());
            }
            
            let mut result_data = Vec::new();
            for i in 0..m1.rows {
                let mut row = Vec::new();
                for j in 0..m1.cols {
                    row.push(m1.data[i][j] + m2.data[i][j]);
                }
                result_data.push(row);
            }
            
            Ok(Matrix {
                name: format!("{}_plus_{}", mat1, mat2),
                rows: m1.rows,
                cols: m1.cols,
                data: result_data,
            })
        } else {
            Err("Matrix not found".to_string())
        }
    }

    /// Matrix multiplication
    pub fn matrix_multiply(&self, mat1: &str, mat2: &str) -> Result<Matrix, String> {
        if let (Some(m1), Some(m2)) = (self.matrices.get(mat1), self.matrices.get(mat2)) {
            if m1.cols != m2.rows {
                return Err("Matrix dimensions incompatible for multiplication".to_string());
            }
            
            let mut result_data = vec![vec![0.0; m2.cols]; m1.rows];
            
            for i in 0..m1.rows {
                for j in 0..m2.cols {
                    for k in 0..m1.cols {
                        result_data[i][j] += m1.data[i][k] * m2.data[k][j];
                    }
                }
            }
            
            Ok(Matrix {
                name: format!("{}_times_{}", mat1, mat2),
                rows: m1.rows,
                cols: m2.cols,
                data: result_data,
            })
        } else {
            Err("Matrix not found".to_string())
        }
    }

    /// Matrix transpose
    pub fn matrix_transpose(&self, mat: &str) -> Result<Matrix, String> {
        if let Some(m) = self.matrices.get(mat) {
            let mut result_data = vec![vec![0.0; m.rows]; m.cols];
            
            for i in 0..m.rows {
                for j in 0..m.cols {
                    result_data[j][i] = m.data[i][j];
                }
            }
            
            Ok(Matrix {
                name: format!("{}_T", mat),
                rows: m.cols,
                cols: m.rows,
                data: result_data,
            })
        } else {
            Err("Matrix not found".to_string())
        }
    }

    /// Matrix determinant (2x2 and 3x3)
    pub fn matrix_determinant(&self, mat: &str) -> Result<f64, String> {
        if let Some(m) = self.matrices.get(mat) {
            if m.rows != m.cols {
                return Err("Matrix must be square".to_string());
            }
            
            if m.rows == 2 {
                Ok(m.data[0][0] * m.data[1][1] - m.data[0][1] * m.data[1][0])
            } else if m.rows == 3 {
                let a = m.data[0][0];
                let b = m.data[0][1];
                let c = m.data[0][2];
                let d = m.data[1][0];
                let e = m.data[1][1];
                let f = m.data[1][2];
                let g = m.data[2][0];
                let h = m.data[2][1];
                let i = m.data[2][2];
                
                Ok(a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g))
            } else {
                Err("Determinant only implemented for 2x2 and 3x3 matrices".to_string())
            }
        } else {
            Err("Matrix not found".to_string())
        }
    }

    /// Create plot
    pub fn create_plot(&mut self, x_data: Vec<f64>, y_data: Vec<f64>, plot_type: String, title: String) -> Plot {
        let plot = Plot {
            id: format!("plot_{}", self.plots.len()),
            x_data,
            y_data,
            plot_type,
            title,
            xlabel: "X".to_string(),
            ylabel: "Y".to_string(),
        };
        
        self.plots.push(plot.clone());
        plot
    }

    /// Define function
    pub fn define_function(&mut self, name: String, parameters: Vec<String>, body: String) {
        self.functions.insert(name.clone(), Function { name, parameters, body });
    }

    /// Evaluate expression
    pub fn evaluate(&self, expr: &str) -> Result<f64, String> {
        // Simple expression evaluation for basic operations
        let tokens: Vec<&str> = expr.split_whitespace().collect();
        if tokens.len() == 3 {
            if let (Ok(a), Ok(b)) = (tokens[0].parse::<f64>(), tokens[2].parse::<f64>()) {
                match tokens[1] {
                    "+" => Ok(a + b),
                    "-" => Ok(a - b),
                    "*" => Ok(a * b),
                    "/" => Ok(a / b),
                    "^" => Ok(a.powf(b)),
                    _ => Err("Unknown operator".to_string()),
                }
            } else {
                Err("Invalid expression".to_string())
            }
        } else {
            Err("Expression must have format: a op b".to_string())
        }
    }

    /// Get matrix by name
    pub fn get_matrix(&self, name: &str) -> Option<&Matrix> {
        self.matrices.get(name)
    }

    /// Get all matrices
    pub fn get_all_matrices(&self) -> Vec<&Matrix> {
        self.matrices.values().collect()
    }

    /// Get variable by name
    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
    }

    /// Get all variables
    pub fn get_all_variables(&self) -> Vec<&Variable> {
        self.variables.values().collect()
    }

    /// Get all plots
    pub fn get_all_plots(&self) -> Vec<&Plot> {
        self.plots.iter().collect()
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────

fn main() {
    let mut manager = SciCompManager::new();
    
    println!("Sigma Scientific Computing v0.1 - Scilab/Octave Style");
    
    loop {
        println!("\n--- Scientific Computing Status ---");
        println!("Variables: {}", manager.variables.len());
        println!("Matrices: {}", manager.matrices.len());
        println!("Functions: {}", manager.functions.len());
        println!("Plots: {}", manager.plots.len());
        
        println!("\nCommands: matrix <name> <rows> <cols>, add <mat1> <mat2>, mul <mat1> <mat2>, transpose <mat>, det <mat>, plot <x_data> <y_data> <type> <title>, func <name> <params> <body>, eval <expr>, matrices, variables, plots, quit");
        println!("Plot types: line, scatter, bar");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "matrix" => {
                if parts.len() >= 4 {
                    let name = parts[1].to_string();
                    if let (Ok(rows), Ok(cols)) = (parts[2].parse::<usize>(), parts[3].parse::<usize>()) {
                        let data = vec![vec![0.0; cols]; rows];
                        let matrix = manager.create_matrix(name, rows, cols, data);
                        println!("Matrix created: {} ({}x{})", matrix.name, matrix.rows, matrix.cols);
                    }
                }
            }
            "add" => {
                if parts.len() >= 3 {
                    match manager.matrix_add(parts[1], parts[2]) {
                        Ok(result) => println!("Result: {} ({}x{})", result.name, result.rows, result.cols),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "mul" => {
                if parts.len() >= 3 {
                    match manager.matrix_multiply(parts[1], parts[2]) {
                        Ok(result) => println!("Result: {} ({}x{})", result.name, result.rows, result.cols),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "transpose" => {
                if let Some(arg) = parts.get(1) {
                    match manager.matrix_transpose(arg) {
                        Ok(result) => println!("Transpose: {} ({}x{})", result.name, result.rows, result.cols),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "det" => {
                if let Some(arg) = parts.get(1) {
                    match manager.matrix_determinant(arg) {
                        Ok(det) => println!("Determinant: {}", det),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "plot" => {
                if parts.len() >= 5 {
                    let x_data: Vec<f64> = parts[1].split(',').filter_map(|s| s.parse().ok()).collect();
                    let y_data: Vec<f64> = parts[2].split(',').filter_map(|s| s.parse().ok()).collect();
                    let plot_type = parts[3].to_string();
                    let title = parts[4..].join(" ");
                    let plot = manager.create_plot(x_data, y_data, plot_type, title);
                    println!("Plot created: {}", plot.id);
                }
            }
            "func" => {
                if parts.len() >= 4 {
                    let name = parts[1].to_string();
                    let params: Vec<String> = parts[2].split(',').map(|s| s.to_string()).collect();
                    let body = parts[3..].join(" ");
                    manager.define_function(name, params, body);
                    println!("Function defined");
                }
            }
            "eval" => {
                if let Some(arg) = parts.get(1) {
                    match manager.evaluate(arg) {
                        Ok(result) => println!("Result: {}", result),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            "matrices" => {
                println!("--- All Matrices ---");
                for matrix in manager.get_all_matrices() {
                    println!("{} - {}x{}", matrix.name, matrix.rows, matrix.cols);
                }
            }
            "variables" => {
                println!("--- All Variables ---");
                for var in manager.get_all_variables() {
                    println!("{} = {} ({})", var.name, var.value, var.var_type);
                }
            }
            "plots" => {
                println!("--- All Plots ---");
                for plot in manager.get_all_plots() {
                    println!("{} - {} ({})", plot.id, plot.title, plot.plot_type);
                }
            }
            "quit" | "exit" => break,
            _ => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}
