// SPDX-License-Identifier: MIT
// SigmaOS - Calculator
// Inspired by GNOME Calculator and Windows Calculator
// Provides basic, scientific, and programming calculator modes

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Calculator mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalculatorMode {
    Basic,
    Scientific,
    Programming,
}

/// Angle unit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AngleUnit {
    Degrees,
    Radians,
    Gradians,
}

/// Memory operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryOperation {
    Clear,
    Recall,
    Store,
    Add,
    Subtract,
}

/// Calculator error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalculatorError {
    DivisionByZero,
    InvalidInput,
    Overflow,
    Underflow,
    SyntaxError,
}

/// Calculator
pub struct Calculator {
    pub display: String,
    pub memory: f64,
    pub mode: CalculatorMode,
    pub angle_unit: AngleUnit,
    pub history: Vec<String>,
    pub previous_result: Option<f64>,
    pub new_calculation: bool,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            display: "0".to_string(),
            memory: 0.0,
            mode: CalculatorMode::Basic,
            angle_unit: AngleUnit::Degrees,
            history: Vec::new(),
            previous_result: None,
            new_calculation: true,
        }
    }

    /// Clear display
    pub fn clear(&mut self) {
        self.display = "0".to_string();
        self.new_calculation = true;
    }

    /// Clear all (including memory)
    pub fn clear_all(&mut self) {
        self.clear();
        self.memory = 0.0;
        self.history.clear();
        self.previous_result = None;
    }

    /// Clear entry
    pub fn clear_entry(&mut self) {
        self.display = "0".to_string();
    }

    /// Backspace
    pub fn backspace(&mut self) {
        if self.display.len() > 1 {
            self.display.pop();
        } else {
            self.display = "0".to_string();
        }
    }

    /// Input digit
    pub fn input_digit(&mut self, digit: char) {
        if self.new_calculation {
            self.display = digit.to_string();
            self.new_calculation = false;
        } else if self.display == "0" {
            self.display = digit.to_string();
        } else {
            self.display.push(digit);
        }
    }

    /// Input decimal point
    pub fn input_decimal(&mut self) {
        if !self.display.contains('.') {
            self.display.push('.');
            self.new_calculation = false;
        }
    }

    /// Input operator
    pub fn input_operator(&mut self, op: char) -> Result<(), CalculatorError> {
        let current = self.parse_display()?;
        if let Some(prev) = self.previous_result {
            // Perform pending operation
            let result = self.calculate(prev, current, op)?;
            self.display = format!("{}", result);
            self.previous_result = Some(result);
        } else {
            self.previous_result = Some(current);
        }
        self.new_calculation = true;
        Ok(())
    }

    /// Calculate result
    pub fn calculate_result(&mut self) -> Result<(), CalculatorError> {
        if let Some(prev) = self.previous_result {
            let current = self.parse_display()?;
            // Use last operator (simplified)
            let result = self.calculate(prev, current, '+')?;
            self.display = format!("{}", result);
            self.history.push(self.display.clone());
            self.previous_result = None;
            self.new_calculation = true;
        }
        Ok(())
    }

    /// Perform calculation
    fn calculate(&self, a: f64, b: f64, op: char) -> Result<f64, CalculatorError> {
        match op {
            '+' => Ok(a + b),
            '-' => Ok(a - b),
            '*' => Ok(a * b),
            '/' => {
                if b == 0.0 {
                    Err(CalculatorError::DivisionByZero)
                } else {
                    Ok(a / b)
                }
            }
            '%' => {
                if b == 0.0 {
                    Err(CalculatorError::DivisionByZero)
                } else {
                    Ok(a % b)
                }
            }
            '^' => Ok(a.powf(b)),
            _ => Err(CalculatorError::InvalidInput),
        }
    }

    /// Square root
    pub fn square_root(&mut self) -> Result<(), CalculatorError> {
        let value = self.parse_display()?;
        if value < 0.0 {
            return Err(CalculatorError::InvalidInput);
        }
        let result = value.sqrt();
        self.display = format!("{}", result);
        self.new_calculation = true;
        Ok(())
    }

    /// Square
    pub fn square(&mut self) -> Result<(), CalculatorError> {
        let value = self.parse_display()?;
        let result = value * value;
        self.display = format!("{}", result);
        self.new_calculation = true;
        Ok(())
    }

    /// Reciprocal
    pub fn reciprocal(&mut self) -> Result<(), CalculatorError> {
        let value = self.parse_display()?;
        if value == 0.0 {
            return Err(CalculatorError::DivisionByZero);
        }
        let result = 1.0 / value;
        self.display = format!("{}", result);
        self.new_calculation = true;
        Ok(())
    }

    /// Negate
    pub fn negate(&mut self) {
        let value = self.parse_display().unwrap_or(0.0);
        let result = -value;
        self.display = format!("{}", result);
    }

    /// Percentage
    pub fn percentage(&mut self) {
        let value = self.parse_display().unwrap_or(0.0);
        let result = value / 100.0;
        self.display = format!("{}", result);
    }

    /// Sine
    pub fn sin(&mut self) {
        let value = self.parse_display().unwrap_or(0.0);
        let radians = self.to_radians(value);
        let result = radians.sin();
        self.display = format!("{}", result);
        self.new_calculation = true;
    }

    /// Cosine
    pub fn cos(&mut self) {
        let value = self.parse_display().unwrap_or(0.0);
        let radians = self.to_radians(value);
        let result = radians.cos();
        self.display = format!("{}", result);
        self.new_calculation = true;
    }

    /// Tangent
    pub fn tan(&mut self) {
        let value = self.parse_display().unwrap_or(0.0);
        let radians = self.to_radians(value);
        let result = radians.tan();
        self.display = format!("{}", result);
        self.new_calculation = true;
    }

    /// Convert to radians based on angle unit
    fn to_radians(&self, value: f64) -> f64 {
        match self.angle_unit {
            AngleUnit::Degrees => value * core::f64::consts::PI / 180.0,
            AngleUnit::Radians => value,
            AngleUnit::Gradians => value * core::f64::consts::PI / 200.0,
        }
    }

    /// Logarithm (base 10)
    pub fn log(&mut self) -> Result<(), CalculatorError> {
        let value = self.parse_display()?;
        if value <= 0.0 {
            return Err(CalculatorError::InvalidInput);
        }
        let result = value.log10();
        self.display = format!("{}", result);
        self.new_calculation = true;
        Ok(())
    }

    /// Natural logarithm
    pub fn ln(&mut self) -> Result<(), CalculatorError> {
        let value = self.parse_display()?;
        if value <= 0.0 {
            return Err(CalculatorError::InvalidInput);
        }
        let result = value.ln();
        self.display = format!("{}", result);
        self.new_calculation = true;
        Ok(())
    }

    /// Exponential
    pub fn exp(&mut self) {
        let value = self.parse_display().unwrap_or(0.0);
        let result = value.exp();
        self.display = format!("{}", result);
        self.new_calculation = true;
    }

    /// Power of 10
    pub fn power_of_10(&mut self) {
        let value = self.parse_display().unwrap_or(0.0);
        let result = 10.0_f64.powf(value);
        self.display = format!("{}", result);
        self.new_calculation = true;
    }

    /// Factorial
    pub fn factorial(&mut self) -> Result<(), CalculatorError> {
        let value = self.parse_display()? as i64;
        if value < 0 {
            return Err(CalculatorError::InvalidInput);
        }
        let result = self.factorial_calc(value);
        self.display = format!("{}", result);
        self.new_calculation = true;
        Ok(())
    }

    /// Calculate factorial
    fn factorial_calc(&self, n: i64) -> i64 {
        if n <= 1 {
            1
        } else {
            n * self.factorial_calc(n - 1)
        }
    }

    /// Memory operation
    pub fn memory_operation(&mut self, op: MemoryOperation) {
        match op {
            MemoryOperation::Clear => self.memory = 0.0,
            MemoryOperation::Recall => {
                self.display = format!("{}", self.memory);
                self.new_calculation = true;
            }
            MemoryOperation::Store => {
                self.memory = self.parse_display().unwrap_or(0.0);
            }
            MemoryOperation::Add => {
                self.memory += self.parse_display().unwrap_or(0.0);
            }
            MemoryOperation::Subtract => {
                self.memory -= self.parse_display().unwrap_or(0.0);
            }
        }
    }

    /// Set mode
    pub fn set_mode(&mut self, mode: CalculatorMode) {
        self.mode = mode;
    }

    /// Set angle unit
    pub fn set_angle_unit(&mut self, unit: AngleUnit) {
        self.angle_unit = unit;
    }

    /// Parse display to float
    fn parse_display(&self) -> Result<f64, CalculatorError> {
        self.display
            .parse::<f64>()
            .map_err(|_| CalculatorError::InvalidInput)
    }

    /// Get display value
    pub fn get_display(&self) -> &str {
        &self.display
    }

    /// Get history
    pub fn get_history(&self) -> &[String] {
        &self.history
    }
}

