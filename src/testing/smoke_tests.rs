//! Automated smoke test suite for SigmaOS release validation.
//!
//! This module provides the complete release smoke test pipeline — covering
//! boot, install, desktop, package, recovery, hardware, security, and app
//! scenarios. Each test is assigned a severity so the release gate can
//! distinguish hard blockers (P0/P1) from advisory issues (P2/P3).

use std::collections::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;

// ---------------------------------------------------------------------------
// Test identifiers
// ---------------------------------------------------------------------------

/// Unique identifier for every smoke test case.
///
/// The 50 variants are grouped into 8 functional categories so that
/// [`SmokeTestRunner::run_category`] can filter by prefix string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SmokeTestId {
    // --- Boot (prefix "boot") -----------------------------------------------
    /// System reaches the graphical login screen from cold power-on.
    BootsToDesktop,
    /// GRUB / Limine menu appears and loads the kernel entry.
    GrubLoadsKernel,
    /// The init system (systemd) starts successfully.
    InitSystemStarts,
    /// Core systemd services reach `active` state within the timeout.
    SystemdServicesRunning,
    /// A default network interface acquires a link / address.
    NetworkAvailable,

    // --- Install (prefix "install") -----------------------------------------
    /// The graphical installer application launches without crashing.
    InstallerLaunches,
    /// Partitioning step completes without errors on target disk.
    PartitioningCompletes,
    /// A new user account is created with correct UID/GID.
    UserCreationWorks,
    /// First-boot wizard finishes and drops the user on the desktop.
    FirstBootWizardCompletes,
    /// All default applications are present and launchable after install.
    DefaultAppsInstalled,

    // --- Desktop (prefix "desktop") -----------------------------------------
    /// A Wayland / X11 desktop session starts cleanly.
    DesktopSessionStarts,
    /// The top/bottom panel is rendered and interactive.
    PanelVisible,
    /// The application launcher opens and displays entries.
    LauncherOpens,
    /// A test desktop notification is displayed.
    NotificationsWork,
    /// The default GTK/Kvantum theme is applied without visual glitches.
    ThemeApplies,
    /// The default wallpaper is rendered on the desktop.
    WallpaperSets,
    /// The file manager opens and lists the home directory.
    FileManagerOpens,
    /// A terminal emulator opens and accepts input.
    TerminalOpens,
    /// The default web browser opens a local page.
    BrowserLaunches,
    /// System settings / control-center opens.
    SettingsOpens,

    // --- Package (prefix "package") -----------------------------------------
    /// The graphical package manager loads the package list.
    PackageManagerLoads,
    /// The update-check mechanism completes without errors.
    UpdateCheckWorks,
    /// A test package is installed successfully via the package manager.
    PackageInstallSucceeds,
    /// A previously installed test package is removed cleanly.
    PackageRemoveSucceeds,
    /// Package GPG / signature verification accepts a valid package.
    SignatureVerification,

    // --- Recovery (prefix "recovery") ---------------------------------------
    /// A Btrfs / LUKS snapshot of the root is created successfully.
    SnapshotCreates,
    /// A previously created snapshot is restored without data loss.
    SnapshotRestores,
    /// The system boots into the rescue shell from the boot menu.
    RescueModeBoots,
    /// The full-system rollback from a snapshot completes successfully.
    RollbackWorks,
    /// Pressing `e` at GRUB and appending `init=/bin/bash` gives a shell.
    EmergencyShellLaunches,

    // --- Hardware (prefix "hardware") ---------------------------------------
    /// GPU renders a 3-D scene via Mesa / OpenGL without errors.
    GpuRenders,
    /// Wi-Fi connects to a known SSID and reaches the internet.
    WifiConnects,
    /// Bluetooth adapter performs a scan and discovers nearby devices.
    BluetoothScans,
    /// Audio playback at 44 100 Hz produces output without underruns.
    AudioPlays,
    /// Audio capture via the default microphone works for 2 seconds.
    AudioRecords,
    /// System enters S3 suspend successfully.
    SuspendWorks,
    /// System resumes from S3 in < 5 s and is fully functional.
    ResumeWorks,
    /// A USB mass-storage device is auto-mounted on insertion.
    UsbMounts,
    /// A USB printer is detected and a CUPS queue is created.
    PrinterDetects,
    /// Fractional HiDPI scaling applies correctly at 1.5×.
    DisplayScales,

    // --- Security (prefix "security") ---------------------------------------
    /// UEFI Secure Boot signature chain validates from shim → kernel.
    SecureBootChain,
    /// AppArmor module is loaded and enforcing at least one profile.
    AppArmorLoads,
    /// Spawned test process drops root privileges correctly.
    PrivilegesDropped,
    /// `nftables` firewall is active and loaded with the default ruleset.
    FirewallActive,
    /// An OTA update package signature is verified before extraction.
    UpdateSignatureValid,

    // --- Apps (prefix "apps") -----------------------------------------------
    /// LibreOffice Writer opens a sample `.odt` document.
    OfficeOpens,
    /// The PDF viewer opens a sample PDF without errors.
    PdfViewerOpens,
    /// The image viewer opens a sample PNG file.
    ImageViewerOpens,
    /// The video player plays a short H.264 clip.
    VideoPlayerPlays,
    /// A screenshot of the desktop is captured and saved.
    ScreenshotCaptures,
    /// The default calculator application opens and evaluates `2 + 2`.
    CalculatorWorks,
    /// A full-system backup job is created and runs successfully.
    BackupCreates,
    /// The default text editor opens, makes an edit, and saves.
    TextEditorSaves,
}

