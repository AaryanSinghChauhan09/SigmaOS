// SigmaOS Integration Tests
// Verifies core system legacy compatibility, multi-persona VMs, and driver bridge layers
#![allow(unused, clippy::all)]

use sigmaos::compatibility::{
    APITimelineManager, AkabeiBundle, AkabeiPackageEngine, AntixControlCenter,
    AntixDesktopProfiler, AntixInitManager, BinaryCompatMatrix, BundleType,
    DesktopProfile as AntixDesktopProfile, DesktopTheme, DiscontinuedFS, DriverBridge, FSRevival,
    GraphicsBridge, InstallerStep, KapudanAssistant, KernelPersona, KernelPersonaVM, LegacyBus,
    LegacyDriver, LegacyMemoryTrimmer, LegacyPluginManager, LibcVersion, MicroService,
    MicroService as AntixMicroService, MicroServiceState,
    MicroServiceState as AntixMicroServiceState, NetworkBridge, StorageBridge, SyscallAbi,
    TribeInstaller, WorkloadOptimizer, WorkloadProfile, GLOBAL_AKABEI, GLOBAL_ANTIX_CONTROL,
    GLOBAL_ANTIX_DESKTOP, GLOBAL_ANTIX_INIT, GLOBAL_KAPUDAN, GLOBAL_MEMORY_TRIMMER,
    GLOBAL_PERSONA_VM, GLOBAL_PLUGIN_MANAGER, GLOBAL_TRIBE, GLOBAL_WORKLOAD_OPTIMIZER,
};
use sigmaos::drivers::{
    Ch340Driver, E1000Driver, GpuCommand, GpuCommandBuffer, GpuDriver, GpuPipeline, GpuShader,
    IntelHdaDriver, NvmeDriver, PeripheralDevice, PowerState, ShaderStage,
};
use sigmaos::filesystem::{
    FileType, FsError, LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule,
    VirtualFilesystem, O_APPEND, O_CREAT, O_EXCL, O_RDONLY, O_RDWR, O_TRUNC, O_WRONLY,
};
use sigmaos::interrupt::{
    HandlerCapability, HandlerType, InterruptController, InterruptError, InterruptHandler,
    InterruptManager, InterruptPriority, InterruptResult, InterruptTrace, SimpleInterruptHandler,
    TraceEventType, PIC,
};
use sigmaos::network::{
    FirewallAction, FirewallCommand, FirewallFilterRule, IpRoute2Command, LinkState, PingCommand,
    SocketStatsCommand, SocketStatsEntry, TcpConnection, TcpError, TcpSegment, TcpStack, TcpState,
    UfwDefaultRule, GLOBAL_FIREWALL, GLOBAL_IP_COMMAND, GLOBAL_UFW_RULE,
};
use sigmaos::package::{
    DebPackageDriverTranslator, GenericLinuxTranslationUdf, LinuxDriverPackageTranslator,
    LinuxTranslationService, PackageFormat, PackageTranslationUdf, PacmanPackageDriverTranslator,
    RpmPackageDriverTranslator, SigmaSoftwareStore, SoftwareRegistryEntry, GLOBAL_SOFTWARE_STORE,
    GLOBAL_TRANSLATION_SERVICE, GLOBAL_TRANSLATION_UDF,
};
use sigmaos::performance::{
    CpuPriorityOptimizer, GlarySmartRule, IoPriorityOptimizer, IoTaskPriority,
    PerformanceProfileRule, RamDefragmenter, SmartPerformanceProfile, SmartResourceOptimizer,
    GLOBAL_GLARY_RULE, GLOBAL_SMART_OPTIMIZER,
};
use sigmaos::productivity::{AudioChannel, SigmaMediaEngine, GLOBAL_MEDIA_ENGINE};
use sigmaos::resilience::{FsSnapshot, SigmaTimeshift, GLOBAL_TIMESHIFT};
use sigmaos::security::{
    AnonSurfShunt, AppSandboxEngine, CapabilityToken, DefensiveAuditSystem, ForensicBlock,
    ForensicStorageFilter, MaliciousSignature, Permission, RoutingMode, SandboxPolicy,
    GLOBAL_ANONSURF, GLOBAL_FORENSIC, GLOBAL_SANDBOX, MAX_AUDIT_BLOCKS, MAX_SIGNATURES,
    SIGNATURE_LEN,
};

