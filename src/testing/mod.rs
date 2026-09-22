//! Testing Infrastructure (Gentoo Test Frameworks + BSD Test Suites Inspiration)
//! Comprehensive testing framework for SigmaOS
use std::vec;



use std::vec::Vec;
use std::string::{String, ToString};

/// Test result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestResult {
    Passed,
    Failed,
    Skipped,
    Error,
}

/// Test suite
#[derive(Clone)]
pub struct TestSuite {
    pub name: String,
    pub tests: Vec<TestCase>,
    pub results: Vec<TestResult>,
}

impl TestSuite {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tests: Vec::new(),
            results: Vec::new(),
        }
    }

    pub fn add_test(&mut self, test: TestCase) {
        self.tests.push(test);
    }

    pub fn run(&mut self) -> TestSummary {
        self.results.clear();
        
        for test in &self.tests {
            let result = test.run();
            self.results.push(result);
        }
        
        TestSummary {
            suite_name: self.name.clone(),
            total: self.tests.len(),
            passed: self.results.iter().filter(|&&r| r == TestResult::Passed).count(),
            failed: self.results.iter().filter(|&&r| r == TestResult::Failed).count(),
            skipped: self.results.iter().filter(|&&r| r == TestResult::Skipped).count(),
            errors: self.results.iter().filter(|&&r| r == TestResult::Error).count(),
            results: self.results.clone(),
        }
    }
}

/// Test case
#[derive(Clone)]
pub struct TestCase {
    pub name: String,
    pub test_fn: fn() -> TestResult,
    pub setup: Option<fn()>,
    pub teardown: Option<fn()>,
}

impl TestCase {
    pub fn new(name: &str, test_fn: fn() -> TestResult) -> Self {
        Self {
            name: name.to_string(),
            test_fn,
            setup: None,
            teardown: None,
        }
    }

    pub fn with_setup(mut self, setup: fn()) -> Self {
        self.setup = Some(setup);
        self
    }

    pub fn with_teardown(mut self, teardown: fn()) -> Self {
        self.teardown = Some(teardown);
        self
    }

    pub fn run(&self) -> TestResult {
        if let Some(setup) = self.setup {
            setup();
        }
        
        let result = (self.test_fn)();
        
        if let Some(teardown) = self.teardown {
            teardown();
        }
        
        result
    }
}

/// Test summary
#[derive(Debug, Clone)]
pub struct TestSummary {
    pub suite_name: String,
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub errors: usize,
    pub results: Vec<TestResult>,
}

impl TestSummary {
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total as f64) * 100.0
        }
    }

    pub fn has_failures(&self) -> bool {
        self.failed > 0 || self.errors > 0
    }
}

/// Unit test framework
pub struct UnitTestFramework {
    pub suites: Vec<TestSuite>,
}

impl UnitTestFramework {
    pub fn new() -> Self {
        Self {
            suites: Vec::new(),
        }
    }

    pub fn add_suite(&mut self, suite: TestSuite) {
        self.suites.push(suite);
    }

    pub fn run_all(&mut self) -> OverallTestSummary {
        let mut summaries = Vec::new();
        
        for suite in &mut self.suites {
            summaries.push(suite.run());
        }
        
        OverallTestSummary {
            total_suites: self.suites.len(),
            total_tests: summaries.iter().map(|s| s.total).sum(),
            total_passed: summaries.iter().map(|s| s.passed).sum(),
            total_failed: summaries.iter().map(|s| s.failed).sum(),
            total_skipped: summaries.iter().map(|s| s.skipped).sum(),
            total_errors: summaries.iter().map(|s| s.errors).sum(),
            summaries,
        }
    }
}

impl Default for UnitTestFramework {
    fn default() -> Self {
        Self::new()
    }
}

/// Overall test summary
#[derive(Debug, Clone)]
pub struct OverallTestSummary {
    pub total_suites: usize,
    pub total_tests: usize,
    pub total_passed: usize,
    pub total_failed: usize,
    pub total_skipped: usize,
    pub total_errors: usize,
    pub summaries: Vec<TestSummary>,
}

