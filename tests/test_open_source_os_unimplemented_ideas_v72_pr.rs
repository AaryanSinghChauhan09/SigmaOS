// SPDX-License-Identifier: MIT
// Integration Test for Open Source OS Unimplemented Ideas V72 PR Suite// (`tests/test_open_source_os_unimplemented_ideas_v72_pr.rs`)

#[path = "../src/distro/open_source_os_unimplemented_ideas_v72_pr.rs"]
mod open_source_os_unimplemented_ideas_v72_pr;

use open_source_os_unimplemented_ideas_v72_pr::*;

#[test]
fn test_open_source_os_unimplemented_ideas_v72_suite() {
    let suite = OpenSourceOsUnimplementedIdeasV72PrSuite::new();
    let specs = suite.collect_all_pr_specs();

    assert_eq!(specs.len(), 9);

    for spec in &specs {
        assert_eq!(spec.status, OsPrStatusV72::Validated);
        assert!(spec.parity_score >= 95);
        assert!(!spec.diff_patch.is_empty());
    }

    // Verify individual engines
    let mut sched = Linux629SchedExtFolioCompactionV72PrEngine::new();
    assert_eq!(sched.compact_folios(), 128);
    let capsicum = FreeBsd161CapsicumCasperNvmeTlsV72PrEngine::new();
    assert!(capsicum.check_casper_right("system.dns"));
    assert!(capsicum.handshake_nvme_tls("session_v72"));

    let openbsd = OpenBsd89FineIbtPinsyscallUnveilV72PrEngine::new();
    assert!(openbsd.validate_pinsyscall(0x7fff0600));    assert!(openbsd.check_unveil_access("/usr/bin", 'r'));

    let rump = NetBsd125RumpPciVirtio9pV72PrEngine::new();
    let packet = [0, 0, 0, 0, 10, 0x12, 0x34];
    assert_eq!(rump.parse_9p_header(&packet), Some((10, 0x3412)));

    let hammer3 = DragonFlyHammer3ErasureDirectIoV72PrEngine::new();    assert!(hammer3.sync_pfs_delta(1, 2));
    assert!(hammer3.verify_scrub_checksum("SHA256:abc123v72"));

    let zfs = IllumosOpenZfsDtraceEbpfV72PrEngine::new();
    let key = [7u8; 32];
    let encrypted = zfs.encrypt_block_aes_gcm(b"hello v72", &key);
    assert_eq!(encrypted.len(), 9);

    let befs = HaikuBeFsRelationalLiveQueryV72PrEngine::new();    assert!(befs.query_index("META:name"));

    let plumber = Plan99p2000LPlumberV72PrEngine::new();
    assert_eq!(
        plumber.route_plumber_message("https://sigmaos.org"),
        Some("dispatch_browser")
    );

    let redox = RedoxFsHermeticCasCowCapabilityV72PrEngine::new();    assert!(redox.verify_scheme_capability("file:"));
}
