// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/education/sigma_math.rs — Sigma Symbolic Math Engine
//
// Implements symbolic mathematics for algebra, calculus, and geometry
// aligned with CBSE mathematics curriculum (similar to Wolfram Alpha).
//
// Language: Rust (std for userland applications)

use std::collections::HashMap;

// ─── Expression Types ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
    Function(String, Vec<Expression>),
}

#[derive(Debug, Clone)]
pub struct SolutionStep {
    pub description: String,
    pub expression: String,
    pub explanation: String,
}

// ─── Symbolic Math Engine ─────────────────────────────────────────────────────

pub struct SymbolicMath {
    pub variables: HashMap<String, f64>,
    pub history: Vec<String>,
}

impl SymbolicMath {
    pub fn new() -> Self {
        SymbolicMath {
            variables: HashMap::new(),
            history: Vec::new(),
        }
    }

    /// Parse simple expression (stub implementation)
    pub fn parse(&self, input: &str) -> Result<Expression, String> {
        // Simple parsing for basic arithmetic
        let tokens: Vec<&str> = input.split_whitespace().collect();
        
        if tokens.len() == 1 {
            if let Ok(n) = tokens[0].parse::<f64>() {
                return Ok(Expression::Number(n));
            }
            if tokens[0].chars().all(|c| c.is_alphabetic()) {
                return Ok(Expression::Variable(tokens[0].to_string()));
            }
        }
        
        // Simple binary operations
        if tokens.len() == 3 {
            let left = self.parse(tokens[0])?;
            let right = self.parse(tokens[2])?;
            
            match tokens[1] {
                "+" => Ok(Expression::Add(Box::new(left), Box::new(right))),
                "-" => Ok(Expression::Subtract(Box::new(left), Box::new(right))),
                "*" => Ok(Expression::Multiply(Box::new(left), Box::new(right))),
                "/" => Ok(Expression::Divide(Box::new(left), Box::new(right))),
                "^" => Ok(Expression::Power(Box::new(left), Box::new(right))),
                _ => Err(format!("Unknown operator: {}", tokens[1])),
            }
        } else {
            Err("Complex parsing not implemented".to_string())
        }
    }

