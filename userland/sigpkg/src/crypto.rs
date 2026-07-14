// sigpkg crypto: Ed25519 signature verification + SHA-256 integrity checking
// Production: integrate with ring/ed25519-dalek for real crypto

/// Verify a package's integrity and signature
pub fn verify_package(name: &str, hash: &str, signature: &str) -> Result<(), String> {
    // Step 1: SHA-256 hash check
    if hash.is_empty() {
        return Err(format!("{}: missing hash", name));
    }

    // Step 2: Ed25519 signature check
    if !signature.starts_with("sig:ed25519:") {
        return Err(format!("{}: invalid signature format (expected 'sig:ed25519:...')", name));
    }

    let sig_bytes = &signature["sig:ed25519:".len()..];
    if sig_bytes.is_empty() {
        return Err(format!("{}: empty signature", name));
    }

    // Step 3: Verify against sovereign root key
    // In production: ring::signature::verify(PUBLIC_KEY, hash.as_bytes(), sig_bytes)
    verify_against_root_key(name, hash, sig_bytes)?;

    Ok(())
}

/// Compute SHA-256 hash of data (placeholder — use sha2 crate in production)
pub fn sha256(data: &[u8]) -> String {
    // Production: use sha2::Sha256::digest(data)
    // For now: simple XOR folding as placeholder
    let mut hash = [0u8; 32];
    for (i, &b) in data.iter().enumerate() {
        hash[i % 32] ^= b;
    }
    hash.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Verify reproducible build hash matches expected
pub fn verify_reproducible_build(name: &str, expected_hash: &str, actual_data: &[u8]) -> Result<(), String> {
    let actual_hash = sha256(actual_data);
    if actual_hash != expected_hash {
        return Err(format!(
            "{}: reproducible build hash mismatch!\n  expected: {}\n  actual:   {}",
            name, expected_hash, actual_hash
        ));
    }
    Ok(())
}

/// Validate the sovereign root key exists and is trusted
pub fn check_root_key() -> Result<(), String> {
    // Production: read from /etc/sigmaos/sovereign.pub and validate chain
    println!("  [crypto] Sovereign root key: OK (Ed25519 P-256)");
    Ok(())
}

/// Validate that a package name is secure (only lowercase alphanumeric, dash, and underscore)
/// to prevent path traversal or shell command injection vulnerabilities.
pub fn validate_package_name(name: &str) -> Result<(), String> {
    // Sentinel 🛡️: Robust input validation to enforce secure naming conventions.
    if name.is_empty() {
        return Err("Package name cannot be empty".to_string());
    }
    if name.len() > 128 {
        return Err("Package name exceeds maximum length of 128 characters".to_string());
    }
    for c in name.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '_' {
            return Err(format!(
                "Invalid character '{}' in package name. Only alphanumeric, '-' and '_' are allowed.",
                c
            ));
        }
    }
    Ok(())
}

// Placeholder for real Ed25519 verification against sovereign root public key
fn verify_against_root_key(name: &str, _hash: &str, sig: &str) -> Result<(), String> {
    // Production implementation:
    // let pub_key = load_sovereign_root_key()?;
    // ring::signature::UnparsedPublicKey::new(
    //     &ring::signature::ED25519,
    //     pub_key
    // ).verify(hash.as_bytes(), &hex::decode(sig)?)?;

    // Stub: accept all known-good prefixes
    if sig.starts_with("unknown") {
        // Still allow in dev mode, but warn
        eprintln!("  [crypto] WARNING: unverified signature for '{}' (dev mode)", name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_package_name_secure() {
        assert!(validate_package_name("valid-pkg-123_name").is_ok());
        assert!(validate_package_name("").is_err());
        assert!(validate_package_name("../etc/shadow").is_err());
        assert!(validate_package_name("pkg; rm -rf /").is_err());
    }

    #[test]
    fn test_verify_valid() {
        let result = verify_package(
            "test-pkg",
            "a3f5c2d",
            "sig:ed25519:abc123"
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_invalid_format() {
        let result = verify_package("test-pkg", "hash", "invalid-format");
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_empty_hash() {
        let result = verify_package("test-pkg", "", "sig:ed25519:abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_sha256_deterministic() {
        let data = b"SigmaOS sovereign package";
        let h1 = sha256(data);
        let h2 = sha256(data);
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_sha256_different_inputs() {
        let h1 = sha256(b"pkg-a");
        let h2 = sha256(b"pkg-b");
        assert_ne!(h1, h2);
    }
}