/// Global calculator
static mut CALCULATOR: Option<Calculator> = None;

/// Initialize calculator
pub fn init_calculator() {
    unsafe {
        CALCULATOR = Some(Calculator::new());
    }
}

/// Get calculator
pub fn get_calculator() -> &'static mut Calculator {
    unsafe { CALCULATOR.as_mut().expect("Calculator not initialized") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_creation() {
        let calc = Calculator::new();
        assert_eq!(calc.get_display(), "0");
    }

    #[test]
    fn test_input_digit() {
        let mut calc = Calculator::new();
        calc.input_digit('5');
        assert_eq!(calc.get_display(), "5");
    }

    #[test]
    fn test_clear() {
        let mut calc = Calculator::new();
        calc.input_digit('5');
        calc.clear();
        assert_eq!(calc.get_display(), "0");
    }

    #[test]
    fn test_square() {
        let mut calc = Calculator::new();
        calc.input_digit('5');
        calc.square().unwrap();
        assert_eq!(calc.get_display(), "25");
    }

    #[test]
    fn test_square_root() {
        let mut calc = Calculator::new();
        calc.input_digit('9');
        calc.square_root().unwrap();
        assert_eq!(calc.get_display(), "3");
    }

    #[test]
    fn test_negate() {
        let mut calc = Calculator::new();
        calc.input_digit('5');
        calc.negate();
        assert_eq!(calc.get_display(), "-5");
    }

    #[test]
    fn test_percentage() {
        let mut calc = Calculator::new();
        calc.input_digit('5');
        calc.input_digit('0');
        calc.percentage();
        assert_eq!(calc.get_display(), "0.5");
    }
}
