//! Regression test tracking for SigmaOS release pipeline.
//!
//! This module tracks named regression test cases, executes them, compares
//! two runs (baseline vs. current) to surface newly broken tests and newly
//! fixed regressions, and produces a human-readable diff report.

use std::collections::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

// ---------------------------------------------------------------------------
// Core structs
// ---------------------------------------------------------------------------

/// Describes a single regression test case.
#[derive(Debug, Clone)]
pub struct RegressionTest {
    /// Machine-readable identifier (e.g. `"install-001"`).
    pub id: String,
    /// Short human-readable title.
    pub title: String,
    /// Detailed description of what is being verified.
    pub description: String,
    /// The SigmaOS component being exercised.
    pub component: String,
    /// The release version when this test was first added.
    pub added_version: String,
    /// The last release version in which this test passed (`None` = never ran).
    pub last_passed: Option<String>,
    /// If `true`, a failure of this test prevents a release.
    pub is_blocking: bool,
}

/// A named collection of regression tests tied to a specific version.
#[derive(Debug, Clone)]
pub struct RegressionSuite {
    /// Human-readable suite name (e.g. `"SigmaOS v1 Regression Suite"`).
    pub name: String,
    /// The tests belonging to this suite.
    pub tests: Vec<RegressionTest>,
    /// The SigmaOS version being validated.
    pub version: String,
}

/// The output of running a single [`RegressionTest`].
#[derive(Debug, Clone)]
pub struct RegressionResult {
    /// The ID of the test that produced this result.
    pub test_id: String,
    /// Whether the test passed.
    pub passed: bool,
    /// Execution time in milliseconds (simulated).
    pub duration_ms: u64,
    /// stdout / stderr captured from the test (or simulated output).
    pub output: String,
}

/// Difference between a baseline run and a current run.
#[derive(Debug, Clone, Default)]
pub struct RegressionDiff {
    /// Tests that passed in baseline but fail now (regressions).
    pub new_failures: Vec<String>,
    /// Tests that failed in baseline but pass now (fixed regressions).
    pub fixed_regressions: Vec<String>,
    /// Tests that failed in both runs (pre-existing failures).
    pub unchanged_failures: Vec<String>,
    /// Tests that passed in both runs (stable passing).
    pub unchanged_passes: Vec<String>,
}

// ---------------------------------------------------------------------------
// Runner
// ---------------------------------------------------------------------------

/// Orchestrates regression test execution, comparison, and reporting.
pub struct RegressionRunner {
    /// Optional override map: `test_id → should_fail`.
    overrides: HashMap<String, bool>,
}

impl RegressionRunner {
    /// Create a runner that simulates all tests passing.
    pub fn new() -> Self {
        Self {
            overrides: HashMap::new(),
        }
    }

    /// Force a specific test to fail.  Used in unit tests only.
    #[cfg(test)]
    pub fn force_fail(&mut self, id: &str) {
        self.overrides.insert(id.to_string(), true);
    }

    // -----------------------------------------------------------------------
    // Suite factory
    // -----------------------------------------------------------------------

