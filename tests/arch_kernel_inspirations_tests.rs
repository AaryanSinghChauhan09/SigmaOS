extern crate alloc;

use alloc::vec::Vec;

use sigmaos::arch_kernel_inspirations::{
    AdvisorySeverity, AlpmAction, AlpmPackage, AlpmResolutionError, AlpmTransactionEngine,
    Expectation, ExpectationKind, HookAction, KUnitEngine, MkinitcpioHookFramework,
    PackageSignoff, RebuildOrderSolver, ReproducibleBuildVerdict, ReproducibleStatus,
    SecurityAdvisory, SecurityAdvisoryTracker, Signer, SignerPolicy, SignstarService,
};

#[test]
fn kunit_engine_reports_failures() {
    let mut eng = KUnitEngine::new();
    let eval = eng.evaluate(ExpectationKind::Eq, "42", "42", "foo.c", 10);
    assert!(eval.passed);
    let cases: Vec<(String, Box<dyn FnOnce(&mut Vec<Expectation>) + Send>)> = vec![
        (
            "test_ok".to_string(),
            Box::new(|e: &mut Vec<Expectation>| {
                e.push(eng_eval(ExpectationKind::Eq, "1", "1"));
            }),
        ),
        (
            "test_bad".to_string(),
            Box::new(|e: &mut Vec<Expectation>| {
                e.push(eng_eval(ExpectationKind::True, "false", "true"));
            }),
        ),
    ];
    let result = eng.run_suite("kernel_drivers", cases);
    assert_eq!(result.passed, 1);
    assert_eq!(result.failed, 1);
    assert_eq!(eng.total_failed(), 1);
}

fn eng_eval(kind: ExpectationKind, left: &str, right: &str) -> Expectation {
    Expectation {
        kind,
        left: left.to_string(),
        right: right.to_string(),
        file: "t.c".to_string(),
        line: 1,
        passed: match kind {
            ExpectationKind::Eq => left == right,
            ExpectationKind::True => left == "true",
            _ => false,
        },
    }
}

#[test]
fn alpm_transaction_resolves_dependencies_and_commits() {
    let mut eng = AlpmTransactionEngine::new();
    eng.seed_available(vec![
        AlpmPackage {
            name: "libc".into(),
            version: "2.0".into(),
            depends: vec![],
            provides: vec![],
            conflicts: vec![],
            replaces: vec![],
            files: vec!["/usr/lib/libc.so".into()],
        },
        AlpmPackage {
            name: "app".into(),
            version: "1.0".into(),
            depends: vec!["libc".into()],
            provides: vec![],
            conflicts: vec![],
            replaces: vec![],
            files: vec!["/usr/bin/app".into()],
        },
        AlpmPackage {
            name: "proxy".into(),
            version: "1.0".into(),
            depends: vec![],
            provides: vec!["virtual-net".into()],
            conflicts: vec![],
            replaces: vec![],
            files: vec!["/usr/bin/proxy".into()],
        },
    ]);
    assert!(eng.add_install("app").is_ok());
    // Provide-name resolution: `vnc` maps to the `virtual-vnc` provider.
    assert!(eng.add_install("virtual-net").is_ok());
    assert_eq!(eng.resolve_dependencies().unwrap(), 1);
    assert!(eng.detect_file_conflicts().is_empty());
    assert!(eng.prepare().is_ok());
    let committed = eng.commit().unwrap();
    assert!(committed >= 3);
    assert!(eng.installed.iter().any(|p| p.name == "app"));
}

#[test]
fn alpm_rejects_missing_dependency() {
    let mut eng = AlpmTransactionEngine::new();
    eng.seed_available(vec![AlpmPackage {
        name: "needy".into(),
        version: "1.0".into(),
        depends: vec!["missing-lib".into()],
        provides: vec![],
        conflicts: vec![],
        replaces: vec![],
        files: vec![],
    }]);
    assert!(eng.add_install("needy").is_ok());
    assert_eq!(
        eng.resolve_dependencies().unwrap_err(),
        AlpmResolutionError::MissingDependency
    );
}

