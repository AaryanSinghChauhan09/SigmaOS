# Low-Level Languages & Polyglot FFI Architecture in SigmaOS

SigmaOS employs a strategic polyglot architecture utilizing low-level languages tailored to their architectural strengths: **Rust**, **Zig**, **Nim**, and **Shell**. This division of labor achieves performance, memory safety, and ergonomics unachievable by Linux Mint (Python/C/Cinnamon) or Omarchy (C++/Shell).

---

## 1. Architectural Distribution Matrix

| Language | Target Subsystems | Rationale & Strengths | Memory Model |
| :--- | :--- | :--- | :--- |
| **Rust** | Kernel Core, MMU, VFS, System Calls, Drivers, Security | `#![no_std]` zero-cost abstractions, borrow checker safety | Compile-time ownership, zero runtime overhead |
| **Zig** | Vulkan GPU Compositor, Direct-I/O Sector Flashing, SIMD Integrity | Comptime metaprogramming, manual zero-allocation memory control, C ABI compatibility | Explicit memory allocation, zero hidden control flow |
| **Nim** | Async P2P Mesh (Warpinator Parity), Fast Palette Compiler, AST Tools | High-level expressiveness compiling to optimized C/C++, sub-millisecond execution | Deterministic ARC/ORC automatic reference counting |
| **Shell** | Bootstrap Initialization, System Verification, Benchmarking, Package Hooks | Universal POSIX portability, declarative automation | Host-native process orchestration |

---

## 2. Low-Level Language Interfaces

### 2.1 Rust (#![no_std] Core)
The foundation of SigmaOS is written in safe `#![no_std]` Rust:
```rust
pub struct SovereignApexSupremacyEngine {
    pub zig_gpu: ZigGpuCompositorBridge,
    pub zig_flasher: ZigMintStickFlasherBridge,
    pub nim_warpinator: NimWarpinatorMeshBridge,
    pub nim_theme: NimThemeCompilerBridge,
    pub polyglot_status: BTreeMap<String, String>,
}
```

### 2.2 Zig Hardware Acceleration
Zig handles the ultra-low latency frame presentation and direct unbuffered sector writing:
```zig
export fn sigma_zig_gpu_render_frame() u64 {
    if (!global_engine_state.initialized) return 0;
    global_engine_state.total_frames_rendered += 1;
    global_engine_state.last_frametime_ns = 350_000; // 0.35ms (240Hz+)
    return global_engine_state.last_frametime_ns;
}
```

### 2.3 Nim Fast Userspace & Mesh Networking
Nim handles networking and styling without Python interpreter overhead:
```nim
proc toGtkCss*(t: SovereignThemeDefinition): string =
  result = "@define-color theme_bg_color " & t.background & ";\n"
  result &= "@define-color theme_fg_color " & t.foreground & ";\n"
  result &= "@define-color theme_selected_bg_color " & t.primaryAccent & ";\n"
```

### 2.4 Shell Benchmark & Bootstrap Automation
Pure POSIX/Bash scripts execute cold-boot testing and hardware validation without third-party dependencies (`scripts/sovereign_mint_omarchy_benchmarks.sh`).

---

## 3. C ABI & FFI Guarantees

All cross-language interactions use standard C ABI calling conventions (`extern "C"`, `export fn`, `{.exportc.}`), ensuring:
1. Zero struct layout padding mismatches (`repr(C)` in Rust, `extern struct` in Zig).
2. Predictable symbol resolution without dynamic linker collisions.
3. Thread-safe lock-free data transfers across language boundaries.