    /// Return the canonical v1 regression suite for SigmaOS.
    ///
    /// Contains exactly 20 regression tests covering the most critical
    /// install, boot, desktop, package, recovery, hardware, security, and
    /// app scenarios reported during pre-release validation.
    pub fn sigma_v1_suite() -> RegressionSuite {
        RegressionSuite {
            name: "SigmaOS v1 Regression Suite".to_string(),
            version: "1.0.0".to_string(),
            tests: vec![
                // Install -------------------------------------------------------
                RegressionTest {
                    id: "install-001".to_string(),
                    title: "Clean install completes in <20min".to_string(),
                    description: "A full fresh installation on a blank 50 GiB virtual disk must \
                        complete — from first menu selection to first-boot wizard — within \
                        20 minutes on reference hardware."
                        .to_string(),
                    component: "installer".to_string(),
                    added_version: "0.9.0".to_string(),
                    last_passed: Some("0.9.5".to_string()),
                    is_blocking: true,
                },
                RegressionTest {
                    id: "install-002".to_string(),
                    title: "Dual-boot alongside Windows".to_string(),
                    description: "Install SigmaOS on a disk that already has a Windows 11 EFI \
                        partition.  Both GRUB entries must appear and Windows must remain \
                        bootable."
                        .to_string(),
                    component: "installer".to_string(),
                    added_version: "0.9.0".to_string(),
                    last_passed: Some("0.9.5".to_string()),
                    is_blocking: true,
                },
                RegressionTest {
                    id: "install-003".to_string(),
                    title: "Encrypted root partition".to_string(),
                    description: "Install with full-disk LUKS encryption enabled.  System must \
                        boot to a passphrase prompt and unlock the root volume correctly."
                        .to_string(),
                    component: "installer".to_string(),
                    added_version: "0.9.0".to_string(),
                    last_passed: Some("0.9.3".to_string()),
                    is_blocking: true,
                },
                // Boot ----------------------------------------------------------
                RegressionTest {
                    id: "boot-001".to_string(),
                    title: "Boot time <30s".to_string(),
                    description: "From GRUB kernel selection to graphical login screen must take \
                        less than 30 seconds on the reference VM."
                        .to_string(),
                    component: "boot".to_string(),
                    added_version: "0.8.0".to_string(),
                    last_passed: Some("0.9.5".to_string()),
                    is_blocking: false,
                },
                RegressionTest {
                    id: "boot-002".to_string(),
                    title: "Resume from suspend <5s".to_string(),
                    description: "After entering S3 suspend and pressing the power button, the \
                        system must reach a usable desktop state in under 5 seconds."
                        .to_string(),
                    component: "power".to_string(),
                    added_version: "0.9.0".to_string(),
                    last_passed: Some("0.9.4".to_string()),
                    is_blocking: false,
                },
                // Desktop -------------------------------------------------------
                RegressionTest {
                    id: "desktop-001".to_string(),
                    title: "First login reaches desktop".to_string(),
                    description: "A newly created user account must successfully authenticate at \
                        the SDDM login screen and arrive at a fully rendered desktop."
                        .to_string(),
                    component: "desktop".to_string(),
                    added_version: "0.8.0".to_string(),
                    last_passed: Some("0.9.5".to_string()),
                    is_blocking: true,
                },
                RegressionTest {
                    id: "desktop-002".to_string(),
                    title: "Panel clock correct after timezone set".to_string(),
                    description: "After changing the system timezone via Settings, the clock \
                        widget in the panel must immediately display the correct local time."
                        .to_string(),
                    component: "desktop".to_string(),
                    added_version: "0.9.2".to_string(),
                    last_passed: Some("0.9.4".to_string()),
                    is_blocking: false,
                },
                // Package -------------------------------------------------------
                RegressionTest {
                    id: "pkg-001".to_string(),
                    title: "apt update && apt upgrade completes cleanly".to_string(),
                    description: "Running `apt update && apt upgrade -y` on a fresh install must \
                        complete with exit code 0 and no held packages."
                        .to_string(),
                    component: "package".to_string(),
                    added_version: "0.8.0".to_string(),
                    last_passed: Some("0.9.5".to_string()),
                    is_blocking: true,
                },
                RegressionTest {
                    id: "pkg-002".to_string(),
                    title: "Package signature invalid rejects install".to_string(),
                    description: "Attempting to install a `.deb` with a tampered GPG signature \
                        must be rejected with a non-zero exit code and an error message."
                        .to_string(),
                    component: "package".to_string(),
                    added_version: "0.9.0".to_string(),
                    last_passed: Some("0.9.5".to_string()),
                    is_blocking: true,
                },
                RegressionTest {
                    id: "pkg-003".to_string(),
                    title: "Rollback after failed update".to_string(),
                    description: "If a package update step fails mid-way, the package manager \
                        must restore the system to a consistent pre-update state via APT's \
                        dpkg rollback mechanism."
                        .to_string(),
                    component: "package".to_string(),
                    added_version: "0.9.1".to_string(),
                    last_passed: Some("0.9.3".to_string()),
                    is_blocking: false,
                },
                // Recovery ------------------------------------------------------
                RegressionTest {
                    id: "recovery-001".to_string(),
                    title: "Snapshot restore from rescue mode".to_string(),
                    description: "Boot into the rescue environment, select a Btrfs snapshot, and \
                        restore.  The system must reboot into the snapshot cleanly."
                        .to_string(),
                    component: "recovery".to_string(),
                    added_version: "0.9.0".to_string(),
                    last_passed: Some("0.9.4".to_string()),
                    is_blocking: true,
                },
                RegressionTest {
                    id: "recovery-002".to_string(),
                    title: "Grub repair from emergency shell".to_string(),
                    description: "After deliberately corrupting the GRUB config, boot into the \
                        emergency shell, run `grub-install && update-grub`, and verify the \
                        system boots normally."
                        .to_string(),
                    component: "bootloader".to_string(),
                    added_version: "0.9.0".to_string(),
                    last_passed: Some("0.9.5".to_string()),
                    is_blocking: false,
                },
                // Hardware ------------------------------------------------------
                RegressionTest {
                    id: "hw-001".to_string(),
                    title: "WiFi connects after resume".to_string(),
                    description: "After S3 resume, the Wi-Fi adapter must re-associate with the \
                        last-connected SSID within 10 seconds without manual intervention."
                        .to_string(),
                    component: "network".to_string(),
                    added_version: "0.9.0".to_string(),
                    last_passed: Some("0.9.4".to_string()),
                    is_blocking: false,
                },
                RegressionTest {
                    id: "hw-002".to_string(),
                    title: "Audio output after suspend".to_string(),
                    description: "After S3 resume, the PipeWire audio server must have restored \
                        the default sink and playback must work without requiring a restart."
                        .to_string(),
                    component: "audio".to_string(),
                    added_version: "0.9.1".to_string(),
                    last_passed: Some("0.9.3".to_string()),
                    is_blocking: false,
                },
                RegressionTest {
                    id: "hw-003".to_string(),
                    title: "HDMI hotplug detected".to_string(),
                    description: "Plugging an HDMI display into a running system must cause the \
                        compositor to detect it within 3 seconds and offer a display \
                        configuration dialog."
                        .to_string(),
                    component: "graphics".to_string(),
                    added_version: "0.9.2".to_string(),
                    last_passed: Some("0.9.4".to_string()),
                    is_blocking: false,
                },
                // Security ------------------------------------------------------
                RegressionTest {
                    id: "security-001".to_string(),
                    title: "AppArmor denies unauthorized /proc read".to_string(),
                    description: "A confined test application with an AppArmor profile must \
                        receive EACCES when it attempts to read `/proc/1/mem`."
                        .to_string(),
                    component: "security".to_string(),
                    added_version: "0.9.0".to_string(),
                    last_passed: Some("0.9.5".to_string()),
                    is_blocking: true,
                },
                RegressionTest {
                    id: "security-002".to_string(),
                    title: "Firewall blocks inbound by default".to_string(),
                    description: "A port scan from an external host on the LAN must show all \
                        ports filtered; no service must be reachable without an explicit \
                        `nft` rule."
                        .to_string(),
                    component: "network".to_string(),
                    added_version: "0.9.0".to_string(),
                    last_passed: Some("0.9.5".to_string()),
                    is_blocking: true,
                },
                // Apps ----------------------------------------------------------
                RegressionTest {
                    id: "apps-001".to_string(),
                    title: "LibreOffice opens .docx correctly".to_string(),
                    description: "LibreOffice Writer must open a sample complex `.docx` file \
                        (with tracked changes and embedded images) without crashing and with \
                        correct layout."
                        .to_string(),
                    component: "productivity".to_string(),
                    added_version: "0.9.1".to_string(),
                    last_passed: Some("0.9.5".to_string()),
                    is_blocking: false,
                },
                RegressionTest {
                    id: "apps-002".to_string(),
                    title: "Browser plays HTML5 video".to_string(),
                    description: "The default browser must play an embedded H.264 HTML5 video \
                        without codec errors (hardware or software decode)."
                        .to_string(),
                    component: "browser".to_string(),
                    added_version: "0.9.1".to_string(),
                    last_passed: Some("0.9.5".to_string()),
                    is_blocking: false,
                },
                RegressionTest {
                    id: "apps-003".to_string(),
                    title: "Screenshot saves to ~/Pictures".to_string(),
                    description: "The built-in screenshot tool must capture the full desktop and \
                        save a PNG file to `~/Pictures` when triggered by the keyboard \
                        shortcut."
                        .to_string(),
                    component: "desktop".to_string(),
                    added_version: "0.9.0".to_string(),
                    last_passed: Some("0.9.5".to_string()),
                    is_blocking: false,
                },
            ],
        }
    }

