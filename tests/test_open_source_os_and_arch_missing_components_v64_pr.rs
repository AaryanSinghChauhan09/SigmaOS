// SPDX-License-Identifier: MIT
// Comprehensive Integration Test Suite for Open Source OS Unimplemented Ideas
// & Arch Linux GitHub Repo Missing Components PR Engines (V63/V64)

#[path = "../src/distro/open_source_os_missing_components_v63_pr.rs"]
mod open_source_os_missing_components_v63_pr;

#[path = "../src/distro/arch_linux_missing_components_v64_pr.rs"]
mod arch_linux_missing_components_v64_pr;

use arch_linux_missing_components_v64_pr::*;
use open_source_os_missing_components_v63_pr::*;

#[test]
fn test_open_source_os_v63_pr_suite_full_cycle() {
    let mut suite = SovereignOpenSourceOsMissingComponentsV63PrSuite::new();

    suite
        .linux_engine
        .register_sched_policy("scx_photonic_v63", "0xphotonic63");
    suite
        .freebsd_engine
        .spawn_pqc_vnet_jail("jail_v63", vec!["CAP_READ"]);
    suite.openbsd_engine.configure_security(2048, 128);
    suite
        .dragonfly_engine
        .record_pfs_snapshot("pfs_v63", 1700000000);
    suite
        .netbsd_engine
        .register_rump_driver("rumpuser_hypercall_v63");
    suite
        .haiku_engine
        .add_attribute_index("BEOS:TYPE_V63", "string");
    suite.plan9_engine.mount_synthetic_namespace("#s/proc_v63");
    suite
        .nix_guix_engine
        .register_closure_hash("/nix/store/pkg_v63", "hash_v63");
    suite.illumos_engine.configure_vnic_mesh("vnic_v63", 5000);
    suite.thousands_engine.register_component("mod_v63");

    let prs = suite.collect_all_proposals();
    assert_eq!(prs.len(), 10, "Expected 10 Open-Source OS PR specifications");
    assert_eq!(
        suite.compute_v63_parity_score(),
        100,
        "Gap closure score must be 100"
    );

    for pr in &prs {
        assert_eq!(pr.status, OsPrStatusV63::Validated);
        assert!(!pr.impacted_subsystems.is_empty());
    }
}

#[test]
fn test_arch_linux_v64_pr_suite_full_cycle() {
    let suite = ArchLinuxMissingComponentsV64PrSuite::new();
    let prs = suite.run_all_prs();

    assert_eq!(prs.len(), 10, "Expected 10 Arch Linux PR specifications");
    assert_eq!(
        suite.compute_gap_closure_score(),
        100,
        "Gap closure score must be 100"
    );

    let expected_target_repos = [
        "archlinux/pacman",
        "archlinux/pacman",
        "archlinux/aurutils",
        "archlinux/devtools",
        "archlinux/systemd",
        "archlinux/reflector",
        "archlinux/pacman-contrib",
        "archlinux/mkinitcpio",
        "archlinux/arch-install-scripts",
        "archlinux/systemd",
    ];

    for (idx, pr) in prs.iter().enumerate() {
        assert_eq!(pr.status, ArchV64PrStatus::Merged);
        assert_eq!(pr.target_repo, expected_target_repos[idx]);
        assert_eq!(pr.pr_id, (6401 + idx) as u32);
        assert!(!pr.summary.is_empty());
        assert!(!pr.patch_diff.is_empty());
    }

    // Engine specific runtime command generation tests
    let pkglist = ArchPkglistExportBackupManagerPrEngine::new();
    assert!(pkglist.verify());
    assert_eq!(pkglist.format_export_cmd(), "pacman -Qqe > /etc/pkglist.txt");

    let reflector = ArchReflectorCountryMirrorFilterPrEngine::new();
    assert!(reflector.verify());
    assert!(reflector.format_reflector_cmd().contains("reflector --country US"));

    let paccache = ArchPaccacheRetentionCleanerPrEngine::new();
    assert!(paccache.verify());
    assert_eq!(paccache.format_clean_cmd(), "paccache -rk2 -c /var/cache/pacman/pkg");

    let mkinitcpio = ArchMkinitcpioCompressionTunerPrEngine::new();
    assert!(mkinitcpio.verify());
    assert_eq!(mkinitcpio.format_build_cmd(), "mkinitcpio -p linux");

    let genfstab = ArchGenfstabLabelUuidGeneratorPrEngine::new("/mnt");
    assert!(genfstab.verify());
    assert_eq!(genfstab.format_genfstab_cmd(), "genfstab -U /mnt");

    let boot = ArchSystemdBootEntriesManagerPrEngine::new();
    assert!(boot.verify());
    assert!(boot.format_entry_config().contains("title Arch Linux"));
}
