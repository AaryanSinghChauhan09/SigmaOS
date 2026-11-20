//! Integration tests: package signature and integrity verification.
//!
//! Covers the [`PackageVerifier`] contract:
//!   - Checksum matching (happy path and mismatch)
//!   - Missing metadata rejection (name, checksum fields)
//!   - Untrusted packages (no registered checksum) still pass if checksum is present
//!   - Multiple packages verified independently (no cross-contamination)

use sigmaos_packages::{Package, PackageVerifier, Version, VerifyError};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn pkg(name: &str, checksum: &str) -> Package {
    Package::new(name, Version::new(1, 0, 0)).with_checksum(checksum)
}

// ---------------------------------------------------------------------------
// Happy-path
// ---------------------------------------------------------------------------

#[test]
fn verifies_package_with_trusted_matching_checksum() {
    // Arrange
    let mut v = PackageVerifier::new();
    v.trust("libssl", "aabbcc112233");

    // Act & Assert
    assert!(v.verify(&pkg("libssl", "aabbcc112233")).is_ok());
}

#[test]
fn verifies_package_with_no_registered_trust_entry() {
    // A package whose name is not in the trust store is still accepted as long
    // as its checksum field is non-empty (defensive: presence check).
    let v = PackageVerifier::new();
    assert!(v.verify(&pkg("unknown-pkg", "anynonempychecksum")).is_ok());
}

#[test]
fn verifies_multiple_packages_independently() {
    // Arrange
    let mut v = PackageVerifier::new();
    v.trust("pkgA", "aaaa");
    v.trust("pkgB", "bbbb");

    // Act & Assert
    assert!(v.verify(&pkg("pkgA", "aaaa")).is_ok());
    assert!(v.verify(&pkg("pkgB", "bbbb")).is_ok());
}

// ---------------------------------------------------------------------------
// Checksum mismatch
// ---------------------------------------------------------------------------

#[test]
fn rejects_package_with_wrong_checksum() {
    // Arrange
    let mut v = PackageVerifier::new();
    v.trust("libssl", "correct_checksum");

    // Act
    let err = v.verify(&pkg("libssl", "wrong_checksum")).unwrap_err();

    // Assert
    assert!(
        matches!(
            err,
            VerifyError::ChecksumMismatch { ref name, .. } if name == "libssl"
        ),
        "expected ChecksumMismatch for libssl, got {err:?}"
    );
}

#[test]
fn checksum_mismatch_contains_both_expected_and_actual() {
    // Arrange
    let mut v = PackageVerifier::new();
    v.trust("pkg", "expected");

    // Act
    let err = v.verify(&pkg("pkg", "actual")).unwrap_err();

    // Assert
    if let VerifyError::ChecksumMismatch { expected, actual, .. } = err {
        assert_eq!(expected, "expected");
        assert_eq!(actual, "actual");
    } else {
        panic!("wrong error variant");
    }
}

// ---------------------------------------------------------------------------
// Missing metadata
// ---------------------------------------------------------------------------

#[test]
fn rejects_package_with_empty_name() {
    // Arrange
    let v = PackageVerifier::new();
    let p = Package::new("", Version::new(1, 0, 0)).with_checksum("ck");

    // Act
    let err = v.verify(&p).unwrap_err();

    // Assert
    assert!(
        matches!(err, VerifyError::MissingMetadata { ref field, .. } if field == "name"),
        "expected MissingMetadata(name), got {err:?}"
    );
}

#[test]
fn rejects_package_with_empty_checksum() {
    // Arrange
    let v = PackageVerifier::new();
    let p = Package::new("mypkg", Version::new(1, 0, 0)); // no .with_checksum()

    // Act
    let err = v.verify(&p).unwrap_err();

    // Assert
    assert!(
        matches!(err, VerifyError::MissingMetadata { ref field, .. } if field == "checksum"),
        "expected MissingMetadata(checksum), got {err:?}"
    );
}

// ---------------------------------------------------------------------------
// Trust-store isolation
// ---------------------------------------------------------------------------

#[test]
fn updating_trust_does_not_affect_already_verified_results() {
    // Arrange
    let mut v = PackageVerifier::new();
    v.trust("lib", "original");

    // Verify with correct checksum
    assert!(v.verify(&pkg("lib", "original")).is_ok());

    // Update trust store
    v.trust("lib", "updated");

    // Old checksum should now fail
    let err = v.verify(&pkg("lib", "original")).unwrap_err();
    assert!(matches!(err, VerifyError::ChecksumMismatch { .. }));

    // New checksum should pass
    assert!(v.verify(&pkg("lib", "updated")).is_ok());
}

#[test]
fn trust_store_is_per_package_not_global() {
    // Trusting pkgA with a checksum must not affect pkgB verification.
    let mut v = PackageVerifier::new();
    v.trust("pkgA", "aaaa");

    // pkgB has no trust entry — any non-empty checksum passes
    assert!(v.verify(&pkg("pkgB", "bbbb")).is_ok());

    // pkgA with wrong checksum must still fail
    let err = v.verify(&pkg("pkgA", "bbbb")).unwrap_err();
    assert!(matches!(err, VerifyError::ChecksumMismatch { .. }));
}
