# Policy: Mandatory Test Evidence Before Status Promotion (.agents/policies/test-evidence.md)

1. **Evidence Standard**:
   - No feature or capability may be marked `Integrated` or `Supported` in `CAPABILITY_MATRIX.toml` or `FEATURE_STATUS.toml` without attached, reproducible evidence.
   - Allowed evidence types:
     * Passing unit/integration test output logs (`reports/test/`).
     * QEMU boot & serial log records (`reports/boot/`).
     * Installer/recovery test result digests (`reports/installer/`).
     * Hardware verification records with device models (`reports/boot/hardware/`).

2. **Runtime Proof Dominance**:
   - Unit tests demonstrate logic; boot/QEMU runtime logs demonstrate OS capability. When both exist, runtime evidence controls public capability status.
