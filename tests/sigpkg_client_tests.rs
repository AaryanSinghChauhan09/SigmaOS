// Integration tests for the SigmaPkg TUF-style repository client.
use sigmaos::sigpkg::client::{parse_manifest, Manifest, SigpkgClient, TufRole};
use sigmaos::sigpkg::{CryptoVerifier, Version};

#[test]
fn test_sigpkg_client_repository_flow() {
    let mut client = SigpkgClient::new("https://repo.sigmaos.dev/sigma");
    client.add_trusted_key("root-key");

    // Fetch + verify signed root metadata.
    let payload = b"{\"role\":\"root\",\"version\":1}";
    let sig = client.verifier.sign("root-key", payload);
    assert!(client.fetch_metadata(TufRole::Root, payload, &sig));
    assert!(client.metadata.contains_key("root"));

    // An unsigned timestamp must be rejected.
    assert!(!client.fetch_metadata(TufRole::Timestamp, payload, &[]));
    assert!(!client.metadata.contains_key("timestamp"));
}

#[test]
fn test_sigpkg_client_install_from_manifest() {
    let mut client = SigpkgClient::new("https://repo.sigmaos.dev/sigma");
    client.add_trusted_key("pkg-key");

    let payload: &[u8] = b"hello-package-bytes";
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in payload.iter() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    let checksum = format!("{:x}", h);

    let manifest = parse_manifest(&format!(
        "name: hello\nversion: 2.0.0\ndescription: hello util\nchecksum: {}\ndependencies:\n",
        checksum
    ))
    .expect("manifest parses");

    let installed = std::collections::BTreeMap::new();
    let hash = client
        .install_from_manifest(&manifest, payload, &installed)
        .expect("verified install succeeds");
    assert!(!hash.is_empty());
    assert_eq!(client.installed_packages().len(), 1);
    assert_eq!(client.installed_packages()[0].0, "hello");
    assert_eq!(client.installed_packages()[0].1, "2.0.0");

    // Tampered payload must be rejected (checksum mismatch).
    assert!(client
        .install_from_manifest(&manifest, b"tampered-payload", &installed)
        .is_err());
}

#[test]
fn test_sigpkg_manifest_build_and_resolve() {
    let mut manifest = Manifest::new("app", Version::new(3, 0, 0), "app desc", "beef");
    manifest.add_dependency("libfoo");
    manifest.add_dependency("missing-pkg");

    let mut installed = std::collections::BTreeMap::new();
    installed.insert("libfoo".to_string(), Version::new(1, 5, 0));

    let pkg = manifest.to_package(&installed);
    assert_eq!(pkg.dependencies.len(), 2);
    assert_eq!(
        pkg.dependencies[0].version_constraint,
        sigmaos::sigpkg::VersionConstraint::Exact(Version::new(1, 5, 0))
    );
    assert_eq!(
        pkg.dependencies[1].version_constraint,
        sigmaos::sigpkg::VersionConstraint::Any
    );
}

#[test]
fn test_sigpkg_manifest_roundtrip() {
    let verifier = CryptoVerifier::new();
    let sig = verifier.sign("some-key", b"meta");
    assert!(!sig.is_empty());
}

#[test]
fn test_sigpkg_daemon_sync_verify_and_gc() {
    let mut daemon = sigmaos::sigpkg::SigpkgDaemon::new("https://repo.sigmaos.dev/sigma");
    daemon.add_trusted_key("root-key");
    let payload: &[u8] = b"root-metadata";
    let sig = daemon.verifier().sign("root-key", payload);

    assert!(matches!(
        daemon.sync_repository(payload, &sig),
        sigmaos::sigpkg::SyncStatus::Synced { .. }
    ));
    // Unsigned must fail all-or-nothing.
    assert!(matches!(
        daemon.sync_repository(payload, &[]),
        sigmaos::sigpkg::SyncStatus::Failed { .. }
    ));

    // Deploy two packages; keep one referenced and GC the other.
    let installed = std::collections::BTreeMap::new();
    let p1: &[u8] = b"keep-bytes";
    let mut h1: u64 = 0xcbf29ce484222325;
    for &b in p1.iter() {
        h1 ^= b as u64;
        h1 = h1.wrapping_mul(0x100000001b3);
    }
    let m1 = sigmaos::sigpkg::parse_manifest(&format!(
        "name: keep\nversion: 1.0.0\ndescription: x\nchecksum: {:x}\ndependencies:\n",
        h1
    ))
    .unwrap();
    daemon.deploy(&m1, p1, &installed).unwrap();

    let p2: &[u8] = b"orphan-bytes";
    let mut h2: u64 = 0xcbf29ce484222325;
    for &b in p2.iter() {
        h2 ^= b as u64;
        h2 = h2.wrapping_mul(0x100000001b3);
    }
    let m2 = sigmaos::sigpkg::parse_manifest(&format!(
        "name: orphan\nversion: 1.0.0\ndescription: x\nchecksum: {:x}\ndependencies:\n",
        h2
    ))
    .unwrap();
    daemon.deploy(&m2, p2, &installed).unwrap();

    daemon.mark_referenced("keep");
    assert_eq!(daemon.gc_store(), 1);
    assert!(daemon.client.store.get("keep").is_some());
    assert!(daemon.client.store.get("orphan").is_none());
}