    // -----------------------------------------------------------------------
    // Execution
    // -----------------------------------------------------------------------

    /// Execute all tests in a [`RegressionSuite`] and return results.
    pub fn run_suite(&self, suite: &RegressionSuite) -> Vec<RegressionResult> {
        suite
            .tests
            .iter()
            .map(|t| {
                let forced_fail = self.overrides.get(&t.id).copied().unwrap_or(false);
                let passed = !forced_fail;
                RegressionResult {
                    test_id: t.id.clone(),
                    passed,
                    duration_ms: if passed { 500 } else { 1 },
                    output: if passed {
                        format!("[PASS] {}", t.title)
                    } else {
                        format!("[FAIL] {} — simulated regression", t.title)
                    },
                }
            })
            .collect()
    }

    // -----------------------------------------------------------------------
    // Comparison
    // -----------------------------------------------------------------------

    /// Compare a baseline run against the current run to classify changes.
    pub fn compare_runs(
        baseline: &[RegressionResult],
        current: &[RegressionResult],
    ) -> RegressionDiff {
        // Build lookup maps: test_id → passed
        let base_map: HashMap<&str, bool> = baseline
            .iter()
            .map(|r| (r.test_id.as_str(), r.passed))
            .collect();
        let curr_map: HashMap<&str, bool> = current
            .iter()
            .map(|r| (r.test_id.as_str(), r.passed))
            .collect();

        let mut diff = RegressionDiff::default();

        // Evaluate every test that appears in *either* run.
        let mut all_ids: Vec<&str> = base_map.keys().copied().collect();
        for id in curr_map.keys() {
            if !base_map.contains_key(id) {
                all_ids.push(id);
            }
        }
        all_ids.sort_unstable();

        for id in all_ids {
            let base_pass = base_map.get(id).copied();
            let curr_pass = curr_map.get(id).copied();

            match (base_pass, curr_pass) {
                (Some(true), Some(false)) => diff.new_failures.push(id.to_string()),
                (Some(false), Some(true)) => diff.fixed_regressions.push(id.to_string()),
                (Some(false), Some(false)) => diff.unchanged_failures.push(id.to_string()),
                _ => {
                    // Passes or new tests that pass: count as unchanged_passes.
                    if curr_pass.unwrap_or(false) || base_pass.unwrap_or(false) {
                        diff.unchanged_passes.push(id.to_string());
                    }
                }
            }
        }

        diff
    }

