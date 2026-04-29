# SigmaOS — Contribution Guide

> How to add new modules, tools, shaders, and features to SigmaOS.

---

## 🗺️ Quick Contribution Map

```
Have an idea?
    │
    ├─ New kernel primitive?  → suites/S01_Genesis/sigma_<name>.h
    ├─ New hardware driver?   → suites/S04_HAL/sigma_<name>.hpp
    ├─ New security module?   → suites/S08_Security/sigma_<name>.h
    ├─ New network feature?   → suites/S37_SovereignWire/sigma_<name>.h
    ├─ New perf optimization? → suites/S28_PerformanceLattice/sigma_<name>.h
    ├─ New OOP driver class?  → sigmaos/core/src/atomic_sigma_<name>_oop.cpp
    └─ New CLI command?       → orchestrator/main.cpp → add ICommand subclass
```

---

## 📐 Module Naming Conventions

| Category | Prefix | Example |
|---------|--------|---------|
| System core | `sigma_sys_` | `sigma_sys_snapshot.h` |
| Networking | `sigma_net_` | `sigma_net_vpn.h` |
| Security | `sigma_sec_` | `sigma_sec_tpm.h` |
| Performance | `sigma_perf_` | `sigma_perf_isolator.h` |
| Multimedia | `sigma_media_` | `sigma_media_codec.h` |
| Automation | `sigma_auto_` | `sigma_auto_watchdog.h` |
| UI | `sigma_ui_` | `sigma_ui_shader.h` |
| Package | `sigma_pkg_` | `sigma_pkg_resolver.h` |

---

## 🔢 Picking a Suite Number

Find the right `suites/S<NN>_*` directory:

| Suite Range | Domain |
|-------------|--------|
| S01–S10 | Kernel core (allocator, HAL, IPC, security) |
| S11–S20 | System services (VFS, networking, process mgmt) |
| S21–S30 | Performance (NUMA, cache, BPF, containers) |
| S31–S40 | Storage, immutability, ZKP, sovereign wire |
| S41–S50 | Boot, self-healing, caps, mesh |
| S51–S65 | Applications, AI, developer tools |

---

## 🧩 Step-by-Step: Adding a New Module

### 1. Create the header


```bash


# Example: adding a VPN tunnel module


touch suites/S37_SovereignWire/sigma_vpn.h
```

### 2. Write the module


Follow the **Atomic Module Contract** (see Developer Guide):
- One `#ifndef` guard
- One primary struct + 3–5 static inline functions
- Zero external includes

### 3. Wire into CI tests


Edit `orchestrator/main.cpp`, find `TestCommand::run_subsystem_test()`:
```cpp
static void run_subsystem_test(const char* subsystem) {
    // Add your new subsystem here:
    std::cout << "[✓] " << subsystem << " → All shards passed.\n";
}
```

### 4. Add a CLI command (optional)


```cpp
// In orchestrator/main.cpp, add a new ICommand subclass:
class VPNCommand : public ICommand {
public:
    bool matches(const std::string& cmd) const override { return cmd == "vpn"; }
    int execute(int argc, char** argv) const override {
        std::cout << "[SigmaOS] VPN tunnel established via Sigma-Net.\n";
        return 0;
    }
};
// Then register in CommandDispatcher constructor
```

### 5. Test locally


```bash
g++ -std=c++20 orchestrator/main.cpp -o s-cli
./s-cli test --subsystem networking
```

### 6. Push & watch CI


```bash
git add suites/S37_SovereignWire/sigma_vpn.h orchestrator/main.cpp
git commit -m "feat: Add sigma_vpn atomic networking module"
git push origin main
```

CI will automatically run all 4 workflows and report results.

---

## 💡 Ideas Backlog (Up for Grabs)

| Module | Inspired By | Priority |
|--------|------------|---------|
| `sigma_vpn.h` | WireGuard | 🔥 High |
| `sigma_tpm.h` | TPM 2.0 | 🔥 High |
| `sigma_codec_h264.h` | FFmpeg | 🟡 Medium |
| `sigma_dns_resolver.h` | Unbound DNS | 🟡 Medium |
| `sigma_thermal.h` | Linux ACPI | 🟢 Low |
| `sigma_power_mgmt.h` | Windows ACPI | 🟢 Low |


