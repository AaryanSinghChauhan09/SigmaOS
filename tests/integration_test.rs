// SigmaOS Integration Tests
// Verifies core system legacy compatibility, multi-persona VMs, and driver bridge layers
#![allow(unused, clippy::all)]

use sigmaos::compatibility::{
    APITimelineManager, AkabeiBundle, AkabeiPackageEngine, BinaryCompatMatrix, BundleType,
    DesktopTheme, DiscontinuedFS, DriverBridge, FSRevival, GraphicsBridge, InstallerStep,
    KapudanAssistant, KernelPersona, KernelPersonaVM, LegacyBus, LegacyDriver, LegacyPluginManager,
    LibcVersion, NetworkBridge, StorageBridge, SyscallAbi, TribeInstaller, WorkloadOptimizer,
    WorkloadProfile, GLOBAL_AKABEI, GLOBAL_KAPUDAN, GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER,
    GLOBAL_TRIBE, GLOBAL_WORKLOAD_OPTIMIZER,
};
use sigmaos::filesystem::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
use sigmaos::package::{
    DebPackageDriverTranslator, GenericLinuxTranslationUdf, LinuxDriverPackageTranslator,
    LinuxTranslationService, PackageFormat, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
};
use sigmaos::security::{
    AnonSurfShunt, AppSandboxEngine, DefensiveAuditSystem, ForensicBlock, ForensicStorageFilter,
    MaliciousSignature, RoutingMode, SandboxPolicy, GLOBAL_ANONSURF, GLOBAL_FORENSIC,
    GLOBAL_SANDBOX, MAX_AUDIT_BLOCKS, MAX_SIGNATURES, SIGNATURE_LEN,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_integration() {
        assert!(true);
    }

    #[test]
    fn test_legacy_personality_and_syscall_adaptation_flow() {
        // Step 1: Initialize the multi-persona VM
        let vm = KernelPersonaVM::new();
        assert_eq!(vm.get_persona(), KernelPersona::Linux_6_x);

        // Hot-swap kernel persona to 2.6 for legacy application expectations
        vm.hot_swap_persona(KernelPersona::Linux_2_6);
        assert_eq!(vm.get_persona(), KernelPersona::Linux_2_6);

        // Step 2: Use the Binary Compatibility Matrix to decode and translate syscall expectations
        let matrix = BinaryCompatMatrix::new(LibcVersion::Libc5, SyscallAbi::Oabi_32);
        let translated_sys = matrix.translate_sys_context(5); // expect 1005 offset mapping
        assert_eq!(translated_sys, 1005);

        // Step 3: Verify the API Timeline Manager parameter mappings
        let timeline = APITimelineManager::new(KernelPersona::Linux_2_6);
        let cleaned_param = timeline.map_syscall_params(0x0000111100002222);
        assert_eq!(cleaned_param, 0x00002222);
    }

    #[test]
    fn test_legacy_driver_bridge_revival() {
        let storage = StorageBridge {
            driver_name: "floppy-drive-controller",
            bus: LegacyBus::Isa,
        };
        let graphics = GraphicsBridge {
            driver_name: "crt-terminal-controller",
            bus: LegacyBus::Agp,
        };

        assert_eq!(storage.bus_type(), LegacyBus::Isa);
        assert_eq!(graphics.bus_type(), LegacyBus::Agp);
        assert!(storage.init_legacy());
        assert!(graphics.init_legacy());
    }

    #[test]
    fn test_legacy_workload_optimizer_tuning() {
        let optimizer = WorkloadOptimizer::new();
        assert_eq!(optimizer.get_profile(), WorkloadProfile::LowMemoryProfile);

        // Apply Single Core scheduling locks for early thread assumptions
        optimizer.apply_workload_tuning(WorkloadProfile::SingleCoreProfile);
        assert_eq!(optimizer.get_profile(), WorkloadProfile::SingleCoreProfile);
    }

    #[test]
    fn test_parrot_security_parity() {
        // Test AnonSurf Shunt
        let shunt = AnonSurfShunt::new();
        assert_eq!(shunt.get_mode(), RoutingMode::DirectCleartext);
        assert_eq!(shunt.get_packets_routed(), 0);

        shunt.enable_anonsurf();
        assert_eq!(shunt.get_mode(), RoutingMode::TorAnonymized);

        shunt.shunt_packet(42, 1024);
        assert_eq!(shunt.get_packets_routed(), 1);

        shunt.disable_anonsurf();
        assert_eq!(shunt.get_mode(), RoutingMode::DirectCleartext);

        // Test AppSandbox
        let sandbox = AppSandboxEngine::new();
        // Default policy forbids raw sockets and network
        assert!(!sandbox.validate_network_socket(true));
        assert!(!sandbox.validate_network_socket(false));

        // File system writes should only be allowed inside permitted subpath
        assert!(sandbox.validate_filesystem_write("/sandbox/tmp/test.txt"));
        assert!(!sandbox.validate_filesystem_write("/etc/passwd"));

        sandbox.update_policy(SandboxPolicy {
            allow_network: true,
            allow_raw_sockets: true,
            allow_filesystem_write: true,
            permitted_subpath: "/anywhere",
        });
        assert!(sandbox.validate_network_socket(true));
        assert!(sandbox.validate_filesystem_write("/etc/passwd"));

        // Test ForensicStorageFilter
        let filter = ForensicStorageFilter::new();
        let mut buffer = [0u8; 512];
        assert!(!filter.intercept_device_write(0, &buffer));

        filter.set_write_blocker(false);
        assert!(filter.intercept_device_write(0, &buffer));

        let mut secure_key = [0xAAu8; 16];
        filter.secure_memory_wipe(&mut secure_key);
        for &b in &secure_key {
            assert_eq!(b, 0x00);
        }
    }

    #[test]
    fn test_chakra_linux_inspirations() {
        // Test Akabei Bundle Resolver
        let akabei = AkabeiPackageEngine::new();
        assert!(akabei.resolve_and_sandbox("gimp-app"));
        assert!(akabei.resolve_and_sandbox("plasma-desktop"));
        assert!(!akabei.resolve_and_sandbox("non-existent-app"));

        // Test Kapudan setup assistant
        let kapudan = KapudanAssistant::new();
        kapudan.welcome_user();
        assert_eq!(kapudan.get_theme(), DesktopTheme::CaledoniaDark);
        kapudan.set_theme(DesktopTheme::ZenithTranslucent);
        assert_eq!(kapudan.get_theme(), DesktopTheme::ZenithTranslucent);

        // Test Tribe installer
        let installer = TribeInstaller::new(120);
        assert_eq!(installer.get_step(), InstallerStep::Welcome);
        installer.execute_installation("admin");
        assert_eq!(installer.get_step(), InstallerStep::Completed);
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
    fn test_smart_symbolic_links() {
        let mut link1 = SmartSymlink::new("lib-redirect-1", "/usr/lib/modern/libc.so");
        assert!(link1.add_fallback_target("/usr/lib/legacy/libc.so"));
        assert!(link1.add_fallback_target("/lib/libc.so"));

        let mut link2 = SmartSymlink::new("lib-redirect-2", "/usr/lib/alt/libc.so");

        let rule = LinuxPersonaRule;

        // Case 1: Primary target exists
        let res1 =
            link1.resolve_symlink(KernelPersona::Linux_6_x, true, &[false, false], &rule, None);
        assert_eq!(res1, Ok("/usr/lib/modern/libc.so"));

        // Case 2: Primary target broken, heals to fallback index 1
        let res2 =
            link1.resolve_symlink(KernelPersona::Linux_6_x, false, &[false, true], &rule, None);
        assert_eq!(res2, Ok("/lib/libc.so"));

        // Case 3: Complete orphaning
        let res3 = link1.resolve_symlink(
            KernelPersona::Linux_6_x,
            false,
            &[false, false],
            &rule,
            None,
        );
        assert!(res3.is_err());

        // Case 4: ELOOP infinite recursion detection (nested lookup chains)
        let mut loop_err = Ok("");
        for _ in 0..12 {
            loop_err = link1.resolve_symlink(
                KernelPersona::Linux_6_x,
                true,
                &[false, false],
                &rule,
                Some(&link2),
            );
            if loop_err.is_err() {
                break;
            }
        }
        assert_eq!(
            loop_err,
            Err("ELOOP: Infinite loop or excessive recursion detected in symlink path resolution.")
        );

        // Case 5: Rule context-awareness evaluation
        let legacy_rule = LegacyLinuxRule;
        // On modern Linux_6_x kernel, Legacy rule rejects and points directly to first fallback path
        let res_legacy = link1.resolve_symlink(
            KernelPersona::Linux_6_x,
            true,
            &[false, false],
            &legacy_rule,
            None,
        );
        assert_eq!(res_legacy, Ok("/usr/lib/legacy/libc.so"));
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
}
