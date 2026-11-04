//! Integration test for Sovereign Security and Hardening Advancements (v89 Parity Suite)
//!
//! Validates parity for SELinux, AppArmor, seccomp BPF, capabilities, namespaces, cgroups v2,
//! Landlock, pledge, unveil, FreeBSD Jails, Capsicum, MAC Biba/MLS framework, securelevel,
//! auditd, fail2ban, Lynis, Tripwire/AIDE, ClamAV, GnuPG, TPM 2.0, Secure Boot, Kerberos v5,
//! LDAP v3, and SSSD.

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
    assert_eq!(proposals[0].target_subsystem, "Security / Linux Sandboxing");
    assert_eq!(proposals[0].status, "Ready for Merge");
    assert!(proposals[0].components_implemented.contains(&String::from(
        "SELinux v40 AVC Decision Engine & Multilevel Security Contexts"
    )));

    // PR #9047: BSD Hardening & Isolation
    assert_eq!(proposals[1].pr_id, 9047);
    assert_eq!(
        proposals[1].title,
        "BSD Hardening, Isolation & Capability Security Parity (v89)"
    );
    assert_eq!(proposals[1].target_subsystem, "Security / BSD Hardening");
    assert_eq!(proposals[1].status, "Ready for Merge");

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
}

#[test]
fn test_integration_linux_access_control_sub_engine_v89() {
    let engine = LinuxAccessControlAndSandboxingV89Engine::new();

    // Landlock access check
    assert!(LinuxAccessControlAndSandboxingV89Engine::check_landlock_access(0b1111, 0b0011));
    assert!(!LinuxAccessControlAndSandboxingV89Engine::check_landlock_access(0b0001, 0b0011));

    // SELinux context formatting & AVC evaluation
    let scontext = LinuxAccessControlAndSandboxingV89Engine::format_selinux_context(
        "system_u", "system_r", "httpd_t", "s0",
    );
    assert_eq!(scontext, "system_u:system_r:httpd_t:s0");

    let tcontext = "system_u:object_r:httpd_sys_content_t:s0";
    assert!(engine.evaluate_selinux_avc(&scontext, tcontext, "file", "read"));
    assert!(engine.evaluate_selinux_avc(&scontext, tcontext, "file", "open"));
    assert!(!engine.evaluate_selinux_avc(
        &scontext,
        "system_u:object_r:shadow_t:s0",
        "file",
        "read"
    ));

    // Landlock network port access
    assert!(
        LinuxAccessControlAndSandboxingV89Engine::check_landlock_net_port_access(
            &[80, 443],
            80,
            true,
            0x01
        )
    );
    assert!(
        !LinuxAccessControlAndSandboxingV89Engine::check_landlock_net_port_access(
            &[80, 443],
            22,
            true,
            0x01
        )
    );

    // AppArmor path profile
    assert!(engine.matches_apparmor_path_profile("/etc/ssl/cert.pem", "/etc/ssl/*"));
    assert!(engine.matches_apparmor_path_profile("/var/log/audit/audit.log", "/var/log/**"));
    assert!(!engine.matches_apparmor_path_profile("/etc/ssl/certs/sub/cert.pem", "/etc/ssl/*"));

    // Seccomp BPF & cgroups
    assert!(LinuxAccessControlAndSandboxingV89Engine::validate_seccomp_bpf_arch(0xC000003E));
    assert!(
        LinuxAccessControlAndSandboxingV89Engine::is_cgroup_controller_enabled(
            "cpu memory io pids cpuset",
            "pids"
        )
    );
    assert_eq!(engine.evaluate_seccomp_bpf(0, &[0, 1, 2]), 3);
    assert_eq!(engine.evaluate_seccomp_bpf(59, &[0, 1, 2]), 0);

    // Capabilities & Namespaces
    assert!(engine.check_capability(1 << 21, 21)); // CAP_SYS_ADMIN
    assert!(!engine.check_capability(1 << 21, 12)); // CAP_NET_ADMIN
    assert!(LinuxAccessControlAndSandboxingV89Engine::evaluate_namespace_isolation(0b0110, 0b0010));
}

#[test]
fn test_integration_bsd_hardening_sub_engine_v89() {
    // OpenBSD pledge & unveil
    assert!(BsdHardeningAndIsolationV89Engine::check_pledge_promise(
        "stdio rpath wpath cpath inet dns",
        "dns"
    ));
    assert!(!BsdHardeningAndIsolationV89Engine::check_pledge_promise(
        "stdio rpath",
        "inet"
    ));

    let unveil_table = [("/usr", "rx"), ("/var/log", "rw"), ("/tmp", "rwc")];
    assert!(BsdHardeningAndIsolationV89Engine::check_unveil_access(
        &unveil_table,
        "/usr/bin/tar",
        'x'
    ));
    assert!(BsdHardeningAndIsolationV89Engine::check_unveil_access(
        &unveil_table,
        "/var/log/messages",
        'w'
    ));
    assert!(!BsdHardeningAndIsolationV89Engine::check_unveil_access(
        &unveil_table,
        "/usr/bin/tar",
        'w'
    ));

    // FreeBSD Jails & Capsicum
    assert!(BsdHardeningAndIsolationV89Engine::evaluate_freebsd_jail_perm(true, 0, "raw_socket"));
    assert!(!BsdHardeningAndIsolationV89Engine::evaluate_freebsd_jail_perm(false, 0, "raw_socket"));
    assert!(BsdHardeningAndIsolationV89Engine::check_capsicum_rights(
        0x7F, 0x07
    ));

    // MAC Biba integrity & BSD securelevel
    assert!(BsdHardeningAndIsolationV89Engine::evaluate_mac_biba_rule(
        3, 2, "read"
    )); // read down allowed
    assert!(!BsdHardeningAndIsolationV89Engine::evaluate_mac_biba_rule(
        1, 2, "read"
    )); // read up denied
    assert!(BsdHardeningAndIsolationV89Engine::evaluate_mac_biba_rule(
        1, 3, "write"
    )); // write up allowed
    assert!(!BsdHardeningAndIsolationV89Engine::evaluate_mac_biba_rule(
        3, 1, "write"
    )); // write down denied

    assert!(!BsdHardeningAndIsolationV89Engine::check_securelevel_allow_mod(1, "kldload"));
    assert!(!BsdHardeningAndIsolationV89Engine::check_securelevel_allow_mod(2, "disk_raw_write"));
    assert!(BsdHardeningAndIsolationV89Engine::check_securelevel_allow_mod(0, "kldload"));
}