#[test]
fn test_sigpkg_daemon_update_check() {
    let mut daemon = sigmaos::sigpkg::SigpkgDaemon::default();
    let installed = std::collections::BTreeMap::new();
    let p: &[u8] = b"hello-bytes";
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in p.iter() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    let m = sigmaos::sigpkg::parse_manifest(&format!(
        "name: hello\nversion: 1.0.0\ndescription: x\nchecksum: {:x}\ndependencies:\n",
        h
    ))
    .unwrap();
    daemon.deploy(&m, p, &installed).unwrap();

    let mut repo = std::collections::BTreeMap::new();
    repo.insert(
        "hello".to_string(),
        sigmaos::sigpkg::Manifest::new(
            "hello",
            sigmaos::sigpkg::Version::new(2, 0, 0),
            "x",
            "dead",
        ),
    );
    let updates = daemon.check_updates(&repo);
    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].name, "hello");
    assert_eq!(updates[0].installed, sigmaos::sigpkg::Version::new(1, 0, 0));
    assert_eq!(updates[0].available, sigmaos::sigpkg::Version::new(2, 0, 0));
}

#[test]
fn test_foundation_governance_and_bounties() {
    use sigmaos::compatibility::{BountySeverity, FoundationRole, SovereignFoundationManager};
    let mut foundation = SovereignFoundationManager::new("SigmaOS Foundation");
    foundation.register_member("alice", FoundationRole::BoardMember);
    foundation.register_member("bob", FoundationRole::SecurityAuditor);
    assert_eq!(foundation.members.len(), 2);
    // Re-registration updates the role in place.
    foundation.register_member("alice", FoundationRole::CoreMaintainer);
    assert_eq!(foundation.members.len(), 2);
    assert_eq!(foundation.members[0].1, FoundationRole::CoreMaintainer);

    let bounty_id = foundation.submit_security_bounty(
        "Kernel heap overflow in VFS",
        "bob_security",
        BountySeverity::Critical,
    );
    assert_eq!(bounty_id, 1);
    assert_eq!(foundation.bounties[0].reward_usd, 15000);
    assert_eq!(foundation.resolve_bounty(1).unwrap(), 15000);
    assert!(foundation.resolve_bounty(1).is_err());

    foundation.organize_hackathon("Global Kernel Hackathon 2026", "Zero-Copy IPC");
    assert!(foundation
        .register_hackathon_participant("Global Kernel Hackathon 2026", "charlie")
        .is_ok());
    assert!(foundation
        .submit_hackathon_project("Global Kernel Hackathon 2026", "SovereignRingFS")
        .is_ok());
    assert_eq!(foundation.hackathons[0].projects_submitted.len(), 1);
}

#[test]
fn test_universal_package_adapter_multi_distro_conversions() {
    use sigmaos::sigpkg::{UniversalPackageAdapter, UniversalDependencyMapper, UniversalDryRunSimulator};
    use sigmaos::sigpkg::universal_engine::PackageFormat;

    let adapter = UniversalPackageAdapter::new();
    let dep_mapper = UniversalDependencyMapper::new();

    // 1. Debian/APT control conversion
    let deb_manifest = "Package: nginx\nVersion: 1.24.0-1\nDepends: libssl-dev, libc6\nDescription: Nginx HTTP server\n";
    let deb_pkg = adapter.parse_and_translate_manifest("nginx.deb", deb_manifest).unwrap();
    assert_eq!(deb_pkg.name, "nginx");
    assert_eq!(dep_mapper.to_canonical_name(&deb_pkg.dependencies[0].name), "openssl");

    // 2. Arch Linux PKGBUILD conversion
    let pkgbuild_manifest = "pkgname=ripgrep\npkgver=14.1.0\ndepends=('pcre2' 'libc6')\n";
    let arch_pkg = adapter.parse_and_translate_manifest("PKGBUILD", pkgbuild_manifest).unwrap();
    assert_eq!(arch_pkg.name, "ripgrep");

    // 3. FreeBSD +MANIFEST conversion
    let fbsd_manifest = "name: curl\nversion: 8.5.0\ncomment: command line tool\ndeps: {\n  openssl: {origin: \"security/openssl\"}\n}\n";
    let fbsd_pkg = adapter.parse_and_translate_manifest("curl.pkg", fbsd_manifest).unwrap();
    assert_eq!(fbsd_pkg.name, "curl");

    // 4. OpenBSD +CONTENTS conversion
    let obsd_manifest = "@name wget-1.21.4\n@comment retrieval tool\n@depend security/openssl:openssl-*:openssl->=3.0\n";
    let obsd_pkg = adapter.parse_and_translate_manifest("wget.pkg", obsd_manifest).unwrap();
    assert_eq!(obsd_pkg.name, "wget");

    // 5. Dry-run simulator
    let simulator = UniversalDryRunSimulator::new();
    let dry_run = simulator.simulate_install(PackageFormat::Apt, deb_manifest.as_bytes()).unwrap();
    assert!(dry_run.is_valid);
    assert_eq!(dry_run.package_name, "nginx");
    assert_eq!(dry_run.resolved_dependencies.len(), 2);
}
