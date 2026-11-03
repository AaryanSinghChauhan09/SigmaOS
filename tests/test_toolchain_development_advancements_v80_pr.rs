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
    let gcc_flags = gateway.gcc_engine.generate_gcc_flags();
    assert!(gcc_flags.contains(&"-O3".to_string()));
    assert!(gcc_flags.contains(&"-D_FORTIFY_SOURCE=3".to_string()));
    assert!(gcc_flags.contains(&"-fuse-ld=mold".to_string()));

    let clang_flags = gateway.gcc_engine.generate_clang_flags();
    assert!(clang_flags.contains(&"-flto=thin".to_string()));

    let bolt_spec = gateway
        .gcc_engine
        .generate_bolt_reorder_spec("/usr/bin/sigmaos");
    assert!(bolt_spec.contains("llvm-bolt"));

    let polly_flags = gateway.gcc_engine.generate_llvm_polly_flags();
    assert!(polly_flags.contains(&"-polly".to_string()));

    let mlgo_flags = gateway.gcc_engine.generate_mlgo_inline_flags();
    assert!(mlgo_flags.contains(&"-enable-ml-inliner=release".to_string()));

    let ctf_count = gateway
        .gcc_engine
        .parse_ctf_compact_c_type_info(&[0xf2, 0xd9, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    assert_eq!(ctf_count, 1);

    let comp_profile = gateway.gcc_engine.estimate_compiler_optimization(
        ToolchainOptModeV80::ClearLinuxO3,
        ToolchainHardeningModeV80::FedoraFortifyLevel3,
    );
    assert_eq!(comp_profile.estimated_speedup_pct, 36);

    // 2. Verify Build Systems & Pkg-Config Engine
    let jobserver_fd = gateway
        .build_engine
        .parse_make_jobserver_flags("MAKEFLAGS=--jobserver-auth=4,5");
    assert_eq!(jobserver_fd, Some(4));

    let acquired_tokens = gateway.build_engine.jobserver_acquire_tokens(2, 8);
    assert_eq!(acquired_tokens, 6);

    let cmake_targets = gateway
        .build_engine
        .parse_cmake_file_api_reply("{\"name\": \"target1\"}, {\"name\": \"target2\"}");
    assert_eq!(
        cmake_targets,
        vec!["target1".to_string(), "target2".to_string()]
    );

    let genex = gateway
        .build_engine
        .generate_cmake_generator_expression("OPENSSL", "-lssl");
    assert_eq!(genex, "$<$<TARGET_EXISTS:OPENSSL>:-lssl>");

    let ninja_node = NinjaBuildGraphNodeV80 {
        name: "link".to_string(),
        inputs: vec!["a.o".to_string(), "b.o".to_string()],
        outputs: vec!["app".to_string()],
        command: "mold -o app a.o b.o".to_string(),
    };
    assert!(!gateway
        .build_engine
        .detect_ninja_graph_cycles(&[ninja_node]));

    let comp_db =
        gateway
            .build_engine
            .generate_compilation_database_entry("foo.c", "gcc -c foo.c", "/src");
    assert!(comp_db.contains("\"file\": \"foo.c\""));

    let wrap_content = "directory = libz\nurl = https://zlib.net/zlib-1.3.tar.gz\n";
    let (wrap_dir, wrap_url) = gateway
        .build_engine
        .parse_meson_wrap_file(wrap_content)
        .unwrap();
    assert_eq!(wrap_dir, "libz");
    assert_eq!(wrap_url, "https://zlib.net/zlib-1.3.tar.gz");

    let autoconf_cache = gateway.build_engine.generate_autoconf_config_cache("musl");
    assert!(autoconf_cache.contains("musl"));

    let libtool_deps = gateway
        .build_engine
        .parse_libtool_la_dependency_libs("dependency_libs=' -lz -lssl '");
    assert_eq!(libtool_deps, vec!["-lz".to_string(), "-lssl".to_string()]);

    let paths = gateway
        .build_engine
        .resolve_multiarch_pkg_config_paths("x86_64-sigmaos");
    assert_eq!(paths.len(), 4);

    let pkg_provider = gateway
        .build_engine
        .pkgconf_virtual_provider_lookup("wayland");
    assert_eq!(pkg_provider, "wayland-client");

    let pkg_query = gateway
        .build_engine
        .query_pkg_config("glib-2.0", "x86_64-sigmaos");
    assert_eq!(pkg_query.package_name, "glib-2.0");

    // 3. Verify Debuggers, Tracing & Profilers Engine
    let (rsp_payload, rsp_checksum) = gateway
        .debug_engine
        .parse_gdb_rsp_packet("$Hg0#db")
        .unwrap();
    assert_eq!(rsp_payload, "Hg0");
    assert_eq!(rsp_checksum, "db");
    assert_eq!(gateway.debug_engine.format_gdb_rsp_ack(true), "+");
    assert_eq!(gateway.debug_engine.format_gdb_rsp_ack(false), "-");

    let sbapi_expr = gateway
        .debug_engine
        .format_lldb_sbapi_expression("process.id");
    assert!(sbapi_expr.contains("process.id"));

    let (sc_name, sc_args) = gateway
        .debug_engine
        .parse_strace_ebpf_syscall_args("mprotect(0x1000, 1024, 0x7)")
        .unwrap();
    assert_eq!(sc_name, "mprotect");
    assert_eq!(sc_args, vec![0x1000, 1024, 0x7]);

    let (func, ret) = gateway
        .debug_engine
        .parse_ltrace_plt_call("getpid() -> 42")
        .unwrap();
    assert_eq!(func, "getpid()");
    assert_eq!(ret, "42");

    let valgrind_leak = gateway
        .debug_engine
        .evaluate_valgrind_memcheck_leak(4096, 1);
    assert!(valgrind_leak.contains("4096 bytes"));

    let (retiring, bad_spec, fe, be) = gateway
        .debug_engine
        .parse_perf_topdown_metrics(200, 80, 20, 40, 60);
    assert!((retiring - 40.0).abs() < 0.001);
    assert!((bad_spec - 10.0).abs() < 0.001);
    assert!((fe - 20.0).abs() < 0.001);
    assert!((be - 30.0).abs() < 0.001);

    let flame = gateway
        .debug_engine
        .generate_perf_flamegraph_ascii(&[("kernel;sched;idle", 15)]);
    assert!(flame.contains("kernel;sched;idle 15"));

    let usdt_probe = gateway
        .debug_engine
        .format_dtrace_usdt_probe("sigma", "fs", "lookup", "hit");
    assert_eq!(usdt_probe, "sigma:fs:lookup:hit");

    let ktrace_entry = gateway.debug_engine.parse_ktrace_sysrel_event(2048, 12, 0);
    assert!(ktrace_entry.contains("pid=2048"));

    let ptrace_str = gateway.debug_engine.format_ptrace_seize_opts(0x80);
    assert_eq!(ptrace_str, "PTRACE_SEIZE opts=0x80");

    // 4. Verify VCS & Git Topology Engine
    let midx_lookup = gateway
        .vcs_engine
        .git_midx_reachability_bitmap_lookup("deadbeef");
    assert!(midx_lookup.is_some());

    let pack_search = gateway
        .vcs_engine
        .git_packfile_binary_search_object(&[0x10, 0x20], 500);
    assert!(pack_search.is_some());

    let reftable_entry = gateway
        .vcs_engine
        .parse_git_reftable_ref_entry("refs/tags/v80.0.0 a1b2c3d4")
        .unwrap();
    assert_eq!(reftable_entry.0, "refs/tags/v80.0.0");
    assert_eq!(reftable_entry.1, "a1b2c3d4");

    let jj_hash = gateway
        .vcs_engine
        .jujutsu_change_id_to_git_oid_bridge("zyxw987");
    assert_eq!(jj_hash, "jj-rev-zyxw987");

    let commits = [
        ("init", &[][..]),
        ("feat1", &["init"][..]),
        ("feat2", &["feat1"][..]),
    ];
    let sorted_commits = gateway.vcs_engine.topological_commit_sort(&commits);
    assert_eq!(sorted_commits.len(), 3);

    // 5. Verify Polyglot Language Runtimes Engine
    let py_flags = gateway
        .polyglot_engine
        .python_free_threaded_jit_config(true);
    assert!(py_flags.contains(&"jit=on".to_string()));
    assert!(py_flags.contains(&"gil=0".to_string()));

    let pyc = gateway
        .polyglot_engine
        .python_bytecode_aot_compile("/usr/lib/sigma.py", 3);
    assert_eq!(pyc, "/usr/lib/sigma.pyc (opt_level=3)");

    let perl_xs = gateway
        .polyglot_engine
        .perl_xs_c_bridge_generate("Sigma::IO");
    assert!(perl_xs.contains("Sigma::IO"));
    assert!(gateway
        .polyglot_engine
        .perl_cpan_vendor_sandbox_verify("Net::SSLeay"));

    let ruby_yjit = gateway.polyglot_engine.ruby_yjit_profile_evaluator(50);
    assert!(ruby_yjit.contains("50ms"));

    let cargo_sbom = gateway
        .polyglot_engine
        .cargo_auditable_sbom_json_generate("sigma-kernel", "0.80.0");
    assert!(cargo_sbom.contains("sigma-kernel"));

    let cargo_verify =
        gateway
            .polyglot_engine
            .cargo_vendor_mirror_verify("syn", "checksum-syn", b"blob");
    assert!(cargo_verify.is_valid);

    let go_pgo = gateway
        .polyglot_engine
        .build_go_pgo_profile("profiles/cpu.pprof");
    assert!(go_pgo.contains("-pgo=profiles/cpu.pprof"));

    let cgo_bridge = gateway
        .polyglot_engine
        .go_cgo_zero_copy_bridge_dispatch(0x4000, 1024);
    assert_eq!(cgo_bridge, Ok(1024));

    let shenandoah = gateway
        .polyglot_engine
        .openjdk_shenandoah_pause_time_tuner(5);
    assert!(shenandoah.contains(&"-XX:MaxGCPauseMillis=5".to_string()));

    let zgc = gateway.polyglot_engine.openjdk_graalvm_zgc_config(8192);
    assert!(zgc.contains(&"-Xmx8192m".to_string()));

    let graal_native = gateway
        .polyglot_engine
        .openjdk_graalvm_native_image_optimize(20_000_000);
    assert_eq!(graal_native.estimated_startup_ms, 7);

    let zig_runner = gateway.polyglot_engine.zig_build_runner("aarch64-linux");
    assert!(zig_runner.contains("zig build -Dtarget=aarch64-linux"));

    let polyglot_eval = gateway.polyglot_engine.polyglot_runtime_evaluator("ocaml");
    assert!(polyglot_eval.contains("OCaml 5.3"));

    let bun_eval = gateway.polyglot_engine.bun_deno_runtime_evaluator("deno");
    assert!(bun_eval.contains("deno V8 isolate engine"));

    // 6. Verify Master Gateway and PR Proposal Manifests
    let proposals = gateway.collect_all_proposals();
    assert_eq!(proposals.len(), 5);
    assert_eq!(proposals[0].pr_number, 6031);
    assert_eq!(proposals[1].pr_number, 6032);
    assert_eq!(proposals[2].pr_number, 6033);
    assert_eq!(proposals[3].pr_number, 6034);
    assert_eq!(proposals[4].pr_number, 6035);

    for p in &proposals {
        assert_eq!(p.status, "Ready for Merge");
        assert!(!p.tools_covered.is_empty());
        assert!(!p.key_features_added.is_empty());
        assert!(!p.distro_inspirations.is_empty());
    }

    let summary = gateway.generate_master_summary();
    assert!(summary.contains("SIGMAOS TOOLCHAIN & DEVELOPMENT PARITY PR PROPOSAL SUMMARY (V80)"));
    assert!(summary.contains("Total PR Proposals: 5"));

    let markdown = gateway.generate_pr_proposal_markdown();
    assert!(markdown
        .contains("# Sovereign Toolchain and Development Ecosystem Parity PR Proposals (V80)"));
    assert!(markdown.contains("PR #6031"));
    assert!(markdown.contains("PR #6032"));
    assert!(markdown.contains("PR #6033"));
    assert!(markdown.contains("PR #6034"));
    assert!(markdown.contains("PR #6035"));
}
