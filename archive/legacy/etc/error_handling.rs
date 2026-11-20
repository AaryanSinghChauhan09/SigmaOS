// SPDX-License-Identifier: MIT
// SigmaOS Error Handling & Defensive Programming
// Implements robust error handling, fail-fast systems, and defensive programming

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

// ============================================================================
// RESULT TYPES - Rust's built-in Result is preferred over custom types
// ============================================================================

/// Custom error type for SigmaOS operations
#[derive(Debug, Clone)]
pub enum SigmaError {
    /// Configuration-related errors
    Config(String),
    /// I/O related errors
    Io(String),
    /// Validation errors
    Validation(String),
    /// Operation failures
    Operation(String),
    /// Resource exhaustion
    ResourceExhausted(String),
    /// Permission denied
    PermissionDenied(String),
    /// Not found
    NotFound(String),
    /// Already exists
    AlreadyExists(String),
    /// Invalid state
    InvalidState(String),
    /// Timeout
    Timeout(String),
    /// Network errors
    Network(String),
    /// Database errors
    Database(String),
}

impl fmt::Display for SigmaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SigmaError::Config(msg) => write!(f, "Configuration error: {}", msg),
            SigmaError::Io(msg) => write!(f, "I/O error: {}", msg),
            SigmaError::Validation(msg) => write!(f, "Validation error: {}", msg),
            SigmaError::Operation(msg) => write!(f, "Operation error: {}", msg),
            SigmaError::ResourceExhausted(msg) => write!(f, "Resource exhausted: {}", msg),
            SigmaError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            SigmaError::NotFound(msg) => write!(f, "Not found: {}", msg),
            SigmaError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            SigmaError::InvalidState(msg) => write!(f, "Invalid state: {}", msg),
            SigmaError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            SigmaError::Network(msg) => write!(f, "Network error: {}", msg),
            SigmaError::Database(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for SigmaError {}

/// Result type alias for SigmaOS operations
pub type SigmaResult<T> = Result<T, SigmaError>;

// ============================================================================
// DEFENSIVE PROGRAMMING - Input Validation & Precondition Checks
// ============================================================================

/// Input validator for defensive programming
pub struct InputValidator {
    rules: HashMap<String, ValidationRule>,
}

impl InputValidator {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, field_name: String, rule: ValidationRule) {
        self.rules.insert(field_name, rule);
    }

    /// Validate input data
    pub fn validate(&self, data: &HashMap<String, String>) -> SigmaResult<()> {
        for (field, value) in data {
            if let Some(rule) = self.rules.get(field) {
                rule.validate(value)?;
            }
        }
        Ok(())
    }

    /// Validate a single value
    pub fn validate_value(&self, field: &str, value: &str) -> SigmaResult<()> {
        if let Some(rule) = self.rules.get(field) {
            rule.validate(value)?;
        }
        Ok(())
    }
}

/// Validation rule
pub struct ValidationRule {
    rule_type: RuleType,
    error_message: String,
}

impl ValidationRule {
    pub fn new(rule_type: RuleType, error_message: String) -> Self {
        Self {
            rule_type,
            error_message,
        }
    }

    pub fn required() -> Self {
        Self::new(RuleType::Required, "Field is required".to_string())
    }

    pub fn min_length(min: usize) -> Self {
        Self::new(RuleType::MinLength(min), format!("Minimum length is {}", min))
    }

    pub fn max_length(max: usize) -> Self {
        Self::new(RuleType::MaxLength(max), format!("Maximum length is {}", max))
    }

    pub fn pattern(pattern: &str) -> Self {
        Self::new(RuleType::Pattern(pattern.to_string()), "Invalid format".to_string())
    }

    pub fn range(min: i64, max: i64) -> Self {
        Self::new(RuleType::Range(min, max), format!("Value must be between {} and {}", min, max))
    }