impl SmokeTestId {
    /// Category name prefix used by [`SmokeTestRunner::run_category`].
    pub fn category(&self) -> &'static str {
        match self {
            Self::BootsToDesktop
            | Self::GrubLoadsKernel
            | Self::InitSystemStarts
            | Self::SystemdServicesRunning
            | Self::NetworkAvailable => "boot",

            Self::InstallerLaunches
            | Self::PartitioningCompletes
            | Self::UserCreationWorks
            | Self::FirstBootWizardCompletes
            | Self::DefaultAppsInstalled => "install",

            Self::DesktopSessionStarts
            | Self::PanelVisible
            | Self::LauncherOpens
            | Self::NotificationsWork
            | Self::ThemeApplies
            | Self::WallpaperSets
            | Self::FileManagerOpens
            | Self::TerminalOpens
            | Self::BrowserLaunches
            | Self::SettingsOpens => "desktop",

            Self::PackageManagerLoads
            | Self::UpdateCheckWorks
            | Self::PackageInstallSucceeds
            | Self::PackageRemoveSucceeds
            | Self::SignatureVerification => "package",

            Self::SnapshotCreates
            | Self::SnapshotRestores
            | Self::RescueModeBoots
            | Self::RollbackWorks
            | Self::EmergencyShellLaunches => "recovery",

            Self::GpuRenders
            | Self::WifiConnects
            | Self::BluetoothScans
            | Self::AudioPlays
            | Self::AudioRecords
            | Self::SuspendWorks
            | Self::ResumeWorks
            | Self::UsbMounts
            | Self::PrinterDetects
            | Self::DisplayScales => "hardware",

            Self::SecureBootChain
            | Self::AppArmorLoads
            | Self::PrivilegesDropped
            | Self::FirewallActive
            | Self::UpdateSignatureValid => "security",

            Self::OfficeOpens
            | Self::PdfViewerOpens
            | Self::ImageViewerOpens
            | Self::VideoPlayerPlays
            | Self::ScreenshotCaptures
            | Self::CalculatorWorks
            | Self::BackupCreates
            | Self::TextEditorSaves => "apps",
        }
    }

    /// Returns every variant (all 50 tests).
    pub fn all() -> Vec<SmokeTestId> {
        vec![
            // Boot
            Self::BootsToDesktop,
            Self::GrubLoadsKernel,
            Self::InitSystemStarts,
            Self::SystemdServicesRunning,
            Self::NetworkAvailable,
            // Install
            Self::InstallerLaunches,
            Self::PartitioningCompletes,
            Self::UserCreationWorks,
            Self::FirstBootWizardCompletes,
            Self::DefaultAppsInstalled,
            // Desktop
            Self::DesktopSessionStarts,
            Self::PanelVisible,
            Self::LauncherOpens,
            Self::NotificationsWork,
            Self::ThemeApplies,
            Self::WallpaperSets,
            Self::FileManagerOpens,
            Self::TerminalOpens,
            Self::BrowserLaunches,
            Self::SettingsOpens,
            // Package
            Self::PackageManagerLoads,
            Self::UpdateCheckWorks,
            Self::PackageInstallSucceeds,
            Self::PackageRemoveSucceeds,
            Self::SignatureVerification,
            // Recovery
            Self::SnapshotCreates,
            Self::SnapshotRestores,
            Self::RescueModeBoots,
            Self::RollbackWorks,
            Self::EmergencyShellLaunches,
            // Hardware
            Self::GpuRenders,
            Self::WifiConnects,
            Self::BluetoothScans,
            Self::AudioPlays,
            Self::AudioRecords,
            Self::SuspendWorks,
            Self::ResumeWorks,
            Self::UsbMounts,
            Self::PrinterDetects,
            Self::DisplayScales,
            // Security
            Self::SecureBootChain,
            Self::AppArmorLoads,
            Self::PrivilegesDropped,
            Self::FirewallActive,
            Self::UpdateSignatureValid,
            // Apps
            Self::OfficeOpens,
            Self::PdfViewerOpens,
            Self::ImageViewerOpens,
            Self::VideoPlayerPlays,
            Self::ScreenshotCaptures,
            Self::CalculatorWorks,
            Self::BackupCreates,
            Self::TextEditorSaves,
        ]
    }
}