    /// Evaluate expression with variable substitution
    pub fn evaluate(&self, expr: &Expression) -> Result<f64, String> {
        match expr {
            Expression::Number(n) => Ok(*n),
            Expression::Variable(name) => {
                if let Some(&value) = self.variables.get(name) {
                    Ok(value)
                } else {
                    Err(format!("Variable '{}' not defined", name))
                }
            }
            Expression::Add(a, b) => {
                Ok(self.evaluate(a)? + self.evaluate(b)?)
            }
            Expression::Subtract(a, b) => {
                Ok(self.evaluate(a)? - self.evaluate(b)?)
            }
            Expression::Multiply(a, b) => {
                Ok(self.evaluate(a)? * self.evaluate(b)?)
            }
            Expression::Divide(a, b) => {
                let divisor = self.evaluate(b)?;
                if divisor == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(self.evaluate(a)? / divisor)
                }
            }
            Expression::Power(a, b) => {
                Ok(self.evaluate(a)?.powf(self.evaluate(b)?))
            }
            Expression::Function(name, args) => {
                self.evaluate_function(name, args)
            }
        }
    }

    /// Evaluate mathematical functions
    fn evaluate_function(&self, name: &str, args: &[Expression]) -> Result<f64, String> {
        match name.to_lowercase().as_str() {
            "sin" => {
                if args.len() == 1 {
                    Ok(self.evaluate(&args[0])?.sin())
                } else {
                    Err("sin() takes 1 argument".to_string())
                }
            }
            "cos" => {
                if args.len() == 1 {
                    Ok(self.evaluate(&args[0])?.cos())
                } else {
                    Err("cos() takes 1 argument".to_string())
                }
            }
            "tan" => {
                if args.len() == 1 {
                    Ok(self.evaluate(&args[0])?.tan())
                } else {
                    Err("tan() takes 1 argument".to_string())
                }
            }
            "sqrt" => {
                if args.len() == 1 {
                    let val = self.evaluate(&args[0])?;
                    if val >= 0.0 {
                        Ok(val.sqrt())
                    } else {
                        Err("sqrt() requires non-negative argument".to_string())
                    }
                } else {
                    Err("sqrt() takes 1 argument".to_string())
                }
            }
            "log" => {
                if args.len() == 1 {
                    let val = self.evaluate(&args[0])?;
                    if val > 0.0 {
                        Ok(val.ln())
                    } else {
                        Err("log() requires positive argument".to_string())
                    }
                } else {
                    Err("log() takes 1 argument".to_string())
                }
            }
            "abs" => {
                if args.len() == 1 {
                    Ok(self.evaluate(&args[0])?.abs())
                } else {
                    Err("abs() takes 1 argument".to_string())
                }
            }
            _ => Err(format!("Unknown function: {}", name)),
        }
    }

    /// Simplify expression (basic algebraic simplification)
    pub fn simplify(&self, expr: &Expression) -> Expression {
        match expr {
            Expression::Number(n) => Expression::Number(*n),
            Expression::Variable(name) => Expression::Variable(name.clone()),
            Expression::Add(a, b) => {
                let a_simp = self.simplify(a);
                let b_simp = self.simplify(b);
                
                match (&a_simp, &b_simp) {
                    (Expression::Number(0.0), _) => b_simp,
                    (_, Expression::Number(0.0)) => a_simp,
                    (Expression::Number(x), Expression::Number(y)) => Expression::Number(x + y),
                    _ => Expression::Add(Box::new(a_simp), Box::new(b_simp)),
                }
            }
            Expression::Subtract(a, b) => {
                let a_simp = self.simplify(a);
                let b_simp = self.simplify(b);
                
                match (&a_simp, &b_simp) {
                    (_, Expression::Number(0.0)) => a_simp,
                    (Expression::Number(x), Expression::Number(y)) => Expression::Number(x - y),
                    _ => Expression::Subtract(Box::new(a_simp), Box::new(b_simp)),
                }
            }
            Expression::Multiply(a, b) => {
                let a_simp = self.simplify(a);
                let b_simp = self.simplify(b);
                
                match (&a_simp, &b_simp) {
                    (Expression::Number(0.0), _) => Expression::Number(0.0),
                    (_, Expression::Number(0.0)) => Expression::Number(0.0),
                    (Expression::Number(1.0), _) => b_simp,
                    (_, Expression::Number(1.0)) => a_simp,
                    (Expression::Number(x), Expression::Number(y)) => Expression::Number(x * y),
                    _ => Expression::Multiply(Box::new(a_simp), Box::new(b_simp)),
                }
            }
            Expression::Divide(a, b) => {
                let a_simp = self.simplify(a);
                let b_simp = self.simplify(b);
                
                match (&a_simp, &b_simp) {
                    (Expression::Number(0.0), _) => Expression::Number(0.0),
                    (_, Expression::Number(1.0)) => a_simp,
                    (Expression::Number(x), Expression::Number(y)) if *y != 0.0 => Expression::Number(x / y),
                    _ => Expression::Divide(Box::new(a_simp), Box::new(b_simp)),
                }
            }
            Expression::Power(a, b) => {
                let a_simp = self.simplify(a);
                let b_simp = self.simplify(b);
                
                match (&a_simp, &b_simp) {
                    (_, Expression::Number(0.0)) => Expression::Number(1.0),
                    (Expression::Number(1.0), _) => Expression::Number(1.0),
                    (Expression::Number(x), Expression::Number(y)) => Expression::Number(x.powf(*y)),
                    _ => Expression::Power(Box::new(a_simp), Box::new(b_simp)),
                }
            }
            Expression::Function(name, args) => {
                let args_simp: Vec<Expression> = args.iter().map(|a| self.simplify(a)).collect();
                Expression::Function(name.clone(), args_simp)
            }
        }
    }

    /// Differentiate expression (basic calculus)
    pub fn differentiate(&self, expr: &Expression, var: &str) -> Result<Expression, String> {
        match expr {
            Expression::Number(_) => Ok(Expression::Number(0.0)),
            Expression::Variable(name) => {
                if name == var {
                    Ok(Expression::Number(1.0))
                } else {
                    Ok(Expression::Number(0.0))
                }
            }
            Expression::Add(a, b) => {
                let da = self.differentiate(a, var)?;
                let db = self.differentiate(b, var)?;
                Ok(Expression::Add(Box::new(da), Box::new(db)))
            }
            Expression::Subtract(a, b) => {
                let da = self.differentiate(a, var)?;
                let db = self.differentiate(b, var)?;
                Ok(Expression::Subtract(Box::new(da), Box::new(db)))
            }
            Expression::Multiply(a, b) => {
                // Product rule: (uv)' = u'v + uv'
                let da = self.differentiate(a, var)?;
                let db = self.differentiate(b, var)?;
                let term1 = Expression::Multiply(Box::new(da), b.clone());
                let term2 = Expression::Multiply(Box::new(a.clone()), Box::new(db));
                Ok(Expression::Add(Box::new(term1), Box::new(term2)))
            }
            Expression::Divide(a, b) => {
                // Quotient rule: (u/v)' = (u'v - uv')/v²
                let da = self.differentiate(a, var)?;
                let db = self.differentiate(b, var)?;
                let numerator = Expression::Subtract(
                    Box::new(Expression::Multiply(Box::new(da), b.clone())),
                    Box::new(Expression::Multiply(Box::new(a.clone()), Box::new(db))),
                );
                let denominator = Expression::Power(Box::new(b.clone()), Box::new(Expression::Number(2.0)));
                Ok(Expression::Divide(Box::new(numerator), Box::new(denominator)))
            }
            Expression::Power(a, b) => {
                // Chain rule for x^n: n*x^(n-1)
                if let Expression::Number(n) = &**b {
                    let new_exp = Expression::Number(n - 1.0);
                    let base = Expression::Power(a.clone(), Box::new(new_exp));
                    Ok(Expression::Multiply(Box::new(Expression::Number(*n)), Box::new(base)))
                } else {
                    Err("General power rule not implemented".to_string())
                }
            }
            Expression::Function(name, args) => {
                match name.to_lowercase().as_str() {
                    "sin" => {
                        if args.len() == 1 {
                            let d = self.differentiate(&args[0], var)?;
                            Ok(Expression::Multiply(Box::new(d), Box::new(Expression::Function("cos".to_string(), args.clone()))))
                        } else {
                            Err("sin() takes 1 argument".to_string())
                        }
                    }
                    "cos" => {
                        if args.len() == 1 {
                            let d = self.differentiate(&args[0], var)?;
                            let neg_cos = Expression::Multiply(Box::new(Expression::Number(-1.0)), Box::new(Expression::Function("sin".to_string(), args.clone())));
                            Ok(Expression::Multiply(Box::new(d), Box::new(neg_cos)))
                        } else {
                            Err("cos() takes 1 argument".to_string())
                        }
                    }
                    _ => Err(format!("Derivative of {} not implemented", name)),
                }
            }
        }
    }

    /// Set variable value
    pub fn set_variable(&mut self, name: &str, value: f64) {
        self.variables.insert(name.to_string(), value);
    }

    /// Get variable value
    pub fn get_variable(&self, name: &str) -> Option<f64> {
        self.variables.get(name).copied()
    }

    /// Convert expression to string
    pub fn to_string(&self, expr: &Expression) -> String {
        match expr {
            Expression::Number(n) => format!("{}", n),
            Expression::Variable(name) => name.clone(),
            Expression::Add(a, b) => format!("({} + {})", self.to_string(a), self.to_string(b)),
            Expression::Subtract(a, b) => format!("({} - {})", self.to_string(a), self.to_string(b)),
            Expression::Multiply(a, b) => format!("({} * {})", self.to_string(a), self.to_string(b)),
            Expression::Divide(a, b) => format!("({} / {})", self.to_string(a), self.to_string(b)),
            Expression::Power(a, b) => format!("({} ^ {})", self.to_string(a), self.to_string(b)),
            Expression::Function(name, args) => {
                let args_str: Vec<String> = args.iter().map(|a| self.to_string(a)).collect();
                format!("{}({})", name, args_str.join(", "))
            }
        }
    }
}

