# CI Pipeline

The SigmaOS CI pipeline is an industrial-grade, automated sentinel that ensures absolute kernel integrity and silicon parity.

---

## Pipeline Overview

The CI pipeline automatically builds, tests, and deploys SigmaOS across all supported architectures and deployment profiles. It ensures reproducible builds, security verification, and quality standards for every commit.

---

## Pipeline Stages

### 1. Shard Forge (Build)

Cross-compiles the 500-shard lattice for x86_64, ARM64, and RISC-V:

```yaml
build:
  stage: build
  script:
    - cargo build --release --target x86_64-unknown-none
    - cargo build --release --target aarch64-unknown-none
    - cargo build --release --target riscv64gc-unknown-none
  artifacts:
    paths:
      - target/x86_64-unknown-none/release/sigma-kernel
      - target/aarch64-unknown-none/release/sigma-kernel
      - target/riscv64gc-unknown-none/release/sigma-kernel
```

**Build Profiles**:
- `standalone`: Full desktop environment
- `microkernel`: Minimal kernel (< 512 KB)
- `mobile`: ARM64 with touch UI
- `rtos`: Real-time with EDF scheduler
- `cloud`: Immutable root with A/B partitions
- `distributed`: CRDT mesh networking

### 2. Shard Test Nexus

Runs the autonomous `SovereignUnitTestShard` to verify kernel-level primitives:

```yaml
test:
  stage: test
  script:
    - cargo test --release
    - ./scripts/fuzz-tests.sh
    - ./scripts/security-audit.sh
  coverage: '/^\s*lines:\s*\d+.\d+\%/'
```

**Test Categories**:
- Memory management (buddy allocator, slab allocator, paging)
- Security (sigma_pledge, sigma_unveil, AVC, PQC)
- Networking (TLS 1.3, DNS, DHCP, firewall)
- Filesystem (VFS, SigmaFS, Ext4, dm-verity)
- Scheduler (MLFQ, CFS, EDF, RT)
- IPC and synchronization

### 3. Security Verification

Verifies post-quantum signatures and TPM2 measurements:

```yaml
security:
  stage: security
  script:
    - ./scripts/verify-pqc-signatures.sh
    - ./scripts/tpm2-measure.sh
    - ./scripts/dependabot-scan.sh
    - ./scripts/codeql-scan.sh
```

**Security Checks**:
- Dilithium-5 signature verification
- Kyber-1024 KEM verification
- TPM2 PCR measurements
- Static analysis (CodeQL)
- Dependency vulnerability scanning
- Fuzzing regression tests

### 4. Package Nexus (Deploy)

Orchestrates the distribution of verified silicon shards to the global lattice:

```yaml
deploy:
  stage: deploy
  script:
    - ./scripts/build-iso.sh
    - ./scripts/sign-release.sh
    - ./scripts/upload-repo.sh
  only:
    - main
    - tags
```

**Deployment Artifacts**:
- Bootable ISO images
- Container images (OCI)
- APK packages (mobile)
- WASM modules (browser)
- RTOS binaries
- Cloud images (qcow2, vmdk)

---

## Triggers

The pipeline is automatically triggered on:
- Every push to the `main` branch
- Every pull request
- Every tag (for releases)
- Manual trigger via GitHub Actions UI

**Manual Override**:
```bash
make industrial_sync
```

---

## Architecture Support

| Architecture | Status | Build Target | Test Coverage |
|-------------|--------|--------------|---------------|
| x86_64 | ✅ Stable | x86_64-unknown-none | 95% |
| ARM64 | 🟡 Beta | aarch64-unknown-none | 80% |
| RISC-V | 🔴 Alpha | riscv64gc-unknown-none | 50% |

---

## Performance Benchmarks

The pipeline tracks performance regressions:

| Metric | Target | Current |
|--------|--------|---------|
| Build time (x86_64) | < 10 min | 8 min |
| Build time (ARM64) | < 15 min | 12 min |
| Build time (RISC-V) | < 20 min | 18 min |
| Test execution | < 5 min | 4 min |
| Security scan | < 3 min | 2 min |

---

## Quality Gates

The pipeline enforces these quality gates before merging:

1. **Build Gate**: All architectures must build successfully
2. **Test Gate**: All tests must pass with > 90% coverage
3. **Security Gate**: No critical or high severity vulnerabilities
4. **Performance Gate**: No performance regression > 5%
5. **Documentation Gate**: All changes must include documentation

---

## Failure Handling

**Build Failures**:
- Automatic notification to contributors
- Build logs archived for 30 days
- Automatic issue creation for recurring failures

**Test Failures**:
- Bisect to identify breaking commit
- Automatic rollback on main branch
- Require manual review for fix

**Security Failures**:
- Immediate block on deployment
- Security team notification
- Required fix before merge

---

## Monitoring

Pipeline metrics are monitored via:
- GitHub Actions dashboard
- Custom Prometheus metrics
- Slack notifications for failures
- Weekly performance reports

---

## Future Enhancements

- [ ] Distributed build farm for faster builds
- [ ] Cached dependencies for incremental builds
- [ ] Parallel test execution
- [ ] Automated performance regression detection
- [ ] Integration with external security scanners
- [ ] Multi-region artifact distribution

---

*See also: [BUILD.md](BUILD.md) · [TESTING.md](TESTING.md) · [SECURITY_POLICY.md](SECURITY_POLICY.md)*
