# SigmaOS Developer Onboarding Guide

Welcome to the **SigmaOS Zenith** development team! This guide establishes the coding practices, compilation paradigms, and contribution pipelines required to maintain architectural integrity across the 600-shard lattice.

---

## 🛠️ 1. Core Development Paradigms

SigmaOS is designed for silicon-direct, high-security operations. You must strictly adhere to the following three code design constraints:

### A. Zero Monolithic Dependencies

- Do not include any standard library headers (`<vector>`, `<string>`, `<iostream>`, etc.) as they assume host operating system runtimes.
- Use fixed-width primitive types defined in [sigma_kernel_types.h](file:///c:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS/include/sigma_kernel_types.h) (`sigma_u32`, `sigma_u64`, etc.).

### B. Stable C++ Meyer Singletons

- All active driver and core system shards must derive from `SigmaOS::SigmaObject` and implement standard Meyer singletons to ensure safe static execution limits:

```cpp
#include "../../include/SigmaOOP.hpp"
#include "../../include/sigma_log.h"

namespace SigmaOS {
namespace MySubsystem {

class MyComponent : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "MyComponent"; }

    static MyComponent& getInstance() {
        static MyComponent instance;
        return instance;
    }

    void run() {
        sigma_log_info("[SUB] Active.");
    }

private:
    MyComponent() = default;
};

} // namespace MySubsystem
} // namespace SigmaOS

```

### C. Zero-Loss Fixed-Point Calculations

- Never use standard CPU float/double math in financial or statutory calculators.
- Calculate monetary values exclusively as integers in **paise** (1 Rupee = 100 paise) to prevent precision loss.

---

## 🚀 2. Local Compiling & Boot Verification

Verify compilation and boot stage behavior locally using the QEMU target suite:

1. **Clean Object Directory**:

   ```bash
   make clean
   ```

2. **Build Bootable ISO**:

   ```bash
   make iso
   ```

3. **Ignite the Emulation Target**:

   ```bash
   make qemu
   ```

Observe the direct serial telemetry stream to confirm the **Asynchronous Shard Ignition (ASI)** successfully bootstrapped all shards.
