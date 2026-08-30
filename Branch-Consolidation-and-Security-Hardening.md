# Branch Consolidation & Security Hardening

This page documents the work to consolidate every active branch of SigmaOS into a
single `main` branch, clear the repository's GitHub code-scanning alerts, and
extend the OS with self-hosted (dependency-free) primitives inspired by Linux/BSD
distributions.

## 1. Branch consolidation

All feature/automation branches were merged into `main` and the redundant
branches deleted, leaving **`main` as the only branch** in the repository.

Merged branches (examples):

* `jules-…` — general subsystem work
* `docs/master-absorption-plan-…` — documentation absorption plan
* `improve-sigpkg-linux-bsd-…` — sigpkg Linux/BSD parity
* `fix-path-traversal-validation-bypass-…` — path-traversal hardening
* `bolt-thermal-sensor-name-len-cache-…` — thermal sensor cache fix

Conflicts were resolved in favour of the most correct/feature-complete variant,
notably:

* De-duplicated three divergent copies of `ApkXbpsHookEngine`,
  `OpenBsdRetguardEngine` and `MapStackRegion` in
  `src/distro/linux_bsd_inspirations.rs` into one best-of-breed implementation
  (transactional LIFO undo stack + FNV-1a RETGUARD cookie + `MAP_STACK` range
  check hardened with saturating arithmetic).
* Implemented the missing `BedrockLinuxStrataEngine` (Bedrock Linux strata /
  `strat`) and `SmartOsZoneEngine` (SmartOS `vmadm`/`imgadm` zone lifecycle) that
  the test suite already referenced.
* Replaced a duplicated hash-order-dependent `select_next_rt_task` with a
  deterministic Earliest-Deadline-First selector plus a SCHED_DEADLINE-style
  admission test (`admit_rt_task`).

## 2. Critical kernel bug fixed

`RcuSynchronizer::synchronize_rcu` previously spun forever on an immutable
`&[TaskStruct]` snapshot — a reader that had not yet called `read_unlock` could
never be observed leaving its epoch, wedging the writer (a real kernel hang).
It was reworked into a bounded grace-period wait with `grace_period_elapsed` /
`stalled_readers` / `synchronize_rcu_checked` and an `RCU_EPOCH_INACTIVE`
sentinel plus an RCU stall limit, so a stuck reader is reported instead of
hanging the kernel.

## 3. Code-scanning security alerts

* **GitHub Actions supply-chain** (`PinnedDependenciesID`,
  `github-actions-mutable-action-tag`, `TokenPermissionsID` — ~88 alerts): every
  `uses:` reference in all 58 workflow files was pinned to a full 40-char commit
  SHA (resolved via the GitHub API), and a least-privilege `permissions:` block
  was injected under every job (`contents: read` by default; `contents: write` +
  `pages: write` + `id-token: write` for deploy/release jobs).
* **Rust lints** (`non_camel_case_types`, `static_mut_refs`, `unexpected_cfgs`,
  `private_interfaces`): 49 enum variants renamed to `UpperCamelCase`,
  `addr_of!`/`addr_of_mut!` used instead of references to mutable statics,
  `kali_stack`'s private `Vec` shim replaced with `crate::klib::Vec`, and the
  `custom_alloc_error_handler` feature registered in `Cargo.toml`.

`cargo clippy --lib` now reports **0 warnings**.

## 4. Self-hosted primitives (reduced dependency on predefined libraries)

New `no_std` modules with zero external crates — these replace the need for
serde/toml/base64 crates and `std` string helpers in kernel context:

* `klib/base64.rs` — base64 encode/decode with strict length checking.
* `klib/config_parser.rs` — INI/key-value `ConfigStore` (`[section]` + `key = value`).
* `klib/toml.rs` — TOML-subset parser (tables, strings, ints, bools, string arrays).
* `klib/utf8_utils.rs` — `no_std` UTF-8 validation + whitespace tokenizer.
* `klib/merkle.rs` — append-only Merkle accumulator (FNV-1a based) for firmware
  measurement chains.
* `security/pqc_enclave.rs` + `security/pqc_measurement.rs` — Kyber-1024 KEM
  plus a hybrid PQC firmware-measurement signature engine (`sign` /
  `verify_hybrid_signature`).
* `distro/void_xbps_src.rs` — Void Linux `xbps-src` bootstrap model with a
  Kahn topological planner for source builds.

## 5. Test status

* `cargo test --lib` — **2675 passed**, 0 failed.
* `cargo test --test integration_test` — **28 passed**, 0 failed.
* All 9 additional integration-test targets pass.
* `cargo clippy --lib` — **0 warnings**.
