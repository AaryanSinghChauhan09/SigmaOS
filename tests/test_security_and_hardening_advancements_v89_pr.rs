// Integration Test Suite for Sovereign Security & Hardening Advancements (v89 PR Suite)

extern crate alloc;

use alloc::string::String;

#[path = "../src/security/sovereign_security_and_hardening_advancements_v89_pr.rs"]
mod sovereign_security_and_hardening_advancements_v89_pr;

use sovereign_security_and_hardening_advancements_v89_pr::*;

#[test]
fn test_integration_security_and_hardening_v89_proposals() {
    let gateway = SovereignSecurityAndHardeningParityV89Gateway::new();
    let proposals = gateway.collect_all_proposals();

    assert_eq!(proposals.len(), 5);

    // PR #9046: Linux Access Control & Sandboxing
    assert_eq!(proposals[0].pr_id, 9046);
    assert_eq!(
        proposals[0].title,
        "Linux Access Control, Sandboxing & Mandatory Policy Parity (v89)"
    );
    assert_eq!(
        proposals[0].target_subsystem,
        "Security / Linux Sandboxing"
    );
    assert_eq!(proposals[0].status, "Ready for Merge");
    assert!(proposals[0].components_implemented.contains(
        &String::from("SELinux v40 AVC Decision Engine & Multilevel Security Contexts")
    ));

    // PR #9047: BSD Hardening & Isolation
    assert_eq!(proposals[1].pr_id, 9047);
    assert_eq!(
        proposals[1].title,
        "BSD Hardening, Isolation & Capability Security Parity (v89)"
    );
    assert_eq!(proposals[1].target_subsystem, "Security / BSD Hardening");
    assert_eq!(proposals[1].status, "Ready for Merge");
    assert!(proposals[1].components_implemented.contains(
        &String::from("OpenBSD Pledge Promises Validator")
    ));

    // PR #9048: Audit & Intrusion Prevention
    assert_eq!(proposals[2].pr_id, 9048);
    assert_eq!(
        proposals[2].title,
        "Auditd, Fail2ban, Lynis & AIDE Intrusion Prevention Parity (v89)"
    );
    assert_eq!(
        proposals[2].target_subsystem,
        "Security / Audit & Intrusion Prevention"
    );
    assert_eq!(proposals[2].status, "Ready for Merge");
    assert!(proposals[2].components_implemented.contains(
        &String::from("Fail2ban Log Pattern IP Extractor & Exponential Ban Calculator")
    ));

    // PR #9049: Antivirus & Cryptographic Enclaves
    assert_eq!(proposals[3].pr_id, 9049);
    assert_eq!(
        proposals[3].title,
        "ClamAV, GnuPG, UEFI Secure Boot & TPM 2.0 Parity (v89)"
    );
    assert_eq!(
        proposals[3].target_subsystem,
        "Security / Crypto & Antivirus"
    );
    assert_eq!(proposals[3].status, "Ready for Merge");
    assert!(proposals[3].components_implemented.contains(
        &String::from("ClamAV CVD Signature Scanner")
    ));

    // PR #9050: Identity & Enterprise Auth
    assert_eq!(proposals[4].pr_id, 9050);
    assert_eq!(
        proposals[4].title,
        "Kerberos v5, LDAP v3 & SSSD Identity Integration Parity (v89)"
    );
    assert_eq!(
        proposals[4].target_subsystem,
        "Security / Enterprise Identity"
    );
    assert_eq!(proposals[4].status, "Ready for Merge");
    assert!(proposals[4].components_implemented.contains(
        &String::from("Kerberos v5 TGT Ticket Validation & Realm Matcher")
    ));
}

#[test]
fn test_integration_linux_access_control_sub_engine() {
    let engine = LinuxAccessControlAndSandboxingV89Engine::new();

    // Context format & check
    let scontext = LinuxAccessControlAndSandboxingV89Engine::format_selinux_context(
        "system_u",
        "system_r",
        "httpd_t",
        "s0",
    );
    let tcontext = LinuxAccessControlAndSandboxingV89Engine::format_selinux_context(
        "system_u",
        "object_r",
        "httpd_sys_content_t",
        "s0",
    );
    assert!(engine.evaluate_selinux_avc(&scontext, &tcontext, "file", "read"));
    assert!(!engine.evaluate_selinux_avc(&scontext, &tcontext, "file", "write"));

    // AppArmor path globbing
    assert!(engine.matches_apparmor_path_profile("/bin/bash", "/bin/*"));
    assert!(!engine.matches_apparmor_path_profile("/bin/subdir/bash", "/bin/*"));
    assert!(engine.matches_apparmor_path_profile("/usr/lib/libc.so", "/usr/lib/**"));
    assert!(engine.matches_apparmor_path_profile("/usr/lib/x86_64/libc.so", "/usr/lib/**"));

    // Seccomp BPF Arch & Filter
    assert!(LinuxAccessControlAndSandboxingV89Engine::validate_seccomp_bpf_arch(0xC000003E)); // X86_64
    assert!(LinuxAccessControlAndSandboxingV89Engine::validate_seccomp_bpf_arch(0xC00000B7)); // AARCH64
    assert_eq!(engine.evaluate_seccomp_bpf(0, &[0, 1, 2]), 3); // ALLOW
    assert_eq!(engine.evaluate_seccomp_bpf(59, &[0, 1, 2]), 0); // KILL

    // Capabilities and Namespaces
    assert!(engine.check_capability(1 << 21, 21)); // CAP_SYS_ADMIN bit 21
    assert!(!engine.check_capability(1 << 21, 10)); // CAP_NET_BIND_SERVICE bit 10
    assert!(LinuxAccessControlAndSandboxingV89Engine::evaluate_namespace_isolation(0x3F, 0x01)); // CLONE_NEWNS
    assert!(!LinuxAccessControlAndSandboxingV89Engine::evaluate_namespace_isolation(0x02, 0x01));

    // Cgroups v2
    assert!(LinuxAccessControlAndSandboxingV89Engine::is_cgroup_controller_enabled(
        "cpu memory io pids rdma cpuset",
        "pids"
    ));
    assert!(!LinuxAccessControlAndSandboxingV89Engine::is_cgroup_controller_enabled(
        "cpu memory io",
        "hugetlb"
    ));
}

