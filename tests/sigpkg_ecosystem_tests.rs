// Comprehensive unit & integration tests for the 20-pillar Sigpkg Package & Build System

use sigmaos::sigpkg::*;
use std::collections::BTreeMap;

#[test]
fn test_pillar1_sigpkg_header_spec() {
    let checksum = [0xAB; 32];
    let signature = [0xCD; 64];
    let header = SigpkgHeader::new(1024, 2048, checksum, signature);

    assert!(header.verify_magic());
    assert_eq!(header.compression, SigpkgCompression::Zstd);
    assert_eq!(header.payload_size, 1024);
    assert_eq!(header.uncompressed_size, 2048);
}

#[test]
fn test_pillar2_central_repo_and_cdn_mirrors() {
    let mut repo = CentralRepositoryManager::new();
    repo.add_mirror("https://us.repo.sigmaos.org", "US-East", 45);
    repo.add_mirror("https://eu.repo.sigmaos.org", "EU-West", 20);
    repo.add_mirror("https://asia.repo.sigmaos.org", "Asia-East", 120);

    let fastest = repo.select_fastest_mirror().unwrap();
    assert_eq!(fastest.region, "EU-West");
    assert_eq!(fastest.latency_ms, 20);

    repo.add_trusted_gpg_key([0x55; 32]);
    assert_eq!(repo.gpg_keyring.len(), 1);
}

#[test]
fn test_pillar3_reproducible_build_context() {
    let context = ReproducibleBuildContext::new(1700000000);
    let src_hash = [0x11; 32];
    let mut env_vars = BTreeMap::new();
    env_vars.insert("CC".to_string(), "gcc".to_string());

    let deriv_a = context.compute_derivation_hash(&src_hash, &env_vars);
    let deriv_b = context.compute_derivation_hash(&src_hash, &env_vars);

    assert_eq!(deriv_a, deriv_b);
}

#[test]
fn test_pillar4_source_first_builder() {
    let mut builder = SourceFirstBuilder::new(BuildPreference::BinaryCachePreferred);
    let deriv_hash = [0x22; 32];
    let artifact = vec![1, 2, 3, 4, 5];

    builder.store_binary_cache(deriv_hash, artifact.clone());

    let result = builder
        .fetch_or_build(&deriv_hash, || Err("Should not build from source"))
        .unwrap();

    assert_eq!(result, artifact);
}

#[test]
fn test_pillar5_deterministic_dependency_resolver() {
    let mut resolver = DeterministicDependencyResolver::new();
    resolver.add_package_spec(
        "zenith",
        PackageRequirement {
            name: "zenith".to_string(),
            version_min: (1, 0, 0),
            conflicts_with: vec!["legacy-gui".to_string()],
        },
    );

    let resolved = resolver.resolve_dependencies(&["zenith"]).unwrap();
    assert_eq!(resolved, vec!["zenith".to_string()]);

    let mut conflicting_resolver = DeterministicDependencyResolver::new();
    conflicting_resolver.add_package_spec(
        "app_a",
        PackageRequirement {
            name: "app_a".to_string(),
            version_min: (1, 0, 0),
            conflicts_with: vec!["app_b".to_string()],
        },
    );
    conflicting_resolver.add_package_spec(
        "app_b",
        PackageRequirement {
            name: "app_b".to_string(),
            version_min: (1, 0, 0),
            conflicts_with: vec![],
        },
    );

    let err = conflicting_resolver
        .resolve_dependencies(&["app_b", "app_a"])
        .unwrap_err();
    assert_eq!(err.package_a, "app_a");
    assert_eq!(err.package_b, "app_b");
}