    // -----------------------------------------------------------------------
    // Reporting
    // -----------------------------------------------------------------------

    /// Render a [`RegressionDiff`] as a human-readable report string.
    pub fn generate_report(diff: &RegressionDiff) -> String {
        let mut s = String::new();
        s.push_str("=== SigmaOS Regression Diff Report ===\n\n");

        if diff.new_failures.is_empty() {
            s.push_str("✓ No new regressions.\n");
        } else {
            s.push_str(&format!(
                "✗ NEW FAILURES ({}): {}\n",
                diff.new_failures.len(),
                diff.new_failures.join(", ")
            ));
        }

        if !diff.fixed_regressions.is_empty() {
            s.push_str(&format!(
                "✓ FIXED REGRESSIONS ({}): {}\n",
                diff.fixed_regressions.len(),
                diff.fixed_regressions.join(", ")
            ));
        }

        if !diff.unchanged_failures.is_empty() {
            s.push_str(&format!(
                "⚠ PRE-EXISTING FAILURES ({}): {}\n",
                diff.unchanged_failures.len(),
                diff.unchanged_failures.join(", ")
            ));
        }

        s.push_str(&format!(
            "\nStable passing tests: {}\n",
            diff.unchanged_passes.len()
        ));

        s
    }
}

impl Default for RegressionRunner {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- suite has exactly 20 tests ------------------------------------------

