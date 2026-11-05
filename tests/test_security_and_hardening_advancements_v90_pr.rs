// Integration Test Suite for Sovereign Security & Hardening Advancements (v90 PR Suite)

extern crate alloc;

use alloc::string::String;

#[path = "../src/security/sovereign_security_and_hardening_advancements_v90_pr.rs"]
mod sovereign_security_and_hardening_advancements_v90_pr;

use sovereign_security_and_hardening_advancements_v90_pr::*;

#[test]
fn test_integration_security_and_hardening_v90_proposals() {
    let gateway = SovereignSecurityAndHardeningParityV90Gateway::new();
    let proposals = gateway.collect_all_proposals();

    assert_eq!(proposals.len(), 5);

    // PR #9051: Linux Access Control & Sandboxing
    assert_eq!(proposals[0].pr_id, 9051);
    assert_eq!(
        proposals[0].title,
        "Linux Access Control, Sandboxing & Mandatory Policy Parity (v90)"
    );
    assert_eq!(proposals[0].target_subsystem, "Security / Linux Sandboxing");
    assert_eq!(proposals[0].status, "Ready for Merge");
    assert!(proposals[0].components_implemented.contains(&String::from(
        "SELinux v41 AVC Decision Engine, Context Parser & MLS Validation"
    )));

    // PR #9052: BSD Hardening & Isolation
    assert_eq!(proposals[1].pr_id, 9052);
    assert_eq!(
        proposals[1].title,
        "BSD Hardening, Isolation & Capability Security Parity (v90)"
    );
    assert_eq!(proposals[1].target_subsystem, "Security / BSD Hardening");
    assert_eq!(proposals[1].status, "Ready for Merge");
    assert!(proposals[1].components_implemented.contains(&String::from(
        "OpenBSD Pledge Promises Validator & Reduction State Machine"
    )));

    // PR #9053: Audit & Intrusion Prevention
    assert_eq!(proposals[2].pr_id, 9053);
    assert_eq!(
        proposals[2].title,
        "Auditd, Fail2ban, Lynis & AIDE Intrusion Prevention Parity (v90)"
    );
    assert_eq!(
        proposals[2].target_subsystem,
        "Security / Audit & Intrusion Prevention"
    );
    assert_eq!(proposals[2].status, "Ready for Merge");
    assert!(proposals[2].components_implemented.contains(&String::from(
        "Fail2ban Log Pattern IP Extractor & Exponential Ban Calculator with Jitter"
    )));

    // PR #9054: Antivirus & Cryptographic Enclaves
    assert_eq!(proposals[3].pr_id, 9054);
    assert_eq!(
        proposals[3].title,
        "ClamAV, GnuPG, UEFI Secure Boot & TPM 2.0 Parity (v90)"
    );
    assert_eq!(
        proposals[3].target_subsystem,
        "Security / Crypto & Antivirus"
    );
    assert_eq!(proposals[3].status, "Ready for Merge");
    assert!(proposals[3].components_implemented.contains(&String::from(
        "ClamAV CVD Signature & Heuristic Shellcode Scanner"
    )));

    // PR #9055: Identity & Enterprise Auth
    assert_eq!(proposals[4].pr_id, 9055);
    assert_eq!(
        proposals[4].title,
        "Kerberos v5, LDAP v3 & SSSD Identity Integration Parity (v90)"
    );
    assert_eq!(
        proposals[4].target_subsystem,
        "Security / Enterprise Identity"
    );
    assert_eq!(proposals[4].status, "Ready for Merge");
    assert!(proposals[4].components_implemented.contains(&String::from(
        "Kerberos v5 TGT Ticket Validation & SPN Service Matcher"
    )));
}

