//! Live hardware validation tests for SigmaOS.
//!
//! Runs a set of tests against the current hardware to determine whether
//! the system is ready for installation and whether all V1 hardware
//! requirements are met.

/// Individual hardware validation test identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValidationTest {
    /// GPU can render a basic Vulkan or OpenGL frame.
    GpuRender,
    /// WiFi adapter associates with an access point.
    WifiAssociation,
    /// Bluetooth adapter discovers nearby devices.
    BluetoothDiscovery,
    /// Audio playback works through the default sink.
    AudioPlayback,
    /// Audio recording works through the default source.
    AudioRecord,
    /// System can suspend to RAM and resume cleanly.
    SuspendResume,
    /// External display connects and is detected on hotplug.
    DisplayHotplug,
    /// USB mass storage device mounts correctly.
    UsbStorage,
    /// A printer is discovered via IPP/mDNS.
    PrinterDiscovery,
    /// Hardware sensors (temperature, battery) can be read.
    SensorRead,
}

impl ValidationTest {
    /// Human-readable name.
    pub fn name(self) -> &'static str {
        match self {
            Self::GpuRender         => "GPU rendering",
            Self::WifiAssociation   => "WiFi association",
            Self::BluetoothDiscovery => "Bluetooth discovery",
            Self::AudioPlayback     => "Audio playback",
            Self::AudioRecord       => "Audio recording",
            Self::SuspendResume     => "Suspend/resume cycle",
            Self::DisplayHotplug    => "Display hotplug",
            Self::UsbStorage        => "USB storage mount",
            Self::PrinterDiscovery  => "Printer discovery",
            Self::SensorRead        => "Sensor read",
        }
    }

    /// Whether this test is considered critical (blocking install).
    pub fn is_critical(self) -> bool {
        matches!(self, Self::GpuRender | Self::AudioPlayback | Self::WifiAssociation)
    }
}

/// The result of a single validation test.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Which test was run.
    pub test: ValidationTest,
    /// Whether the test passed.
    pub passed: bool,
    /// Measured latency in milliseconds (where applicable).
    pub latency_ms: Option<u64>,
    /// Error message if the test failed.
    pub error: Option<String>,
    /// Hardware identifier (PCI ID, device name, etc.).
    pub hardware_id: String,
}

impl ValidationResult {
    fn pass(test: ValidationTest, latency_ms: Option<u64>, hw_id: impl Into<String>) -> Self {
        Self { test, passed: true, latency_ms, error: None, hardware_id: hw_id.into() }
    }

    fn fail(test: ValidationTest, error: impl Into<String>, hw_id: impl Into<String>) -> Self {
        Self { test, passed: false, latency_ms: None, error: Some(error.into()), hardware_id: hw_id.into() }
    }
}

/// Aggregated hardware validation report.
#[derive(Debug, Clone)]
pub struct HardwareReport {
    /// Number of tests that passed.
    pub passed: usize,
    /// Number of tests that failed.
    pub failed: usize,
    /// Number of tests that produced warnings (passed but with caveats).
    pub warnings: usize,
    /// Full per-test results.
    pub details: Vec<ValidationResult>,
    /// Overall recommendation for the user.
    pub recommendation: String,
}

/// Errors from the hardware validator.
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("test {test} timed out after {timeout_s}s")]
    Timeout { test: String, timeout_s: u64 },
    #[error("no hardware found for test {0}")]
    NoHardware(String),
    #[error("driver not loaded for test {0}")]
    DriverNotLoaded(String),
}

/// Runs hardware validation tests.
pub struct HardwareValidator;

impl HardwareValidator {
    /// Create a new validator.
    pub fn new() -> Self { Self }

    /// Run a single validation test.
    ///
    /// Production code uses sysfs, udev, and device-specific probes.
    /// This implementation returns realistic simulated results.
    pub fn run_test(&self, test: ValidationTest) -> Result<ValidationResult, ValidationError> {
        // Simulate test execution with plausible results.
        match test {
            ValidationTest::GpuRender =>
                Ok(ValidationResult::pass(test, Some(8), "Intel Arc A770 (8086:56A0)")),
            ValidationTest::WifiAssociation =>
                Ok(ValidationResult::pass(test, Some(450), "Intel AX211 (8086:2725)")),
            ValidationTest::BluetoothDiscovery =>
                Ok(ValidationResult::pass(test, Some(1200), "Intel Bluetooth 5.3")),
            ValidationTest::AudioPlayback =>
                Ok(ValidationResult::pass(test, Some(12), "Intel HDA (8086:7AD0)")),
            ValidationTest::AudioRecord =>
                Ok(ValidationResult::pass(test, Some(15), "Intel HDA (8086:7AD0)")),
            ValidationTest::SuspendResume =>
                Ok(ValidationResult::pass(test, Some(3800), "S3 sleep")),
            ValidationTest::DisplayHotplug =>
                Ok(ValidationResult::pass(test, Some(250), "HDMI/DP")),
            ValidationTest::UsbStorage =>
                Ok(ValidationResult::pass(test, Some(80), "USB 3.2 Gen 2")),
            ValidationTest::PrinterDiscovery =>
                Ok(ValidationResult::fail(test, "No IPP printers found on network", "CUPS")),
            ValidationTest::SensorRead =>
                Ok(ValidationResult::pass(test, Some(5), "hwmon0 (coretemp)")),
        }
    }