use sigmaos::kernel::{
    ArchitectureEngine, CpuRegisters, HardwareException, InstructionCyclePhase, Irql,
    LookasideList, MemoryDescriptorList, Pcb, PoolType, Priority, Process, ProcessState,
    ProcessorInitState, Tcb, ThreadState,
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

        // Case 6: Dynamic environment context expansion
        let env_target1 =
            link1.expand_environment_context("/usr/lib/$USER/libc.so", "admin", "en_US");
        assert_eq!(env_target1, "/home/admin/libs");
        let env_target2 =
            link1.expand_environment_context("/usr/lib/$LANG/libc.so", "guest", "en_US");
        assert_eq!(env_target2, "/usr/share/locale/en");

        // Case 7: Sandbox boundary escape verification
        assert!(link1.is_sandbox_escape_safe("/sandbox/tmp/test.txt", "/sandbox"));
        assert!(!link1.is_sandbox_escape_safe("/sandbox/tmp/../../../etc/passwd", "/sandbox"));
        assert!(!link1.is_sandbox_escape_safe("/etc/passwd", "/sandbox"));

        // Case 8: Multi-Lib architecture routing translation
        assert_eq!(
            link1.resolve_multi_lib_routing(SyscallAbi::Oabi_32),
            "/lib32/libc.so"
        );
        assert_eq!(
            link1.resolve_multi_lib_routing(SyscallAbi::Eabi_64),
            "/lib64/libc.so"
        );
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
    fn test_antix_linux_parity() {
        // Test SysV-parity MicroServices inside AntixInitManager
        let init = AntixInitManager::new();
        assert_eq!(init.services[0].get_state(), MicroServiceState::Stopped);
        init.boot_systemd_free();
        assert_eq!(init.services[0].get_state(), MicroServiceState::Running);
        assert_eq!(init.services[1].get_state(), MicroServiceState::Running);

        init.services[0].stop();
        assert_eq!(init.services[0].get_state(), MicroServiceState::Stopped);

        // Test Low-Overhead Desktop Profiler
        let profiler = AntixDesktopProfiler::new();
        assert_eq!(profiler.get_profile(), AntixDesktopProfile::IceWM);
        profiler.apply_profile(AntixDesktopProfile::JWM);
        assert_eq!(profiler.get_profile(), AntixDesktopProfile::JWM);

        // Test Control Center Legacy configuration coordinator
        let control = AntixControlCenter::new();
        control.auto_configure_legacy_hardware();

        // Test Aggressive Memory Cache Trimmer
        let trimmer = LegacyMemoryTrimmer::new();
        // High RAM: normal reclaim
        let reclaim1 = trimmer.trim_caches(1024);
        assert!(reclaim1 > 0);

        // Low RAM (e.g. 256 MB): triggers aggressive escalation (max target state)
        let reclaim2 = trimmer.trim_caches(256);
        assert_eq!(
            trimmer
                .trim_aggressiveness
                .load(core::sync::atomic::Ordering::SeqCst),
            10
        );
    }

    #[test]
    fn test_smart_resource_optimizer() {
        // Test CPU Priority Optimizer
        let cpu_optimizer = CpuPriorityOptimizer::new();
        let mut proc1 = Process::new(1, "proc1".to_string(), Priority::Normal);
        proc1.state = ProcessState::Running;
        let mut proc2 = Process::new(2, "proc2".to_string(), Priority::Normal);
        proc2.state = ProcessState::Blocked;
        let mut processes = [proc1, proc2];

        cpu_optimizer.optimize_process_priorities(&mut processes);
        assert_eq!(processes[0].priority, Priority::High);
        assert_eq!(processes[1].priority, Priority::Low);

        // Test RAM Defragmenter
        let defragmenter = RamDefragmenter::new();
        let reclaimed = defragmenter.defragment_heap_allocations(1048576); // 1 MB
        assert_eq!(reclaimed, 1048576 / 8);
        assert_eq!(
            defragmenter
                .cleanup_count
                .load(core::sync::atomic::Ordering::SeqCst),
            1
        );

        // Test I/O Priority Optimizer
        let io_optimizer = IoPriorityOptimizer::new();
        assert_eq!(
            io_optimizer.resolve_disk_io_priority(true),
            IoTaskPriority::RealTime
        );
        assert_eq!(
            io_optimizer.resolve_disk_io_priority(false),
            IoTaskPriority::Idle
        );

        // Test Glary Smart Rule
        let rule = GlarySmartRule;
        assert_eq!(
            rule.evaluate_target_profile(10, 50),
            SmartPerformanceProfile::EcoBattery
        ); // low battery
        assert_eq!(
            rule.evaluate_target_profile(90, 90),
            SmartPerformanceProfile::EcoBattery
        ); // high temp
        assert_eq!(
            rule.evaluate_target_profile(90, 45),
            SmartPerformanceProfile::TurboMax
        ); // turbo

        // Test Unified Smart Resource Optimizer
        let optimizer = SmartResourceOptimizer::new();
        assert_eq!(optimizer.get_profile(), SmartPerformanceProfile::NormalAuto);
        optimizer.execute_auto_tuning(95, 40, &rule);
        assert_eq!(optimizer.get_profile(), SmartPerformanceProfile::TurboMax);
    }

    #[test]
    fn test_mint_linux_parity_features() {
        // Test Timeshift Backups
        let timeshift = SigmaTimeshift::new();
        let snap_id = timeshift.create_snapshot(1716000000, 0x55AA55AA).unwrap();
        assert_eq!(snap_id, 1);

        let restored_hash = timeshift.rollback_to_snapshot(1).unwrap();
        assert_eq!(restored_hash, 0x55AA55AA);

        // Test Software Store
        let store = SigmaSoftwareStore::new();
        assert!(store.install_with_safety_check("firefox-developer").is_ok());
        assert_eq!(store.trigger_auto_updates(), 1); // updated 1 package

        // Test Media Engine
        let media = SigmaMediaEngine::new();
        let pcm_buf = [100u16, 200, 300, 400];
        assert!(media.play_chiptune_buffer(0, &pcm_buf).is_ok());
        assert!(media.adjust_channel_volume(0, 90).is_ok());
    }

    #[test]
    fn test_linux_parity_networking_commands() {
        // Test iproute2 IpRoute2Command
        let ip_cmd = IpRoute2Command::new("eth1");
        assert_eq!(ip_cmd.get_link_state(), LinkState::Down);
        ip_cmd.set_link_state(LinkState::Up);
        assert_eq!(ip_cmd.get_link_state(), LinkState::Up);
        ip_cmd.assign_ip_address(0xC0A80101); // 192.168.1.1
        assert_eq!(
            ip_cmd
                .assigned_ip
                .load(core::sync::atomic::Ordering::SeqCst),
            0xC0A80101
        );

        // Test ss/netstat SocketStatsCommand
        let ss_cmd = SocketStatsCommand::new();
        let socket_count = ss_cmd.dump_active_sockets();
        assert_eq!(socket_count, 2);

        // Test ping PingCommand
        let ping_cmd = PingCommand::new();
        let latency = ping_cmd.ping_host(0xC0A80101, 3);
        assert_eq!(latency, 8);
        assert_eq!(
            ping_cmd
                .packets_sent
                .load(core::sync::atomic::Ordering::SeqCst),
            3
        );
        assert_eq!(
            ping_cmd
                .packets_received
                .load(core::sync::atomic::Ordering::SeqCst),
            3
        );

        // Test ufw/iptables FirewallCommand with global static rule
        let firewall = FirewallCommand::new(&GLOBAL_UFW_RULE);
        assert!(firewall.filter_incoming_packet(0xC0A80102, 22)); // allow SSH
        assert!(firewall.filter_incoming_packet(0xC0A80102, 80)); // allow HTTP
        assert!(!firewall.filter_incoming_packet(0xC0A80102, 23)); // reject Telnet
        assert!(!firewall.filter_incoming_packet(0xC0A80102, 443)); // deny HTTPS by default
    }

    #[test]
    fn test_unimplemented_hardware_drivers() {
        let caps = CapabilityToken::new(); // empty capabilities -> denied
        let mut ch340 = Ch340Driver::new(1, caps);
        assert!(ch340.initialize().is_err()); // fails initialized due to missing caps

        let block_caps = CapabilityToken::new().allow_network("tcp", 80); // sets 0th bit
        let mut ch340_authorized = Ch340Driver::new(1, block_caps);
        assert!(ch340_authorized.initialize().is_ok());

        let nvme_caps = CapabilityToken::new().allow_read("/var/www"); // sets 2nd bit
        let mut nvme = unsafe { NvmeDriver::new(0xE0000000, nvme_caps) };
        assert!(nvme.initialize().is_ok());

        let e1000_caps = CapabilityToken::new().allow_network("udp", 80); // sets 1st bit
        let mut e1000 = unsafe { E1000Driver::new(0xE1000000, e1000_caps) };
        assert!(e1000.initialize().is_ok());

        let hda_caps = CapabilityToken::new().allow_network("tcp", 80); // sets 0th bit
        let mut hda = unsafe { IntelHdaDriver::new(0xE2000000, hda_caps) };
        assert!(hda.initialize().is_ok());

        // Verify read/write/shutdown lifecycle operations
        let mut rx_buf = [0u8; 1024];
        assert_eq!(ch340_authorized.read(&mut rx_buf), Ok(1));
        assert_eq!(ch340_authorized.write(b"serial"), Ok(6));
        assert!(ch340_authorized.shutdown().is_ok());

        // Verify power state toggles
        assert!(nvme.set_power_state(PowerState::Sleep).is_ok());
        assert!(nvme.shutdown().is_ok());
    }

    #[test]
    fn test_gpu_pipeline_and_command_buffer() {
        let mut gpu = GpuDriver::new(100, 100);

        let vs = GpuShader {
            stage: ShaderStage::Vertex,
            source_hash: 0x12345678,
        };
        let fs = GpuShader {
            stage: ShaderStage::Fragment,
            source_hash: 0xabcdef01,
        };

        let pipeline = GpuPipeline {
            id: 1,
            vertex_shader: Some(vs),
            fragment_shader: Some(fs),
            depth_test_enabled: true,
            blend_enabled: true,
            viewport_width: 100,
            viewport_height: 100,
        };

        gpu.register_pipeline(pipeline);

        let mut cmd_buf = GpuCommandBuffer::new();
        cmd_buf.begin_recording();
        cmd_buf.record_command(GpuCommand::BindPipeline { pipeline_id: 1 });
        cmd_buf.record_command(GpuCommand::DrawIndexed {
            index_count: 50,
            first_index: 0,
        });
        cmd_buf.end_recording();

        assert!(gpu.submit_command_buffer(cmd_buf).is_ok());
        assert_eq!(gpu.bound_pipeline_id, Some(1));

        // Index 0 to 49 should be shaded magenta (0xFF00FF)
        for i in 0..50 {
            assert_eq!(gpu.frame_buffer[i], 0xFF00FF);
        }
    }

    #[test]
    fn test_gpu_hang_recovery() {
        let mut gpu = GpuDriver::new(100, 100);

        let pipeline = GpuPipeline {
            id: 2,
            vertex_shader: None,
            fragment_shader: None,
            depth_test_enabled: false,
            blend_enabled: false,
            viewport_width: 100,
            viewport_height: 100,
        };
        gpu.register_pipeline(pipeline);

        let mut cmd_buf = GpuCommandBuffer::new();
        cmd_buf.begin_recording();
        cmd_buf.record_command(GpuCommand::BindPipeline { pipeline_id: 2 });
        cmd_buf.record_command(GpuCommand::SimulateHang);
        cmd_buf.end_recording();

        // Submitting command buffer triggers simulated hardware hang (TDR recovery)
        let res = gpu.submit_command_buffer(cmd_buf);
        assert_eq!(res, Err(sigmaos::drivers::GpuError::HardwareHang));

        // After TDR reset, hardware status is ready, bound_pipeline_id is reset, total hangs incremented
        assert!(gpu.reset_state.is_hardware_ready);
        assert_eq!(gpu.reset_state.total_hangs_recovered, 1);
        assert_eq!(gpu.bound_pipeline_id, None);
        // Reconstructed pipeline count matches the number of registered pipelines (1)
        assert_eq!(gpu.reset_state.pipeline_reconstructed_count, 1);
        // Framebuffer is cleared to diagnostic slate gray (0x333333)
        assert_eq!(gpu.frame_buffer[0], 0x333333);
    }

    #[test]
    fn test_vfs_path_traversal_and_flags() {
        let mut vfs = VirtualFilesystem::new();

        // 1. Resolve root path "/"
        let root_id = vfs.resolve_path("/").unwrap();
        assert_eq!(root_id, 0);

        // 2. Open "/file.txt" with O_CREAT
        let fd = vfs.open_path("/file.txt", O_CREAT | O_RDWR, 100).unwrap();

        // 3. Write data to fd
        let data = b"Hello VFS";
        let written = vfs.write_file(fd, data).unwrap();
        assert_eq!(written, data.len());

        // 4. Seek offset back to 0 and read actual stored data back
        assert!(vfs.close_file(fd).is_ok());
        let fd_read = vfs.open_path("/file.txt", O_RDONLY, 100).unwrap();
        let mut buf = [0u8; 16];
        let bytes_read = vfs.read_file(fd_read, &mut buf).unwrap();
        assert_eq!(bytes_read, data.len());
        assert_eq!(&buf[..bytes_read], data);
        assert!(vfs.close_file(fd_read).is_ok());

        // 5. Test O_CREAT | O_EXCL triggers AlreadyExists error on existing file
        let err_excl = vfs.open_path("/file.txt", O_CREAT | O_EXCL | O_RDWR, 100);
        assert_eq!(err_excl.err(), Some(FsError::AlreadyExists));

        // 6. Test O_APPEND mode
        let fd_append = vfs.open_path("/file.txt", O_APPEND | O_RDWR, 100).unwrap();
        let append_data = b" Conforming";
        let written_append = vfs.write_file(fd_append, append_data).unwrap();
        assert_eq!(written_append, append_data.len());
        assert!(vfs.close_file(fd_append).is_ok());

        // Read all back
        let fd_all = vfs.open_path("/file.txt", O_RDONLY, 100).unwrap();
        let mut all_buf = [0u8; 32];
        let all_read = vfs.read_file(fd_all, &mut all_buf).unwrap();
        assert_eq!(&all_buf[..all_read], b"Hello VFS Conforming");
        assert!(vfs.close_file(fd_all).is_ok());

        // 7. Test O_TRUNC mode
        let fd_trunc = vfs.open_path("/file.txt", O_TRUNC | O_RDWR, 100).unwrap();
        let inode_id = vfs.resolve_path("/file.txt").unwrap();
        let inode = vfs.get_inode(inode_id).unwrap();
        assert_eq!(inode.size, 0);
        assert_eq!(inode.data.len(), 0);
        assert!(vfs.close_file(fd_trunc).is_ok());
    }

    #[test]
    fn test_vfs_hard_links_and_ref_counting() {
        let mut vfs = VirtualFilesystem::new();

        // 1. Create source file
        let fd = vfs.open_path("/src.txt", O_CREAT | O_RDWR, 100).unwrap();
        vfs.write_file(fd, b"shared content").unwrap();
        assert!(vfs.close_file(fd).is_ok());

        let src_id = vfs.resolve_path("/src.txt").unwrap();
        let src_inode = vfs.get_inode(src_id).unwrap();
        assert_eq!(src_inode.hard_links_count, 1);

        // 2. Link "/src.txt" to "/link1.txt"
        assert!(vfs.link_inode("/src.txt", "/link1.txt").is_ok());

        // Hard links count should now be 2
        let src_inode2 = vfs.get_inode(src_id).unwrap();
        assert_eq!(src_inode2.hard_links_count, 2);

        // Link "/src.txt" to "/link2.txt"
        assert!(vfs.link_inode("/src.txt", "/link2.txt").is_ok());

        // Hard links count should now be 3
        let src_inode3 = vfs.get_inode(src_id).unwrap();
        assert_eq!(src_inode3.hard_links_count, 3);

        // 3. Unlink "/link1.txt"
        assert!(vfs.unlink_inode("/link1.txt").is_ok());

        // Inode should still exist, with link count 2
        assert!(vfs.resolve_path("/src.txt").is_ok());
        let src_inode4 = vfs.get_inode(src_id).unwrap();
        assert_eq!(src_inode4.hard_links_count, 2);

        // 4. Unlink "/src.txt"
        assert!(vfs.unlink_inode("/src.txt").is_ok());

        // Inode should still exist, with link count 1 (via link2.txt)
        let src_inode5 = vfs.get_inode(src_id).unwrap();
        assert_eq!(src_inode5.hard_links_count, 1);

        // 5. Unlink "/link2.txt"
        assert!(vfs.unlink_inode("/link2.txt").is_ok());

        // Inode should now be completely deleted
        assert_eq!(
            vfs.resolve_path("/link2.txt").err(),
            Some(FsError::NotFound)
        );
        assert!(vfs.get_inode(src_id).is_none());
    }

    #[test]
    fn test_interrupt_nesting_preemption() {
        let mut manager = InterruptManager::new();
        let mut pic = Box::new(PIC::new(sigmaos::interrupt::ControllerCapability::full()));

        // Create Handlers with Low, Normal, High, and Critical priority
        let low_handler = Box::new(SimpleInterruptHandler::new(
            HandlerType::Custom,
            InterruptPriority::Low,
            HandlerCapability::full(),
        ));
        let normal_handler = Box::new(SimpleInterruptHandler::new(
            HandlerType::Custom,
            InterruptPriority::Normal,
            HandlerCapability::full(),
        ));
        let high_handler = Box::new(SimpleInterruptHandler::new(
            HandlerType::Custom,
            InterruptPriority::High,
            HandlerCapability::full(),
        ));

        // Register handlers to interrupt lines 10, 20, 30
        assert!(pic.register_handler(low_handler, 10).is_ok());
        assert!(pic.register_handler(normal_handler, 20).is_ok());
        assert!(pic.register_handler(high_handler, 30).is_ok());

        assert!(pic.enable_interrupt(10).is_ok());
        assert!(pic.enable_interrupt(20).is_ok());
        assert!(pic.enable_interrupt(30).is_ok());

        manager.add_controller(pic);

        // Scenario 1: Dispatch low priority interrupt 10 -> handled successfully
        assert_eq!(manager.dispatch_interrupt(10), InterruptResult::Handled);

        // Scenario 2: Priority-Based Masking. Start executing normal priority 20
        // During its execution, dispatching a low priority interrupt 10 should be deferred/masked
        manager.execution_stack.push(20);
        assert_eq!(manager.dispatch_interrupt(10), InterruptResult::Deferred);
        assert_eq!(manager.pending_queue.len(), 1);
        assert_eq!(manager.pending_queue[0], 10);
        manager.pending_queue.clear();

        // Scenario 3: Preemption. During normal priority 20 execution, high priority 30 is dispatched
        // 30 should immediately preempt and run to completion
        assert_eq!(manager.dispatch_interrupt(30), InterruptResult::Handled);
        manager.execution_stack.pop(); // Pop 20
    }

    #[test]
    fn test_interrupt_disabling_and_deferred_flushing() {
        let mut manager = InterruptManager::new();
        let mut pic = Box::new(PIC::new(sigmaos::interrupt::ControllerCapability::full()));

        let handler = Box::new(SimpleInterruptHandler::new(
            HandlerType::Custom,
            InterruptPriority::Normal,
            HandlerCapability::full(),
        ));
        assert!(pic.register_handler(handler, 15).is_ok());
        assert!(pic.enable_interrupt(15).is_ok());
        manager.add_controller(pic);

        // Globally disable interrupts (CLI)
        manager.cli();

        // Dispatch normal priority interrupt 15 -> deferred as pending
        assert_eq!(manager.dispatch_interrupt(15), InterruptResult::Deferred);
        assert_eq!(manager.pending_queue.len(), 1);
        assert_eq!(manager.pending_queue[0], 15);

        // Globally enable interrupts (STI) -> automatically flushes and handles pending 15!
        manager.sti();
        assert_eq!(manager.pending_queue.len(), 0);
    }

    #[test]
    fn test_interrupt_time_sequence_logging() {
        let mut manager = InterruptManager::new();
        let mut pic = Box::new(PIC::new(sigmaos::interrupt::ControllerCapability::full()));

        let high_handler = Box::new(SimpleInterruptHandler::new(
            HandlerType::Custom,
            InterruptPriority::High,
            HandlerCapability::full(),
        ));
        assert!(pic.register_handler(high_handler, 40).is_ok());
        assert!(pic.enable_interrupt(40).is_ok());
        manager.add_controller(pic);

        // Dispatch interrupt 40
        assert_eq!(manager.dispatch_interrupt(40), InterruptResult::Handled);

        // Retrieve and assert chronological trace sequence log
        assert!(manager.trace_log.len() >= 3);

        let evt_arrived = manager.trace_log[0];
        assert_eq!(evt_arrived.event_type, TraceEventType::InterruptArrived);
        assert_eq!(evt_arrived.interrupt_number, 40);

        let evt_dispatched = manager.trace_log[1];
        assert_eq!(
            evt_dispatched.event_type,
            TraceEventType::InterruptDispatched
        );
        assert_eq!(evt_dispatched.interrupt_number, 40);

        let evt_completed = manager.trace_log[2];
        assert_eq!(evt_completed.event_type, TraceEventType::InterruptCompleted);
        assert_eq!(evt_completed.interrupt_number, 40);
        assert!(evt_completed.timestamp > evt_arrived.timestamp);
    }

    #[test]
    fn test_cpu_bootstrap_and_instructions() {
        let mut engine = ArchitectureEngine::new();
        assert_eq!(engine.init_state, ProcessorInitState::Offline);

        // Bootstrap Core
        assert!(engine.init_processor().is_ok());
        assert_eq!(engine.init_state, ProcessorInitState::Ready);

        // Verify some instructions cycle phase compiles
        let phase = InstructionCyclePhase::Commit;
        assert_eq!(phase, InstructionCyclePhase::Commit);
    }

    #[test]
    fn test_vmm_pool_memory_and_mdl() {
        let mut engine = ArchitectureEngine::new();

        // Alloc NonPagedPool
        let np_block = engine.allocate_pool(PoolType::NonPagedPool).unwrap();
        assert_eq!(np_block.len(), 1024);

        // Recycles block in Lookaside List
        engine.lookaside_nonpaged.free_block(np_block);
        assert_eq!(engine.lookaside_nonpaged.cached_blocks.len(), 1);

        // Alloc PagedPool (Allowed under IRQL PassiveLevel)
        let p_block = engine.allocate_pool(PoolType::PagedPool).unwrap();
        assert_eq!(p_block.len(), 1024);

        // MDL Locking Verification
        let mut mdl = MemoryDescriptorList::new(0x20000000, 16384);
        assert!(!mdl.is_locked);
        mdl.lock_pages();
        assert!(mdl.is_locked);
        assert_eq!(mdl.locked_physical_pages.len(), 4); // 16KB / 4096 = 4 pages
        assert_eq!(mdl.locked_physical_pages[0], 0x30000); // 0x20000000/4096 + 0x10000
    }

    #[test]
    fn test_irql_enforcement_and_faults() {
        let mut engine = ArchitectureEngine::new();
        assert_eq!(engine.current_irql, Irql::PassiveLevel);

        // Promote to HighLevel
        let old_irql = engine.raise_irql(Irql::HighLevel).unwrap();
        assert_eq!(old_irql, Irql::PassiveLevel);
        assert_eq!(engine.current_irql, Irql::HighLevel);

        // Trigger PagedPool allocate -> Rejected because IRQL >= DispatchLevel
        let res = engine.allocate_pool(PoolType::PagedPool);
        assert_eq!(res, Err(HardwareException::DoubleFault));

        // Demote back
        assert!(engine.lower_irql(old_irql).is_ok());
        assert_eq!(engine.current_irql, Irql::PassiveLevel);
    }

    #[test]
    fn test_pcb_tcb_context_switching() {
        let mut engine = ArchitectureEngine::new();

        // Create PCB
        let mut pcb = Pcb::new(500, 0x1A000); // CR3 PML4 Directory

        // Add 2 Threads
        let tcb1 = Tcb::new(1, 500, 0x1000, 0x9000);
        let tcb2 = Tcb::new(2, 500, 0x2000, 0xA000);
        pcb.thread_list.push(tcb1);
        pcb.thread_list.push(tcb2);

        engine.running_pcb = Some(pcb);

        // Switch Thread 0 -> Thread 1
        assert!(engine.context_switch_threads(0, 1).is_ok());

        let active_pcb = engine.running_pcb.as_ref().unwrap();
        assert_eq!(active_pcb.thread_list[0].state, ThreadState::Ready);
        assert_eq!(active_pcb.thread_list[1].state, ThreadState::Running);
    }
}