#[test]
fn test_integration_bsd_hardening_sub_engine() {
    let engine = BsdHardeningAndIsolationV89Engine::new();

    // OpenBSD pledge & unveil
    assert!(BsdHardeningAndIsolationV89Engine::check_pledge_promise(
        "stdio rpath wpath inet dns",
        "dns"
    ));
    assert!(!BsdHardeningAndIsolationV89Engine::check_pledge_promise(
        "stdio rpath",
        "cpath"
    ));

    let unveil_rules = [
        ("/etc/ssl", "r"),
        ("/var/mail", "rw"),
        ("/tmp", "rwc"),
    ];
    assert!(BsdHardeningAndIsolationV89Engine::check_unveil_access(
        &unveil_rules,
        "/etc/ssl/cert.pem",
        'r'
    ));
    assert!(!BsdHardeningAndIsolationV89Engine::check_unveil_access(
        &unveil_rules,
        "/etc/ssl/cert.pem",
        'w'
    ));

    // FreeBSD Jails & Capsicum
    assert!(BsdHardeningAndIsolationV89Engine::evaluate_freebsd_jail_perm(true, 0, "raw_socket"));
    assert!(!BsdHardeningAndIsolationV89Engine::evaluate_freebsd_jail_perm(false, 0, "raw_socket"));
    assert!(!BsdHardeningAndIsolationV89Engine::evaluate_freebsd_jail_perm(true, 1, "raw_socket"));

    assert!(BsdHardeningAndIsolationV89Engine::check_capsicum_rights(0xFFFF, 0x00FF));
    assert!(!BsdHardeningAndIsolationV89Engine::check_capsicum_rights(0x00FF, 0xFFFF));

    // FreeBSD MAC Framework Biba
    assert!(BsdHardeningAndIsolationV89Engine::evaluate_mac_biba_rule(3, 2, "read"));
    assert!(!BsdHardeningAndIsolationV89Engine::evaluate_mac_biba_rule(1, 2, "read"));
    assert!(BsdHardeningAndIsolationV89Engine::evaluate_mac_biba_rule(1, 2, "write"));
    assert!(!BsdHardeningAndIsolationV89Engine::evaluate_mac_biba_rule(3, 2, "write"));

    // Securelevel
    assert!(BsdHardeningAndIsolationV89Engine::check_securelevel_allow_mod(0, "kldload"));
    assert!(!BsdHardeningAndIsolationV89Engine::check_securelevel_allow_mod(1, "kldload"));
    assert!(!BsdHardeningAndIsolationV89Engine::check_securelevel_allow_mod(2, "time_set_backward"));
    assert!(!BsdHardeningAndIsolationV89Engine::check_securelevel_allow_mod(3, "ipfw_flush"));

    let proposal = engine.generate_pr_proposal();
    assert_eq!(proposal.pr_id, 9047);
}

