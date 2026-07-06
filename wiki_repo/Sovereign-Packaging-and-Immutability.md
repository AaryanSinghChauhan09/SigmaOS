# Sovereign Packaging & Immutable Updates 📦🔒

SigmaOS balances rapid userland agility with deep system stability by borrowing structural integrity models from NixOS, Slackware, and Flatcar CoreOS.

---

## 📦 Sovereign Packaging System (`.spkg` & `.srecipe`)

SigmaOS rejects untrusted, pre-compiled binary payloads. Instead, we implement a **Sovereign Build Script Registry** (SlackBuilds-inspired):

- **Source-First Compilation:** Applications declare reproducible recipe schemas (`.srecipe`).

- **Containerized Compilation:** The package manager fetches source tarballs, validates SHA256 checksums, and compiles the code locally inside isolated orchestrator sandbox containers.

- **Verification:** Final compiled code bundles into a safe, cryptographically signed `.spkg` file (`sigma_package_parser.cpp`) that enforces tight container boundaries.

---

## 🔒 Immutable System Updates (Flatcar paradigm)

The base kernel and primary driver structures are protected by a rigid system partition model:

- **Atomic Updates:** The auto-update daemon (`sigma_update_daemon.cpp`) queries curated release channels (stable, developer, forensic) to verify image checksum signatures.

- **Block-Level Writes:** The daemon writes updates directly to disk sectors without touching active userspace chroots.

- **A/B Partition Swapping:** Cryptographically enforced boot overrides swap base OS images on the next system reboot cycle safely.
