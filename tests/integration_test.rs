// SigmaOS Master Integration Tests
// Sequentially orchestrates, verifies, and asserts the correct behavioral execution of ALL custom systems together

use sigmaos::filesystem::sigma_fs::*;
use sigmaos::kernel::scheduler::*;
use sigmaos::package::store::*;
use sigmaos::kernel::breakthroughs::*;
use sigmaos::security::CapabilityToken;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_integration() {
        // 1. Filesystem (SigmaFS) & VFS Integration
        let mut fs = SigmaFS::new();
        let block_hash = fs.write_file_block("financial_report.csv", b"SALES_DATA_X").unwrap();
        assert!(fs.verify_audit_trail_integrity());

        // 2. Hybrid Scheduler Integration
        let mut scheduler = Scheduler::new();
        let p_normal = Process::new(1, "normal".to_string(), Priority::Normal);
        let p_rt = Process::new(2, "realtime".to_string(), Priority::Realtime).with_edf(50);
        scheduler.add_process(p_normal);
        scheduler.add_process(p_rt);
        scheduler.tick();

        let chosen = scheduler.schedule().unwrap();
        assert_eq!(chosen.pid, 2); // EDF prioritized

        // 3. Package Software Store Safety Check
        let mut store = SigmaSoftwareStore::new();
        assert!(store.install_app("sigma-paint").is_ok());
        assert_eq!(store.check_for_updates(), 1);

        // 4. Breakthrough Tools Validation
        let translator = UniversalAbiTranslator::new("SigmaOS");
        let win_sys = translator.translate_abi_syscall("Windows", 0x2A).unwrap();
        assert_eq!(win_sys, "sys_win32_create_window");

        let self_healing = SelfHealingKernel::new(0xABCDEF);
        assert!(self_healing.verify_and_heal(0xABCDEF).is_ok());

        let sandbox = PrivacyFirstSandbox::new();
        let token = CapabilityToken::from_bits(0x0F);
        assert!(sandbox.validate_and_execute_secure_call(&token, 0x0C));
    }
}
