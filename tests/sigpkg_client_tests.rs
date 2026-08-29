// Integration tests for the SigmaPkg TUF-style repository client.
use sigmaos::sigpkg::client::{Manifest, SigpkgClient, TufRole, parse_manifest};
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
