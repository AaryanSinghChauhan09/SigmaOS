# Policy: Release Channels and Artifact Requirements (.agents/policies/release.md)

1. **Release Channels**:
   - `dev`: Active development branches passing `cargo check` and `cargo test`.
   - `nightly`: Daily automated build requiring clean checkout build, test suite pass, and QEMU x86_64 boot-to-shell smoke test pass.
   - `validated`: Candidate release requiring full boot matrix verification, installer test pass, known issues document, and rollback instructions.
   - `experimental`: Bleeding-edge experimental features, clearly marked as non-production.

2. **No Unverified Stable Claims**:
   - Stable distribution releases are prohibited until reproducible boot, install, update, and desktop execution are proven.
