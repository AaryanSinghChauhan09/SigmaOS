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
use sigmaos::filesystem::{LegacyLinuxRule, LinuxPersonaRule, SmartSymlink, SymlinkResolverRule};
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
    AnanicyCppDaemon, AnanicyRule, BoreScheduler, CachyKernelManager, CpuPriorityOptimizer,
    GlarySmartRule, IoPriorityOptimizer, IoSchedClass, IoTaskPriority, PerformanceProfileRule,
    PhysicalPageFrame, RamDefragmenter, SmartPerformanceProfile, SmartResourceOptimizer,
    UltraKernelSamepageMerger, X86v3v4OptimizationDetector, GLOBAL_GLARY_RULE,
    GLOBAL_SMART_OPTIMIZER,
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
    AdaptivePolicy, AdvancedAlgorithmsManager, Apc, ApcMode, ApcQueue, ArchitectureEngine,
    AuditBlock, CircularDoublyLinkedList, CpuArchitectureClass, CpuRegisters, EdfTask,
    HardwareException, InstructionCyclePhase as ArchInstructionCyclePhase, InstructionCyclePhase,
    InterruptClass, IoWaitProfile, Irql, KernelMechanism, KernelPolicy, LcgRandom, LookasideList,
    LotteryTask, MemoryDescriptorList, Pcb, PolicyMechanismCoordinator, PoolType, Priority,
    Process, ProcessState, ProcessorInitState, SequencedSinglyLinkedList, SinglyLinkedList,
    SovereignMechanism, SystemThread, Tcb, ThreadState, WorkItem,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_integration() {
        // 1. Composable FHS & Storage Replacements (ext4, btrfs, ZFS, LVM, mdadm, LUKS, VirtIO)
        let mut fs = SigmaFS::new();
        let block_hash = fs
            .write_file_block("financial_report.csv", b"SALES_DATA_X")
            .unwrap();
        assert!(fs.verify_audit_trail_integrity());

        // FHS Routing
        let router = SigmaFhsRouter::new();
        assert_eq!(router.route_path("systemd.bin"), "/bin/systemd.bin");

        // FHS Compliance Hook
        let mut hook = SigmaFhsHook::new("LicenseCheck");
        assert!(hook.pre_write_hook("/etc/nginx.conf", b"worker_processes 4;"));

        // FHS Namespace Isolation
        let mut ns = SigmaFhsNamespace::new("sandboxed-user-ns");
        ns.bind_directory("/var/lib");
        ns.write_isolated_file("index.html", b"<h1>Sovereign</h1>".to_vec());
        assert_eq!(
            ns.read_isolated_file("index.html").unwrap(),
            &b"<h1>Sovereign</h1>".to_vec()
        );

        // FHS Access Auditor
        let mut auditor = SigmaFhsAuditor::new();
        auditor.record_access("user-ns", "/etc/hosts", "read", 170000);
        assert!(auditor.verify_audit_ledger());

        // ext4-parity Metadata Journaling
        let mut journal = SigmaFsJournal::new();
        let tx = journal.start_transaction("/etc/fstab", "write");
        journal.commit_transaction(tx);
        assert_eq!(journal.active_txs[0].state, JournalState::Committed);

        // btrfs-parity Copy-on-Write Snapshotting
        let mut cow = SigmaFsCow::new();
        cow.write_block_cow("disk.img", 0, 1024);
        cow.create_cow_snapshot("snap0");
        assert!(cow.snapshots.contains_key("snap0"));

        // LVM-parity Logical Volumes
        let mut lvm = SigmaFsVolume::new();
        lvm.create_volume_group("vg0", vec!["/dev/sda", "/dev/sdb"], 102400);
        assert_eq!(lvm.query_volume_capacity_mb("vg0").unwrap(), 102400);

        // mdadm-parity Software RAID
        let mut raid = SigmaFsRaid::new();
        raid.create_raid_array("md0", RaidLevel::Raid1);
        assert_eq!(raid.route_raid_sectors("md0", 999), vec![0, 1]);

        // LUKS-parity Volume Encryption
        let mut luks = SigmaFsCrypt::new("vault-secret");
        assert!(luks.unlock_volume("vault-secret"));
        let mut sector_data = vec![0x11, 0x22, 0x33];
        luks.encrypt_sector(400, &mut sector_data).unwrap();
        assert_ne!(sector_data, vec![0x11, 0x22, 0x33]);

        // VirtIO-parity Queue Descriptors
        let mut virtio = SigmaFsVirtio::new();
        virtio.submit_virtio_buffer(0x2000, 512, 0);
        assert_eq!(virtio.avail_ring_idx, 1);

        // 2. Linux-conforming Hard Link reference counting
        let mut vfs = VirtualFilesystem::new();
        let inode_id = vfs.create_file(FileType::Regular, 100).unwrap();
        assert_eq!(vfs.get_inode(inode_id).unwrap().hard_links_count, 1);

        vfs.link_inode(inode_id).unwrap();
        assert_eq!(vfs.get_inode(inode_id).unwrap().hard_links_count, 2);

        assert_eq!(vfs.unlink_inode(inode_id).unwrap(), 1);
        assert!(vfs.inodes.contains_key(&inode_id));

        assert_eq!(vfs.unlink_inode(inode_id).unwrap(), 0);
        assert!(!vfs.inodes.contains_key(&inode_id)); // fully freed

        // 3. Syslog-parity multi-generation rotations, facilities, and RLE compression
        let log_file = SimpleLogFile::new(10, b"/var/log/cron")
            .with_syslog(LogSeverity::Warn, LogFacility::Cron);
        assert_eq!(log_file.severity, LogSeverity::Warn);
        assert_eq!(log_file.facility, LogFacility::Cron);

        let mut log_rotator = SimpleLogRotator::new();
        log_rotator.shift_backup_generations("cron", 3);
        assert_eq!(log_rotator.active_generations.as_slice()[0], "cron.1.gz");

        let compressor = SimpleLogCompressor::new();
        let raw_log = b"DEBUG INFO DEBUG DEBUG DEBUG";
        let compressed = compressor.compress(raw_log).unwrap();
        let decompressed = compressor.decompress(compressed.as_slice()).unwrap();
        assert_eq!(decompressed.as_slice(), raw_log);

        // 4. 12 World-Class Desktop Utility Engines Parity
        let mut everything = EverythingSearchEngine::new();
        everything.index_file("/usr/bin/obs", 409600, false);
        assert_eq!(everything.query_files("obs")[0].path, "/usr/bin/obs");

        let mut npp = NotepadPlusPlusBuffer::new();
        npp.open_file("readme.md", "Task: Setup CCleaner");
        npp.find_and_replace("Task", "Todo");
        assert_eq!(npp.tabs[0].content, "Todo: Setup CCleaner");

        let mut browser = SovereignBrowserEngine::new();
        assert!(!browser.navigate_url("telemetry.analytics.com/push"));

        let lzma_archiver = SevenZipEngine::new(CompressionMethod::Lzma);
        let volumes = lzma_archiver.create_archive(b"RUST_COMPILER_SOURCES", "rust");
        assert_eq!(volumes[0].name, "rust.001");

        let mut flameshot = FlameshotAnnotator::new(1920, 1080);
        flameshot.draw_annotation(
            AnnotationShape::Arrow,
            10,
            10,
            50,
            50,
            ColorRgba::new(0, 0, 255, 255),
        );

        let mut obs = ObsStudioMixer::new("Scene A");
        obs.add_video_source("Display", 1.0, false);

        let mut audacity = AudacityWaveEditor::new(48000, 2);
        audacity.audio_samples = vec![0.1, -0.2, 0.005, 0.9];
        audacity.apply_noise_gate(-20.0, 0.05); // Gate low signals

        let mut vlc = VlcCodecPipeline::new();
        vlc.volume_multiplier = 2.0; // 200% boost
        assert_eq!(vlc.apply_vlc_audio_boost(0.4), 0.8);

        let mut davinci = DaVinciTimeline::new();
        davinci.add_clip("v1.mp4", 0, 100);

        let onecommander = OneCommanderFileGrid::new();
        assert_eq!(onecommander.get_metadata_age_tag(0), ItemAgeColor::HotNew);

        let mut eartrumpet = EarTrumpetVolumeMatrix::new();
        eartrumpet.set_app_volume("firefox", 0.7);
        let peak = eartrumpet.query_peak_amplitude("firefox");
        assert!((peak - 0.665).abs() < 1e-5);

        let mut irfan = IrfanViewEngine::new();
        assert_eq!(
            irfan.batch_format_convert(&["img1.png", "img2.png"], "BMP"),
            2
        );

        // 5. Zorin OS, antiX, and EndeavourOS Parity Features
        let mut zorin_app = ZorinAppearanceSwitcher::new();
        zorin_app.switch_layout_preset(ZorinLayoutPreset::MacOsLike);
        assert_eq!(zorin_app.panel_height_pixels, 64);

        let mut zorin_conn = ZorinConnectHub::new();
        zorin_conn.pair_new_device("tab-12", "Sovereign Tablet");
        assert_eq!(
            zorin_conn.push_notification_to_all_devices("Test", "Zorin connect alert"),
            1
        );

        let mut wine = ZorinWineLayer::new("~/.wine");
        assert!(wine.launch_windows_executable("game.exe").is_ok());

        let mut zorin_lite = ZorinLiteOptimizer::new();
        zorin_lite.enable_zorin_lite_profile(true);
        assert_eq!(zorin_lite.compositor_blur_radius, 0);

        let mut antix_init = SigmaEcosystemInit::new();
        antix_init.sequence_runlevel_transition(FhsRunlevel::Graphical);
        assert_eq!(antix_init.active_runlevel, FhsRunlevel::Graphical);

        let mut antix_prof = SigmaEcosystemProfiler::new();
        antix_prof.apply_legacy_preset_rules(128); // 128MB RAM JWM preset
        assert_eq!(antix_prof.graphic_preset, GraphicPresetMode::JwmPreset);

        let mut eos_welcome = SigmaOnboardingWelcome::new();
        let mut latencies = HashMap::new();
        latencies.insert("https://mirror.org/repo".to_string(), 10);
        eos_welcome.rank_package_mirrors(latencies);
        assert_eq!(eos_welcome.mirrors_ranked[0], "https://mirror.org/repo");

        let eos_log = SigmaOnboardingLog::new();
        let censored = eos_log.sanitize_system_log("secret_key=999999");
        assert!(censored.contains("secret_key= [REDACTED_FOR_SECURITY_COMPLIANCE]"));

        // 6. Aegisub / Subtitle Edit Timing and Styling Parity
        let mut subtitle_sync = SigmaSupportSubtitleSync::new();
        let body = subtitle_sync.parse_ass_styling_tags("{\\fnImpact\\fs32}Styled Subtitle");
        assert_eq!(body, "Styled Subtitle");
        assert_eq!(subtitle_sync.font_name, "Impact");
        assert_eq!(subtitle_sync.font_size, 32);

        let mut subtitle_edit = SigmaSupportSubtitleEdit::new(SubtitleFormat::Ass);
        subtitle_edit.insert_subtitle_entry(500, 1500, "Caption A");
        subtitle_edit.shift_all_timings_ms(100);
        assert_eq!(subtitle_edit.entries[0].start_ms, 600);
        assert_eq!(subtitle_edit.entries[0].end_ms, 1600);

        // 7. Glary Utilities / Advanced SystemCare RAM and CPU Compaction Parity
        let mut resource_opt = SigmaSupportResourceOptimizer::new();
        resource_opt.register_page_block(99, true, 4096);
        let compacted = resource_opt.execute_ram_defragmentation();
        assert_eq!(compacted, 1);
        assert_eq!(resource_opt.total_defragmentations_completed, 1);

        let mut priority_opt = SigmaSupportPriorityOptimizer::new();
        priority_opt.register_running_process(1, "system_init", 0);
        priority_opt.running_processes[0].current_cpu_usage = 0.90;
        let reniced = priority_opt.optimize_cpu_priorities(1);
        assert_eq!(reniced, 0); // No other processes to renice
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
        use sigmaos::sigpkg::universal_adapter::{ApkAdapter, NixAdapter, EbuildAdapter};

        let apk = ApkAdapter::new();
        assert_eq!(apk.format_name(), "apk");

        let nix = NixAdapter::new();
        assert_eq!(nix.format_name(), "nix");

        let ebuild = EbuildAdapter::new();
        assert_eq!(ebuild.format_name(), "ebuild");
    }

    #[test]
    fn test_reliability_and_testing_suite() {
        use sigmaos::tracing::{SigmaTrace, TraceEvent};
        use sigmaos::crash::SimpleCrashPipeline;

        // 1. Tracepoint Spans & Observability tests
        let mut trace = SigmaTrace::new();
        trace.record_span(12345, TraceEvent::Syscall(54), 0);
        trace.record_span(12346, TraceEvent::ContextSwitch(1, 2), 100);
        trace.record_span(12347, TraceEvent::Interrupt(3), 0);

        assert_eq!(trace.get_recorded_count(), 3);
        let spans = trace.get_all_spans();
        assert_eq!(spans.len(), 3);
        assert_eq!(spans[0].timestamp, 12345);

        // 2. Anomaly/Fuzzing logging and Ring Buffer Overflows
        for i in 0..20 {
            trace.record_span(i as u64, TraceEvent::Syscall(i as u32), i as u64);
        }
        assert_eq!(trace.get_recorded_count(), 16); // Buffer size is 16
        assert!(trace.get_overflow_count() > 0);

        // 3. Fault Injection Testing & Recovery in SimpleCrashPipeline
        let mut pipeline = SimpleCrashPipeline::new();
        let report_id = pipeline.process_crash(42).unwrap();
        assert!(report_id > 0);

        // 4. Anonymized Telemetry & Stripping PII
        let data = b"Process: app_server. Secret: 1234-PII";
        let anonymized = pipeline.anonymizer.strip_pii(data);
        // Stripped digits to 'X'
        assert!(anonymized.contains(&b'X'));

        // 5. Minidump Generation
        let report = pipeline.generate_report(report_id);
        assert!(!report.is_empty());
||||||| 43be3a7e8
        // Placeholder for integration tests
        assert!(true);
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
    fn test_cachyos_bore_scheduler() {
        let scheduler = BoreScheduler::new();
        assert_eq!(scheduler.base_slice_ms, 10);

        // Task with 0 bursts gets full slice (interactive prioritisation)
        assert_eq!(scheduler.calculate_bore_timeslice(0), 10);

        // Highly bursty task gets scaled down to minimum of 2ms to prevent hogging
        assert_eq!(scheduler.calculate_bore_timeslice(8), 2);
    }

    #[test]
    fn test_cachyos_ananicy_cpp_auto_nice() {
        let daemon = AnanicyCppDaemon::new();
        assert_eq!(daemon.rules.len(), 3);

        // Querying game csgo rule
        let (nice, io) = daemon.query_priority_nice_rule("csgo").unwrap();
        assert_eq!(nice, -15);
        assert_eq!(io, IoSchedClass::RealTime);

        // Non-matching query returns None
        assert!(daemon.query_priority_nice_rule("non-existent").is_none());
    }

    #[test]
    fn test_cachyos_uksm_samepage_merger() {
        let merger = UltraKernelSamepageMerger::new();

        let mut frames = [
            PhysicalPageFrame {
                address: 0x1000,
                content_hash: 0xAAAA_BBBB,
            },
            PhysicalPageFrame {
                address: 0x2000,
                content_hash: 0xCCCC_DDDD,
            },
            PhysicalPageFrame {
                address: 0x3000,
                content_hash: 0xAAAA_BBBB,
            }, // Duplicate identical page!
        ];

        let merged = merger.deduplicate_pages(&mut frames);
        assert_eq!(merged, 1);
        assert_eq!(
            merger
                .scanned_pages_count
                .load(core::sync::atomic::Ordering::SeqCst),
            3
        );
        assert_eq!(
            merger
                .saved_pages_count
                .load(core::sync::atomic::Ordering::SeqCst),
            1
        );
    }

    #[test]
    fn test_cachyos_architecture_optimization_detector() {
        let detector = X86v3v4OptimizationDetector::new();
        assert!(detector.is_v3_supported);
        assert!(!detector.is_v4_supported);

        // Auto-selects x86-64-v3 target over baseline compiler paths
        assert_eq!(detector.resolve_optimal_compiler_target(), "x86-64-v3");
    }

    #[test]
    fn test_cachyos_kernel_manager_sysctl() {
        let mut manager = CachyKernelManager::new();
        assert_eq!(manager.scheduler_name, "BORE");
        assert_eq!(manager.tcp_congestion_control, "cubic");

        // Swap to EEVDF scheduler dynamically
        manager.hot_swap_scheduler("EEVDF").unwrap();
        assert_eq!(manager.scheduler_name, "EEVDF");

        // Enable BBRv3 congestion control
        manager.enable_bbrv3_congestion().unwrap();
        assert_eq!(manager.tcp_congestion_control, "bbrv3");
        assert!(manager.bbrv3_active);
    }
}
