# Contributing to SigmaOS 🚀

Thank you for choosing to help shape the future of sovereign computing! SigmaOS is a zero-dependency operating system built for strict privacy, extreme resilience, and declarative configuration. 

We welcome contributions from kernel-level hardware engineers, UI developers, and security auditors.

---

## 🔒 Strict Quality Gateway

To preserve the zero-trust security paradigm of SigmaOS, all contributions must clear our quality verification guidelines:
1.  **Zero External Dependencies:** Core kernel, bootloader, and VFS logic must compile without importing standard GNU/Linux or external C headers.
2.  **No Direct Memory Management in Sandboxes:** Workstation and Zenith applications must rely strictly on our FFI sandbox memory interfaces rather than raw `malloc` sweeps.
3.  **Audit Logs Mandatory:** All dynamic subsystems must emit structured diagnostic updates matching the `sigma_error_codes.h` format.
4.  **Local Test Suite Pass:** Ensure your code compiles cleanly and passes:
    *   `make test` to execute allocator and scheduling mocks.
    *   No compiler warnings or undefined references under static audit gates.

---

## 🛠️ Hybrid C/Rust Development Workflow

SigmaOS utilizes a modern hybrid architecture:
*   **Kernel Core, VFS & Bootloader:** Authored in robust ISO C11 for direct low-level predictability.
*   **Zenith SDK, Diagnostic Utilities & GUI Workloads:** Incremental additions and libraries are encouraged in memory-safe **Rust** utilizing our `zenith-sdk` interop crate (`zenith_desktop/sdk/rust/`).

### Submitting a Sovereign Profile Spin
We natively support **Sovereign System Profiles** (standard, CAINE-inspired forensic, IoT, enterprise, and education) to tailer resource budgets:
*   To submit an adaptation spin, configure targeted capability filters directly inside `sigma_control_center.cpp` and update `include/sigma_profiles.h`.

---

## 🚀 Getting Started

1.  Clone our main repository and read our [Architecture Blueprint](docs/wiki/Home.md).
2.  Inspect outstanding tasks in the [Issue Tracker](https://github.com/AaryanSinghChauhan09/SigmaOS/issues).
3.  Open a Pull Request ensuring your commit description explicitly states the targeted subsystem and security mapping.
