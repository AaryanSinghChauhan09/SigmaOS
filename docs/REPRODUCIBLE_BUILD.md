# Reproducible Builds & Supply Chain Security

SigmaOS guarantees that every official artifact is **bit-for-bit reproducible** — given the same source commit and toolchain, anyone can rebuild and get an identical binary.

---

## What "Reproducible" Means

```
Source commit X + Toolchain Y → Binary Z (always the same bytes)
```

This allows:
- Independent verification of official builds
- Detection of compiler/build-system backdoors (XZ attack prevention)
- Auditable supply chain for enterprise deployments

---

## How We Achieve It

### 1. Pinned Toolchain

```toml
# rust-toolchain.toml
[toolchain]
channel = "nightly-2026-06-01"
components = ["rust-src", "llvm-tools-preview"]
targets = ["x86_64-unknown-none"]
```

Every developer and CI run uses exactly this toolchain version.

### 2. Deterministic Cargo Build

```toml
# Cargo.toml profile
[profile.release]
opt-level     = "z"
lto           = true
codegen-units = 1
strip         = "debuginfo"
panic         = "abort"
```

Single `codegen-unit` ensures no non-deterministic parallelism in code generation.

### 3. No Timestamps in Binaries

```makefile
# Makefile
export SOURCE_DATE_EPOCH = $(shell git log -1 --format=%ct)
```

All timestamps in binaries, archive headers, and filesystem metadata are set to the git commit timestamp.

### 4. Locked Dependencies

```
Cargo.lock  — exact crate versions (committed to repo)
```

No floating version ranges in release builds.

### 5. Reproducible ISO

```bash
make iso PROFILE=standalone SOURCE_DATE_EPOCH=$(git log -1 --format=%ct)
```

`mkisofs` uses `--reproducible` flag and fixed dates.

---

## Build Provenance

Every CI build produces `provenance.json`:

```json
{
  "schema": "https://slsa.dev/provenance/v1",
  "subject": [
    {
      "name": "sigmaos-zenith-15.1.0-x86_64.iso",
      "digest": { "sha256": "abc123..." }
    }
  ],
  "predicate": {
    "builder": { "id": "https://github.com/AaryanSinghChauhan09/SigmaOS/.github/workflows/build.yml" },
    "buildType": "https://sigmaos.io/buildType/v1",
    "materials": [
      { "uri": "git+https://github.com/AaryanSinghChauhan09/SigmaOS",
        "digest": { "gitCommit": "abc123def456" } }
    ],
    "metadata": {
      "buildStartedOn": "2026-07-03T00:00:00Z",
      "completeness": { "parameters": true, "environment": true, "materials": true },
      "reproducible": true
    }
  }
}
```

---

## Verifying a Build

```bash
# Download official ISO and provenance
wget https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/v15.1.0/sigmaos-15.1.0-x86_64.iso
wget https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/v15.1.0/provenance.json
wget https://github.com/AaryanSinghChauhan09/SigmaOS/releases/download/v15.1.0/sigmaos-15.1.0-x86_64.iso.sig

# Verify checksum matches provenance
sha256sum sigmaos-15.1.0-x86_64.iso
# Compare with provenance.json digest

# Verify Dilithium-5 signature
sigma-verify-sig sigmaos-15.1.0-x86_64.iso sigmaos-15.1.0-x86_64.iso.sig

# Rebuild and compare
git checkout v15.1.0
make iso PROFILE=standalone
diff sigmaos-15.1.0-x86_64.iso build/sigmaos.iso
# Should produce no output (identical)
```

---

## CI Pipeline

```yaml
# .github/workflows/reproducible_build.yml
jobs:
  reproducible:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Pin toolchain
        run: rustup show
      - name: Set SOURCE_DATE_EPOCH
        run: echo "SOURCE_DATE_EPOCH=$(git log -1 --format=%ct)" >> $GITHUB_ENV
      - name: Build ISO
        run: make iso PROFILE=standalone
      - name: Build again (verify reproducibility)
        run: make iso PROFILE=standalone
        env:
          BUILD_DIR: build2
      - name: Compare
        run: diff build/sigmaos.iso build2/sigmaos.iso
      - name: Generate provenance
        run: ./build/provenance.sh > provenance.json
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: sigmaos-iso
          path: |
            build/sigmaos.iso
            provenance.json
```

---

## Measured Boot + TPM Attestation

For hardware deployments, SigmaOS extends the reproducible build with measured boot:

```
UEFI → sigma-boot.efi → TPM PCR[0] = hash(sigma-boot.efi)
                      → TPM PCR[1] = hash(kernel)
                      → TPM PCR[2] = hash(initramfs)
                      → TPM PCR[3] = hash(cmdline)
```

The TPM seals the disk encryption key against these PCR values.
If any component changes (or is tampered with), the key cannot be unsealed.

---

*See also: [Security Model](Security-Model) · [Verified Boot](../wiki_repo/Verified-Boot.md)*