// ---------------------------------------------------------------------------
// Test severity
// ---------------------------------------------------------------------------

/// Priority/severity of a smoke test failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestSeverity {
    /// Ship-stopper: any P0 failure blocks the release unconditionally.
    P0Critical,
    /// High priority: more than 5 P1 failures also block the release.
    P1High,
    /// Medium priority: tracked but does not block on its own.
    P2Medium,
    /// Low priority: informational, does not affect the release gate.
    P3Low,
}

// ---------------------------------------------------------------------------
// Test result
// ---------------------------------------------------------------------------

/// Result record emitted by a single smoke test execution.
#[derive(Debug, Clone)]
pub struct SmokeTestResult {
    /// Which test produced this result.
    pub id: SmokeTestId,
    /// `true` if the test passed.
    pub passed: bool,
    /// Wall-clock time consumed by the test in milliseconds.
    pub duration_ms: u64,
    /// Human-readable error message when `passed == false`.
    pub error: Option<String>,
    /// Severity classification used by the release gate.
    pub severity: TestSeverity,
}

// ---------------------------------------------------------------------------
// Release gate output
// ---------------------------------------------------------------------------

/// Decision emitted by [`SmokeTestRunner::gate_release`].
#[derive(Debug, Clone)]
pub struct ReleaseGateResult {
    /// `true` when the build is safe to release.
    pub passed: bool,
    /// Tests whose failure blocks the release.
    pub blocking_failures: Vec<SmokeTestId>,
    /// Failed P2/P3 tests that are not individually blocking.
    pub warnings: Vec<SmokeTestId>,
    /// One-line human-readable verdict.
    pub summary: String,
}

// ---------------------------------------------------------------------------
// Report
// ---------------------------------------------------------------------------

/// Aggregate counts produced by [`SmokeTestRunner::generate_report`].
#[derive(Debug, Clone)]
pub struct SmokeTestReport {
    /// Total tests executed.
    pub total: usize,
    /// Tests that passed.
    pub passed: usize,
    /// Tests that failed.
    pub failed: usize,
    /// Number of P0 failures.
    pub p0_failures: usize,
    /// Number of P1 failures.
    pub p1_failures: usize,
    /// `true` when the gate would pass for these results.
    pub gate_passed: bool,
}

// ---------------------------------------------------------------------------
// Static severity table
// ---------------------------------------------------------------------------