#[test]
fn test_integration_audit_intrusion_sub_engine_v89() {
    // auditd formatting
    let ev =
        AuditAndIntrusionPreventionV89Engine::format_audit_event("SYSCALL", 5432, "sshd", 0, true);
    assert!(ev.contains("type=SYSCALL"));
    assert!(ev.contains("comm=\"sshd\""));

    // fail2ban parsing & exponential backoff
    let log_line = "sshd[9876]: Failed password for root from 10.0.0.42 port 43210 ssh2";
    let ip = AuditAndIntrusionPreventionV89Engine::extract_fail2ban_failed_ip(log_line);
    assert_eq!(ip, Some(String::from("10.0.0.42")));

    let ban_time = AuditAndIntrusionPreventionV89Engine::calculate_fail2ban_ban_time(600, 3, 2);
    assert_eq!(ban_time, 2400); // 600 * 2^(3-1) = 2400

    // Lynis hardening index & AIDE integrity check
    let lynis_score = AuditAndIntrusionPreventionV89Engine::compute_lynis_hardening_index(185, 200);
    assert_eq!(lynis_score, 92);

    let aide_rec = AuditAndIntrusionPreventionV89Engine::generate_aide_baseline_record(
        "/usr/bin/sudo",
        150000,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    );
    assert!(AuditAndIntrusionPreventionV89Engine::check_aide_integrity(
        &aide_rec,
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    ));
    assert!(!AuditAndIntrusionPreventionV89Engine::check_aide_integrity(
        &aide_rec,
        "tampered_hash_here"
    ));
}

#[test]
fn test_integration_antivirus_crypto_identity_sub_engines_v89() {
    let crypto_engine = AntivirusAndCryptoEngineV89Engine::new();

    // ClamAV pattern scan
    assert!(crypto_engine.scan_clamav_signature(b"MALWARE_STRING_IN_PAYLOAD", b"MALWARE_STRING"));
    assert!(!crypto_engine.scan_clamav_signature(b"CLEAN_PAYLOAD_HERE", b"MALWARE_STRING"));

    // GnuPG armor
    let pgp_armor =
        "-----BEGIN PGP SIGNATURE-----\niQEzBAEBCAAdFiEE...\n-----END PGP SIGNATURE-----";
    assert!(crypto_engine.parse_gnupg_ascii_armor(pgp_armor));

    // TPM 2.0 PCR extend & quote
    let pcr0 = [0x5Au8; 32];
    let event_digest = [0xA5u8; 32];
    let extended = crypto_engine.compute_tpm2_pcr_extend(&pcr0, &event_digest);
    assert_ne!(extended, [0u8; 32]);
    assert!(crypto_engine.verify_tpm2_pcr_quote(&extended, &[0x42u8; 64], &[0x99u8; 32]));

    // UEFI Secure Boot verification
    let pe_hash = [0x12u8; 32];
    let db = [[0x12u8; 32]];
    let dbx = [[0x99u8; 32]];
    assert!(crypto_engine.verify_uefi_secure_boot_binary(&pe_hash, &db, &dbx));

    let dbx_revoked = [[0x12u8; 32]];
    assert!(!crypto_engine.verify_uefi_secure_boot_binary(&pe_hash, &db, &dbx_revoked));

    // Enterprise Identity & Auth (Kerberos, LDAP, SSSD)
    let ident_engine = IdentityAndEnterpriseAuthV89Engine::new();
    let dn = "cn=admin, ou=groups, dc=sigma, dc=local";
    let dn_parts = ident_engine.parse_ldap_dn(dn);
    assert_eq!(dn_parts.len(), 4);
    assert_eq!(dn_parts[0], (String::from("cn"), String::from("admin")));

    assert!(ident_engine.validate_kerberos_tgt("CORP.LOCAL", "alice@CORP.LOCAL", &[0xFE; 16]));
    assert!(!ident_engine.validate_kerberos_tgt("OTHER.LOCAL", "alice@CORP.LOCAL", &[0xFE; 16]));

    let sssd_cfg = IdentityAndEnterpriseAuthV89Engine::generate_sssd_domain_conf(
        "CORP.LOCAL",
        "ldap",
        "ldaps://auth.corp.local:636",
    );
    assert!(sssd_cfg.contains("[domain/CORP.LOCAL]"));
    assert!(sssd_cfg.contains("ldaps://auth.corp.local:636"));

    let pam_resp = ident_engine.generate_sssd_pam_nss_response("alice", 1001, 1001);
    assert_eq!(pam_resp.0, "alice");
    assert_eq!(pam_resp.1, 1001);
    assert_eq!(pam_resp.2, 1001);
}