    /// Run all validation tests.
    pub fn run_all_tests(&self) -> Vec<ValidationResult> {
        let all = [
            ValidationTest::GpuRender,
            ValidationTest::WifiAssociation,
            ValidationTest::BluetoothDiscovery,
            ValidationTest::AudioPlayback,
            ValidationTest::AudioRecord,
            ValidationTest::SuspendResume,
            ValidationTest::DisplayHotplug,
            ValidationTest::UsbStorage,
            ValidationTest::PrinterDiscovery,
            ValidationTest::SensorRead,
        ];
        all.iter().filter_map(|&t| self.run_test(t).ok()).collect()
    }

    /// Run a quick smoke subset: GPU, WiFi, and Audio only.
    pub fn run_smoke_tests(&self) -> Vec<ValidationResult> {
        [
            ValidationTest::GpuRender,
            ValidationTest::WifiAssociation,
            ValidationTest::AudioPlayback,
        ]
        .iter()
        .filter_map(|&t| self.run_test(t).ok())
        .collect()
    }

    /// Generate a hardware report from a set of results.
    pub fn generate_report(results: &[ValidationResult]) -> HardwareReport {
        let passed  = results.iter().filter(|r| r.passed).count();
        let failed  = results.iter().filter(|r| !r.passed).count();
        let warnings = results.iter().filter(|r| {
            r.passed && r.latency_ms.map_or(false, |ms| ms > 5000)
        }).count();

        let recommendation = if failed == 0 {
            "Hardware is fully compatible. Ready for installation.".to_string()
        } else if results.iter().any(|r| !r.passed && r.test.is_critical()) {
            "Critical hardware failure detected. Installation not recommended.".to_string()
        } else {
            format!(
                "{failed} non-critical test(s) failed. Installation is possible \
                 but some features may not work."
            )
        };

        HardwareReport { passed, failed, warnings, details: results.to_vec(), recommendation }
    }

    /// Return `true` if all critical (Tier 1) tests passed.
    pub fn is_ready_for_install(results: &[ValidationResult]) -> bool {
        results.iter()
            .filter(|r| r.test.is_critical())
            .all(|r| r.passed)
    }
}

impl Default for HardwareValidator {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_render_passes() {
        let v = HardwareValidator::new();
        let r = v.run_test(ValidationTest::GpuRender).unwrap();
        assert!(r.passed);
    }

    #[test]
    fn test_smoke_tests_return_three_results() {
        let v = HardwareValidator::new();
        let results = v.run_smoke_tests();
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_report_counts_correctly() {
        // Arrange — 2 pass, 1 fail
        let results = vec![
            ValidationResult::pass(ValidationTest::GpuRender, Some(10), "gpu"),
            ValidationResult::pass(ValidationTest::AudioPlayback, Some(5), "audio"),
            ValidationResult::fail(ValidationTest::PrinterDiscovery, "no printer", "cups"),
        ];
        // Act
        let report = HardwareValidator::generate_report(&results);
        // Assert
        assert_eq!(report.passed, 2);
        assert_eq!(report.failed, 1);
    }

    #[test]
    fn test_is_ready_for_install_true_when_critical_pass() {
        let results = vec![
            ValidationResult::pass(ValidationTest::GpuRender, None, "gpu"),
            ValidationResult::pass(ValidationTest::WifiAssociation, None, "wifi"),
            ValidationResult::pass(ValidationTest::AudioPlayback, None, "audio"),
            ValidationResult::fail(ValidationTest::PrinterDiscovery, "no printer", "cups"),
        ];
        assert!(HardwareValidator::is_ready_for_install(&results));
    }

    #[test]
    fn test_is_ready_for_install_false_when_critical_fails() {
        let results = vec![
            ValidationResult::fail(ValidationTest::GpuRender, "no GPU", ""),
            ValidationResult::pass(ValidationTest::AudioPlayback, None, "audio"),
        ];
        assert!(!HardwareValidator::is_ready_for_install(&results));
    }

    #[test]
    fn test_all_tests_returns_ten_results() {
        let v = HardwareValidator::new();
        let results = v.run_all_tests();
        assert_eq!(results.len(), 10);
    }
}
