//! SigmaOS calculator application.
//!
//! Provides basic and scientific calculations with a headless engine.

#![warn(missing_docs)]

/// A calculator mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Basic arithmetic.
    Basic,
    /// Scientific functions.
    Scientific,
    /// Programmer (base conversion).
    Programmer,
}

/// A parsed expression ready for evaluation.
#[derive(Debug, Clone)]
pub struct Expression {
    /// The raw expression string.
    pub source: String,
    /// Mode the expression was entered in.
    pub mode: Mode,
}

/// Trait for the calculator engine.
pub trait CalculatorEngine {
    /// Evaluate an expression and return the result.
    fn evaluate(&self, expr: &Expression) -> Result<f64, CalcError>;
    /// Store a value in a named memory slot.
    fn store(&mut self, slot: &str, value: f64) -> Result<(), CalcError>;
    /// Recall a value from a named memory slot.
    fn recall(&self, slot: &str) -> Result<f64, CalcError>;
}

/// Errors returned by the calculator engine.
#[derive(Debug)]
pub enum CalcError {
    /// The expression could not be parsed.
    ParseError(String),
    /// Division by zero.
    DivideByZero,
    /// The memory slot does not exist.
    SlotNotFound,
}

impl std::fmt::Display for CalcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalcError::ParseError(msg) => write!(f, "parse error: {msg}"),
            CalcError::DivideByZero => write!(f, "division by zero"),
            CalcError::SlotNotFound => write!(f, "memory slot not found"),
        }
    }
}

impl std::error::Error for CalcError {}