// ─── CLI Interface ─────────────────────────────────────────────────────────--

fn main() {
    let mut math = SymbolicMath::new();
    
    println!("Sigma Symbolic Math v0.1 - Algebra, Calculus, Geometry");
    
    loop {
        println!("\nCommands: eval <expr>, simplify <expr>, diff <expr> <var>, set <var> <value>, get <var>, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| *s).unwrap_or("");
        
        match cmd {
            "eval" => {
                if parts.len() >= 2 {
                    let expr_str = parts[1..].join(" ");
                    match math.parse(&expr_str) {
                        Ok(expr) => {
                            match math.evaluate(&expr) {
                                Ok(result) => println!("Result: {}", result),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Parse error: {}", e),
                    }
                }
            }
            "simplify" => {
                if parts.len() >= 2 {
                    let expr_str = parts[1..].join(" ");
                    match math.parse(&expr_str) {
                        Ok(expr) => {
                            let simplified = math.simplify(&expr);
                            println!("Simplified: {}", math.to_string(&simplified));
                        }
                        Err(e) => eprintln!("Parse error: {}", e),
                    }
                }
            }
            "diff" => {
                if parts.len() >= 3 {
                    let expr_str = parts[1..parts.len()-1].join(" ");
                    let var = parts[parts.len()-1];
                    match math.parse(&expr_str) {
                        Ok(expr) => {
                            match math.differentiate(&expr, var) {
                                Ok(deriv) => println!("d/d{}: {}", var, math.to_string(&deriv)),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Parse error: {}", e),
                    }
                }
            }
            "set" => {
                if parts.len() >= 3 {
                    let var = parts[1];
                    if let Ok(value) = parts[2].parse::<f64>() {
                        math.set_variable(var, value);
                        println!("{} = {}", var, value);
                    }
                }
            }
            "get" => {
                if let Some(arg) = parts.get(1) {
                    match math.get_variable(arg) {
                        Some(value) => println!("{} = {}", arg, value),
                        None => println!("Variable not defined"),
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
