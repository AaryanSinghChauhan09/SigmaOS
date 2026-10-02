// SPDX-License-Identifier: MIT
// Comprehensive Verification Test Suite for Sovereign Linux & BSD Distros, Tech Media & GitHub Wiki Unimplemented Ideas

use sigmaos::distro::sovereign_linux_bsd_media_wiki_unimplemented_ideas_engine::*;

#[test]
fn test_tech_media_portal_intelligence_feed_full_coverage() {
    let mut feed = TechMediaPortalIntelligenceFeed::new();
    assert_eq!(feed.total_portals_count(), 33);

    let portal_keys = [
        "9to5google",
        "9to5linux",
        "9to5mac",
        "androidauthority",
        "androidpolice",
        "appuals",
        "distrowatch",
        "frappe",
        "geekygadgets",
        "hwbusters",
        "howtogeek",
        "infoworld",
        "itsfoss",
        "itdaily",
        "kdnuggets",
        "linuxdotcom",
        "linuxorg",
        "linuxfoundation",
        "linuxteck",
        "makeuseof",
        "marktechpost",
        "opensourceforu",
        "pcmag",
        "pcworld",
        "phoronix",
        "techcrunch",
        "techpowerup",
        "techspot",
        "thenewstack",
        "windowscentral",
        "windowslatest",
        "xdadevelopers",
        "zdnet",
    ];

    for key in &portal_keys {
        let p = feed.get_portal(key);
        assert!(p.is_some(), "Portal missing: {}", key);
        let p_ref = p.unwrap();
        assert!(!p_ref.name.is_empty());
        assert!(!p_ref.canonical_url.is_empty());
        assert!(!p_ref.primary_feature_absorbed.is_empty());
        assert!(p_ref.intelligence_feed_items > 0);
    }

    assert!(feed.ingest_article("9to5linux", "Kernel 6.12 Sched_ext Release"));
    assert!(!feed.ingest_article("9to5linux", "Kernel 6.12 Sched_ext Release"));
    // Duplicate check
}

#[test]
fn test_linux_bsd_distro_unimplemented_ideas_suite() {
    let mut suite = LinuxBsdDistroUnimplementedIdeasSuite::new();
    assert!(suite.openbsd_pledge(&["stdio", "rpath"]).is_ok());
    assert!(suite.openbsd_unveil("/var/log", "rw").is_ok());

    assert!(suite.solve_gentoo_use_flags(&["x265", "vaapi"], &["x265"]));
}

#[test]
fn test_github_wiki_and_md_roadmap_fulfillment_suite() {
    let mut suite = SigmaOsGithubWikiAndMdRoadmapFulfillmentSuite::new();
    assert!(suite.verify_fulfillment());

    // EAS core selection
    let core = suite.eas_router.select_core_for_task(true, 800);
    assert!(core >= 4);

    // GPUDirect DMA descriptor
    let desc_res = suite
        .gpudirect_dma
        .build_dma_descriptor(0x1000_0000, 0xd000_0000, 2048);
    assert!(desc_res.is_ok());

    // eBPF relative jmp calculation
    let jmp = suite
        .ebpf_trampoline
        .generate_x86_relative_jmp(0x1000, 0x5000);
    assert_eq!(jmp[0], 0xE9);

    // PQC Kyber peer
    let ct = vec![0x99u8; 1024];
    assert!(suite.pqc_vpn.register_pqc_peer("kyber-peer-01", &ct));

    // CAS Store generation & rollback
    let gen_id = suite.cas_store.commit_new_generation("hash_gen_002");
    assert_eq!(gen_id, 2);
    let rollback_path = suite.cas_store.rollback_to_generation(1).unwrap();
    assert!(rollback_path.contains("default-1-link"));
}

#[test]
fn test_sovereign_linux_bsd_media_wiki_unimplemented_ideas_master_suite() {
    let master = SovereignLinuxBsdMediaWikiUnimplementedIdeasMasterSuite::new();
    assert!(master.health_check());

    let summary = master.summary_report();
    assert!(summary.contains("33 / 33"));
    assert!(summary.contains("CAS Generations"));
}