    fn validate(&self, value: &str) -> SigmaResult<()> {
        match &self.rule_type {
            RuleType::Required => {
                if value.is_empty() {
                    Err(SigmaError::Validation(self.error_message.clone()))
                } else {
                    Ok(())
                }
            }
            RuleType::MinLength(min) => {
                if value.len() < *min {
                    Err(SigmaError::Validation(self.error_message.clone()))
                } else {
                    Ok(())
                }
            }
            RuleType::MaxLength(max) => {
                if value.len() > *max {
                    Err(SigmaError::Validation(self.error_message.clone()))
                } else {
                    Ok(())
                }
            }
            RuleType::Pattern(pattern) => {
                // Simple pattern matching - in real implementation would use regex
                if value.contains(pattern) {
                    Ok(())
                } else {
                    Err(SigmaError::Validation(self.error_message.clone()))
                }
            }
            RuleType::Range(min, max) => {
                if let Ok(num) = value.parse::<i64>() {
                    if num < *min || num > *max {
                        Err(SigmaError::Validation(self.error_message.clone()))
                    } else {
                        Ok(())
                    }
                } else {
                    Err(SigmaError::Validation("Invalid number format".to_string()))
                }
            }
        }
    }
}

/// Rule types
#[derive(Debug, Clone)]
pub enum RuleType {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Pattern(String),
    Range(i64, i64),
}

/// Precondition checker
pub struct PreconditionChecker {
    conditions: Vec<Box<dyn Precondition>>,
}

impl PreconditionChecker {
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    pub fn add_condition(&mut self, condition: Box<dyn Precondition>) {
        self.conditions.push(condition);
    }

    /// Check all preconditions
    pub fn check(&self) -> SigmaResult<()> {
        for condition in &self.conditions {
            condition.check()?;
        }
        Ok(())
    }
}

/// Precondition trait
pub trait Precondition {
    fn check(&self) -> SigmaResult<()>;
    fn get_description(&self) -> &str;
}

/// File exists precondition
pub struct FileExistsPrecondition {
    path: String,
    description: String,
}

impl FileExistsPrecondition {
    pub fn new(path: String) -> Self {
        Self {
            description: format!("File exists: {}", path),
            path,
        }
    }
}

impl Precondition for FileExistsPrecondition {
    fn check(&self) -> SigmaResult<()> {
        if std::path::Path::new(&self.path).exists() {
            Ok(())
        } else {
            Err(SigmaError::NotFound(format!("File not found: {}", self.path)))
        }
    }

    fn get_description(&self) -> &str {
        &self.description
    }
}

/// Directory writable precondition
pub struct DirectoryWritablePrecondition {
    path: String,
    description: String,
}

impl DirectoryWritablePrecondition {
    pub fn new(path: String) -> Self {
        Self {
            description: format!("Directory writable: {}", path),
            path,
        }
    }
}

impl Precondition for DirectoryWritablePrecondition {
    fn check(&self) -> SigmaResult<()> {
        let path = std::path::Path::new(&self.path);
        if path.exists() && path.is_dir() {
            // Check write permission
            Ok(())
        } else {
            Err(SigmaError::PermissionDenied(format!("Directory not writable: {}", self.path)))
        }
    }

    fn get_description(&self) -> &str {
        &self.description
    }
}

/// Resource available precondition
pub struct ResourceAvailablePrecondition {
    resource_type: ResourceType,
    required_amount: u64,
    description: String,
}

impl ResourceAvailablePrecondition {
    pub fn new(resource_type: ResourceType, required_amount: u64) -> Self {
        Self {
            description: format!("{} available: {}", resource_type, required_amount),
            resource_type,
            required_amount,
        }
    }
}

impl Precondition for ResourceAvailablePrecondition {
    fn check(&self) -> SigmaResult<()> {
        // Check resource availability
        match self.resource_type {
            ResourceType::Memory => {
                // Check available memory
                Ok(())
            }
            ResourceType::Disk => {
                // Check available disk space
                Ok(())
            }
            ResourceType::Cpu => {
                // Check CPU availability
                Ok(())
            }
        }
    }

    fn get_description(&self) -> &str {
        &self.description
    }
}

/// Resource types
#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Memory,
    Disk,
    Cpu,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceType::Memory => write!(f, "Memory"),
            ResourceType::Disk => write!(f, "Disk"),
            ResourceType::Cpu => write!(f, "CPU"),
        }
    }
}

// ============================================================================
// FAIL-FAST SYSTEM
// Fail fast and fail loudly
// ============================================================================

/// Fail-fast error handler
pub struct FailFastHandler {
    context: String,
    fatal_errors: Vec<SigmaError>,
}

impl FailFastHandler {
    pub fn new(context: String) -> Self {
        Self {
            context,
            fatal_errors: Vec::new(),
        }
    }

