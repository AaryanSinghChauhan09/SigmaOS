# SigmaOS Security Test Plan

## Automated Security Checks

1. **Dependency Vulnerability Scanning**:
   - `cargo audit` in `.github/workflows/security.yml`.
   - Automated dependency license and advisory review.

2. **Capability Token Unit Tests**:
   ```bash
   rustc --test tests/security/capability_test.rs --edition=2021 -o build/test_capability
   ./build/test_capability
   ```

3. **ASan & Redzone Memory Error Detection**:
   ```bash
   rustc --test --edition=2021 src/security/address_sanitizer.rs -o build/asan_test
   ./build/asan_test
   ```

4. **Static Analysis & Fuzzing**:
   - `cargo fuzz` target verification for VFS path parsing and shell lexer/parser.
