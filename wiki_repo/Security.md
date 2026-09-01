# 🛡️ SigmaOS Security Architecture & Hardening

SigmaOS enforces a zero-trust, multi-layered security model combining CPU hardware protections, system call sandboxing, capability bitmasks, and post-quantum cryptography.

---

## 1. Hardware & CPU Security Enforcers (`src/security/mod.rs` & `src/arch/cpu_sys.rs`)

* **SMEP / SMAP Enforcer (`SmepSmapEnforcer`):** Prevents Supervisor Mode Execution Protection (SMEP) and Supervisor Mode Access Prevention (SMAP) kernel-mode exploitation.
* **Stack Guard Pages (`has_guard_page`):** Guard page flags placed at stack boundaries to instantly fault on stack overflow attempts.
* **Retpoline & KPTI Mitigations (`RetpolineKptiMitigationEngine`):** Mitigates Spectre v2 branch target injection and Meltdown page table attacks via Kernel Page Table Isolation.
* **KASLR (`SovereignKaslrEngine`):** Randomizes kernel base physical and virtual memory addresses upon every boot.

---

## 2. Post-Quantum Cryptography & Attestation (`src/crypto/vectorized_pqc.rs`)

* **Dilithium-5 Signatures:** Post-quantum lattice-based digital signature verification for system update packages (`src/productivity/mint_competitor.rs`) and release manifests.
* **Dual-Layer GPG & PQC Verifier (`GpgPqcVerifierAdapter`):** Verifies classical 2048-bit GPG keys alongside post-quantum Dilithium-5 signatures in a unified verification pipeline.
* **TPM 2.0 Integration:** Remote attestation, PCR boot measurement, and sealed vault key storage (`src/drivers/linux_bsd_distro_devices.rs`).

---

## 3. Graphical Authentication & Sudo Elevation (`src/security/libgksu.rs`)

* **LibGksu Graphical Sudo Engine (`LibGksuGraphicalSudoEngine`):** Secure PAM-backed password prompt for GUI elevation.
* **Gksu Security Guard (`GksuSecurityGuard`):** Sanitizes process environment variables, stripping dangerous `LD_PRELOAD`, `PATH`, and display server injection vectors before executing elevated commands.