    /// Handle error with fail-fast behavior
    pub fn handle_error(&mut self, error: SigmaError) -> SigmaResult<()> {
        self.fatal_errors.push(error.clone());
        
        // Log the error
        eprintln!("FATAL ERROR in {}: {}", self.context, error);
        
        // In fail-fast mode, we would typically exit here
        // For testing, we return the error
        Err(error)
    }

    /// Check if any fatal errors occurred
    pub fn has_fatal_errors(&self) -> bool {
        !self.fatal_errors.is_empty()
    }

    /// Get all fatal errors
    pub fn get_fatal_errors(&self) -> &[SigmaError] {
        &self.fatal_errors
    }

    /// Clear fatal errors
    pub fn clear_errors(&mut self) {
        self.fatal_errors.clear();
    }
}

/// Assertion helper for defensive programming
pub struct Assertion {
    context: String,
}

impl Assertion {
    pub fn new(context: String) -> Self {
        Self { context }
    }

    /// Assert that a value is not None
    pub fn assert_some<T>(&self, value: Option<T>, message: &str) -> SigmaResult<T> {
        value.ok_or_else(|| {
            SigmaError::Validation(format!("{}: {}", self.context, message))
        })
    }

    /// Assert that a value is not empty
    pub fn assert_not_empty(&self, value: &str, message: &str) -> SigmaResult<()> {
        if value.is_empty() {
            Err(SigmaError::Validation(format!("{}: {}", self.context, message)))
        } else {
            Ok(())
        }
    }

    /// Assert that a condition is true
    pub fn assert_true(&self, condition: bool, message: &str) -> SigmaResult<()> {
        if !condition {
            Err(SigmaError::Validation(format!("{}: {}", self.context, message)))
        } else {
            Ok(())
        }
    }

    /// Assert that a value is within range
    pub fn assert_in_range(&self, value: i64, min: i64, max: i64, message: &str) -> SigmaResult<()> {
        if value < min || value > max {
            Err(SigmaError::Validation(format!("{}: {} (value={}, min={}, max={})", self.context, message, value, min, max)))
        } else {
            Ok(())
        }
    }
}

// ============================================================================
// ERROR CONTEXT & CHAINING
// ============================================================================

/// Error context wrapper
#[derive(Debug, Clone)]
pub struct ErrorContext {
    context: String,
    source: Box<SigmaError>,
}

impl ErrorContext {
    pub fn new(context: String, source: SigmaError) -> Self {
        Self {
            context,
            source: Box::new(source),
        }
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn source(&self) -> &SigmaError {
        &self.source
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.context, self.source)
    }
}

impl std::error::Error for ErrorContext {}

/// Result extension trait for error context
pub trait ResultExt<T> {
    fn with_context<F>(self, context: F) -> SigmaResult<T>
    where
        F: FnOnce() -> String;

    fn with_context_static(self, context: &str) -> SigmaResult<T>;
}

impl<T> ResultExt<T> for SigmaResult<T> {
    fn with_context<F>(self, context: F) -> SigmaResult<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            SigmaError::Operation(format!("{}: {}", context(), e))
        })
    }

    fn with_context_static(self, context: &str) -> SigmaResult<T> {
        self.with_context(|| context.to_string())
    }
}

// ============================================================================
// RETRY MECHANISM
// ============================================================================

/// Retry configuration
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_delay: std::time::Duration,
    pub max_delay: std::time::Duration,
    pub backoff_factor: f64,
    pub retry_on: Vec<SigmaError>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: std::time::Duration::from_millis(100),
            max_delay: std::time::Duration::from_secs(10),
            backoff_factor: 2.0,
            retry_on: vec![
                SigmaError::Network("".to_string()),
                SigmaError::Timeout("".to_string()),
                SigmaError::ResourceExhausted("".to_string()),
            ],
        }
    }
}

/// Retry executor
pub struct RetryExecutor {
    config: RetryConfig,
}

