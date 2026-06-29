# SigmaOS Multi-Language Sovereign Architecture

## Overview

SigmaOS uses **multiple advanced low-level programming languages**, each chosen for the architectural properties of the subsystem it handles. All code is written without predefined standard libraries, without third-party components, and using Object-Oriented principles idiomatic to each language.

---

## Language Assignment

| Language | Role in SigmaOS | OOP Mechanism |
|----------|----------------|---------------|
| **Rust** | Kernel core, IPC, AI scheduler, PQC stubs, agent orchestration, containers | Structs + Traits (`impl`) |
| **Zig** | Bootloader, x86_64 HAL, paging, PS/2 input, framebuffer, audio drivers | Structs + methods, `comptime`, tagged unions |
| **Nim** | Config engine, generation manager, CLI, package manager | Objects + methods + generics |
| **Ada/SPARK** | Crypto (Kyber-1024, Dilithium-5), block storage drivers (AHCI, NVMe) | Tagged records + formal SPARK contracts |

---

## Module Map

### Rust (`no_std`, `no_builtins`)

| File | Replaces |
|------|----------|
| `klib/ai_scheduler.rs` | `core/ai_scheduler/ai_sched.cpp` |
| `klib/pqc.rs` | `crypto/SovereignKyber.cpp`, `SovereignDilithium5.cpp` |
| `init/watchdog.rs` | Self-healing watchdog |
| `agents/orchestration/agent_orchestrator.rs` | `AgentOrchestrator.cpp` |
| `agents/orchestration/sovereign_container.rs` | `SovereignContainer.cpp`, `SovereignContainerRuntime.cpp` |
| `agents/policy/governance_rules.rs` | `GovernanceRules.cpp`, `QuotaManager.cpp` |
| `bin/ipctrace/mod.rs` | `bin/ipctrace/main.cpp` |
| `bin/powerd/mod.rs` | `bin/powerd/main.cpp` |
| `drivers/core/hotplug_manager.rs` | `drivers/core/hotplug_manager.cpp` |
| `drivers/ddk/ddk.rs` | `drivers/ddk/ddk_api.h`, `ddk_stub.c` |
| `drivers/gpu/vulkan.rs` | `drivers/gpu/sigma_gpu_vulkan.cpp`, `sigma_graphics_drm.cpp`, `SovereignMesa.cpp`, `SovereignProton.cpp` |
| `drivers/linux_distros/compat.rs` | `drivers/linux_distros/ArchDriverCompat.cpp`, `DebianDriverCompat.cpp`, `FedoraDriverCompat.cpp`, `UbuntuDriverCompat.cpp` |

### Zig (bare-metal, no stdlib)

| File | Replaces |
|------|----------|
| `arch/x86_64/paging.zig` | `arch/x86_64/paging.c` |
| `hal/x86/hal_io.zig` | `hal/x86/` C HAL files |
| `drivers/input/ps2.zig` | `sigma_keyboard.cpp`, `sigma_mouse.cpp`, `sigma_ps2.cpp` |
| `drivers/display/framebuffer.zig` | `sigma_fb.cpp` |
| `drivers/audio/hda.zig` | `sigma_audio_hda.cpp` |
| `browser/sigma_wasm_bridge.zig` | `browser/sigma_wasm_bridge.c` |
| `drivers/display/vga.zig` | `drivers/display/sigma_vga.cpp`, `sigma_vga_driver.cpp` |


### Nim (compiles to native, no stdlib/libc)

| File | Replaces |
|------|----------|
| `config/core/declarative_engine.nim` | `config/core/DeclarativeEngine.cpp` |
| `config/core/generation_manager.nim` | `config/core/GenerationManager.cpp` |
| `config/cli_main.nim` | `config/cli_main.cpp` |
| `drivers/printing/cups.nim` | `drivers/printing/SovereignCUPS.cpp` |

### Ada/SPARK (formally verified, no runtime exceptions)

| File | Replaces |
|------|----------|
| `crypto/kyber.ads` + `kyber.adb` | `crypto/SovereignKyber.cpp` |
| `crypto/dilithium.ads` + `dilithium.adb` | `crypto/SovereignDilithium5.cpp` |
| `drivers/block/ahci.ads` + `ahci.adb` | `drivers/block/ahci_shard.cpp` |
| `drivers/block/nvme.ads` + `nvme.adb` | `drivers/block/nvme_shard.cpp` |
| `drivers/net/e1000.ads` + `e1000.adb` | `drivers/net/sigma_e1000.cpp` |
| `drivers/net/rtl8139.ads` + `rtl8139.adb` | `drivers/net/sigma_rtl8139.cpp` |

---

## OOP Principles Per Language

### Rust
```rust
// Struct as class, Trait as interface
pub trait SigmaObject { fn initialize(&mut self) -> i32; }
pub struct AgentOrchestrator { agents: [Option<Agent>; 64], count: usize }
impl AgentOrchestrator { pub fn register(&mut self, id: usize, priority: u8) -> bool { ... } }
```

### Zig
```zig
// Struct with methods as OOP class
pub const Framebuffer = struct {
    base: usize, width: u32, height: u32,
    pub fn put_pixel(self: *const Framebuffer, x: u32, y: u32, color: Color) void { ... }
};
```

### Nim
```nim
# Object type with procedures = OOP class
type GenerationManager* = object
  generations*: seq[GenerationInfo]
proc createGeneration*(mgr: var GenerationManager; label: string): int = ...
```

### Ada/SPARK
```ada
-- Tagged Record = OOP class, SPARK aspects = contracts
type AHCI_Port is tagged record
   State : Port_State := Idle;
end record;
procedure Read_Sector (Port: in out AHCI_Port; ...) with Pre => Port.State = Idle;
```

---

## Build System

The CI/CD pipeline (`sigma_ci.yml`) builds all four language families:

```yaml
- Rust:     cargo build --target x86_64-unknown-none
- Zig:      zig build-lib --target x86_64-freestanding-none
- Nim:      nim c --noMain --noLinking --cpu:amd64
- Ada/SPARK: gprbuild -P sigmaos.gpr
```

---

## 🔗 Related Pages

- [Phase 5: Ecosystem & Developer Tools](Phase-5-Ecosystem-And-Developer-Tools)
- [Phase 6: Long-Term Vision](Phase-6-Long-Term-Vision)
- [Zero-Dependency Architecture](Zero-Dependency-Architecture)
- [Roadmap](Roadmap)
