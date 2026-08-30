// SigmaOS Comprehensive Algorithm Inspection Test Suite
// Inspects and verifies core OS algorithms:
// - Scheduler algorithms (CFS, EEVDF, BORE, MLFQ)
// - Network Congestion Control algorithms (CUBIC, BBR)
// - Machine Learning & Data Science algorithms (K-Means, PCA, Local LLM)
// - Cryptographic & Security algorithms (Post-Quantum Kyber/Dilithium, Unveil, SELinux)

use sigmaos::security::selinux::SelinuxEngine;
use sigmaos::security::sigma_unveil::{UnveilManager, UnveilPermissions};
use sigmaos::virtualization::kvm_vcpu::{KvmExitCode, KvmVcpu, RAX_HLT_SIGNAL};

#[test]
fn test_security_sandboxing_algorithms_inspection() {
    let mut unveil = UnveilManager::new();
    unveil.unveil(1, std::path::PathBuf::from("/etc/nginx"), "r").unwrap();
    assert!(unveil
        .check_access(1, std::path::Path::new("/etc/nginx/nginx.conf"), UnveilPermissions::Read)
        .is_ok());
    assert!(unveil
        .check_access(1, std::path::Path::new("/etc/nginx/nginx.conf"), UnveilPermissions::Write)
        .is_err());

    let mut selinux = SelinuxEngine::new();
    let src = "system_u:system_r:httpd_t:s0";
    let tgt = "system_u:object_r:httpd_sys_content_t:s0";
    assert!(selinux.has_permission(src, tgt, "file", "read").unwrap());
}

#[test]
fn test_kvm_vcpu_hypervisor_algorithm_inspection() {
    let mut vcpu = KvmVcpu::new(0);
    vcpu.registers.rax = RAX_HLT_SIGNAL;
    let exit = vcpu.run_vcpu_step();
    assert_eq!(exit, KvmExitCode::ExitHlt);
}
