// SPDX-License-Identifier: MIT
// Integration Test Suite for Sovereign Kernel and Boot Innovations V61 Suite

#[path = "../src/boot/sovereign_kernel_and_boot_innovations_v61_pr.rs"]
mod sovereign_kernel_and_boot_innovations_v61_pr;

use sovereign_kernel_and_boot_innovations_v61_pr::*;

#[test]
fn test_kernel_and_boot_v61_full_suite() {
    let mut suite = SovereignKernelAndBootInnovationsV61Suite::new();
    assert!(suite.run_full_suite_verification());

    let summary = suite.summary_report();
    assert!(summary.contains("Active Kernel Config: Linux 6.14+ Sovereign Core"));
    assert!(summary.contains("Loaded Kernel Modules: 2"));
    assert!(summary.contains("Probed Device Drivers: 2"));
    assert!(summary.contains("Bootloader Entries: 5"));
    assert!(summary.contains("Active GOP Mode: 1920x1080"));
    assert!(summary.contains("Kexec Staged Segments: 3"));
}

#[test]
fn test_kernel_innovations_v61_flavors() {
    let mut engine = LinuxBsdKernelInnovationsV61Engine::new();
    assert_eq!(engine.configs.len(), 5);

    // Test OpenBSD flavor
    assert!(engine.activate_flavor(KernelFlavorV61::OpenBsdPledgeKarlSoftraid));
    let openbsd = engine.get_active_config().unwrap();
    assert_eq!(openbsd.karl_seed, 0xCAFE_BABE_DEAD_FEED);
    assert_eq!(openbsd.pledge_promises.len(), 5);

    // Test DragonFly BSD flavor
    assert!(engine.activate_flavor(KernelFlavorV61::DragonFlyHammer2Varsyms));
    let dragonfly = engine.get_active_config().unwrap();
    assert_eq!(dragonfly.hammer2_pfs_replicas, 5);
    assert_eq!(dragonfly.varsyms_bindings.get("sys").unwrap(), "DragonFly");
}

#[test]
fn test_bootloader_config_v61_generators() {
    let engine = MultiDistroBootloaderConfigV61Engine::new();
    assert_eq!(engine.entries.len(), 5);

    let grub_cfg = engine.generate_grub2_cfg();
    assert!(grub_cfg.contains("menuentry 'SigmaOS Zenith Linux Core 6.14'"));
    assert!(grub_cfg.contains("menuentry 'FreeBSD 15.0+ ZFS Boot Environment'"));

    let limine_cfg = engine.generate_limine_cfg();
    assert!(limine_cfg.contains(":Limine Bootloader Entry"));
    assert!(limine_cfg.contains("PROTOCOL=stivale2"));
}
