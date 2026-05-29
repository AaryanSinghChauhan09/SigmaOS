# Sovereign Build Script Registry Specification 📦🛠️

SigmaOS implements a SlackBuilds-inspired **Sovereign Build Script Registry**. Rather than distributing untrusted, pre-compiled binary packages that could contain backdoors, the Sovereign Registry publishes reproducible, signed source recipe scripts. The base OS builds packages locally inside highly isolated, temporary orchestrator containers.

---

## 🛠️ Recipe Specification Layout (`.srecipe`)

Every package in the registry defines its build instructions inside a declarative, reproducible recipe document:

```toml
[package]
name = "zenith-terminal"
version = "1.2.0"
source = "https://sources.sigmaos.org/zenith-terminal-1.2.0.tar.gz"
sha256 = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"

[build]
# Strictly enforces standard C/Rust build gates inside the sandbox container
commands = [
    "cargo build --release --manifest-path ./Cargo.toml",
    "cp target/release/zenith-terminal $SOVEREIGN_OUT/bin/"
]

[sandbox]
# Declares minimum resource requirements for secure deployment
memory_limit_bytes = 33554432  # 32MB
network_isolation = true       # Enforce Whonix split gateway rules
```

---

## 🔒 Execution Flow

1.  **Recipe Verification:** The package daemon fetches the target `.srecipe` and verifies the cryptographical GPG signature of the registry publisher.
2.  **Container Allocation:** The Sovereign Orchestrator spins up a temporary chroot container sandbox (minimal low-footprint Flatcar model).
3.  **Source Download & Checksum Validation:** The download engine retrieves the source tarball and asserts that the SHA256 matches the baseline.
4.  **Local Compilation:** The code compiles strictly inside the resource-limited sandbox.
5.  **Output Bundle:** The final compiled binary is bundled into our secure `.spkg` package format and cryptographically verified.