#[test]
fn test_pillar6_atomic_transaction_rollback() {
    let mut engine = AtomicTransactionEngine::new();
    let gen2 = engine.commit_transaction(vec!["curl".to_string(), "git".to_string()], 100);
    assert_eq!(gen2, 2);

    let gen3 = engine.commit_transaction(vec!["curl".to_string(), "git".to_string(), "vim".to_string()], 200);
    assert_eq!(gen3, 3);

    let rolled_back = engine.rollback_generation(2).unwrap();
    assert_eq!(rolled_back.installed_packages, vec!["curl".to_string(), "git".to_string()]);
    assert_eq!(engine.active_generation, 2);
}

#[test]
fn test_pillar7_binary_delta_updates() {
    let old_binary = b"SIGMA_OS_V1_CORE_PAYLOAD";
    let new_binary = b"SIGMA_OS_V2_CORE_PAYLOAD";

    let diff = BinaryDeltaGenerator::create_diff(old_binary, new_binary);
    let patched = BinaryDeltaGenerator::apply_patch(old_binary, &diff);

    assert_eq!(&patched[..new_binary.len()], new_binary);
}

#[test]
fn test_pillar8_package_build_sandbox() {
    let policy = SandboxPolicy {
        isolate_network: true,
        isolate_pid: true,
        isolate_ipc: true,
        read_only_root: true,
    };
    let engine = BuildSandboxEngine::new(policy);

    let status = engine.execute_sandboxed_build(|| true).unwrap();
    assert!(status.contains("cleanly"));
}

#[test]
fn test_pillar9_cross_compile_toolchain() {
    let toolchain = CrossCompileToolchain::new(
        TargetArchitecture::X86_64,
        TargetArchitecture::AArch64,
        "/sysroot/aarch64",
    );

    assert_eq!(toolchain.get_target_triple(), "aarch64-sigmaos-linux-gnu");
}

#[test]
fn test_pillar10_slsa_provenance_attestation() {
    let attestation = SlsaProvenanceAttestation::new(
        "builder.sigmaos.org",
        "https://github.com/sigmaos/kernel",
        "a1b2c3d4e5f6",
        1700000000,
    );

    assert!(attestation.verify_provenance());
    assert_eq!(attestation.commit_sha, "a1b2c3d4e5f6");
}

#[test]
fn test_pillar11_local_package_proxy_cache() {
    let mut cache = LocalPackageProxyCache::new();
    let url = "https://cdn.sigmaos.org/pkgs/bash-5.2.tar.gz";

    let download_fn = || Ok(vec![0xAA, 0xBB, 0xCC]);

    let res1 = cache.get_or_download(url, download_fn).unwrap();
    assert_eq!(res1, vec![0xAA, 0xBB, 0xCC]);
    assert_eq!(cache.total_misses, 1);

    let res2 = cache.get_or_download(url, download_fn).unwrap();
    assert_eq!(res2, vec![0xAA, 0xBB, 0xCC]);
    assert_eq!(cache.total_hits, 1);
}

#[test]
fn test_pillar12_vulnerability_scanner() {
    let mut scanner = VulnerabilityScanner::new();
    scanner.add_cve("CVE-2023-12345", "openssl", 9);
    scanner.add_cve("CVE-2023-67890", "curl", 5);

    let vulns = scanner.scan_package("openssl");
    assert_eq!(vulns.len(), 1);
    assert_eq!(vulns[0].cve_id, "CVE-2023-12345");
    assert_eq!(vulns[0].severity, 9);
}

#[test]
fn test_pillar13_build_farm_automation() {
    let mut farm = BuildFarmManager::new();
    farm.register_worker(1, TargetArchitecture::X86_64);
    farm.register_worker(2, TargetArchitecture::RiscV64);

    let worker_id = farm.schedule_build(TargetArchitecture::RiscV64).unwrap();
    assert_eq!(worker_id, 2);

    let busy_err = farm.schedule_build(TargetArchitecture::RiscV64);
    assert!(busy_err.is_err());
}

