// Development Testing Framework for SigmaOS
// Development testing per Wiki 10-Development.md
// Provides unit test, integration test, and standalone test management

use std::string::{String, ToString};
use std::vec::Vec;

/// Test type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestType {
    Unit,
    Integration,
    Standalone,
}

impl TestType {
    pub fn as_str(&self) -> &str {
        match self {
            TestType::Unit => "unit",
            TestType::Integration => "integration",
            TestType::Standalone => "standalone",
        }
    }
}

/// Test status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestStatus {
    Pending,
    Running,
    Passed,
    Failed,
    Skipped,
}

impl TestStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TestStatus::Pending => "pending",
            TestStatus::Running => "running",
            TestStatus::Passed => "passed",
            TestStatus::Failed => "failed",
            TestStatus::Skipped => "skipped",
        }
    }
}

/// Test result
#[derive(Debug, Clone)]
pub struct TestResult {
    pub test_name: String,
    pub test_type: TestType,
    pub status: TestStatus,
    pub duration_ms: u64,
    pub output: String,
    pub error_message: Option<String>,
}

impl TestResult {
    pub fn new(test_name: String, test_type: TestType) -> Self {
        TestResult {
            test_name,
            test_type,
            status: TestStatus::Pending,
            duration_ms: 0,
            output: String::new(),
            error_message: None,
        }
    }

    pub fn passed(&mut self, duration_ms: u64, output: String) {
        self.status = TestStatus::Passed;
        self.duration_ms = duration_ms;
        self.output = output;
    }

    pub fn failed(&mut self, duration_ms: u64, output: String, error_message: String) {
        self.status = TestStatus::Failed;
        self.duration_ms = duration_ms;
        self.output = output;
        self.error_message = Some(error_message);
    }

    pub fn skipped(&mut self) {
        self.status = TestStatus::Skipped;
    }
}

/// Test suite
#[derive(Debug, Clone)]
pub struct TestSuite {
    pub name: String,
    pub test_type: TestType,
    pub tests: Vec<TestResult>,
}

impl TestSuite {
    pub fn new(name: String, test_type: TestType) -> Self {
        TestSuite {
            name,
            test_type,
            tests: Vec::new(),
        }
    }

    pub fn add_test(&mut self, test: TestResult) {
        self.tests.push(test);
    }

    pub fn get_passed_count(&self) -> usize {
        self.tests.iter().filter(|t| t.status == TestStatus::Passed).count()
    }

    pub fn get_failed_count(&self) -> usize {
        self.tests.iter().filter(|t| t.status == TestStatus::Failed).count()
    }

    pub fn get_skipped_count(&self) -> usize {
        self.tests.iter().filter(|t| t.status == TestStatus::Skipped).count()
    }

    pub fn get_total_duration_ms(&self) -> u64 {
        self.tests.iter().map(|t| t.duration_ms).sum()
    }
}

/// Test configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub verbose: bool,
    pub nocapture: bool,
    pub fail_fast: bool,
    pub timeout_seconds: u64,
}

impl Default for TestConfig {
    fn default() -> Self {
        TestConfig {
            verbose: false,
            nocapture: false,
            fail_fast: false,
            timeout_seconds: 300,
        }
    }
}

/// Development testing framework
#[derive(Debug, Clone)]
pub struct DevelopmentTestingFramework {
    pub test_suites: Vec<TestSuite>,
    pub config: TestConfig,
}

impl Default for DevelopmentTestingFramework {
    fn default() -> Self {
        DevelopmentTestingFramework {
            test_suites: Vec::new(),
            config: TestConfig::default(),
        }
    }
}

impl DevelopmentTestingFramework {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_config(&mut self, config: TestConfig) {
        self.config = config;
    }

    pub fn add_test_suite(&mut self, suite: TestSuite) {
        self.test_suites.push(suite);
    }

    pub fn create_unit_test_suite(&mut self, name: String) -> TestSuite {
        let suite = TestSuite::new(name, TestType::Unit);
        self.test_suites.push(suite.clone());
        suite
    }

    pub fn create_integration_test_suite(&mut self, name: String) -> TestSuite {
        let suite = TestSuite::new(name, TestType::Integration);
        self.test_suites.push(suite.clone());
        suite
    }

