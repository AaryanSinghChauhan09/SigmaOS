//! Integration test for Sovereign Toolchain and Development Advancements PR Proposal Suite V80

extern crate alloc;

use alloc::string::ToString;
use alloc::vec;

#[path = "../src/toolchain/sovereign_toolchain_development_advancements_v80_pr.rs"]
mod sovereign_toolchain_development_advancements_v80_pr;

use sovereign_toolchain_development_advancements_v80_pr::*;

#[test]
fn test_toolchain_development_advancements_v80_integration() {
    let gateway = SovereignToolchainDevelopmentParityV80Gateway::new();

    // 1. Verify Compiler and Linker Engine
    let gcc_flags = gateway.compiler_engine.generate_gcc_flags();
    assert!(gcc_flags.contains(&"-O3".to_string()));
    assert!(gcc_flags.contains(&"-D_FORTIFY_SOURCE=3".to_string()));
    assert!(gcc_flags.contains(&"-fuse-ld=mold".to_string()));

    let clang_flags = gateway.compiler_engine.generate_clang_flags();
    assert!(clang_flags.contains(&"-flto=thin".to_string()));

    let bolt_spec = gateway
        .compiler_engine
        .generate_bolt_reorder_spec("/usr/bin/sigmaos");
    assert!(bolt_spec.contains("llvm-bolt"));
    assert!(bolt_spec.contains("--reorder-blocks=ext-tsp"));

    let polly_flags = gateway.compiler_engine.generate_llvm_polly_flags();
    assert!(polly_flags.contains(&"-polly".to_string()));

    let mlgo_flags = gateway.compiler_engine.generate_mlgo_inline_flags();
    assert!(mlgo_flags.contains(&"-enable-ml-inliner=release".to_string()));

    let mold_rsp = gateway.compiler_engine.generate_mold_response_file(
        "libsigma.so",
        &["core.o", "net.o"],
        &["c", "pthread"],
    );
    assert!(mold_rsp.contains("-o libsigma.so"));
    assert!(mold_rsp.contains("-lpthread"));

    let ctf_count = gateway
        .compiler_engine
        .parse_ctf_compact_c_type_info(&[0xf2, 0xd9, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    assert_eq!(ctf_count, 1);

    let btf_valid = gateway
        .compiler_engine
        .parse_btf_header(&[0x9f, 0xeb, 1, 0, 0, 0, 0, 0]);
    assert!(btf_valid);

    let comp_profile = gateway.compiler_engine.estimate_compiler_optimization(
        ToolchainOptModeV80::ClearLinuxO3,
        ToolchainHardeningModeV80::FedoraFortifyLevel3,
    );
    assert_eq!(comp_profile.estimated_speedup_pct, 40);

    // 2. Verify Build Systems & pkg-config Engine
    let fifo = gateway
        .build_engine
        .make_jobserver_fifo_spec("/run/make_fifo", 16);
    assert!(fifo.contains("-j16"));

    let file_api = gateway.build_engine.cmake_file_api_query_spec();
    assert!(file_api.contains("codemodel"));

    let gen_expr = gateway
        .build_engine
        .evaluate_cmake_generator_expression("$<CONFIG:Release>", "Release");
    assert_eq!(gen_expr, "1");

    let presets = gateway
        .build_engine
        .parse_cmake_presets_v10("{\"configurePresets\": [{\"name\": \"ci-linux\"}]}");
    assert_eq!(presets, vec!["ci-linux"]);

    let nodes = vec![NinjaBuildGraphNodeV80 {
        name: "compile".to_string(),
        inputs: vec!["sigma.c".to_string()],
        outputs: vec!["sigma.o".to_string()],
        command: "gcc -c sigma.c".to_string(),
    }];
    let comp_db = gateway
        .build_engine
        .ninja_generate_compilation_db(&nodes, "/workspace");
    assert!(comp_db.contains("\"file\": \"sigma.c\""));

    let wrap_pkg = gateway
        .build_engine
        .meson_resolve_wrap_subproject("sqlite3", "3.45.0");
    assert!(wrap_pkg.contains("[wrap-file]"));

    let autoconf_cache = gateway
        .build_engine
        .autoconf_preseed_cache("x86_64-sigmaos");
    assert!(autoconf_cache
        .iter()
        .any(|line| line.contains("ac_cv_host=x86_64-sigmaos")));

    let pkg_conf = gateway
        .build_engine
        .pkgconf_multiarch_resolver("libuv", "x86_64-sigma-elf");
    assert_eq!(pkg_conf.sysroot, "/usr/x86_64-sigma-elf");

    // 3. Verify Debuggers, Tracing & Profilers Engine
    let rsp_pack = gateway.debug_engine.gdb_encode_rsp_packet("Hg0");
    assert!(rsp_pack.starts_with("$Hg0#"));
    assert_eq!(
        gateway.debug_engine.gdb_decode_rsp_packet(&rsp_pack),
        Some("Hg0".to_string())
    );

    // DWARF stack op: lit0 (0), consts 10 (10), plus (10), consts 3 (3), mul (30)
    let dwarf_ops = [0x1e, 0x11, 10, 0x22, 0x11, 3, 0x24];
    assert_eq!(
        gateway.debug_engine.lldb_evaluate_dwarf_expr(&dwarf_ops),
        Ok(30)
    );

    let ebpf_rule = gateway.debug_engine.strace_ebpf_filter_rule("read", 9999);
    assert!(ebpf_rule.contains("sys_enter_read"));

    let plt_hook = gateway.debug_engine.ltrace_plt_hook("write");
    assert!(plt_hook.contains("hook_plt_trampoline(\"write\""));

    let leak_cat = gateway
        .debug_engine
        .valgrind_categorize_error("<error>Leak_DefinitelyLost 100 bytes</error>");
    assert_eq!(leak_cat, "Definitely Lost (Memory Leak)");

    let (ret, bad, fe, be) = gateway
        .debug_engine
        .perf_topdown_v5_metrics(1000, 350, 150, 200);
    assert!((ret - 35.0).abs() < 0.01);
    assert!((bad - 15.0).abs() < 0.01);
    assert!((fe - 20.0).abs() < 0.01);
    assert!((be - 30.0).abs() < 0.01);

    let usdt = gateway
        .debug_engine
        .format_dtrace_usdt_probe("kernel", "vfs", "lookup", "entry");
    assert_eq!(usdt, "kernel:vfs:lookup:entry");

    let ktrace_str = gateway.debug_engine.parse_ktrace_sysrel_event(101, 12, -1);
    assert!(ktrace_str.contains("ret=-1"));

    let ptrace_str = gateway.debug_engine.format_ptrace_seize_opts(0x10);
    assert_eq!(ptrace_str, "PTRACE_SEIZE opts=0x10");

    // 4. Verify Distributed VCS & Git Topology Engine
    let oid = [0xabu8; 20];
    let reftable_rec = gateway
        .vcs_engine
        .format_git_reftable_record("refs/heads/master", &oid, 42);
    assert!(reftable_rec.contains("ref=refs/heads/master"));
    assert!(reftable_rec.contains("update_index=42"));

    let midx_bitmap = [0x1u64];
    assert!(gateway.vcs_engine.git_midx_bitmap_lookup(0, &midx_bitmap));
    assert!(!gateway.vcs_engine.git_midx_bitmap_lookup(1, &midx_bitmap));

    assert!(gateway
        .vcs_engine
        .evaluate_partial_clone_filter("blob:none", "tree", 100));
    assert!(!gateway
        .vcs_engine
        .evaluate_partial_clone_filter("blob:none", "blob", 100));

    let cone = ["src", "kernel"];
    assert!(gateway
        .vcs_engine
        .sparse_checkout_cone_match("src/lib.rs", &cone));
    assert!(!gateway
        .vcs_engine
        .sparse_checkout_cone_match("docs/info.txt", &cone));

    let jj_bridge = gateway
        .vcs_engine
        .jujutsu_git_commit_bridge("change_1", "git_commit_1");
    assert!(jj_bridge.contains("co-located"));

    let sendbug =
        gateway
            .vcs_engine
            .format_sendbug_patch("toolchain", "New V80 toolchain engine", "+// v80");
    assert!(sendbug.contains("SEND-PR: Category: toolchain"));

    // 5. Verify Language Runtimes & Polyglot Engine
    let vendor_res =
        gateway
            .runtimes_engine
            .cargo_vendor_mirror_verify("tokio", "sha123", b"cargo-archive");
    assert!(vendor_res.is_valid);

    let cranelift_flags = gateway.runtimes_engine.rustc_cranelift_codegen_flags();
    assert!(cranelift_flags.contains(&"-Zcodegen-backend=cranelift".to_string()));

    let go_pgo = gateway
        .runtimes_engine
        .build_go_pgo_profile("production.pprof");
    assert!(go_pgo.contains("go build -pgo=production.pprof"));

    let cgo_res = gateway
        .runtimes_engine
        .go_cgo_zero_copy_bridge_dispatch(0x4000, 1024);
    assert_eq!(cgo_res, Ok(1024));

    let py_free = gateway
        .runtimes_engine
        .python_freethreaded_evaluator(true, 8);
    assert!(py_free.contains("GIL disabled"));

    let py_aot = gateway
        .runtimes_engine
        .python_bytecode_aot_compile("script.py", 1);
    assert_eq!(py_aot, "script.pyc (opt_level=1)");

    let perl_cow = gateway
        .runtimes_engine
        .perl_cow_sv_inspector("buffer", true);
    assert!(perl_cow.contains("Cow=true"));

    let ruby_jit = gateway.runtimes_engine.ruby_yjit_profile_evaluator(100);
    assert!(ruby_jit.contains("100 instructions"));

    let shenandoah_flags = gateway
        .runtimes_engine
        .openjdk_shenandoah_pause_time_tuner(5);
    assert!(shenandoah_flags.contains(&"-XX:ShenandoahMaxPause=5".to_string()));

    let leyden_flags = gateway
        .runtimes_engine
        .openjdk_leyden_aot_config("shared.aot");
    assert!(leyden_flags.contains(&"-XX:AOTCache=shared.aot".to_string()));

    let graal_img = gateway
        .runtimes_engine
        .openjdk_graalvm_native_image_optimize(100_000);
    assert_eq!(graal_img.estimated_startup_ms, 6);

    let zig_eval = gateway.runtimes_engine.polyglot_runtime_evaluator("zig");
    assert!(zig_eval.contains("Zig 0.15"));

    let swift_eval = gateway.runtimes_engine.polyglot_runtime_evaluator("swift");
    assert!(swift_eval.contains("Swift 6.1"));

    // 6. Verify Master Proposals and Markdown Generation
    let proposals = gateway.collect_all_proposals();
    assert_eq!(proposals.len(), 5);
    assert_eq!(proposals[0].pr_number, 6031);
    assert_eq!(proposals[1].pr_number, 6032);
    assert_eq!(proposals[2].pr_number, 6033);
    assert_eq!(proposals[3].pr_number, 6034);
    assert_eq!(proposals[4].pr_number, 6035);

    let summary = gateway.generate_master_summary();
    assert!(summary.contains("SIGMAOS TOOLCHAIN & DEVELOPMENT PARITY PR PROPOSAL SUMMARY (V80)"));
    assert!(summary.contains("Total PR Proposals: 5"));

    let md = gateway.generate_pr_proposal_markdown();
    assert!(
        md.contains("# Sovereign Toolchain and Development Ecosystem Parity PR Proposals (V80)")
    );
    assert!(md.contains("## PR #6031:"));
    assert!(md.contains("## PR #6032:"));
    assert!(md.contains("## PR #6033:"));
    assert!(md.contains("## PR #6034:"));
    assert!(md.contains("## PR #6035:"));
}
