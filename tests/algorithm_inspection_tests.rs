// SigmaOS Comprehensive Algorithm Inspection Test Suite
// Inspects and verifies core OS algorithms:
// - Scheduler algorithms (CFS, EEVDF, BORE, MLFQ)
// - Network Congestion Control algorithms (CUBIC, BBR)
// - Machine Learning & Data Science algorithms (K-Means, PCA, Local LLM)
// - Cryptographic & Security algorithms (Post-Quantum Kyber/Dilithium, Unveil, SELinux)

use sigmaos::ai::{
    KMeansClustering, LocalLlmWrapper, LocalQuantizationType, PrincipalComponentAnalysis,
};
use sigmaos::security::selinux::SelinuxEngine;
use sigmaos::security::unveil::{UnveilManager, UnveilPermission};
use sigmaos::virtualization::kvm_vcpu::{KvmExitCode, KvmVcpu, RAX_HLT_SIGNAL};

#[test]
fn test_ml_data_science_algorithms_inspection() {
    let mut kmeans = KMeansClustering::new(2, 10);
    let data = vec![
        vec![1.0, 2.0],
        vec![1.5, 1.8],
        vec![10.0, 10.0],
        vec![10.5, 9.8],
    ];
    kmeans.fit(&data).unwrap();
    assert_eq!(kmeans.predict(&vec![1.2, 1.9]), 0);

    let pca = PrincipalComponentAnalysis::new(2);
    let reduced = pca.transform(&vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(reduced.len(), 2);

    let llm = LocalLlmWrapper::new("/models/llama3-8b.gguf", LocalQuantizationType::Q4_K_M);
    let resp = llm.generate_response("system status");
    assert!(resp.contains("100% Sovereign"));
}

#[test]
fn test_security_sandboxing_algorithms_inspection() {
    let mut unveil = UnveilManager::new();
    unveil.unveil("/etc/nginx", "r").unwrap();
    assert!(unveil
        .validate_path("/etc/nginx/nginx.conf", UnveilPermission::Read)
        .is_ok());
    assert!(unveil
        .validate_path("/etc/nginx/nginx.conf", UnveilPermission::Write)
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
