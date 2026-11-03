// Integration Test Suite for Sovereign C Runtime and Linker Advancements (v80 PR Suite)

extern crate alloc;

use alloc::string::String;
use alloc::vec;

#[path = "../src/runtime/c_runtime_and_linker_advancements_v80_pr.rs"]
mod c_runtime_and_linker_advancements_v80_pr;

use c_runtime_and_linker_advancements_v80_pr::*;

#[test]
fn test_integration_c_runtime_v80_proposals() {
    let gateway = SovereignCRuntimeAndLinkerParityV80Gateway::new();
    let proposals = gateway.collect_all_proposals();

    assert_eq!(proposals.len(), 10);

    // PR #5111: glibc
    assert_eq!(proposals[0].pr_number, 5111);
    assert_eq!(proposals[0].target_subsystem, "libc/glibc");
    assert_eq!(proposals[0].status, "Ready for Merge");
    assert!(proposals[0]
        .components_added
        .contains(&String::from("DT_GNU_HASH 64-bit Dual Bloom Filter")));
    assert!(proposals[0]
        .components_added
        .contains(&String::from("IFUNC AVX10.2 & APX 32-reg Resolver")));

    // PR #5112: musl libc
    assert_eq!(proposals[1].pr_number, 5112);
    assert_eq!(proposals[1].target_subsystem, "libc/musl");
    assert!(proposals[1]
        .components_added
        .contains(&String::from("malloc-ng Slab Group Checksum Verifier")));
    assert!(proposals[1]
        .components_added
        .contains(&String::from("Zero-Allocation Signal-Safe snprintf")));

    // PR #5113: uClibc-ng
    assert_eq!(proposals[2].pr_number, 5113);
    assert_eq!(proposals[2].target_subsystem, "libc/uclibc");
    assert!(proposals[2]
        .components_added
        .contains(&String::from("FD-PIC Function Descriptor Relocator")));

    // PR #5114: BSD libc
    assert_eq!(proposals[3].pr_number, 5114);
    assert_eq!(proposals[3].target_subsystem, "libc/bsd_libc");
    assert!(proposals[3]
        .components_added
        .contains(&String::from("arc4random ChaCha20 Stir Counter")));

    // PR #5115: Android Bionic
    assert_eq!(proposals[4].pr_number, 5115);
    assert_eq!(proposals[4].target_subsystem, "libc/bionic");
    assert!(proposals[4]
        .components_added
        .contains(&String::from("Scudo Chunk Cookie Header Verifier")));

    // PR #5116: Newlib
    assert_eq!(proposals[5].pr_number, 5116);
    assert_eq!(proposals[5].target_subsystem, "libc/newlib");
    assert!(proposals[5]
        .components_added
        .contains(&String::from("struct _reent Context Manager")));

    // PR #5117: Cosmopolitan Libc
    assert_eq!(proposals[6].pr_number, 5117);
    assert_eq!(proposals[6].target_subsystem, "libc/cosmopolitan");
    assert!(proposals[6]
        .components_added
        .contains(&String::from("APE 3.1 Polyglot Header Validator")));

    // PR #5118: GNU libstdc++
    assert_eq!(proposals[7].pr_number, 5118);
    assert_eq!(proposals[7].target_subsystem, "cxx/libstdc++");
    assert!(proposals[7]
        .components_added
        .contains(&String::from("Itanium C++ ABI __cxa_demangle")));

    // PR #5119: LLVM libc++
    assert_eq!(proposals[8].pr_number, 5119);
    assert_eq!(proposals[8].target_subsystem, "cxx/libc++");
    assert!(proposals[8]
        .components_added
        .contains(&String::from("Fast SIMD to_chars & from_chars")));

    // PR #5120: Dynamic Linker ld.so
    assert_eq!(proposals[9].pr_number, 5120);
    assert_eq!(proposals[9].target_subsystem, "linker/ld.so");
    assert!(proposals[9]
        .components_added
        .contains(&String::from("Search Hierarchy DT_RUNPATH Resolver")));

    let markdown = gateway.generate_pr_proposal_markdown();
    assert!(markdown.contains(
        "# Pull Request Proposal: C Library, C++ Runtime, and Dynamic Linker Parity (V80)"
    ));
    assert!(markdown.contains("### PR #5111:"));
    assert!(markdown.contains("### PR #5120:"));
}

