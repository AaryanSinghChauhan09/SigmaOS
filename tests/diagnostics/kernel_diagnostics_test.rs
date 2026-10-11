use sigmaos::diagnostics::{
    KernelBootDiagnosticReport, KernelPanicDiagnosticReport, SubsystemBootStatus,
    SimpleCrashPipeline, PipelineCapability, CrashSeverity, CrashPipeline,
};

#[test]
fn test_kernel_boot_diagnostics_integration() {
    let mut boot_report = KernelBootDiagnosticReport::new("SigmaOS 1.0-Sovereign");
    boot_report.record_subsystem("firmware", SubsystemBootStatus::Success, 100, "firmware");
    boot_report.record_subsystem("bootloader", SubsystemBootStatus::Success, 200, "bootloader");
    boot_report.record_subsystem("kernel_entry", SubsystemBootStatus::Success, 150, "kernel_entry");
    boot_report.record_subsystem("arch_init", SubsystemBootStatus::Success, 300, "arch_init");
    boot_report.record_subsystem("memory_init", SubsystemBootStatus::Success, 500, "memory_init");
    boot_report.record_subsystem("interrupt_init", SubsystemBootStatus::Success, 250, "interrupt_init");
    boot_report.record_subsystem("scheduler_init", SubsystemBootStatus::Success, 400, "scheduler_init");
    boot_report.record_subsystem("vfs_init", SubsystemBootStatus::Success, 600, "vfs_init");
    boot_report.record_subsystem("driver_init", SubsystemBootStatus::Success, 800, "driver_init");
    boot_report.record_subsystem("userspace_start", SubsystemBootStatus::Success, 700, "userspace_start");
    boot_report.record_subsystem("shell_ready", SubsystemBootStatus::Success, 100, "shell_ready");

    assert!(boot_report.overall_success);
    assert_eq!(boot_report.results.len(), 11);

    let log = boot_report.generate_greppable_log();
    assert!(log.contains("[BOOT_REPORT] os=SigmaOS 1.0-Sovereign success=true"));
    assert!(log.contains("stage=shell_ready name=shell_ready status=SUCCESS"));
}

#[test]
fn test_kernel_panic_diagnostics_integration() {
    let mut panic_report = KernelPanicDiagnosticReport::new("scheduler", "context_switch", "Invalid thread state");
    panic_report.fault_address = Some(0xDEADBEEF);
    panic_report.set_register("RAX", 0x12345678);
    panic_report.set_register("RSP", 0x7FFFFFFF0000);
    panic_report.add_stack_frame(0xFFFFFFFF80001000);

    let formatted = panic_report.generate_stable_report();
    assert!(formatted.contains("Component: scheduler"));
    assert!(formatted.contains("Operation: context_switch"));
    assert!(formatted.contains("Message: Invalid thread state"));
    assert!(formatted.contains("Fault Address: 0x00000000deadbeef"));
    assert!(formatted.contains("RAX: 0x0000000012345678"));
}

#[test]
fn test_crash_pipeline_integration() {
    let mut pipeline = SimpleCrashPipeline::new(PipelineCapability::full());
    let id = pipeline.create_report(b"kernel_task", CrashSeverity::Fatal).expect("Failed to create report");
    let stats = pipeline.stats();
    assert_eq!(stats.total_reports, 1);
    assert!(pipeline.get_report(id).is_some());
}
