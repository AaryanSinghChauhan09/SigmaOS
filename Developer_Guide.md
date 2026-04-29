# SigmaOS — Developer Guide

> How to write, test, and contribute atomic silicon modules to SigmaOS.

---

## 🧬 The Atomic Module Contract

Every module in SigmaOS follows one strict rule:

> **One file = One function = Zero external dependencies**

### ✅ What a valid atomic module looks like

```c
// SigmaOS — sigma-sys-example
// Module: one-sentence description of what it does
// Dependencies: NONE (or only other sigma_*.h headers)

#ifndef SIGMA_EXAMPLE_H
#define SIGMA_EXAMPLE_H

// Only sigma types — never stdint, stdlib, stdio
typedef unsigned long sigma_size_t;

static inline int example_do_thing(unsigned int param) {
    // Direct silicon logic
    return param * 2;
}

#endif /* SIGMA_EXAMPLE_H */
```

### ❌ What is NOT allowed

```c
#include <stdlib.h>    // ❌ forbidden
#include <stdio.h>     // ❌ forbidden
#include <string.h>    // ❌ forbidden
using namespace std;   // ❌ forbidden
```

---

## 🏗️ Module Locations

| Subsystem | Directory | Examples |
|-----------|-----------|---------|
| Kernel core | `suites/S01_Genesis/` | allocator, VMM, spinlock, scheduler |
| Hardware drivers | `suites/S04_HAL/` | NVMe, USB HID, IRQ dispatcher |
| Security | `suites/S08_Security/` | PQC, zero-trust, sandbox, audit |
| IPC / Async IO | `suites/S42_RawIPC/` | ring buffer, AIO |
| Performance | `suites/S28_PerformanceLattice/` | cache, work-stealing |
| Package Manager | `suites/S36_SovereignPackageRegistry/` | sigma_pkg |
| Networking | `suites/S37_SovereignWire/` | netfilter, BPF |
| Self-Healing | `suites/S41_SiliconBoot/` | auto_rollback |
| Core OOP | `sigmaos/core/src/atomic_*.hpp` | base interfaces |
| CLI | `orchestrator/main.cpp` | OOP command dispatcher |

---

## 🔧 Writing a New OOP Module (C++)

```cpp
// sigma_my_driver.hpp
#pragma once
#include "atomic_sigma_oop_base.hpp"
#include "sigma_libc.h"

namespace sigma { namespace hal {

class MyDriver : public sigma::core::ISigmaDriver,
                 public sigma::core::ISigmaModule {
private:
    bool ready;
public:
    MyDriver() : ready(false) {}

    void initialize() override {
        sigma_kprint("[MyDriver] Init\n");
        ready = true;
    }
    void execute()  override { /* hot path */ }
    void shutdown() override { ready = false; }
    int  probe_hardware() override { return 1; }
    void enable_dma()     override { }
};

}} // namespace sigma::hal

extern "C" {
    void my_driver_run() {
        sigma::hal::MyDriver d;
        d.probe_hardware(); d.initialize(); d.execute(); d.shutdown();
    }
}
```

---

## 🧪 Testing Your Module Locally

```bash

# 1. Build the orchestrator

g++ -std=c++20 orchestrator/main.cpp -o s-cli

# 2. Run subsystem tests

./s-cli test --subsystem genesis
./s-cli test --subsystem hal
./s-cli test --subsystem security

# 3. Run full benchmark

./s-cli benchmark --run-all

# 4. Check sovereignty (no forbidden imports)

grep -r "#include <stdlib.h>" suites/S01_Genesis/ sigmaos/core/src/atomic_*

# Should return nothing

# 5. Count your new modules

ls sigmaos/core/src/atomic_* | wc -l
find suites/ -name "sigma_*.h" | wc -l
```

---

## 📋 Checklist Before Submitting


- [ ] File is self-contained (no includes except other `sigma_*.h`)
- [ ] Uses `sigma_size_t`, `sigma_u32` etc. — not `size_t`, `uint32_t`
- [ ] Has a one-line module description comment at the top
- [ ] `#ifndef` guard present
- [ ] Function names follow `subsystem_verb_noun` pattern
- [ ] No dynamic allocation (`new`, `malloc`) — use slab allocator
- [ ] OOP class (if any) inherits from `ISigmaModule` or `ISigmaDriver`
- [ ] `extern "C"` wrapper provided for C-ABI compatibility

---

## 🚀 Contributing

1. Fork the repo
2. Create `suites/S<NN>_<Name>/sigma_<module>.h`
3. Add a `extern "C"` wrapper for CLI integration
4. Wire into `orchestrator/main.cpp` `TestCommand::run_subsystem_test()`
5. Open a PR — CI will automatically audit your module


