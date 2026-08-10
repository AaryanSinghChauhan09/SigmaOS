// SigmaOS Master Integration Tests
// Sequentially orchestrates, verifies, and asserts the correct behavioral execution of ALL custom systems together

use sigmaos::filesystem::sigma_fs::*;
use sigmaos::kernel::scheduler::*;
use sigmaos::package::store::*;
use sigmaos::kernel::breakthroughs::*;
use sigmaos::security::CapabilityToken;
use sigmaos::distro::*;
use sigmaos::network::*;
use sigmaos::drivers::*;

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

        // 5. Arch Linux Parity (ALPM, PKGBUILD & AUR Client) Integration
        let aur_client = AurClient::new();
        let compiler = SandboxedCompiler::new();
        let mut alpm_db = AlpmDatabase::new();
        assert!(aur_client.download_and_compile_aur_package("yay-pqc", &compiler, &mut alpm_db).is_ok());
        assert!(alpm_db.get_package("yay-pqc").is_some());

        // 6. OpenBSD Packet Filter (PF) Stateful Firewall Integration
        let pf = OpenBsdPacketFilter::new();
        let rule = FilterRule {
            id: 1,
            action: FilterAction::Block,
            direction: TrafficDirection::In,
            interface: "em0".to_string(),
            proto: "tcp".to_string(),
            src_ip: "192.168.1.100".to_string(),
            dst_ip: "*".to_string(),
            src_port: None,
            dst_port: Some(80),
        };
        pf.load_ruleset(vec![rule]);

        // Stateful packet checking - rule says block
        let action = pf.check_packet(TrafficDirection::In, "em0", "tcp", "192.168.1.100", "10.0.0.1", 1234, 80);
        assert_eq!(action, FilterAction::Block);

        // Packet with stateful pass (e.g. established connection on port 443)
        let action_pass = pf.check_packet(TrafficDirection::In, "em0", "tcp", "192.168.1.50", "10.0.0.1", 1234, 443);
        assert_eq!(action_pass, FilterAction::Pass);

        // 7. Dial-up 56K Modem Legacy Peripheral Driver Integration
        let mut modem = DialupModemDriver::new();
        modem.initialize().unwrap();
        assert_eq!(modem.name(), "U.S. Robotics 56K Dial-up Faxmodem");
        assert_eq!(modem.generation(), DeviceGeneration::Legacy);

        // Write ATDT command to dial
        modem.write(b"ATDT 555-0199\r").unwrap();
        assert!(modem.is_connected);

        // Read response buffer
        let mut resp = [0u8; 32];
        let bytes_read = modem.read(&mut resp).unwrap();
        assert_eq!(&resp[..bytes_read], b"CONNECT 56000\r\n");

        // Hang up (ATH)
        modem.write(b"ATH\r").unwrap();
        assert!(!modem.is_connected);

        // 8. Debian-style Automated Preseed Installer (S-Preseed) Integration
        let preseed = SovereignPreseedParser::new();
        let preseed_content = r#"
            # S-Preseed config
            d-i passwd/user-fullname string Sovereign User
            d-i netcfg/get_hostname string sigmaos-node
            d-i pkgsel/include string nginx curl git
        "#;
        let count = preseed.parse_preseed_content(preseed_content);
        assert_eq!(count, 3);

        assert_eq!(preseed.get_value("passwd", "user-fullname").unwrap(), "Sovereign User");
        assert_eq!(preseed.get_value("netcfg", "get_hostname").unwrap(), "sigmaos-node");
        assert_eq!(preseed.get_value("pkgsel", "include").unwrap(), "nginx curl git");

        assert!(preseed.execute_automated_installation());
    }
}