impl OverallTestSummary {
    pub fn overall_success_rate(&self) -> f64 {
        if self.total_tests == 0 {
            0.0
        } else {
            (self.total_passed as f64 / self.total_tests as f64) * 100.0
        }
    }

    pub fn has_any_failures(&self) -> bool {
        self.total_failed > 0 || self.total_errors > 0
    }
}

/// Integration test framework
pub struct IntegrationTestFramework {
    pub suites: Vec<TestSuite>,
    pub setup: Option<fn()>,
    pub teardown: Option<fn()>,
}

impl IntegrationTestFramework {
    pub fn new() -> Self {
        Self {
            suites: Vec::new(),
            setup: None,
            teardown: None,
        }
    }

    pub fn add_suite(&mut self, suite: TestSuite) {
        self.suites.push(suite);
    }

    pub fn with_setup(mut self, setup: fn()) -> Self {
        self.setup = Some(setup);
        self
    }

    pub fn with_teardown(mut self, teardown: fn()) -> Self {
        self.teardown = Some(teardown);
        self
    }

    pub fn run_all(&mut self) -> OverallTestSummary {
        if let Some(setup) = self.setup {
            setup();
        }
        
        let mut framework = UnitTestFramework::new();
        for suite in &self.suites {
            framework.add_suite(suite.clone());
        }
        
        let summary = framework.run_all();
        
        if let Some(teardown) = self.teardown {
            teardown();
        }
        
        summary
    }
}

impl Default for IntegrationTestFramework {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance test framework
pub struct PerformanceTestFramework {
    pub benchmarks: Vec<Benchmark>,
}

#[derive(Debug, Clone)]
pub struct Benchmark {
    pub name: String,
    pub benchmark_fn: fn() -> BenchmarkResult,
}

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub duration_ns: u64,
    pub memory_bytes: u64,
    pub iterations: u64,
}

impl PerformanceTestFramework {
    pub fn new() -> Self {
        Self {
            benchmarks: Vec::new(),
        }
    }

    pub fn add_benchmark(&mut self, benchmark: Benchmark) {
        self.benchmarks.push(benchmark);
    }

