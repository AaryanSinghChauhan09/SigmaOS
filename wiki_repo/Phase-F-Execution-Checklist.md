# Phase F — Sovereign Type Migration & Subsystem Hardening

> **Status:** ✅ Complete  
> **Scope:** Bug-fix sweep across compiler errors, type system violations, and subsystem stubs.

---

## Fixes Applied

### 🔴 Critical: Type System Violations

| File | Problem | Fix |
|------|---------|-----|
| `drivers/usb/sigma_usb_hcd.cpp` | Used bare `u32`/`u64`/`u8` types — violated zero-dependency mandate | Migrated all types to `sigma_u32`/`sigma_u64`/`sigma_u8` from `sigma_kernel_types.h` |
| `include/sigma_audit.h` | Used `uint32_t`/`bool` from C stdlib | Replaced with `sigma_u32`/`sigma_bool`; updated all method signatures |
| `include/sigma_pqc.h` | Essentially empty — no types or function prototypes | Fully expanded with Kyber-1024 and Dilithium-5 key/signature structs and API |

### 🟠 High: Missing Declarations

| File | Problem | Fix |
|------|---------|-----|
| `include/sigma_error_codes.h` | Missing `K_ERR_NOMEM`, `K_ERR_NOTFOUND`, `K_ERR_PERM`, `K_ERR_BUSY`, `K_ERR_OVERFLOW`, `K_ERR_UNIMPL`, `sigma_status` | Added all 6 codes + `typedef int sigma_status` |
| `include/sigma_profiles.h` | Missing `sigma_hw_profile_t` bitmask used by hardware test suite | Added full 6-value bitmask enum |
| `kernel/tests/sigma_hw_test.cpp` | Wrong relative include path (`../include/` instead of `../../include/`) | Fixed to `../../include/`; added missing `sigma_profiles.h` + `sigma_error_codes.h` |

### 🟡 Medium: Logic/Syntax Bugs

| File | Problem | Fix |
|------|---------|-----|
| `net/tcp.c` | Missing `}` closing `if (flags & TCP_FLAG_SYN)` in `TCP_STATE_LISTEN` case — caused case fall-through and compiler parse error | Added missing closing brace before `break;` |
| `drivers/usb/sigma_usb_hcd.cpp` | Used `sigma_vga_printf` which is not part of sovereign libc | Replaced with `sys_print` from `sigma_libc.h` |

---

## New Capabilities Added

### `sigma_usb_hcd.cpp` — `sigma_usb_register_device()`
New function to register USB devices discovered during port-change events:
```cpp
int sigma_usb_register_device(sigma_u32 slot, sigma_u16 vid,
                               sigma_u16 pid, const char* desc);
```

### `sigma_pqc.h` — Full CRYSTALS API
Exposed complete post-quantum crypto surface:
- **Kyber-1024**: `sigma_kyber_keypair`, `sigma_kyber_encapsulate`, `sigma_kyber_decapsulate`
- **Dilithium-5**: `sigma_dilithium5_keypair`, `sigma_dilithium5_sign`, `sigma_dilithium5_verify`

### `sigma_profiles.h` — Hardware Platform Bitmask
```c
typedef enum {
    SIGMA_HW_PROFILE_STANDARD  = 0x01,
    SIGMA_HW_PROFILE_GAMING    = 0x02,
    SIGMA_HW_PROFILE_SERVER    = 0x04,
    SIGMA_HW_PROFILE_IOT_ARM64 = 0x08,
    SIGMA_HW_PROFILE_FORENSIC  = 0x10,
} sigma_hw_profile_t;
```

---

## Open Items (Phase G)

| ID | Subsystem | Status | Notes |
|----|-----------|--------|-------|
| #851 | Wi-Fi / Bluetooth | ⚠️ In Progress | xHCI transport ready; WLAN/BT protocol stacks needed |
| #521 | Recovery GUI | ⚠️ Planned | Serial rescue shell exists; Rescuezilla-style GUI pending |
| #512 | Zenith Compositor | ⚠️ Partial | Native C++ compositor exists; needs input integration tests |
| #522 | Auto-tiling | ⚠️ Partial | `sigma_tiling_wm.cpp` implemented; needs compositor wiring |

---

*See [CURRENT_PROBLEMS_MANIFEST.md](../CURRENT_PROBLEMS_MANIFEST.md) for the full live tracking list.*