    pub fn create_standalone_test_suite(&mut self, name: String) -> TestSuite {
        let suite = TestSuite::new(name, TestType::Standalone);
        self.test_suites.push(suite.clone());
        suite
    }

    pub fn run_all_tests(&self) -> Vec<TestResult> {
        let mut all_results = Vec::new();

        for suite in &self.test_suites {
            all_results.extend(suite.tests.clone());
        }

        all_results
    }

    pub fn run_unit_tests(&self) -> Vec<TestResult> {
        self.test_suites.iter()
            .filter(|s| s.test_type == TestType::Unit)
            .flat_map(|s| s.tests.clone())
            .collect()
    }

    pub fn run_integration_tests(&self) -> Vec<TestResult> {
        self.test_suites.iter()
            .filter(|s| s.test_type == TestType::Integration)
            .flat_map(|s| s.tests.clone())
            .collect()
    }

    pub fn run_standalone_tests(&self) -> Vec<TestResult> {
        self.test_suites.iter()
            .filter(|s| s.test_type == TestType::Standalone)
            .flat_map(|s| s.tests.clone())
            .collect()
    }

    pub fn get_statistics(&self) -> String {
        let mut stats = String::from("Development Testing Statistics:\n");

        let total_tests: usize = self.test_suites.iter().map(|s| s.tests.len()).sum();
        let passed: usize = self.test_suites.iter().map(|s| s.get_passed_count()).sum();
        let failed: usize = self.test_suites.iter().map(|s| s.get_failed_count()).sum();
        let skipped: usize = self.test_suites.iter().map(|s| s.get_skipped_count()).sum();
        let total_duration: u64 = self.test_suites.iter().map(|s| s.get_total_duration_ms()).sum();

        stats.push_str(&format!("Total test suites: {}\n", self.test_suites.len()));
        stats.push_str(&format!("Total tests: {}\n", total_tests));
        stats.push_str(&format!("Passed: {}\n", passed));
        stats.push_str(&format!("Failed: {}\n", failed));
        stats.push_str(&format!("Skipped: {}\n", skipped));
        stats.push_str(&format!("Success rate: {:.1}%\n", (passed as f64 / total_tests as f64) * 100.0));
        stats.push_str(&format!("Total duration: {}ms\n", total_duration));

        if self.config.verbose {
            stats.push_str("\nPer-suite statistics:\n");
            for suite in &self.test_suites {
                stats.push_str(&format!("  {}: {}/{} passed ({}ms)\n",
                    suite.name,
                    suite.get_passed_count(),
                    suite.tests.len(),
                    suite.get_total_duration_ms()
                ));
            }
        }

        stats
    }

    pub fn get_test_command(&self, test_type: TestType) -> String {
        match test_type {
            TestType::Unit => {
                let mut cmd = String::from("cargo test");
                if self.config.nocapture {
                    cmd.push_str(" -- --nocapture");
                }
                if self.config.fail_fast {
                    cmd.push_str(" -- --fail-fast");
                }
                cmd
            }
            TestType::Integration => {
                let mut cmd = String::from("cargo test --test integration");
                if self.config.nocapture {
                    cmd.push_str(" -- --nocapture");
                }
                cmd
            }
            TestType::Standalone => {
                String::from("./run_sigma_tests.sh")
            }
        }
    }

    pub fn get_specific_test_command(&self, test_name: String) -> String {
        format!("cargo test {}", test_name)
    }

    pub fn list_all_tests(&self) -> Vec<String> {
        self.test_suites.iter()
            .flat_map(|s| s.tests.iter().map(|t| format!("{}::{} ({})", s.name, t.test_name, t.test_type.as_str())))
            .collect()
    }

    pub fn list_failed_tests(&self) -> Vec<String> {
        self.test_suites.iter()
            .flat_map(|s| s.tests.iter()
                .filter(|t| t.status == TestStatus::Failed)
                .map(|t| format!("{}::{} - {}", s.name, t.test_name, t.error_message.as_ref().unwrap_or(&String::from("Unknown error"))))
            )
            .collect()
    }

