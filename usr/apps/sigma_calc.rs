// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// usr/apps/sigma_calc.rs — Sigma-Calc Calculator
//
// Implements a calculator with basic arithmetic operations,
// memory functions, and history tracking.
//
// Language: Rust (std for userland applications)

// ─── Calculator State ─────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Calculator {
    pub display: String,
    pub current_value: f64,
    pub memory: f64,
    pub operation: Option<char>,
    pub waiting_for_operand: bool,
    pub history: Vec<String>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            display: "0".to_string(),
            current_value: 0.0,
            memory: 0.0,
            operation: None,
            waiting_for_operand: false,
            history: Vec::new(),
        }
    }

    /// Input digit
    pub fn input_digit(&mut self, digit: char) {
        if self.waiting_for_operand {
            self.display.clear();
            self.waiting_for_operand = false;
        }
        
        if self.display == "0" && digit != '0' {
            self.display.clear();
        }
        
        if self.display.len() < 15 {
            self.display.push(digit);
        }
    }

    /// Input decimal point
    pub fn input_decimal(&mut self) {
        if self.waiting_for_operand {
            self.display = "0".to_string();
            self.waiting_for_operand = false;
        }
        
        if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    /// Set operation
    pub fn set_operation(&mut self, op: char) {
        if let Some(prev_op) = self.operation {
            self.calculate();
        }
        
        self.current_value = self.parse_display();
        self.operation = Some(op);
        self.waiting_for_operand = true;
    }

    /// Calculate result
    pub fn calculate(&mut self) {
        if let Some(op) = self.operation {
            let operand = self.parse_display();
            let result = match op {
                '+' => self.current_value + operand,
                '-' => self.current_value - operand,
                '*' => self.current_value * operand,
                '/' => {
                    if operand != 0.0 {
                        self.current_value / operand
                    } else {
                        self.display = "Error".to_string();
                        self.operation = None;
                        self.waiting_for_operand = true;
                        return;
                    }
                }
                '%' => self.current_value % operand,
                '^' => self.current_value.powf(operand),
                _ => operand,
            };
            
            // Add to history
            let history_entry = format!("{} {} {} = {}", self.current_value, op, operand, result);
            self.history.push(history_entry);
            
            self.display = self.format_result(result);
            self.current_value = result;
            self.operation = None;
            self.waiting_for_operand = true;
        }
    }

    /// Clear display
    pub fn clear(&mut self) {
        self.display = "0".to_string();
        self.current_value = 0.0;
        self.operation = None;
        self.waiting_for_operand = false;
    }

    /// Clear entry
    pub fn clear_entry(&mut self) {
        self.display = "0".to_string();
        self.waiting_for_operand = false;
    }

    /// Memory store
    pub fn memory_store(&mut self) {
        self.memory = self.parse_display();
    }

    /// Memory recall
    pub fn memory_recall(&mut self) {
        self.display = self.format_result(self.memory);
        self.waiting_for_operand = true;
    }

    /// Memory clear
    pub fn memory_clear(&mut self) {
        self.memory = 0.0;
    }

    /// Memory add
    pub fn memory_add(&mut self) {
        self.memory += self.parse_display();
    }

    /// Memory subtract
    pub fn memory_subtract(&mut self) {
        self.memory -= self.parse_display();
    }

    /// Negate
    pub fn negate(&mut self) {
        let value = self.parse_display();
        self.display = self.format_result(-value);
    }

    /// Square root
    pub fn square_root(&mut self) {
        let value = self.parse_display();
        if value >= 0.0 {
            self.display = self.format_result(value.sqrt());
        } else {
            self.display = "Error".to_string();
        }
        self.waiting_for_operand = true;
    }

    /// Square
    pub fn square(&mut self) {
        let value = self.parse_display();
        self.display = self.format_result(value * value);
        self.waiting_for_operand = true;
    }

    /// Percentage
    pub fn percentage(&mut self) {
        let value = self.parse_display();
        self.display = self.format_result(value / 100.0);
    }

    /// Parse display to f64
    fn parse_display(&self) -> f64 {
        self.display.parse().unwrap_or(0.0)
    }

    /// Format result for display
    fn format_result(&self, value: f64) -> String {
        if value.fract() == 0.0 {
            format!("{}", value as i64)
        } else {
            format!("{:.6}", value).trim_end_matches('0').trim_end_matches('.').to_string()
        }
    }

    /// Get display
    pub fn get_display(&self) -> &str {
        &self.display
    }

    /// Get history
    pub fn get_history(&self) -> &[String] {
        &self.history
    }
}

// ─── CLI Interface ───────────────────────────────────────────────────────────

fn main() {
    let mut calc = Calculator::new();
    
    println!("Sigma-Calc v0.1 - Calculator");
    println!("Display: {}", calc.get_display());
    
    loop {
        println!("\nCommands: <number>, +, -, *, /, %, ^, =, C, CE, MR, MC, M+, M-, sqrt, sq, %, neg, history, quit");
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        match input {
            "+" | "-" | "*" | "/" | "%" | "^" => {
                calc.set_operation(input.chars().next().unwrap());
            }
            "=" => {
                calc.calculate();
            }
            "C" => {
                calc.clear();
            }
            "CE" => {
                calc.clear_entry();
            }
            "MR" => {
                calc.memory_recall();
            }
            "MC" => {
                calc.memory_clear();
            }
            "M+" => {
                calc.memory_add();
            }
            "M-" => {
                calc.memory_subtract();
            }
            "sqrt" => {
                calc.square_root();
            }
            "sq" => {
                calc.square();
            }
            "%" => {
                calc.percentage();
            }
            "neg" => {
                calc.negate();
            }
            "history" => {
                println!("--- History ---");
                for entry in calc.get_history() {
                    println!("{}", entry);
                }
            }
            "quit" | "exit" => break,
            _ => {
                // Try to parse as number
                for c in input.chars() {
                    if c.is_digit(10) || c == '.' {
                        calc.input_digit(c);
                    } else if c == '.' {
                        calc.input_decimal();
                    }
                }
            }
        }
        
        println!("Display: {}", calc.get_display());
    }
}