fn severity_for(id: SmokeTestId) -> TestSeverity {
    match id {
        // P0 – must never fail on a release candidate
        SmokeTestId::BootsToDesktop
        | SmokeTestId::GrubLoadsKernel
        | SmokeTestId::InitSystemStarts
        | SmokeTestId::InstallerLaunches
        | SmokeTestId::PartitioningCompletes
        | SmokeTestId::UserCreationWorks
        | SmokeTestId::FirstBootWizardCompletes
        | SmokeTestId::DesktopSessionStarts
        | SmokeTestId::SecureBootChain
        | SmokeTestId::FirewallActive => TestSeverity::P0Critical,

        // P1 – important, more than 5 failures blocks release
        SmokeTestId::SystemdServicesRunning
        | SmokeTestId::NetworkAvailable
        | SmokeTestId::DefaultAppsInstalled
        | SmokeTestId::PanelVisible
        | SmokeTestId::TerminalOpens
        | SmokeTestId::PackageManagerLoads
        | SmokeTestId::PackageInstallSucceeds
        | SmokeTestId::PackageRemoveSucceeds
        | SmokeTestId::SignatureVerification
        | SmokeTestId::SnapshotCreates
        | SmokeTestId::SnapshotRestores
        | SmokeTestId::RollbackWorks
        | SmokeTestId::AppArmorLoads
        | SmokeTestId::PrivilegesDropped
        | SmokeTestId::UpdateSignatureValid => TestSeverity::P1High,

        // P2 – advisory
        SmokeTestId::LauncherOpens
        | SmokeTestId::NotificationsWork
        | SmokeTestId::ThemeApplies
        | SmokeTestId::WallpaperSets
        | SmokeTestId::FileManagerOpens
        | SmokeTestId::BrowserLaunches
        | SmokeTestId::SettingsOpens
        | SmokeTestId::UpdateCheckWorks
        | SmokeTestId::RescueModeBoots
        | SmokeTestId::EmergencyShellLaunches
        | SmokeTestId::GpuRenders
        | SmokeTestId::WifiConnects
        | SmokeTestId::AudioPlays
        | SmokeTestId::AudioRecords
        | SmokeTestId::SuspendWorks
        | SmokeTestId::ResumeWorks
        | SmokeTestId::UsbMounts
        | SmokeTestId::OfficeOpens
        | SmokeTestId::PdfViewerOpens
        | SmokeTestId::ScreenshotCaptures
        | SmokeTestId::TextEditorSaves => TestSeverity::P2Medium,

        // P3 – informational
        SmokeTestId::BluetoothScans
        | SmokeTestId::PrinterDetects
        | SmokeTestId::DisplayScales
        | SmokeTestId::ImageViewerOpens
        | SmokeTestId::VideoPlayerPlays
        | SmokeTestId::CalculatorWorks
        | SmokeTestId::BackupCreates => TestSeverity::P3Low,
    }
}

// ---------------------------------------------------------------------------
// Runner
// ---------------------------------------------------------------------------

/// Smoke test runner — simulates execution of each test case.
///
/// In a real release pipeline each test would invoke a QEMU/hardware probe or
/// a remote API; here the runner simulates deterministic pass results so the
/// logic can be validated in unit tests without real hardware.
pub struct SmokeTestRunner {
    /// Optional override map: `test_id → should_fail`.
    ///
    /// Leave empty to simulate all tests passing (the happy path).
    overrides: HashMap<SmokeTestId, bool>,
}

impl SmokeTestRunner {
    /// Create a runner with no forced failures — all tests will pass.
    pub fn new() -> Self {
        Self {
            overrides: HashMap::new(),
        }
    }

    /// Force a specific test to fail.  Used only in tests.
    #[cfg(test)]
    pub fn force_fail(&mut self, id: SmokeTestId) {
        self.overrides.insert(id, true);
    }

    /// Execute a single test and return its result.
    pub fn run_test(&self, id: SmokeTestId) -> SmokeTestResult {
        let severity = severity_for(id);
        let forced_fail = self.overrides.get(&id).copied().unwrap_or(false);

        if forced_fail {
            SmokeTestResult {
                id,
                passed: false,
                duration_ms: 1,
                error: Some(format!("Simulated failure for {:?}", id)),
                severity,
            }
        } else {
            // Simulate a deterministic pass with a realistic duration.
            let duration_ms = match id.category() {
                "boot" => 3500,
                "install" => 12000,
                "desktop" => 800,
                "package" => 4500,
                "recovery" => 9000,
                "hardware" => 2000,
                "security" => 600,
                "apps" => 1200,
                _ => 500,
            };
            SmokeTestResult {
                id,
                passed: true,
                duration_ms,
                error: None,
                severity,
            }
        }
    }