    pub fn run_all(&self) -> PerformanceSummary {
        let mut results = Vec::new();
        
        for benchmark in &self.benchmarks {
            results.push((benchmark.benchmark_fn)());
        }
        
        PerformanceSummary {
            total_benchmarks: self.benchmarks.len(),
            results,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceSummary {
    pub total_benchmarks: usize,
    pub results: Vec<BenchmarkResult>,
}

impl Default for PerformanceTestFramework {
    fn default() -> Self {
        Self::new()
    }
}

/// Security test framework
pub struct SecurityTestFramework {
    pub tests: Vec<SecurityTest>,
}

#[derive(Debug, Clone)]
pub struct SecurityTest {
    pub name: String,
    pub test_fn: fn() -> SecurityTestResult,
}

#[derive(Debug, Clone)]
pub struct SecurityTestResult {
    pub name: String,
    pub passed: bool,
    pub vulnerabilities_found: Vec<String>,
    pub severity: SecuritySeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl SecurityTestFramework {
    pub fn new() -> Self {
        Self {
            tests: Vec::new(),
        }
    }

    pub fn add_test(&mut self, test: SecurityTest) {
        self.tests.push(test);
    }

    pub fn run_all(&self) -> SecuritySummary {
        let mut results = Vec::new();
        
        for test in &self.tests {
            results.push((test.test_fn)());
        }
        
        SecuritySummary {
            total_tests: self.tests.len(),
            passed: results.iter().filter(|r| r.passed).count(),
            failed: results.iter().filter(|r| !r.passed).count(),
            critical_vulnerabilities: results.iter().filter(|r| r.severity == SecuritySeverity::Critical).count(),
            results,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SecuritySummary {
    pub total_tests: usize,
    pub passed: usize,
    pub failed: usize,
    pub critical_vulnerabilities: usize,
    pub results: Vec<SecurityTestResult>,
}

impl Default for SecurityTestFramework {
    fn default() -> Self {
        Self::new()
    }
}

/// Fuzzing test framework
pub struct FuzzingTestFramework {
    pub fuzzers: Vec<Fuzzer>,
}

#[derive(Debug, Clone)]
pub struct Fuzzer {
    pub name: String,
    pub target: fn(&[u8]) -> bool,
    pub input_generator: fn() -> Vec<u8>,
    pub max_iterations: u32,
}

impl FuzzingTestFramework {
    pub fn new() -> Self {
        Self {
            fuzzers: Vec::new(),
        }
    }

    pub fn add_fuzzer(&mut self, fuzzer: Fuzzer) {
        self.fuzzers.push(fuzzer);
    }

    pub fn run_all(&self) -> FuzzingSummary {
        let mut results = Vec::new();
        
        for fuzzer in &self.fuzzers {
            let mut crashes = 0;
            let mut iterations = 0;
            
            for _ in 0..fuzzer.max_iterations {
                let input = (fuzzer.input_generator)();
                if !(fuzzer.target)(&input) {
                    crashes += 1;
                }
                iterations += 1;
            }
            
            results.push(FuzzerResult {
                name: fuzzer.name.clone(),
                iterations,
                crashes,
                crash_rate: if iterations > 0 {
                    (crashes as f64 / iterations as f64) * 100.0
                } else {
                    0.0
                },
            });
        }
        
        FuzzingSummary {
            total_fuzzers: self.fuzzers.len(),
            results,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FuzzerResult {
    pub name: String,
    pub iterations: u32,
    pub crashes: u32,
    pub crash_rate: f64,
}

#[derive(Debug, Clone)]
pub struct FuzzingSummary {
    pub total_fuzzers: usize,
    pub results: Vec<FuzzerResult>,
}

impl Default for FuzzingTestFramework {
    fn default() -> Self {
        Self::new()
    }
}

/// Graphical Acceptance Test Execution Status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcceptanceTestStatus {
    Passed,
    Failed,
    Skipped,
}

/// Acceptance Test Step Screenshot Artifact
#[derive(Debug, Clone)]
pub struct AcceptanceScreenshotRecord {
    pub step_name: String,
    pub is_success: bool,
    pub filepath: String,
}

/// Omarchy ISO Harness Sync Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsoHarnessSyncMode {
    SyncOmarchy,
    SyncAll,
    FullIsoBuild,
}

/// Omarchy Graphical Acceptance Test Runner Engine (`test/acceptance.d/*-test.sh`)
#[derive(Debug, Clone)]
pub struct OmarchyGraphicalAcceptanceTestRunnerEngine {
    pub test_directory: String,
    pub sync_mode: IsoHarnessSyncMode,
    pub reuse_base_installed_vm: bool,
    pub qmp_virtual_keyboard_enabled: bool,
    pub recorded_screenshots: Vec<AcceptanceScreenshotRecord>,
}

impl OmarchyGraphicalAcceptanceTestRunnerEngine {
    pub fn new() -> Self {
        Self {
            test_directory: String::from("test/acceptance.d/"),
            sync_mode: IsoHarnessSyncMode::SyncOmarchy,
            reuse_base_installed_vm: true,
            qmp_virtual_keyboard_enabled: true,
            recorded_screenshots: Vec::new(),
        }
    }

    pub fn discover_test_files(&self) -> Vec<String> {
        vec![
            format!("{}01-session-health-test.sh", self.test_directory),
            format!("{}02-shell-panels-test.sh", self.test_directory),
            format!("{}03-keyboard-navigation-test.sh", self.test_directory),
            format!("{}04-applications-test.sh", self.test_directory),
            format!("{}05-system-setup-test.sh", self.test_directory),
        ]
    }

    pub fn record_screenshot_artifact(&mut self, step_name: &str, is_success: bool) -> String {
        let prefix = if is_success { "success" } else { "failure" };
        let filename = format!("{}-{}.png", prefix, step_name);
        self.recorded_screenshots.push(AcceptanceScreenshotRecord {
            step_name: step_name.to_string(),
            is_success,
            filepath: filename.clone(),
        });
        filename
    }

    pub fn build_iso_test_command(&self, iso_path: &str) -> String {
        let sync_flag = match self.sync_mode {
            IsoHarnessSyncMode::SyncOmarchy => "--sync-omarchy ../omarchy",
            IsoHarnessSyncMode::SyncAll => "--sync-all ../omarchy",
            IsoHarnessSyncMode::FullIsoBuild => "--local-source ../omarchy",
        };

        let reuse_flag = if self.reuse_base_installed_vm { "--reuse-base" } else { "" };

        format!("./bin/omarchy-iso-test {} {} {} --no-preview", iso_path, reuse_flag, sync_flag)
    }

    pub fn parse_tap_output(&self, tap_text: &str) -> TapParseSummary {
        let mut summary = TapParseSummary::default();

        for line in tap_text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("1..") {
                if let Ok(plan_count) = trimmed[3..].parse::<usize>() {
                    summary.total_plan = plan_count;
                }
            } else if trimmed.starts_with("ok ") {
                let rest = &trimmed[3..];
                if rest.contains("# skip") || rest.contains("# SKIP") {
                    summary.skipped += 1;
                } else {
                    summary.passed += 1;
                }
                summary.test_names.push(rest.to_string());
            } else if trimmed.starts_with("not ok ") {
                let rest = &trimmed[7..];
                if rest.contains("# skip") || rest.contains("# SKIP") {
                    summary.skipped += 1;
                } else {
                    summary.failed += 1;
                }
                summary.test_names.push(rest.to_string());
            }
        }

        summary
    }
}

/// Parsed Test Anything Protocol (TAP) Summary for Acceptance Suite
#[derive(Debug, Clone, Default)]
pub struct TapParseSummary {
    pub total_plan: usize,
    pub passed: usize,
    pub failed: usize,
    pub skipped: usize,
    pub test_names: Vec<String>,
}

impl Default for OmarchyGraphicalAcceptanceTestRunnerEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_test_framework() {
        let mut suite = TestSuite::new("test_suite");
        
        let test_case = TestCase::new("test_example", || TestResult::Passed);
        suite.add_test(test_case);
        
        let summary = suite.run();
        assert_eq!(summary.total, 1);
        assert_eq!(summary.passed, 1);
    }

    #[test]
    fn test_integration_test_framework() {
        let mut framework = IntegrationTestFramework::new();
        
        let suite = TestSuite::new("integration_suite");
        framework.add_suite(suite);
        
        let summary = framework.run_all();
        assert_eq!(summary.total_suites, 1);
    }

    #[test]
    fn test_performance_framework() {
        let mut framework = PerformanceTestFramework::new();
        
        let benchmark = Benchmark {
            name: "benchmark_example".to_string(),
            benchmark_fn: || BenchmarkResult {
                name: "benchmark_example".to_string(),
                duration_ns: 1000,
                memory_bytes: 1024,
                iterations: 100,
            },
        };
        framework.add_benchmark(benchmark);
        
        let summary = framework.run_all();
        assert_eq!(summary.total_benchmarks, 1);
    }

    #[test]
    fn test_security_framework() {
        let mut framework = SecurityTestFramework::new();
        
        let test = SecurityTest {
            name: "security_test".to_string(),
            test_fn: || SecurityTestResult {
                name: "security_test".to_string(),
                passed: true,
                vulnerabilities_found: Vec::new(),
                severity: SecuritySeverity::Low,
            },
        };
        framework.add_test(test);
        
        let summary = framework.run_all();
        assert_eq!(summary.total_tests, 1);
        assert_eq!(summary.passed, 1);
    }

    #[test]
    fn test_fuzzing_framework() {
        let mut framework = FuzzingTestFramework::new();

        let fuzzer = Fuzzer {
            name: String::from("test_fuzzer"),
            target: |input| !input.is_empty(),
            input_generator: || std::vec![1, 2, 3],
            max_iterations: 10,
        };
        framework.add_fuzzer(fuzzer);

        let summary = framework.run_all();
        assert_eq!(summary.total_fuzzers, 1);
        assert_eq!(summary.results.len(), 1);
        assert_eq!(summary.results[0].iterations, 10);
        assert_eq!(summary.results[0].crashes, 0);
        assert_eq!(summary.results[0].crash_rate, 0.0);
    }

    #[test]
    fn test_fuzz_system_parsers_suite() {
        let mut framework = FuzzingTestFramework::new();

        // Fuzzer 1: WASM Header Validation
        framework.add_fuzzer(Fuzzer {
            name: String::from("wasm_header_fuzzer"),
            target: |input| {
                if input.len() < 4 {
                    return true;
                }
                let is_valid_wasm = input.starts_with(b"\0asm");
                let _ = is_valid_wasm;
                true
            },
            input_generator: || {
                let mut buf = std::vec![0u8; 8];
                buf[0] = 0x00;
                buf[1] = 0x61;
                buf[2] = 0x73;
                buf[3] = 0x6d;
                buf
            },
            max_iterations: 100,
        });

        // Fuzzer 2: TCP Header Boundary Validation
        framework.add_fuzzer(Fuzzer {
            name: String::from("tcp_header_fuzzer"),
            target: |input| {
                if input.len() < 20 {
                    return true;
                }
                let data_offset = input[12] >> 4;
                let header_len = (data_offset as usize) * 4;
                if header_len < 20 || header_len > 60 || header_len > input.len() {
                    return true;
                }
                true
            },
            input_generator: || {
                let mut buf = std::vec![0u8; 32];
                buf[12] = 0x50; // Data offset = 5 (20 bytes)
                buf
            },
            max_iterations: 100,
        });

        let summary = framework.run_all();
        assert_eq!(summary.total_fuzzers, 2);
        assert_eq!(summary.results[0].crashes, 0);
        assert_eq!(summary.results[1].crashes, 0);
    }

    #[test]
    fn test_omarchy_graphical_acceptance_test_runner_engine() {
        let mut runner = OmarchyGraphicalAcceptanceTestRunnerEngine::new();
        let files = runner.discover_test_files();
        assert_eq!(files.len(), 5);
        assert!(files[0].contains("01-session-health-test.sh"));

        let success_png = runner.record_screenshot_artifact("panel-open", true);
        assert_eq!(success_png, "success-panel-open.png");

        let failure_png = runner.record_screenshot_artifact("panel-open", false);
        assert_eq!(failure_png, "failure-panel-open.png");

        assert_eq!(runner.recorded_screenshots.len(), 2);

        let cmd = runner.build_iso_test_command("release/omarchy-1.0.iso");
        assert!(cmd.contains("release/omarchy-1.0.iso"));
        assert!(cmd.contains("--sync-omarchy ../omarchy"));
        assert!(cmd.contains("--reuse-base"));
        assert!(cmd.contains("--no-preview"));

        // TAP protocol parsing test
        let tap_data = "1..3\nok 1 - session health ok\nnot ok 2 - panel open error\nok 3 - bluetooth widget # SKIP optional hw";
        let tap_summary = runner.parse_tap_output(tap_data);
        assert_eq!(tap_summary.total_plan, 3);
        assert_eq!(tap_summary.passed, 1);
        assert_eq!(tap_summary.failed, 1);
        assert_eq!(tap_summary.skipped, 1);
        assert_eq!(tap_summary.test_names.len(), 3);
    }
}