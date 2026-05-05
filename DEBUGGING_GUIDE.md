# SigmaOS Debugging Guide

Welcome to the SigmaOS debugging guide! Since SigmaOS targets bare-metal execution with a unique 600-shard modular architecture, standard debugging requires a slightly tailored approach.

## 1. Using Sanitizers (ASAN & UBSAN)

Memory safety is a core priority. When compiling the kernel or individual shards for testing on a host OS (e.g., Linux/macOS), we enforce AddressSanitizer and UndefinedBehaviorSanitizer.

- [Enable via Make]: Run `make DEBUG=1 SANITIZE=1`. This appends `-fsanitize=address,undefined -g -O0` to the compilation flags.
- [Interpreting output]: If the kernel panics or crashes, ASAN will output a detailed memory trace to stdout/stderr.

## 2. Using the Shard Logger

Every shard should use the lightweight logging framework.

- `#include "sigma_log.h"` or `#include "sigma_hal.h"`
- Use `sigma_log("[MODULE] Message")` to trace execution paths.
- All logs are routed through the `SovereignLog.cpp` shard to the active telemetry bus.

## 3. Clang-Tidy & Static Analysis

We strictly enforce `clang-tidy` to catch C++ object lifecycle bugs and potential concurrency issues.

- You can run the analysis locally using the provided CI scripts: `python3 scripts/lattice_coverage.py`.
- Ensure you have the latest `.clangd` loaded in your IDE for real-time analysis.

## 4. Rust Interop Safety

If you are modifying low-level parsing or cryptography shards, prefer the Rust implementations mapped via `SovereignRustInterop.cpp`. Rust's borrow checker eliminates a large class of memory bugs by default.

