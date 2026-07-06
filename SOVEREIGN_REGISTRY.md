# Sovereign Build Script Registry (SBR) 📦🛠️

### Inspiration: SlackBuilds.org, Arch AUR, NixOS Packages

SigmaOS does not ship proprietary binaries or obfuscated package formats. To guarantee **absolute sovereignty**, all userland applications are compiled locally from source within an isolated orchestrator container.

---

## 🛠️ Recipe Specification Layout (`.sigmabuild`)

Every package in the registry defines its build instructions inside a declarative, reproducible JSON recipe:

```json
{
  "app_id": "org.sigmaos.terminal",
  "name": "zenith-terminal",
  "version": "1.2.0",
  "author": "SigmaOS Core Team",
  "source": {
    "url": "https://sources.sigmaos.org/zenith-terminal-1.2.0.tar.gz",
    "sha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
  },
  "build": {
    "language": "rust",
    "flags": ["--no-std", "--target=sigmaos-unknown-none"],
    "dependencies": []
  },
  "sandbox": {
    "network": "none",
    "filesystem_rw": ["/home/user/Downloads"],
    "gpu": "compositor_ipc_only",
    "memory_limit_bytes": 33554432
  }
}
```

---

## 🔒 Execution Flow

1. **Recipe Download:** The Zenith App Store fetches the `.sigmabuild` JSON recipe from the Sovereign Registry.

2. **Signature Verification:** The recipe is verified against the local **Sovereign Root CA** (no external trust anchors).

3. **Container Allocation:** `sigma-pod` spins up a fresh ephemeral offline build container (Flatcar-style immutable model).

4. **Source Download & Hash Check:** Source code is fetched and its SHA256 is asserted against the recipe.

5. **Local Compilation:** Code compiles inside the resource-limited sandbox via `zenith-build`.

6. **Output Bundle:** The binary is packaged into an immutable `.spkg` with an embedded `app.json` sandbox manifest.

7. **Container Teardown:** The ephemeral build container is permanently destroyed.

---

## 🔐 Why No Binaries?

By refusing to distribute pre-compiled binaries:

1. **No Supply Chain Attacks:** You compile the exact source hash specified.

2. **Complete Transparency:** Every compiler flag and dependency is visible in the recipe.

3. **Architecture Tuning:** Binaries are optimized for your exact CPU (x86_64 or ARM64) — no generic blobs.