#[test]
fn test_subengine_functional_capabilities_v80() {
    // 1. glibc: DT_GNU_HASH and IFUNC resolution
    let hash = GlibcInspirationsV80Engine::compute_gnu_hash("strcmp");
    let (b1, b2) = GlibcInspirationsV80Engine::compute_gnu_hash_bloom(hash, 6);
    assert_ne!(b1, 0);
    assert_ne!(b2, 0);
    let resolved = GlibcInspirationsV80Engine::resolve_ifunc_symbol(&["apx", "avx10_2"], "strchr");
    assert_eq!(resolved, "strchr_apx_avx10_2");
    let tunables = GlibcInspirationsV80Engine::parse_glibc_tunables(
        "glibc.cpu.x86_shared_cache_size=33554432",
    );
    assert_eq!(
        tunables,
        vec![("glibc.cpu.x86_shared_cache_size", "33554432")]
    );

    // 2. musl: malloc-ng slab groups & zero-allocation signal-safe format string
    let (sc, slot, units) = MuslLibcInspirationsV80Engine::calculate_malloc_ng_group(16);
    assert_eq!(sc, 0);
    assert!(slot < 16);
    assert_eq!(units, 1);
    let mut msg_buf = [0u8; 16];
    let n = MuslLibcInspirationsV80Engine::signal_safe_snprintf(&mut msg_buf, "SIG=", 11);
    assert_eq!(&msg_buf[..n], b"SIG=11");

    // 3. uClibc-ng: FD-PIC relocation and bFLT header check
    let (got, entry) =
        UclibcInspirationsV80Engine::relocate_fdpic_descriptor(0x4000_0000, 0x1000, 0x2000);
    assert_eq!(got, 0x4000_1000);
    assert_eq!(entry, 0x4000_2000);
    assert!(UclibcInspirationsV80Engine::validate_bflt_header(b"bFLT", 4, 0).is_ok());

    // 4. BSD libc: ChaCha20 keystream, memory barriers, strlcpy/strlcat, pledge/unveil
    let seed = [0x42u8; 32];
    let (ks, stir) =
        BsdLibcInspirationsV80Engine::generate_arc4random_chacha20_keystream(&seed, 8, 100);
    assert_eq!(ks.len(), 8);
    assert_eq!(stir, 108);
    let mut scratch = [0xffu8; 8];
    assert_eq!(
        BsdLibcInspirationsV80Engine::freezero_secure(&mut scratch),
        8
    );
    assert_eq!(scratch, [0u8; 8]);
    assert!(BsdLibcInspirationsV80Engine::evaluate_pledge_unveil_policy(
        "stat",
        "/usr/bin/cat"
    ));

    // 5. Bionic: Scudo, ARM PAC/BTI, fdsan, properties
    assert!(BionicInspirationsV80Engine::verify_scudo_chunk_header(
        0xbeef,
        0xcafe,
        0xbeef ^ 0xcafe
    ));
    assert_eq!(
        BionicInspirationsV80Engine::query_android_property("ro.product.model"),
        Some("Pixel 10 Pro".to_string())
    );
    assert!(BionicInspirationsV80Engine::fdsan_owner_tag_check(
        0xdead, 0xdead
    ));

    // 6. Newlib: reent context, nano-malloc, low power syscall
    let ctx = NewlibInspirationsV80Engine::manage_reent_context(2, 0);
    assert_eq!(ctx, "reent_thread_2_errno_0");
    assert_eq!(
        NewlibInspirationsV80Engine::nano_malloc_split_alloc(512, 128, 16),
        Ok((368, 144))
    );

    // 7. Cosmopolitan: APE binary validation, POSIX-to-Win32 mapping
    assert!(CosmopolitanInspirationsV80Engine::validate_ape_binary(
        b"\x7fELF............"
    ));
    assert_eq!(
        CosmopolitanInspirationsV80Engine::dispatch_fat_elf_arch(&["x86_64", "aarch64"]),
        "x86_64_fat_section"
    );

    // 8. libstdc++: demangler, SSO threshold, PMR buffer
    assert_eq!(
        LibstdcxxInspirationsV80Engine::demangle_cxx_symbol("_ZN3std6vectorE"),
        "3std6vector::"
    );
    assert_eq!(
        LibstdcxxInspirationsV80Engine::determine_sso_layout(15),
        "inline_sso"
    );
    assert_eq!(
        LibstdcxxInspirationsV80Engine::determine_sso_layout(16),
        "heap_allocated"
    );

    // 9. libc++: fast SIMD <charconv>, futex atomic wait, weak/shared ptr
    assert_eq!(
        LibcxxInspirationsV80Engine::fast_to_chars_u64_sim(999),
        "999"
    );
    assert_eq!(
        LibcxxInspirationsV80Engine::fast_from_chars_u64_sim("12345"),
        Ok(12345)
    );
    assert_eq!(
        LibcxxInspirationsV80Engine::update_ref_count(2, 1, "dec_shared"),
        (1, 1)
    );

    // 10. Dynamic Linker: search path, DF_1 flags, LD_PRELOAD interposition
    let paths = DynamicLinkerLdSoInspirationsV80Engine::resolve_search_path_order(
        "/opt/lib",
        "/custom/lib",
        "",
    );
    assert_eq!(paths[0], "/custom/lib");
    assert_eq!(paths[1], "/opt/lib");
    assert_eq!(
        DynamicLinkerLdSoInspirationsV80Engine::evaluate_ld_preload_interposition(
            &["liballoc.so"],
            "free"
        ),
        Some("interposed_by_preload")
    );
}