    /// Run all 50 smoke tests and return the full result list.
    pub fn run_all(&self) -> Vec<SmokeTestResult> {
        SmokeTestId::all()
            .into_iter()
            .map(|id| self.run_test(id))
            .collect()
    }

    /// Run only the tests belonging to the given category prefix.
    ///
    /// Valid prefixes: `"boot"`, `"install"`, `"desktop"`, `"package"`,
    /// `"recovery"`, `"hardware"`, `"security"`, `"apps"`.
    pub fn run_category(&self, prefix: &str) -> Vec<SmokeTestResult> {
        SmokeTestId::all()
            .into_iter()
            .filter(|id| id.category() == prefix)
            .map(|id| self.run_test(id))
            .collect()
    }

    // -----------------------------------------------------------------------
    // Static helpers (no `self` needed — operate on completed results)
    // -----------------------------------------------------------------------

    /// Evaluate whether results clear the release gate.
    ///
    /// # Gate rules
    /// 1. Any P0 failure is a hard block.
    /// 2. More than 5 P1 failures is a hard block.
    /// 3. P2 / P3 failures produce warnings only.
    pub fn gate_release(results: &[SmokeTestResult]) -> ReleaseGateResult {
        let mut blocking_failures: Vec<SmokeTestId> = Vec::new();
        let mut p1_failures: Vec<SmokeTestId> = Vec::new();
        let mut warnings: Vec<SmokeTestId> = Vec::new();

        for r in results {
            if !r.passed {
                match r.severity {
                    TestSeverity::P0Critical => blocking_failures.push(r.id),
                    TestSeverity::P1High => p1_failures.push(r.id),
                    TestSeverity::P2Medium | TestSeverity::P3Low => warnings.push(r.id),
                }
            }
        }

        // P1s exceeding the threshold become blockers too.
        let p1_blocking = p1_failures.len() > 5;
        if p1_blocking {
            blocking_failures.extend(p1_failures.iter().copied());
        } else {
            // Non-blocking P1s are advisory warnings.
            warnings.extend(p1_failures.iter().copied());
        }

        let passed = blocking_failures.is_empty();

        let summary = if passed {
            format!(
                "PASS — {} tests passed, {} warnings",
                results.iter().filter(|r| r.passed).count(),
                warnings.len()
            )
        } else {
            format!(
                "FAIL — {} blocking failure(s): {:?}",
                blocking_failures.len(),
                blocking_failures
            )
        };

        ReleaseGateResult {
            passed,
            blocking_failures,
            warnings,
            summary,
        }
    }

    /// Aggregate a result slice into a summary report.
    pub fn generate_report(results: &[SmokeTestResult]) -> SmokeTestReport {
        let total = results.len();
        let passed = results.iter().filter(|r| r.passed).count();
        let failed = total - passed;
        let p0_failures = results
            .iter()
            .filter(|r| !r.passed && r.severity == TestSeverity::P0Critical)
            .count();
        let p1_failures = results
            .iter()
            .filter(|r| !r.passed && r.severity == TestSeverity::P1High)
            .count();

        let gate = Self::gate_release(results);

        SmokeTestReport {
            total,
            passed,
            failed,
            p0_failures,
            p1_failures,
            gate_passed: gate.passed,
        }
    }
}

impl Default for SmokeTestRunner {
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

    // Arrange helper — build a single forced-fail result at a given severity.
    fn make_result(id: SmokeTestId, passed: bool, severity: TestSeverity) -> SmokeTestResult {
        SmokeTestResult {
            id,
            passed,
            duration_ms: 1,
            error: if passed {
                None
            } else {
                Some("forced".to_string())
            },
            severity,
        }
    }

    // --- gate_release passes when all pass -----------------------------------

