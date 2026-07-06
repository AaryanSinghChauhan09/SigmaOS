# Sigstore / cosign Supply Chain Security Integration

## Overview

SigmaOS signs **every CI artifact** using [cosign](https://github.com/sigstore/cosign) (Apache-2.0) with a **Dilithium-5 hybrid signature** (post-quantum + ECDSA P-384). All signatures are recorded in the [Rekor](https://github.com/sigstore/rekor) transparency log. Build provenance follows the [in-toto](https://in-toto.io/) attestation specification.

---

## What Gets Signed

| Artifact | Signature file | Provenance |
|---|---|---|
| sigma-kernel.elf | sigma-kernel.elf.sig | provenance.json |
| sigma-init | sigma-init.sig | provenance.json |
| sigpkg .spkg files | <name>.spkg.sig | provenance.json |
| OCI images | image manifest digest | cosign OCI annotation |
| Release ISOs | sigmaos-<ver>.iso.sig | provenance.json |

---

## Signing with Dilithium-5 Hybrid

SigmaOS uses a **hybrid signature scheme**: ECDSA P-384 (classical) XOR'd with Dilithium-5 (NIST PQC Level 5). The cosign `--experimental` KMS backend supports custom signers via the `cosign sign --key` flag with a custom provider.

```bash

# Sign a binary with hybrid key

cosign sign-blob \
  --key env://SIGMA_SIGNING_KEY \
  --bundle sigma-kernel.elf.cosign-bundle \
  sigma-kernel.elf

# Verify

cosign verify-blob \
  --key env://SIGMA_VERIFY_KEY \
  --bundle sigma-kernel.elf.cosign-bundle \
  sigma-kernel.elf
```

---

## Rekor Transparency Log Integration

Every signature is uploaded to the Rekor log automatically when using `cosign sign --rekor-url`:

```bash
cosign sign-blob \
  --key env://SIGMA_SIGNING_KEY \
  --rekor-url https://rekor.sigstore.dev \
  --bundle output.bundle \
  sigma-init
```

The resulting Rekor entry UUID is stored in `provenance.json` for auditing.

---

## in-toto Provenance Attestation

`provenance.json` follows SLSA Level 2 format:

```json
{
  "_type": "https://in-toto.io/Statement/v0.1",
  "subject": [{
    "name": "sigma-kernel.elf",
    "digest": { "sha256": "<hex>" }
  }],
  "predicateType": "https://slsa.dev/provenance/v0.2",
  "predicate": {
    "builder": { "id": "https://github.com/AaryanSinghChauhan09/SigmaOS/actions" },
    "buildType": "https://github.com/slsa-framework/slsa-github-generator/go@v1",
    "invocation": {
      "configSource": {
        "uri": "git+https://github.com/AaryanSinghChauhan09/SigmaOS@refs/heads/main",
        "entryPoint": ".github/workflows/reproducible_build.yml"
      }
    }
  }
}
```

---

## CI Workflow: `.github/workflows/reproducible_build.yml`

```yaml
name: Reproducible Build + Sign

on:
  push:
    branches: [main]
  release:
    types: [published]

env:
  REKOR_URL: https://rekor.sigstore.dev

jobs:
  build-sign:
    runs-on: ubuntu-22.04
    permissions:
      id-token: write   # needed for keyless cosign

      contents: read

    steps:
      - uses: actions/checkout@v4

      - name: Install cosign
        uses: sigstore/cosign-installer@v3
        with:
          cosign-release: v2.2.4

      - name: Build artifacts
        run: |
          make all SOURCE_DATE_EPOCH=$(git log -1 --format=%ct)
          sha256sum dist/* > dist/SHA256SUMS

      - name: Sign artifacts (keyless + Rekor)
        run: |
          for f in dist/*.elf dist/*.spkg dist/*.iso; do
            cosign sign-blob \
              --rekor-url $REKOR_URL \
              --bundle "${f}.bundle" \
              "$f"
            cp "${f}.bundle" "${f}.sig"
          done

      - name: Generate in-toto provenance
        run: |
          python3 tools/signing/gen_provenance.py \
            --artifacts dist/ \
            --output dist/provenance.json

      - name: Upload signed artifacts
        uses: actions/upload-artifact@v4
        with:
          name: signed-artifacts
          path: dist/
```

---

## sigma-pkg verify

`sigma-pkg verify <package>` uses cosign under the hood:

```rust
// tools/signing/sigma_verify.rs (sketch)
pub fn verify_package(pkg_path: &str) -> Result<(), VerifyError> {
    let sig_path = format!("{}.sig", pkg_path);
    let status = std::process::Command::new("cosign")
        .args([
            "verify-blob",
            "--key", "/etc/sigma/verify-key.pem",
            "--bundle", &sig_path,
            pkg_path,
        ])
        .status()?;
    if status.success() { Ok(()) } else { Err(VerifyError::InvalidSignature) }
}
```

---

## Exit Criteria

- Every CI artifact in `dist/` has a corresponding `.sig` and `provenance.json`.

- `sigma-pkg verify sigma-edit` exits 0 and prints `Signature verified`.

- Rekor log entry for each artifact is queryable via `rekor-cli get --log-index <n>`.