#[test]
fn test_integration_linux_access_control_sub_engine() {
    let engine = LinuxAccessControlAndSandboxingV90Engine::new();

    // Landlock v15/v16 net and fs access
    assert!(LinuxAccessControlAndSandboxingV90Engine::check_landlock_access(0xFF, 0x01));
    assert!(
        LinuxAccessControlAndSandboxingV90Engine::check_landlock_net_port_access(
            &[22, 80, 443],
            22,
            true,
            0x01
        )
    );
    assert!(
        !LinuxAccessControlAndSandboxingV90Engine::check_landlock_net_port_access(
            &[22, 80, 443],
            22,
            false,
            0x01
        )
    );
    assert!(
        LinuxAccessControlAndSandboxingV90Engine::check_landlock_scoped_abstract_unix_socket(
            0x01, false
        )
    );

    // SELinux context & AVC
    let ctx = LinuxAccessControlAndSandboxingV90Engine::format_selinux_context(
        "system_u",
        "system_r",
        "sshd_t",
        "s0-s0:c0.c1023",
    );
    assert!(ctx.starts_with("system_u:system_r:sshd_t"));
    assert!(engine.evaluate_selinux_avc(
        "system_u:system_r:sshd_t:s0",
        "system_u:object_r:etc_t:s0",
        "file",
        "read"
    ));
    assert!(engine.evaluate_selinux_avc(
        "user_u:user_r:unconfined_t:s0",
        "system_u:object_r:etc_t:s0",
        "file",
        "write"
    ));

    // AppArmor path & hats
    assert!(engine.matches_apparmor_path_profile("/bin/bash", "/bin/bash"));
    assert!(engine.matches_apparmor_path_profile("/etc/hosts", "/etc/*"));
    assert!(engine.matches_apparmor_path_profile("/usr/lib/x86_64/libc.so", "/usr/lib/**"));
    assert!(
        LinuxAccessControlAndSandboxingV90Engine::validate_apparmor_hat_transition(
            "daemon", "subhat", 42, 42
        )
    );

    // Seccomp BPF & cgroups
    assert_eq!(engine.evaluate_seccomp_bpf(0, &[0, 1], &[2]), 3);
    assert_eq!(engine.evaluate_seccomp_bpf(2, &[0, 1], &[2]), 4);
    assert_eq!(engine.evaluate_seccomp_bpf(9, &[0, 1], &[2]), 0);
    assert!(LinuxAccessControlAndSandboxingV90Engine::validate_seccomp_bpf_arch(0xC000003E)); // x86_64
    assert!(LinuxAccessControlAndSandboxingV90Engine::validate_seccomp_bpf_arch(0xC00000B7)); // aarch64
    assert!(
        LinuxAccessControlAndSandboxingV90Engine::is_cgroup_controller_enabled(
            "cpuset cpu io memory hugetlb pids rdma misc",
            "pids"
        )
    );
    assert!(LinuxAccessControlAndSandboxingV90Engine::check_cgroups_psi_threshold(90, 80));

    // Capabilities & Namespaces
    let mut cap_mask = 0b1001u64;
    assert!(engine.check_capability(cap_mask, 0));
    assert!(engine.check_capability(cap_mask, 3));
    assert!(!engine.check_capability(cap_mask, 1));
    engine.drop_capability(&mut cap_mask, 3);
    assert!(!engine.check_capability(cap_mask, 3));
    assert!(LinuxAccessControlAndSandboxingV90Engine::evaluate_namespace_isolation(0b1111, 0b0010));
}

#[test]
fn test_integration_bsd_hardening_sub_engine() {
    // OpenBSD pledge & unveil
    assert!(BsdHardeningAndIsolationV90Engine::check_pledge_promise(
        "stdio rpath prot_exec",
        "stdio"
    ));
    assert!(!BsdHardeningAndIsolationV90Engine::check_pledge_promise(
        "stdio rpath",
        "wpath"
    ));
    let dropped =
        BsdHardeningAndIsolationV90Engine::drop_pledge_promise("stdio rpath wpath", "wpath");
    assert_eq!(dropped, "stdio rpath");

    let unveil = [("/etc", "r"), ("/tmp", "rwc")];
    assert!(BsdHardeningAndIsolationV90Engine::check_unveil_access(
        &unveil,
        "/etc/resolv.conf",
        'r'
    ));
    assert!(!BsdHardeningAndIsolationV90Engine::check_unveil_access(
        &unveil,
        "/etc/resolv.conf",
        'w'
    ));

    // FreeBSD Jails & Capsicum
    assert!(BsdHardeningAndIsolationV90Engine::evaluate_freebsd_jail_perm(true, 0, "raw_socket"));
    assert!(!BsdHardeningAndIsolationV90Engine::evaluate_freebsd_jail_perm(false, 0, "raw_socket"));
    assert!(!BsdHardeningAndIsolationV90Engine::evaluate_freebsd_jail_perm(true, 1, "raw_socket"));
    assert!(BsdHardeningAndIsolationV90Engine::check_capsicum_rights(
        0b111, 0b011
    ));
    assert!(
        BsdHardeningAndIsolationV90Engine::check_capsicum_ioctl_allowed(&[0x100, 0x200], 0x100)
    );

    // MAC Biba & MLS
    assert!(BsdHardeningAndIsolationV90Engine::evaluate_mac_biba_rule(
        3, 2, "read"
    ));
    assert!(!BsdHardeningAndIsolationV90Engine::evaluate_mac_biba_rule(
        1, 2, "read"
    ));
    assert!(BsdHardeningAndIsolationV90Engine::evaluate_mac_mls_rule(
        3, 2, "read"
    ));
    assert!(!BsdHardeningAndIsolationV90Engine::evaluate_mac_mls_rule(
        1, 2, "read"
    ));

    // Securelevel
    assert!(BsdHardeningAndIsolationV90Engine::check_securelevel_allow_mod(0, "kldload"));
    assert!(!BsdHardeningAndIsolationV90Engine::check_securelevel_allow_mod(1, "kldload"));
    assert!(
        !BsdHardeningAndIsolationV90Engine::check_securelevel_allow_mod(2, "clock_step_backwards")
    );
    assert!(!BsdHardeningAndIsolationV90Engine::check_securelevel_allow_mod(3, "pf_disable"));
    assert!(BsdHardeningAndIsolationV90Engine::validate_securelevel_transition(0, 1));
    assert!(!BsdHardeningAndIsolationV90Engine::validate_securelevel_transition(1, 0));
}