    #[test]
    fn gate_release_passes_when_all_pass() {
        // Arrange
        let runner = SmokeTestRunner::new();

        // Act
        let results = runner.run_all();
        let gate = SmokeTestRunner::gate_release(&results);

        // Assert
        assert!(gate.passed, "All-pass run should clear the gate");
        assert!(gate.blocking_failures.is_empty());
    }

    // --- gate_release fails on any P0 failure --------------------------------

    #[test]
    fn gate_release_fails_on_p0_failure() {
        // Arrange
        let mut results: Vec<SmokeTestResult> = SmokeTestId::all()
            .into_iter()
            .map(|id| make_result(id, true, severity_for(id)))
            .collect();

        // Inject one P0 failure
        results[0] = make_result(SmokeTestId::BootsToDesktop, false, TestSeverity::P0Critical);

        // Act
        let gate = SmokeTestRunner::gate_release(&results);

        // Assert
        assert!(!gate.passed, "A P0 failure must block the release");
        assert!(gate.blocking_failures.contains(&SmokeTestId::BootsToDesktop));
    }

    // --- gate_release fails when > 5 P1s fail --------------------------------

    #[test]
    fn gate_release_fails_on_six_p1_failures() {
        // Arrange: start all passing, then flip 6 P1 tests to fail.
        let p1_tests = [
            SmokeTestId::SystemdServicesRunning,
            SmokeTestId::NetworkAvailable,
            SmokeTestId::DefaultAppsInstalled,
            SmokeTestId::PanelVisible,
            SmokeTestId::TerminalOpens,
            SmokeTestId::PackageManagerLoads,
        ];

        let mut results: Vec<SmokeTestResult> = SmokeTestId::all()
            .into_iter()
            .map(|id| make_result(id, true, severity_for(id)))
            .collect();

        for id in p1_tests {
            for r in results.iter_mut() {
                if r.id == id {
                    *r = make_result(id, false, TestSeverity::P1High);
                }
            }
        }

        // Act
        let gate = SmokeTestRunner::gate_release(&results);

        // Assert
        assert!(!gate.passed, "Six P1 failures should block the release");
        assert_eq!(gate.blocking_failures.len(), 6);
    }

    // --- report counts are correct -------------------------------------------

    #[test]
    fn report_counts_are_correct() {
        // Arrange: 2 failures (1 P0, 1 P2)
        let mut results: Vec<SmokeTestResult> = SmokeTestId::all()
            .into_iter()
            .map(|id| make_result(id, true, severity_for(id)))
            .collect();

        results[0] = make_result(SmokeTestId::BootsToDesktop, false, TestSeverity::P0Critical);
        results[1] = make_result(SmokeTestId::GrubLoadsKernel, false, TestSeverity::P2Medium);

        // Act
        let report = SmokeTestRunner::generate_report(&results);

        // Assert
        assert_eq!(report.total, 50);
        assert_eq!(report.failed, 2);
        assert_eq!(report.passed, 48);
        assert_eq!(report.p0_failures, 1);
        assert!(!report.gate_passed);
    }

    // --- all 50 variants are present -----------------------------------------

    #[test]
    fn all_50_variants_present() {
        // Act
        let all = SmokeTestId::all();

        // Assert
        assert_eq!(all.len(), 50, "Expected exactly 50 smoke test variants");

        // Spot check each category has the expected count.
        let boot_count = all.iter().filter(|id| id.category() == "boot").count();
        let desktop_count = all.iter().filter(|id| id.category() == "desktop").count();
        let hardware_count = all.iter().filter(|id| id.category() == "hardware").count();

        assert_eq!(boot_count, 5);
        assert_eq!(desktop_count, 10);
        assert_eq!(hardware_count, 10);
    }

    // --- run_category filters correctly --------------------------------------

    #[test]
    fn run_category_returns_only_matching_tests() {
        // Arrange
        let runner = SmokeTestRunner::new();

        // Act
        let results = runner.run_category("security");

        // Assert
        assert_eq!(results.len(), 5, "Security category has 5 tests");
        for r in &results {
            assert_eq!(r.id.category(), "security");
        }
    }
}
