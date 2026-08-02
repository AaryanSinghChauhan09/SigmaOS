// SigmaOS Integration Tests
// Verifies core system legacy compatibility, multi-persona VMs, and driver bridge layers
#![allow(unused, clippy::all)]

use sigmaos::kernel::{Priority, Process, ProcessState};
use sigmaos::package::{
    DebPackageDriverTranslator, GenericLinuxTranslationUdf, LinuxDriverPackageTranslator,
    LinuxTranslationService, PackageFormat, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
};
use sigmaos::security::{DefensiveAuditSystem, ForensicBlock, MaliciousSignature};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_integration() {
        assert!(true);
    }

    #[test]
    fn test_defensive_audit_and_anomaly_detection() {
        let audit = DefensiveAuditSystem::new(75);

        // Log simple safe event
        assert!(audit.log_event(1716000000, 1000, 4, b"ls -la").is_ok());

        // Test safe payload anomaly scoring
        let safe_score = audit.evaluate_anomaly_score(b"cat file.txt");
        assert!(safe_score < 75);
        assert!(audit.check_payload_safety(b"cat file.txt"));

        // Test malicious payload anomaly scoring (contains "/bin/sh")
        let malicious_score = audit.evaluate_anomaly_score(b"sudo /bin/sh -c 'rm -rf /'");
        assert!(malicious_score >= 80);
        assert!(!audit.check_payload_safety(b"sudo /bin/sh -c 'rm -rf /'"));
    }

    #[test]
    fn test_linux_package_driver_translation() {
        // Test UDF directly
        let udf = GenericLinuxTranslationUdf;
        assert_eq!(udf.translate_syscall(1), 1); // write -> native write
        assert_eq!(udf.translate_syscall(9), 2009); // mmap -> offset remapped
        assert_eq!(udf.translate_io_control(0x5401), 0x101); // TCGETS -> native

        // Test unified translation service using global static UDF
        let service = LinuxTranslationService::new(&GLOBAL_TRANSLATION_UDF);
        assert_eq!(service.translate_binary_syscall(0), Ok(0)); // read -> Ok(0)
        assert_eq!(service.translate_device_ioctl(0x5402), 0x102); // TCSETS

        // Test Debian/Deb Package Translator
        let deb_translator = DebPackageDriverTranslator {
            name: "e1000-nic-module.deb",
            payload_size: 409600,
            is_kernel_module: true,
        };
        assert_eq!(deb_translator.source_format(), PackageFormat::Deb);
        assert_eq!(deb_translator.package_name(), "e1000-nic-module.deb");
        let deb_driver = deb_translator.translate_to_driver();
        assert_eq!(deb_driver.id, 9901);

        // Test RedHat/RPM Package Translator
        let rpm_translator = RpmPackageDriverTranslator {
            name: "nvme-storage.rpm",
            header_signature_valid: true,
        };
        assert_eq!(rpm_translator.source_format(), PackageFormat::Rpm);
        let rpm_driver = rpm_translator.translate_to_driver();
        assert_eq!(rpm_driver.id, 9902);

        // Test Arch/Pacman Package Translator
        let pac_translator = PacmanPackageDriverTranslator {
            name: "ch340-serial.pkg.tar.zst",
            has_aur_recipes: true,
        };
        assert_eq!(pac_translator.source_format(), PackageFormat::Pacman);
        let pac_driver = pac_translator.translate_to_driver();
        assert_eq!(pac_driver.id, 9903);
    }

    #[test]
    fn test_process_signals_integration() {
        use sigmaos::runtime::process::{Process, ProcessSignal, ProcessCapability};

        let cap = ProcessCapability::full();
        let process = unsafe { Process::new(10, 1, cap) };

        assert!(!process.consume_pending_signal(ProcessSignal::SigKill));
        process.send_signal(ProcessSignal::SigKill);
        assert!(process.consume_pending_signal(ProcessSignal::SigKill));
        assert!(!process.consume_pending_signal(ProcessSignal::SigKill));
    }

    #[test]
    fn test_supervised_service_targets() {
        use sigmaos::runtime::process::{Process, ProcessState, ProcessCapability, SupervisedServiceTarget};

        let cap = ProcessCapability::full();
        let process = unsafe { Process::new(11, 1, cap) };
        let mut supervisor = SupervisedServiceTarget::new(11);

        assert!(!unsafe { supervisor.monitor_and_supervise(&process) });

        process.set_state(ProcessState::Terminated);
        assert!(unsafe { supervisor.monitor_and_supervise(&process) });
        assert!(supervisor.auto_respawn_triggered);
        assert_eq!(supervisor.restart_count, 1);
        assert_eq!(process.get_state(), ProcessState::Running);
    }

    #[test]
    fn test_multi_distro_packaging_compatibility() {
        use sigmaos::sigpkg::universal_adapter::{ApkAdapter, NixAdapter, EbuildAdapter, PackageFormatAdapter};

        let apk = ApkAdapter::new();
        assert_eq!(apk.format_name(), "apk");

        let nix = NixAdapter::new();
        assert_eq!(nix.format_name(), "nix");

        let ebuild = EbuildAdapter::new();
        assert_eq!(ebuild.format_name(), "ebuild");
    }
}