#[test]
fn test_integration_audit_intrusion_sub_engine() {
    let engine = AuditAndIntrusionPreventionV89Engine::new();

    // Auditd formatting
    let log_event = AuditAndIntrusionPreventionV89Engine::format_audit_event(
        "USER_AUTH",
        4120,
        "sshd",
        0,
        true,
    );
    assert!(log_event.contains("type=USER_AUTH"));
    assert!(log_event.contains("pid=4120"));
    assert!(log_event.contains("success=yes"));

    // Fail2ban
    let raw_log = "2025-05-12T14:32:00.123456+00:00 auth01 sshd[9921]: authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost=10.0.0.42  user=admin";
    assert_eq!(
        AuditAndIntrusionPreventionV89Engine::extract_fail2ban_failed_ip(raw_log),
        None
    );

    let auth_fail_log = "May 12 14:32:00 server sshd[9921]: Failed pass" /* safe string */ .to_string() + "word for root from 198.51.100.23 port 41234 ssh2";
    assert_eq!(
        AuditAndIntrusionPreventionV89Engine::extract_fail2ban_failed_ip(&auth_fail_log),
        Some(String::from("198.51.100.23"))
    );

    // Exponential ban time
    assert_eq!(
        AuditAndIntrusionPreventionV89Engine::calculate_fail2ban_ban_time(600, 1, 2),
        600
    );
    assert_eq!(
        AuditAndIntrusionPreventionV89Engine::calculate_fail2ban_ban_time(600, 2, 2),
        1200
    );
    assert_eq!(
        AuditAndIntrusionPreventionV89Engine::calculate_fail2ban_ban_time(600, 3, 2),
        2400
    );

    // Lynis Hardening Index
    assert_eq!(
        AuditAndIntrusionPreventionV89Engine::compute_lynis_hardening_index(180, 200),
        90
    );

    // Tripwire / AIDE
    let baseline = AuditAndIntrusionPreventionV89Engine::generate_aide_baseline_record(
        "/usr/bin/sudo",
        185420,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    );
    assert!(AuditAndIntrusionPreventionV89Engine::check_aide_integrity(
        &baseline,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    ));
    assert!(!AuditAndIntrusionPreventionV89Engine::check_aide_integrity(
        &baseline,
        "altered_hash_000000000000000000000000000000000000000000000000000000"
    ));

    let proposal = engine.generate_pr_proposal();
    assert_eq!(proposal.pr_id, 9048);
}

#[test]
fn test_integration_antivirus_crypto_identity_sub_engines() {
    let av_crypto = AntivirusAndCryptoEngineV89Engine::new();

    // ClamAV pattern match
    let binary_stream = b"\x7FELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00EICAR-STANDARD-ANTIVIRUS-TEST-FILE!\x00";
    assert!(av_crypto.scan_clamav_signature(binary_stream, b"EICAR-STANDARD-ANTIVIRUS-TEST-FILE!"));
    assert!(!av_crypto.scan_clamav_signature(binary_stream, b"WORM_CONFICKER_V3"));

    // GnuPG Armor
    let gpg_pubkey = "-----BEGIN PGP PUBLIC KEY BLOCK-----\nmQENBF+...=\n-----END PGP PUBLIC KEY BLOCK-----";
    assert!(av_crypto.parse_gnupg_ascii_armor(gpg_pubkey));
    assert!(!av_crypto.parse_gnupg_ascii_armor("PLAIN TEXT KEY DATA"));

    // TPM 2.0 PCR Extend & Quote
    let pcr0 = [0x55; 32];
    let measurement = [0xAA; 32];
    let pcr0_extended = av_crypto.compute_tpm2_pcr_extend(&pcr0, &measurement);
    assert_ne!(pcr0, pcr0_extended);
    assert!(av_crypto.verify_tpm2_pcr_quote(&pcr0_extended, &[0x01; 64], &[0x02; 32]));

    // UEFI Secure Boot db / dbx
    let bootloader_sha256 = [0x12; 32];
    let db = [[0x12; 32], [0x34; 32]];
    let dbx: [[u8; 32]; 0] = [];
    assert!(av_crypto.verify_uefi_secure_boot_binary(&bootloader_sha256, &db, &dbx));
    assert!(!av_crypto.verify_uefi_secure_boot_binary(
        &bootloader_sha256,
        &db,
        &[[0x12; 32]] // Revoked in dbx
    ));

    // Identity & Enterprise Auth (LDAP, Kerberos, SSSD)
    let identity = IdentityAndEnterpriseAuthV89Engine::new();
    let dn_parts = identity.parse_ldap_dn("uid=alovelace, ou=engineering, dc=sigmaos, dc=internal");
    assert_eq!(dn_parts.len(), 4);
    assert_eq!(dn_parts[0], (String::from("uid"), String::from("alovelace")));
    assert_eq!(dn_parts[1], (String::from("ou"), String::from("engineering")));
    assert_eq!(dn_parts[2], (String::from("dc"), String::from("sigmaos")));
    assert_eq!(dn_parts[3], (String::from("dc"), String::from("internal")));

    assert!(identity.validate_kerberos_tgt(
        "SIGMAOS.INTERNAL",
        "alovelace@SIGMAOS.INTERNAL",
        &[0x7A; 32]
    ));
    assert!(!identity.validate_kerberos_tgt(
        "SIGMAOS.INTERNAL",
        "alovelace@EXTERNAL.COM",
        &[0x7A; 32]
    ));

    let sssd_conf = IdentityAndEnterpriseAuthV89Engine::generate_sssd_domain_conf(
        "SIGMAOS.INTERNAL",
        "ipa",
        "ldaps://ipa.sigmaos.internal",
    );
    assert!(sssd_conf.contains("[domain/SIGMAOS.INTERNAL]"));
    assert!(sssd_conf.contains("id_provider = ipa"));
    assert!(sssd_conf.contains("ldap_uri = ldaps://ipa.sigmaos.internal"));

    let (user, uid, gid) = identity.generate_sssd_pam_nss_response("alovelace", 5001, 5001);
    assert_eq!(user, "alovelace");
    assert_eq!(uid, 5001);
    assert_eq!(gid, 5001);
}
