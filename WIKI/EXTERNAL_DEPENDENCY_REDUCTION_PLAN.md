# External Library Dependency Reduction Plan

**Date**: August 10, 2026
**Status**: In Progress
**Repository**: SigmaOS

---

## Current External Dependencies

### Cargo.toml Analysis
```toml
[dependencies]
# Core dependencies (minimal as per SigmaOS philosophy)
uuid = { version = "1.4", features = ["v4"] }
rand = "0.8"
```

### Dependency Analysis

#### UUID Crate (v1.4)
**Purpose**: UUID generation for unique identifiers
**Usage**: Likely used for system identifiers, session IDs, etc.
**Security Implications**: Version 4 UUIDs require cryptographically secure random numbers
**Replacement**: Implement UUID v4 generation using kernel's CSPRNG

#### Rand Crate (v0.8)
**Purpose**: Random number generation
**Usage**: Cryptographic operations, session keys, nonces, etc.
**Security Implications**: Must use cryptographically secure random numbers
**Replacement**: Use kernel's CSPRNG (`src/security/rng.rs`)

---

## Replacement Strategy

### Phase 1: UUID Replacement
**Target**: Eliminate uuid crate dependency

**Implementation**:
1. Create `klib::uuid` module with UUID v4 implementation
2. Use kernel's CSPRNG for random bytes
3. Implement standard UUID formatting and parsing
4. Replace all uuid crate usage with klib::uuid

**Code Structure**:
```rust
// klib/uuid.rs
pub struct Uuid([u8; 16]);

impl Uuid {
    pub fn new_v4() -> Self {
        let mut bytes = [0u8; 16];
        // Use kernel CSPRNG
        fill_random_bytes(&mut bytes);
        // Set version and variant bits
        bytes[6] = (bytes[6] & 0x0F) | 0x40; // Version 4
        bytes[8] = (bytes[8] & 0x3F) | 0x80; // Variant 1
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    pub fn to_string(&self) -> String {
        // Standard UUID formatting
    }
}
```

### Phase 2: Rand Replacement
**Target**: Eliminate rand crate dependency

**Implementation**:
1. Audit all rand crate usage across codebase
2. Replace with kernel's CSPRNG calls
3. For non-cryptographic random, use klib's PRNG
4. Ensure all cryptographic operations use CSPRNG

**Code Structure**:
```rust
// klib/rand.rs
pub fn random_bytes(buf: &mut [u8]) {
    // Call kernel's CSPRNG
}

pub fn random_u32() -> u32 {
    let mut bytes = [0u8; 4];
    random_bytes(&mut bytes);
    u32::from_le_bytes(bytes)
}

pub fn random_u64() -> u64 {
    let mut bytes = [0u8; 8];
    random_bytes(&mut bytes);
    u64::from_le_bytes(bytes)
}
```

---

## Implementation Plan

### Step 1: Dependency Audit
1. Search for all uuid crate usage
2. Search for all rand crate usage
3. Identify critical vs non-critical usage
4. Document each usage pattern

### Step 2: Klib Implementation
1. Implement klib::uuid module
2. Implement klib::rand module
3. Add comprehensive tests
4. Ensure cryptographic security

### Step 3: Systematic Replacement
1. Replace uuid crate usage one module at a time
2. Replace rand crate usage one module at a time
3. Test compilation after each replacement
4. Run security tests to ensure CSPRNG usage

### Step 4: Validation
1. Ensure all tests pass
2. Verify cryptographic security
3. Check performance is not degraded
4. Remove external dependencies from Cargo.toml

---

## Security Considerations

### Cryptographic Security
- All UUID v4 generation must use CSPRNG
- All cryptographic random numbers must use CSPRNG
- Non-cryptographic random can use faster PRNG
- Audit all random number generation

### Deterministic Testing
- Provide deterministic random for testing
- Use seedable PRNG for reproducible tests
- Separate test random from production random

---

## Priority Order

### High Priority (Immediate)
1. Implement klib::uuid module
2. Implement klib::rand module
3. Replace critical security-sensitive usage

### Medium Priority (Week 2)
1. Replace non-critical rand usage
2. Update tests to use deterministic random
3. Comprehensive security audit

### Low Priority (Week 3)
1. Performance optimization
2. Advanced UUID features
3. Random number quality testing

---

## Success Metrics

**Target**: Zero external library dependencies (except testing)

**Current Dependencies**:
- uuid = "1.4"
- rand = "0.8"

**Target Dependencies**:
- None (for production)
- Only test dependencies remain

**Milestones**:
- Week 1: Implement klib replacements
- Week 2: Replace all usage
- Week 3: Remove external dependencies

---

## Challenges & Solutions

### Challenge 1: API Compatibility
**Issue**: External crates may have better APIs
**Solution**: Implement compatible APIs in klib

### Challenge 2: Performance
**Issue**: klib implementations may be slower
**Solution**: Optimize critical paths after migration

### Challenge 3: Test Coverage
**Issue**: Tests may rely on external crate behavior
**Solution**: Update tests to use klib equivalents

---

## Validation Plan

### Security Validation
1. Verify all UUID generation uses CSPRNG
2. Verify all cryptographic random uses CSPRNG
3. Run cryptographic security tests
4. External security audit if possible

### Performance Validation
1. Benchmark UUID generation performance
2. Benchmark random number generation
3. Compare with original crate performance
4. Optimize if significant degradation

### Functionality Validation
1. Test UUID parsing and formatting
2. Test random number quality
3. Test statistical properties of random
4. Ensure API compatibility

---

**Status**: Ready to begin external dependency elimination