//! Integration test for C Library, C++ Runtime, and Dynamic Linker Parity Advancements V91

extern crate alloc;

use alloc::string::ToString;
use alloc::vec;

#[path = "../src/runtime/c_runtime_and_linker_advancements_v91_pr.rs"]
mod c_runtime_and_linker_advancements_v91_pr;

use c_runtime_and_linker_advancements_v91_pr::*;

#[test]
fn test_c_runtime_and_linker_advancements_v91_integration() {
    let gateway = SovereignCRuntimeAndLinkerParityV91Gateway::new();

    // 1. Verify PR proposal manifests collection (#5121 to #5130)
    let proposals = gateway.collect_all_proposals();
    assert_eq!(proposals.len(), 10);

    for (idx, p) in proposals.iter().enumerate() {
        let expected_pr = 5121 + idx as u32;
        assert_eq!(p.pr_number, expected_pr);
        assert!(!p.pr_title.is_empty());
        assert!(!p.target_subsystem.is_empty());
        assert!(!p.original_runtime.is_empty());
        assert!(!p.components_added.is_empty());
        assert!(!p.performance_multiplier.is_empty());
        assert_eq!(p.status, "Ready for Merge");
    }

    // 2. Verify glibc 2.46+ engine
    let hash = GlibcInspirationsV91Engine::compute_gnu_hash("malloc");
    assert_ne!(hash, 0);
    let (b1, b2) = GlibcInspirationsV91Engine::compute_gnu_hash_bloom(hash, 6);
    assert_ne!(b1, 0);
    assert_ne!(b2, 0);
    let resolver = GlibcInspirationsV91Engine::resolve_ifunc_symbol(&["apx", "avx10_2"], "memset");
    assert_eq!(resolver, "memset_apx_avx10_2_512");
    let tunables = GlibcInspirationsV91Engine::parse_glibc_tunables(
        "glibc.cpu.x86_ibt=on:glibc.cpu.x86_shstk=on",
    );
    assert_eq!(tunables.len(), 2);
    let (dtv_offset, next_gen) =
        GlibcInspirationsV91Engine::calculate_dtv_generation(1, 5, 128, 32);
    assert_eq!(dtv_offset, 160);
    assert_eq!(next_gen, 6);
    let xml = GlibcInspirationsV91Engine::generate_malloc_info_v2_xml(4096, 16, 4, 2);
    assert!(xml.contains("version=\"2\""));
    assert_eq!(
        GlibcInspirationsV91Engine::evaluate_malloc_trim(5000, 2000),
        3000
    );

    // 3. Verify musl libc 1.3.0+ engine
    let (class, slot, units) = MuslLibcInspirationsV91Engine::calculate_malloc_ng_group(128);
    assert_eq!(class, 3);
    assert!(slot < 16);
    assert_eq!(units, 8);
    assert!(MuslLibcInspirationsV91Engine::verify_malloc_ng_header(
        0xDEAD_BEEF,
        0x2000,
        0xDEAD_BEEF ^ 0x2000
    ));
    let mut buf = [0u8; 64];
    let written = MuslLibcInspirationsV91Engine::signal_safe_snprintf(&mut buf, "signo=", 11);
    assert_eq!(written, 8);
    assert_eq!(&buf[..8], b"signo=11");
    let (synced, ok) = MuslLibcInspirationsV91Engine::simulate_synccall_barrier(4, 33);
    assert_eq!(synced, 4);
    assert!(ok);
    let unmap_eval = MuslLibcInspirationsV91Engine::evaluate_unmapself_stack(0x8000_0000, 16384);
    assert_eq!(unmap_eval, "unmap_range(0x80000000, 16384)");
    let dns_queries =
        MuslLibcInspirationsV91Engine::res_msend_query(&["internal", "lan"], "cluster");
    assert_eq!(dns_queries.len(), 2);

    // 4. Verify uClibc-ng 1.0.55+ engine
    let (got, entry) =
        UclibcInspirationsV91Engine::relocate_fdpic_descriptor(0x4000_0000, 0x1000, 0x2000);
    assert_eq!(got, 0x4000_1000);
    assert_eq!(entry, 0x4000_2000);
    let active_feats = UclibcInspirationsV91Engine::strip_unsupported_target_features(
        &["mmu", "fpu", "dsp"],
        0b101,
    );
    assert_eq!(active_feats, vec!["mmu", "dsp"]);
    let (sysv_h, bucket) = UclibcInspirationsV91Engine::compute_sysv_elf_hash("strcpy");
    assert_ne!(sysv_h, 0);
    assert!(bucket < 1024);
    assert_eq!(
        UclibcInspirationsV91Engine::validate_bflt_header(b"bFLT", 5, 0x02),
        Ok("bFLT_v5_flags_0x2".to_string())
    );

    // 5. Verify BSD libc engine
    let seed = [0x42u8; 32];
    let (stream, stir_out) =
        BsdLibcInspirationsV91Engine::generate_arc4random_chacha20_keystream(&seed, 32, 100);
    assert_eq!(stream.len(), 32);
    assert_eq!(stir_out, 132);
    let mut scrub_buf = [0xffu8; 16];
    let scrubbed = BsdLibcInspirationsV91Engine::freezero_secure(&mut scrub_buf);
    assert_eq!(scrubbed, 16);
    assert_eq!(scrub_buf, [0u8; 16]);
    let mut dst = [0u8; 12];
    let lcpy_len = BsdLibcInspirationsV91Engine::strlcpy_safe(&mut dst, "SigmaOS");
    assert_eq!(lcpy_len, 7);
    assert_eq!(&dst[..7], b"SigmaOS");
    let lcat_len = BsdLibcInspirationsV91Engine::strlcat_safe(&mut dst, " C", 12);
    assert_eq!(lcat_len, 9);
    assert_eq!(&dst[..9], b"SigmaOS C");
    assert_eq!(
        BsdLibcInspirationsV91Engine::reallocarray_overflow_check(100, 100),
        Ok(10000)
    );
    assert!(BsdLibcInspirationsV91Engine::evaluate_pledge_unveil_policy(
        "read",
        "/usr/bin/app"
    ));
    assert!(!BsdLibcInspirationsV91Engine::evaluate_pledge_unveil_policy("open", "/etc/shadow"));
    assert_eq!(
        BsdLibcInspirationsV91Engine::parse_sysctl_mib("kern.securelevel"),
        vec![1, 9]
    );

    // 6. Verify Android Bionic engine
    assert!(BionicInspirationsV91Engine::verify_scudo_chunk_header(
        0xAAAA,
        0x5555,
        0xAAAA ^ 0x5555
    ));
    assert!(BionicInspirationsV91Engine::verify_arm_pac_bti_tags(
        0x0000_0001_2345_6789,
        0xffff_0000_0000_0000
    ));
    assert_eq!(
        BionicInspirationsV91Engine::query_android_property("ro.product.model"),
        Some("Pixel 10 Pro".to_string())
    );
    assert!(BionicInspirationsV91Engine::fdsan_owner_tag_check(
        0x1234, 0x1234
    ));
    assert_eq!(
        BionicInspirationsV91Engine::async_safe_format_log("I", "bionic", "booted"),
        "[I][bionic] booted"
    );
    let bionic_flags = BionicInspirationsV91Engine::parse_android_dlopen_flags(0x03);
    assert_eq!(
        bionic_flags,
        vec![
            "ANDROID_DLEXT_USE_NAMESPACE",
            "ANDROID_DLEXT_USE_LIBRARY_FD"
        ]
    );

    // 7. Verify Newlib engine
    assert_eq!(
        NewlibInspirationsV91Engine::manage_reent_context(2, 0),
        "reent_thread_2_errno_0"
    );
    assert_eq!(
        NewlibInspirationsV91Engine::nano_malloc_split_alloc(2048, 512, 16),
        Ok((1520, 528))
    );
    assert_eq!(
        NewlibInspirationsV91Engine::low_power_syscall_bridge(0x10),
        0x8000_0010
    );
    assert!(NewlibInspirationsV91Engine::soft_float_sin_approx(256) > 0);

    // 8. Verify Cosmopolitan Libc engine
    assert!(CosmopolitanInspirationsV91Engine::validate_ape_binary(
        b"\x7fELF............."
    ));
    assert_eq!(
        CosmopolitanInspirationsV91Engine::map_posix_to_win32_oflag(1),
        0x4000_0000
    );
    assert_eq!(
        CosmopolitanInspirationsV91Engine::dispatch_fat_elf_arch(&["aarch64", "x86_64"]),
        "x86_64_fat_section"
    );

    // 9. Verify GNU libstdc++ engine
    assert_eq!(
        LibstdcxxInspirationsV91Engine::demangle_cxx_symbol("_ZN3std6vectorE"),
        "3std6vector::"
    );
    assert_eq!(
        LibstdcxxInspirationsV91Engine::determine_sso_layout(15),
        "inline_sso"
    );
    assert_eq!(
        LibstdcxxInspirationsV91Engine::determine_sso_layout(16),
        "heap_allocated"
    );
    assert_eq!(
        LibstdcxxInspirationsV91Engine::pmr_allocate_buffer(1024, 256),
        Ok(768)
    );
    assert_eq!(
        LibstdcxxInspirationsV91Engine::lookup_dwarf_eh_frame_unwind(0x4000_1000),
        Some(0x4000_1000 ^ 0x1234)
    );

    // 10. Verify LLVM libc++ engine
    assert_eq!(
        LibcxxInspirationsV91Engine::fast_to_chars_u64_sim(9999),
        "9999"
    );
    assert_eq!(
        LibcxxInspirationsV91Engine::fast_from_chars_u64_sim("9999"),
        Ok(9999)
    );
    assert_eq!(
        LibcxxInspirationsV91Engine::futex_atomic_wait(10, 10),
        "futex_wait_blocked"
    );
    assert_eq!(
        LibcxxInspirationsV91Engine::update_ref_count(2, 1, "dec_shared"),
        (1, 1)
    );
    assert_eq!(
        LibcxxInspirationsV91Engine::parse_format_replacements("A: {}, B: {}, C: {}"),
        3
    );

    // 11. Verify Dynamic Linker ld.so engine
    let search_paths = DynamicLinkerLdSoInspirationsV91Engine::resolve_search_path_order(
        "/opt/lib",
        "/app/lib",
        "/usr/local/lib",
    );
    assert_eq!(search_paths[0], "/app/lib");
    assert_eq!(search_paths[1], "/opt/lib");
    let flags1 = DynamicLinkerLdSoInspirationsV91Engine::parse_dt_flags_1(0x0000_0009);
    assert_eq!(flags1, vec!["DF_1_NOW", "DF_1_NODELETE"]);
    assert_eq!(
        DynamicLinkerLdSoInspirationsV91Engine::evaluate_ld_preload_interposition(
            &["libshim.so"],
            "free"
        ),
        Some("interposed_by_preload")
    );
    assert_eq!(
        DynamicLinkerLdSoInspirationsV91Engine::execute_rtld_audit_hooks("la_objsearch"),
        "rtld_audit_la_objsearch_handled"
    );
    assert!(DynamicLinkerLdSoInspirationsV91Engine::verify_gnu_relro_protection(1));
    assert!(
        DynamicLinkerLdSoInspirationsV91Engine::match_elf_symbol_versions(
            "LIBC_2.46",
            &["LIBC_2.46", "LIBC_2.45"]
        )
    );

    // 12. Verify PR Proposal Markdown Report
    let markdown = gateway.generate_pr_proposal_markdown();
    assert!(markdown.contains(
        "# Pull Request Proposal: C Library, C++ Runtime, and Dynamic Linker Parity (V91)"
    ));
    assert!(markdown.contains("PR #5121"));
    assert!(markdown.contains("PR #5130"));
    assert!(markdown.contains("glibc 2.46+"));
    assert!(markdown.contains("musl libc 1.3.0+"));
    assert!(markdown.contains("uClibc-ng 1.0.55+"));
}
