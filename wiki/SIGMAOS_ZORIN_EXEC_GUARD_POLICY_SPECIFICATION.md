# 🛡️ Zorin Exec Guard Policy & Usability Specification

To prevent untrusted binary execution while ensuring developers don't experience persistent friction, **SigmaOS** defines a fine-grained, configurable policy for the **Zorin Exec Guard** (`src/security/exec_guard.rs`) subsystem. This specification balances the **default-deny capability model** with customizable, developer-friendly rule overrides.

---

### 1. The Default-Deny Capability Philosophy
* **Default Action:** Any binary execution not explicitly matching a cryptographic signature or a developer rule is dynamically blocked.
* **Alternative Suggestion Routing:** When blocked, instead of a dry termination, Zorin Exec Guard suggests a verified native or containerized (`apx`/`sigpkg`) alternative via a non-intrusive Zenith GUI popup.

---

### 2. Developer Exceptions Configuration (`/system/profile.toml` or `/user/preferences.toml` Overrides)
To allow rapid compilation, toolchain testing, and debugging, developers can configure fine-grained exception blocks in TOML format:

```toml
[zorin_exec_guard.developer_mode]
enabled = true
interactive_prompts = true       # Query developer via GUI/CLI instead of hard-deny
log_violations = true            # Write blocked executions to security journal

# Cryptographic trust rules based on developer certificates
[[zorin_exec_guard.trusted_certificates]]
issuer = "SigmaOS Local CA"
thumbprint = "4a8b9c...f1a23"
actions = ["Execute", "NetworkTcp", "FileRead"]

# Path-based sandboxed exceptions
[[zorin_exec_guard.path_rules]]
path_prefix = "/home/developer/workspace/"
allow_unsigned_execution = true
enforce_strict_sandboxing = true
allowed_capabilities = [
    "FileRead",
    "FileWrite",
    "ProcessExec"
]
blocked_capabilities = [
    "NetworkTcp",
    "NetworkUdp",
    "DisplayAccess"
]

# Target execution overrides with interactive elevation
[[zorin_exec_guard.binary_overrides]]
binary_name = "cargo"
override_hash = "sha256:7b5c8d...a3e9"
allow_all_child_processes = true
```

---

### 3. Bridging Security and Usability
* **Interactive Prompting:** If `interactive_prompts` is enabled, an execution from an unknown path triggers a quick Zenith notification. The developer can whitelist the path, the binary hash, or spawn the program within an isolated fallback sandbox on-the-fly.
* **Capabilities Attenuation:** Unsigned compiled developer binaries run under restricted capabilities. For example, a newly compiled test program inside `/home/developer/workspace` can read and write files within its workspace but cannot bind to external TCP ports or access physical audio hardware without an explicit override signature.

---

### 4. Implementation Reference in SigmaOS Codebase

The production implementation is available in `src/security/exec_guard.rs`:
* **Engine Struct**: `ZorinExecGuardPolicyEngine`
* **Capability Matrix**: `ExecCapability` (`FileRead`, `FileWrite`, `ProcessExec`, `NetworkTcp`, `NetworkUdp`, `DisplayAccess`)
* **Decision Types**: `ExecDecision::Allow`, `ExecDecision::PromptDeveloper`, `ExecDecision::HardDeny`