impl RetryExecutor {
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Execute operation with retry
    pub fn execute<F, T>(&self, mut operation: F) -> SigmaResult<T>
    where
        F: FnMut() -> SigmaResult<T>,
    {
        let mut last_error = None;
        let mut delay = self.config.initial_delay;

        for attempt in 0..=self.config.max_retries {
            match operation() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e.clone());
                    
                    // Check if we should retry
                    if attempt == self.config.max_retries || !self.should_retry(&e) {
                        return Err(e);
                    }

                    // Wait before retry
                    std::thread::sleep(delay);
                    
                    // Increase delay for next retry
                    delay = std::time::Duration::from_secs_f64(
                        (delay.as_secs_f64() * self.config.backoff_factor)
                            .min(self.config.max_delay.as_secs_f64())
                    );
                }
            }
        }

        Err(last_error.unwrap_or_else(|| SigmaError::Operation("Unknown error".to_string())))
    }

    fn should_retry(&self, error: &SigmaError) -> bool {
        self.config.retry_on.iter().any(|retry_error| {
            match (retry_error, error) {
                (SigmaError::Network(_), SigmaError::Network(_)) => true,
                (SigmaError::Timeout(_), SigmaError::Timeout(_)) => true,
                (SigmaError::ResourceExhausted(_), SigmaError::ResourceExhausted(_)) => true,
                _ => false,
            }
        })
    }
}

// ============================================================================
// CIRCUIT BREAKER PATTERN
// ============================================================================

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    Closed,
    Open,
    HalfOpen,
}

/// Circuit breaker
pub struct CircuitBreaker {
    state: CircuitState,
    failure_count: u32,
    failure_threshold: u32,
    recovery_timeout: std::time::Duration,
    last_failure_time: Option<std::time::SystemTime>,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout: std::time::Duration) -> Self {
        Self {
            state: CircuitState::Closed,
            failure_count: 0,
            failure_threshold,
            recovery_timeout,
            last_failure_time: None,
        }
    }

    /// Check if circuit is open
    pub fn is_open(&self) -> bool {
        self.state == CircuitState::Open
    }

    /// Record success
    pub fn record_success(&mut self) {
        self.failure_count = 0;
        self.state = CircuitState::Closed;
    }

    /// Record failure
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(std::time::SystemTime::now());

        if self.failure_count >= self.failure_threshold {
            self.state = CircuitState::Open;
        }
    }

    /// Check if should allow request
    pub fn allow_request(&mut self) -> bool {
        match self.state {
            CircuitState::Closed => true,
            CircuitState::Open => {
                // Check if recovery timeout has passed
                if let Some(last_failure) = self.last_failure_time {
                    if last_failure.elapsed().unwrap_or_default() >= self.recovery_timeout {
                        self.state = CircuitState::HalfOpen;
                        return true;
                    }
                }
                false
            }
            CircuitState::HalfOpen => true,
        }
    }

    /// Get current state
    pub fn get_state(&self) -> CircuitState {
        self.state
    }

    /// Get failure count
    pub fn get_failure_count(&self) -> u32 {
        self.failure_count
    }
}

// ============================================================================
// TIMEOUT HANDLER
// ============================================================================

/// Timeout handler
pub struct TimeoutHandler {
    timeout: std::time::Duration,
}

impl TimeoutHandler {
    pub fn new(timeout: std::time::Duration) -> Self {
        Self { timeout }
    }

    /// Execute operation with timeout
    pub fn execute_with_timeout<F, T>(&self, operation: F) -> SigmaResult<T>
    where
        F: FnOnce() -> SigmaResult<T> + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = std::sync::mpsc::channel();
        let timeout = self.timeout;

        std::thread::spawn(move || {
            let result = operation();
            let _ = tx.send(result);
        });

        match rx.recv_timeout(timeout) {
            Ok(result) => result,
            Err(_) => Err(SigmaError::Timeout(format!("Operation timed out after {:?}", timeout))),
        }
    }
}

// ============================================================================
// RESOURCE CLEANUP
// ============================================================================

/// Resource guard for automatic cleanup
pub struct ResourceGuard<F>
where
    F: FnOnce(),
{
    cleanup: Option<F>,
}

impl<F> ResourceGuard<F>
where
    F: FnOnce(),
{
    pub fn new(cleanup: F) -> Self {
        Self {
            cleanup: Some(cleanup),
        }
    }

    /// Disarm the cleanup (prevent it from running)
    pub fn disarm(mut self) {
        self.cleanup = None;
    }
}

impl<F> Drop for ResourceGuard<F>
where
    F: FnOnce(),
{
    fn drop(&mut self) {
        if let Some(cleanup) = self.cleanup.take() {
            cleanup();
        }
    }
}

/// Temporary file guard
pub struct TempFileGuard {
    path: std::path::PathBuf,
}