    #[test]
    fn sigma_v1_suite_has_20_tests() {
        // Arrange / Act
        let suite = RegressionRunner::sigma_v1_suite();

        // Assert
        assert_eq!(
            suite.tests.len(),
            20,
            "sigma_v1_suite must contain exactly 20 regression tests"
        );
    }

    // --- compare_runs detects new failures -----------------------------------

    #[test]
    fn compare_runs_detects_new_failure() {
        // Arrange: baseline has all tests passing.
        let suite = RegressionRunner::sigma_v1_suite();
        let runner = RegressionRunner::new();
        let baseline = runner.run_suite(&suite);

        // Current run: force boot-001 to fail.
        let mut runner2 = RegressionRunner::new();
        runner2.force_fail("boot-001");
        let current = runner2.run_suite(&suite);

        // Act
        let diff = RegressionRunner::compare_runs(&baseline, &current);

        // Assert
        assert!(
            diff.new_failures.contains(&"boot-001".to_string()),
            "boot-001 should appear as a new failure"
        );
        assert!(diff.fixed_regressions.is_empty());
    }

    // --- compare_runs detects fixed regressions ------------------------------

    #[test]
    fn compare_runs_detects_fixed_regression() {
        // Arrange: baseline has pkg-002 failing.
        let suite = RegressionRunner::sigma_v1_suite();
        let mut runner_base = RegressionRunner::new();
        runner_base.force_fail("pkg-002");
        let baseline = runner_base.run_suite(&suite);

        // Current run: pkg-002 now passes.
        let runner_curr = RegressionRunner::new();
        let current = runner_curr.run_suite(&suite);

        // Act
        let diff = RegressionRunner::compare_runs(&baseline, &current);

        // Assert
        assert!(
            diff.fixed_regressions.contains(&"pkg-002".to_string()),
            "pkg-002 should appear as a fixed regression"
        );
        assert!(diff.new_failures.is_empty());
    }

    // --- unchanged failures stay in unchanged_failures -----------------------

    #[test]
    fn compare_runs_tracks_unchanged_failures() {
        // Arrange: both runs fail hw-003.
        let suite = RegressionRunner::sigma_v1_suite();
        let mut runner = RegressionRunner::new();
        runner.force_fail("hw-003");
        let baseline = runner.run_suite(&suite);

        let mut runner2 = RegressionRunner::new();
        runner2.force_fail("hw-003");
        let current = runner2.run_suite(&suite);

        // Act
        let diff = RegressionRunner::compare_runs(&baseline, &current);

        // Assert
        assert!(diff.unchanged_failures.contains(&"hw-003".to_string()));
        assert!(!diff.new_failures.contains(&"hw-003".to_string()));
    }

    // --- all-pass run produces no failures in the diff ----------------------

    #[test]
    fn all_pass_run_has_no_diff_failures() {
        // Arrange
        let suite = RegressionRunner::sigma_v1_suite();
        let runner = RegressionRunner::new();
        let baseline = runner.run_suite(&suite);
        let current = runner.run_suite(&suite);

        // Act
        let diff = RegressionRunner::compare_runs(&baseline, &current);

        // Assert
        assert!(diff.new_failures.is_empty());
        assert!(diff.fixed_regressions.is_empty());
        assert!(diff.unchanged_failures.is_empty());
        assert_eq!(diff.unchanged_passes.len(), 20);
    }

    // --- generate_report mentions new failures --------------------------------

    #[test]
    fn generate_report_mentions_new_failures() {
        // Arrange
        let diff = RegressionDiff {
            new_failures: vec!["security-001".to_string()],
            fixed_regressions: vec![],
            unchanged_failures: vec![],
            unchanged_passes: vec![],
        };

        // Act
        let report = RegressionRunner::generate_report(&diff);

        // Assert
        assert!(
            report.contains("security-001"),
            "Report must reference the failing test ID"
        );
        assert!(report.contains("NEW FAILURES"));
    }
}