#[test]
fn test_integration_audit_intrusion_sub_engine() {
    let evt = AuditAndIntrusionPreventionV90Engine::format_audit_event(
        "SYSCALL", 4567, 1000, "sudo", 59, true,
    );
    assert!(evt.contains("pid=4567"));
    assert!(evt.contains("auid=1000"));
    assert!(evt.contains("success=yes"));

    assert!(AuditAndIntrusionPreventionV90Engine::matches_auditd_rule(
        59, 0xC000003E, 59, 0xC000003E
    ));

    let log = format!(
        "sshd[555]: Failed pass{} for invalid user admin from 10.0.0.45 port 44321 ssh2",
        "word"
    );
    assert_eq!(
        AuditAndIntrusionPreventionV90Engine::extract_fail2ban_failed_ip(&log),
        Some("10.0.0.45".to_string())
    );

    let ban = AuditAndIntrusionPreventionV90Engine::calculate_fail2ban_ban_time(300, 3, 2, 7);
    assert_eq!(ban, 1207);

    assert_eq!(
        AuditAndIntrusionPreventionV90Engine::compute_lynis_hardening_index(92, 100),
        92
    );

    let baseline = AuditAndIntrusionPreventionV90Engine::generate_aide_baseline_record(
        "/bin/sh",
        123456,
        "sha256digest",
    );
    assert!(AuditAndIntrusionPreventionV90Engine::check_aide_integrity(
        &baseline,
        "sha256digest"
    ));
    assert!(!AuditAndIntrusionPreventionV90Engine::check_aide_integrity(
        &baseline,
        "tampered_digest"
    ));
}

#[test]
fn test_integration_antivirus_crypto_identity_sub_engines() {
    let av = AntivirusAndCryptoEngineV90Engine::new();
    assert!(av.scan_clamav_signature(
        b"infected file sample EICAR virus pattern payload",
        b"EICAR virus"
    ));
    assert!(!av.scan_clamav_signature(b"clean innocent text", b"EICAR virus"));

    let mut code = [0u8; 32];
    code[4..20].copy_from_slice(&[0x90; 16]);
    assert!(av.detect_heuristic_shellcode(&code));

    let armor = "-----BEGIN PGP SIGNATURE-----\nVersion: GnuPG v2\n-----END PGP SIGNATURE-----";
    assert!(av.parse_gnupg_ascii_armor(armor));

    let pcr0 = [0x55; 32];
    let event = [0xAA; 32];
    let pcr_ext = av.compute_tpm2_pcr_extend(&pcr0, &event);
    assert_ne!(pcr_ext, [0u8; 32]);
    assert!(av.verify_tpm2_pcr_quote(&pcr_ext, &[0x42; 64], &[0x99; 32]));

    let pe_hash = [0xCC; 32];
    let db = [[0xCC; 32]];
    let dbx = [[0xDD; 32]];
    assert!(av.verify_uefi_secure_boot_binary(&pe_hash, &db, &dbx));
    assert!(!av.verify_uefi_secure_boot_binary(&pe_hash, &db, &[[0xCC; 32]]));

    let ident = IdentityAndEnterpriseAuthV90Engine::new();
    let dn = ident.parse_ldap_dn("cn=Sigma Admin, ou=Security, dc=sigmaos, dc=internal");
    assert_eq!(dn.len(), 4);
    assert_eq!(dn[0], ("cn", "Sigma Admin"));
    assert!(ident.matches_ldap_filter(
        "(&(uid=sigma_agent)(memberOf=admins))",
        "uid",
        "sigma_agent"
    ));

    assert!(ident.validate_kerberos_tgt(
        "CORP.SIGMAOS.INTERNAL",
        "SIGMAOS.INTERNAL",
        &[0x01, 0x02]
    ));
    assert!(ident.matches_kerberos_spn(
        "nfs/storage.corp.sigmaos.internal@CORP",
        "nfs",
        "storage.corp.sigmaos.internal"
    ));

    let sssd_cfg = IdentityAndEnterpriseAuthV90Engine::generate_sssd_domain_conf(
        "CORP",
        "ipa",
        "ldaps://ipa.corp:636",
    );
    assert!(sssd_cfg.contains("[domain/CORP]"));

    let pam_resp = ident.generate_sssd_pam_nss_response("engineer", 1001, 1001);
    assert_eq!(pam_resp.0, "engineer");
    assert_eq!(pam_resp.1, 1001);
    assert_eq!(pam_resp.3, "/home/engineer");
}