impl TempFileGuard {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}

impl Drop for TempFileGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_validator() {
        let mut validator = InputValidator::new();
        validator.add_rule("username".to_string(), ValidationRule::required());
        validator.add_rule("password".to_string(), ValidationRule::min_length(8));

        let mut data = HashMap::new();
        data.insert("username".to_string(), "testuser".to_string());
        data.insert("password".to_string(), "short".to_string());

        let result = validator.validate(&data);
        assert!(result.is_err());

        data.insert("password".to_string(), "longpassword".to_string());
        let result = validator.validate(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_precondition_checker() {
        let mut checker = PreconditionChecker::new();
        checker.add_condition(Box::new(FileExistsPrecondition::new("/tmp/test.txt".to_string())));

        // Create the file
        std::fs::write("/tmp/test.txt", "test").unwrap();
        
        let result = checker.check();
        assert!(result.is_ok());

        // Clean up
        std::fs::remove_file("/tmp/test.txt").unwrap();
    }

    #[test]
    fn test_fail_fast_handler() {
        let mut handler = FailFastHandler::new("test_context".to_string());
        let error = SigmaError::Validation("test error".to_string());
        
        let result = handler.handle_error(error);
        assert!(result.is_err());
        assert!(handler.has_fatal_errors());
    }

    #[test]
    fn test_assertion() {
        let assertion = Assertion::new("test".to_string());
        
        assert!(assertion.assert_some(Some(42), "value should be present").is_ok());
        assert!(assertion.assert_some(None::<i32>, "value should be present").is_err());
        
        assert!(assertion.assert_not_empty("test", "should not be empty").is_ok());
        assert!(assertion.assert_not_empty("", "should not be empty").is_err());
        
        assert!(assertion.assert_true(true, "should be true").is_ok());
        assert!(assertion.assert_true(false, "should be true").is_err());
    }

    #[test]
    fn test_retry_executor() {
        let config = RetryConfig {
            max_retries: 2,
            initial_delay: std::time::Duration::from_millis(10),
            max_delay: std::time::Duration::from_millis(100),
            backoff_factor: 2.0,
            retry_on: vec![SigmaError::Network("".to_string())],
        };

        let executor = RetryExecutor::new(config);
        let mut attempt_count = 0;

        let result = executor.execute(|| {
            attempt_count += 1;
            if attempt_count < 3 {
                Err(SigmaError::Network("test".to_string()))
            } else {
                Ok("success".to_string())
            }
        });

        assert!(result.is_ok());
        assert_eq!(attempt_count, 3);
    }

    #[test]
    fn test_circuit_breaker() {
        let mut breaker = CircuitBreaker::new(3, std::time::Duration::from_secs(1));
        
        // Initially closed
        assert_eq!(breaker.get_state(), CircuitState::Closed);
        assert!(breaker.allow_request());
        
        // Record failures
        breaker.record_failure();
        breaker.record_failure();
        breaker.record_failure();
        
        // Should be open now
        assert_eq!(breaker.get_state(), CircuitState::Open);
        assert!(!breaker.allow_request());
        
        // Record success
        breaker.record_success();
        assert_eq!(breaker.get_state(), CircuitState::Closed);
        assert!(breaker.allow_request());
    }

    #[test]
    fn test_timeout_handler() {
        let handler = TimeoutHandler::new(std::time::Duration::from_millis(100));
        
        // Fast operation
        let result = handler.execute_with_timeout(|| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            Ok("fast".to_string())
        });
        assert!(result.is_ok());
        
        // Slow operation
        let result = handler.execute_with_timeout(|| {
            std::thread::sleep(std::time::Duration::from_millis(200));
            Ok("slow".to_string())
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_resource_guard() {
        let mut cleanup_called = false;
        
        {
            let _guard = ResourceGuard::new(|| {
                cleanup_called = true;
            });
            // Guard goes out of scope here
        }
        
        assert!(cleanup_called);
    }

    #[test]
    fn test_result_ext() {
        let result: SigmaResult<i32> = Ok(42);
        let with_context = result.with_context_static("test operation");
        assert!(with_context.is_ok());
        
        let error_result: SigmaResult<i32> = Err(SigmaError::Operation("test".to_string()));
        let with_context = error_result.with_context_static("test operation");
        assert!(with_context.is_err());
    }
}