#[test]
fn security_tracker_flags_cves_and_upgrades() {
    let mut t = SecurityAdvisoryTracker::new();
    t.add(SecurityAdvisory {
        cve: "CVE-2026-1001".into(),
        package: "openssl".into(),
        affected_versions: vec!["1.1.1".into(), "3.0.0".into()],
        fixed_version: Some("3.0.1".into()),
        severity: AdvisorySeverity::Critical,
        description: "remote buffer overflow".into(),
    });
    t.add(SecurityAdvisory {
        cve: "CVE-2026-1002".into(),
        package: "bash".into(),
        affected_versions: vec!["5.1".into()],
        fixed_version: Some("5.2".into()),
        severity: AdvisorySeverity::High,
        description: "arbitrary command injection".into(),
    });
    assert_eq!(t.affected("openssl", "1.1.1").len(), 1);
    assert_eq!(t.affected("openssl", "3.0.1").len(), 0);
    assert_eq!(t.recommended_upgrades("bash", "5.1"), vec!["5.2".to_string()]);
    assert_eq!(t.critical_count(), 1);
}

#[test]
fn signstar_requires_all_mandatory_signers() {
    let mut s = SignstarService::new("core-utils");
    s.add_signer("primary", SignerPolicy::Mandatory);
    s.add_signer("release", SignerPolicy::Mandatory);
    s.add_signer("community", SignerPolicy::Optional);
    s.record_signature("primary");
    assert!(!s.fully_signed);
    s.record_signature("release");
    assert!(s.fully_signed);
    assert!(s.all_mandatory_signed());
}

#[test]
fn mkinitcpio_hooks_build_payload() {
    let mut f = MkinitcpioHookFramework::new();
    f.add_hook(
        "block",
        vec![HookAction::AddModule { module: "virtio_blk".into() }],
    );
    f.add_hook(
        "filesystems",
        vec![
            HookAction::AddModule { module: "ext4".into() },
            HookAction::AddModule { module: "btrfs".into() },
        ],
    );
    f.disable("filesystems");
    let payload = f.build_payload();
    assert_eq!(f.enabled_hook_count(), 1);
    assert!(payload.iter().any(|p| p.contains("virtio_blk")));
    assert!(!payload.iter().any(|p| p.contains("ext4")));
}

#[test]
fn rebuild_order_solves_topologically() {
    let mut solver = RebuildOrderSolver::new();
    solver.add_dependency("app", "libfoo");
    solver.add_dependency("app", "libbar");
    solver.add_dependency("libbar", "libfoo");
    let order = solver.solve().unwrap();
    // libfoo must be built before libbar before app.
    assert_eq!(order[0], "libfoo");
    assert!(order.iter().position(|p| p == "libfoo").unwrap()
        < order.iter().position(|p| p == "libbar").unwrap());
    assert!(order.iter().position(|p| p == "libbar").unwrap()
        < order.iter().position(|p| p == "app").unwrap());
}

#[test]
fn rebuild_order_detects_cycles() {
    let mut solver = RebuildOrderSolver::new();
    solver.add_dependency("a", "b");
    solver.add_dependency("b", "a");
    assert!(solver.solve().is_err());
}

#[test]
fn signoff_needs_community_quorum() {
    let mut s = PackageSignoff::new(2);
    s.register("linux", "6.10");
    assert!(!s.ready("linux"));
    s.sign("linux", false).unwrap();
    assert!(!s.ready("linux"));
    s.sign("linux", false).unwrap();
    assert!(s.ready("linux"));
}

#[test]
fn reproducible_build_verdict_computes_ratio() {
    let mut v = ReproducibleBuildVerdict::new();
    v.record("a", ReproducibleStatus::Reproducible);
    v.record("b", ReproducibleStatus::Reproducible);
    v.record("c", ReproducibleStatus::Unreproducible);
    assert_eq!(v.reproducible_count(), 2);
    assert!((v.ratio() - 2.0f32 / 3.0).abs() < 0.001);
}
