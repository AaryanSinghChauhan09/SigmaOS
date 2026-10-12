# Release and Distribution Channel Policy

## 1. Release Channel Hierarchy

SigmaOS adopts a 4-tier release channel model:

| Channel | Target Audience | Requirements for Artifact Inclusion |
|---|---|---|
| `dev` | Active developers & agents | Clean checkout build succeeds; unit tests pass (`cargo check`, `cargo test`). |
| `nightly` | Continuous integration & testers | Clean build succeeds; unit tests pass; QEMU x86_64 boot-to-shell smoke test succeeds; serial log recorded without kernel panic. |
| `validated` | General test users | Complete boot matrix (x86_64 and secondary targets); installer and update recovery tests succeed; known-issue list and rollback instructions published. |
| `experimental` | Early tech previews | Labeled as experimental; isolated new subsystem builds; not intended for daily use. |

## 2. Production / Stable Release Gate
SigmaOS WILL NOT declare a "Stable" or "1.0" release until:
1. Firmware-to-desktop boot path is fully proven in QEMU and target hardware.
2. Installer, system update, and transactional snapshot rollback are reproducibly verified.
3. Every claim in `CAPABILITY_MATRIX.toml` and `README.md` is backed by active test evidence under `reports/`.