    pub fn reset(&mut self) {
        self.test_suites.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_type_as_str() {
        assert_eq!(TestType::Unit.as_str(), "unit");
        assert_eq!(TestType::Integration.as_str(), "integration");
    }

    #[test]
    fn test_test_status_as_str() {
        assert_eq!(TestStatus::Passed.as_str(), "passed");
        assert_eq!(TestStatus::Failed.as_str(), "failed");
    }

    #[test]
    fn test_test_result_creation() {
        let result = TestResult::new(String::from("test_name"), TestType::Unit);
        assert_eq!(result.test_name, "test_name");
        assert_eq!(result.status, TestStatus::Pending);
    }

    #[test]
    fn test_test_result_passed() {
        let mut result = TestResult::new(String::from("test_name"), TestType::Unit);
        result.passed(100, String::from("test output"));
        assert_eq!(result.status, TestStatus::Passed);
        assert_eq!(result.duration_ms, 100);
    }

    #[test]
    fn test_test_result_failed() {
        let mut result = TestResult::new(String::from("test_name"), TestType::Unit);
        result.failed(100, String::from("test output"), String::from("error"));
        assert_eq!(result.status, TestStatus::Failed);
        assert!(result.error_message.is_some());
    }

    #[test]
    fn test_test_suite_creation() {
        let suite = TestSuite::new(String::from("suite_name"), TestType::Unit);
        assert_eq!(suite.name, "suite_name");
        assert_eq!(suite.test_type, TestType::Unit);
    }

    #[test]
    fn test_test_suite_add_test() {
        let mut suite = TestSuite::new(String::from("suite_name"), TestType::Unit);
        suite.add_test(TestResult::new(String::from("test_name"), TestType::Unit));
        assert_eq!(suite.tests.len(), 1);
    }

    #[test]
    fn test_test_suite_get_passed_count() {
        let mut suite = TestSuite::new(String::from("suite_name"), TestType::Unit);
        let mut test1 = TestResult::new(String::from("test1"), TestType::Unit);
        test1.passed(100, String::new());
        let mut test2 = TestResult::new(String::from("test2"), TestType::Unit);
        test2.failed(100, String::new(), String::new());
        suite.add_test(test1);
        suite.add_test(test2);
        assert_eq!(suite.get_passed_count(), 1);
    }

    #[test]
    fn test_development_testing_framework_creation() {
        let framework = DevelopmentTestingFramework::new();
        assert_eq!(framework.test_suites.len(), 0);
    }

    #[test]
    fn test_development_testing_framework_add_test_suite() {
        let mut framework = DevelopmentTestingFramework::new();
        framework.add_test_suite(TestSuite::new(String::from("suite"), TestType::Unit));
        assert_eq!(framework.test_suites.len(), 1);
    }

    #[test]
    fn test_development_testing_framework_create_unit_test_suite() {
        let mut framework = DevelopmentTestingFramework::new();
        framework.create_unit_test_suite(String::from("unit_suite"));
        assert_eq!(framework.test_suites.len(), 1);
    }

    #[test]
    fn test_development_testing_framework_get_statistics() {
        let mut framework = DevelopmentTestingFramework::new();
        let mut suite = framework.create_unit_test_suite(String::from("unit_suite"));
        let mut test = TestResult::new(String::from("test"), TestType::Unit);
        test.passed(100, String::new());
        suite.add_test(test);
        framework.test_suites[0] = suite;

        let stats = framework.get_statistics();
        assert!(stats.contains("Total test suites: 1"));
        assert!(stats.contains("Total tests: 1"));
    }

    #[test]
    fn test_development_testing_framework_get_test_command() {
        let framework = DevelopmentTestingFramework::new();
        let cmd = framework.get_test_command(TestType::Unit);
        assert!(cmd.contains("cargo test"));
    }

    #[test]
    fn test_development_testing_framework_get_specific_test_command() {
        let framework = DevelopmentTestingFramework::new();
        let cmd = framework.get_specific_test_command(String::from("test_name"));
        assert!(cmd.contains("cargo test test_name"));
    }

    #[test]
    fn test_development_testing_framework_reset() {
        let mut framework = DevelopmentTestingFramework::new();
        framework.create_unit_test_suite(String::from("suite"));
        framework.reset();
        assert_eq!(framework.test_suites.len(), 0);
    }
}