#[test]
fn test_pillar14_unified_runtime_manager() {
    let mut manager = UnifiedRuntimeManager::new();
    manager.set_runtime_version(LanguageRuntime::Python, "3.11.4");
    manager.set_runtime_version(LanguageRuntime::NodeJS, "20.5.0");

    assert_eq!(manager.get_runtime_version(LanguageRuntime::Python), Some("3.11.4"));
    assert_eq!(manager.get_runtime_version(LanguageRuntime::NodeJS), Some("20.5.0"));
    assert_eq!(manager.get_runtime_version(LanguageRuntime::Java), None);
}

#[test]
fn test_pillar15_flatpak_container_integration() {
    let mut integration = FlatpakContainerIntegration::new("org.gimp.GIMP", ApplicationType::FlatpakSandbox);
    integration.add_permission("--socket=x11");
    integration.add_permission("--filesystem=home");

    assert_eq!(integration.sandbox_flags.len(), 2);
    assert_eq!(integration.app_type, ApplicationType::FlatpakSandbox);
}

#[test]
fn test_pillar16_package_quality_gates() {
    assert!(PackageQualityChecker::check_quality("git", "GPL-2.0", true).is_ok());
    assert!(PackageQualityChecker::check_quality("", "GPL-2.0", true).is_err());
    assert!(PackageQualityChecker::check_quality("git", "", true).is_err());
    assert!(PackageQualityChecker::check_quality("git", "GPL-2.0", false).is_err());
}

#[test]
fn test_pillar17_binary_compatibility_layer() {
    let bcl = BinaryCompatibilityLayer::new(CRuntimeProvider::Glibc);
    assert_eq!(
        bcl.resolve_symbol_shim("__libc_start_main"),
        Some("sovereign_libc_start_main")
    );
    assert_eq!(bcl.resolve_symbol_shim("malloc"), Some("sovereign_malloc"));
    assert_eq!(bcl.resolve_symbol_shim("unknown_sym"), None);
}

#[test]
fn test_pillar18_developer_package_templates() {
    let cmake_template = DeveloperPackageTemplateManager::generate_spec_template("libxyz", TemplateKind::CCppCmake);
    assert!(cmake_template.contains("name = \"libxyz\""));
    assert!(cmake_template.contains("build_system = \"cmake\""));

    let cargo_template = DeveloperPackageTemplateManager::generate_spec_template("mycrate", TemplateKind::RustCargo);
    assert!(cargo_template.contains("build_system = \"cargo\""));
}

#[test]
fn test_pillar19_package_analytics_dashboard() {
    let mut dashboard = PackageAnalyticsDashboard::new();
    dashboard.record_download("coreutils", 5000);
    dashboard.record_download("coreutils", 5000);

    assert_eq!(dashboard.get_total_downloads("coreutils"), 2);
    assert_eq!(dashboard.bandwidth_bytes_served, 10000);
}

#[test]
fn test_pillar20_migration_tooling() {
    let deb_control = "Package: nginx\nVersion: 1.24.0-1\nArchitecture: amd64\n";
    let deb_sigpkg = LegacyPackageMigrator::convert_deb_control(deb_control).unwrap();
    assert!(deb_sigpkg.contains("name = \"nginx\""));
    assert!(deb_sigpkg.contains("version = \"1.24.0-1\""));
    assert!(deb_sigpkg.contains("converted_from = \"debian\""));

    let pkgbuild = "pkgname=\"htop\"\npkgver=\"3.2.2\"\n";
    let arch_sigpkg = LegacyPackageMigrator::convert_arch_pkgbuild(pkgbuild).unwrap();
    assert!(arch_sigpkg.contains("name = \"htop\""));
    assert!(arch_sigpkg.contains("converted_from = \"arch\""));

    let fedora_spec = "Name: curl\nVersion: 8.1.2\n";
    let fedora_sigpkg = LegacyPackageMigrator::convert_fedora_spec(fedora_spec).unwrap();
    assert!(fedora_sigpkg.contains("name = \"curl\""));
    assert!(fedora_sigpkg.contains("converted_from = \"fedora\""));
}
